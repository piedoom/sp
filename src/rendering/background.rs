#![allow(clippy::default_trait_access)]
#![allow(unused_imports, unused_variables)]

use amethyst_core::{
    ecs::{Join, Read, ReadExpect, ReadStorage, Resources, SystemData},
    math::{self, convert, Matrix4, Point3, Vector3, Vector4},
    transform::Transform,
    Hidden,
};

use amethyst_assets::AssetStorage;
use amethyst_rendy::{
    batch::{GroupIterator, OneLevelBatch, OrderedOneLevelBatch},
    camera::Camera,
    pipeline::{PipelineDescBuilder, PipelinesBuilder},
    pod::{IntoPod, SpriteArgs},
    rendy::{
        command::{QueueId, RenderPassEncoder},
        factory::Factory,
        graph::{
            render::{PrepareResult, RenderGroup, RenderGroupDesc},
            GraphContext, NodeBuffer, NodeImage,
        },
        hal::{
            self,
            device::Device,
            pso::{self, ShaderStageFlags},
        },
        mesh::AsVertex,
        shader::{
            Shader, ShaderKind, ShaderSetBuilder, SourceLanguage, SourceShaderInfo,
            SpirvReflection, SpirvShader,
        },
    },
    sprite::{SpriteRender, SpriteSheet},
    sprite_visibility::SpriteVisibility,
    submodules::{
        gather::CameraGatherer, DynamicVertexBuffer, FlatEnvironmentSub, TextureId, TextureSub,
    },
    types::{Backend, Texture},
    util,
};
use derivative::Derivative;
use std::marker::PhantomData;

/// Draw the background
#[derive(Clone, PartialEq, Derivative)]
#[derivative(Default(bound = ""), Debug(bound = ""))]
pub struct DrawBackgroundDesc<T: Tile, E: CoordinateEncoder> {
    #[derivative(Debug = "ignore")]
    _marker: PhantomData<(T, E)>,
}

impl<T: Tile, E: CoordinateEncoder> DrawBackgroundDesc<T, E> {
    /// Create instance of `DrawBackground` render group
    pub fn new() -> Self {
        Self::default()
    }
}

impl<B: Backend, T: Tile, E: CoordinateEncoder> RenderGroupDesc<B, Resources>
    for DrawBackgroundDesc<T, E>
{
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        _aux: &Resources,
        framebuffer_width: u32,
        framebuffer_height: u32,
        subpass: hal::pass::Subpass<'_, B>,
        _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, Resources>>, failure::Error> {
        #[cfg(feature = "profiler")]
        profile_scope!("build");

        let env = FlatEnvironmentSub::new(factory)?;
        let textures = TextureSub::new(factory)?;
        let vertex = DynamicVertexBuffer::new();

        let (pipeline, pipeline_layout) = build_tiles_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            false,
            vec![env.raw_layout(), textures.raw_layout()],
        )?;

        Ok(Box::new(DrawBackground::<B, T, E> {
            pipeline,
            pipeline_layout,
            env,
            vertex,
            _marker: PhantomData::default(),
        }))
    }
}

#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct DrawBackground<B: Backend, T: Tile, E: CoordinateEncoder> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    env: FlatEnvironmentSub<B>,
    vertex: DynamicVertexBuffer<B, SpriteArgs>,
    #[derivative(Debug = "ignore")]
    _marker: PhantomData<(T, E)>,
}

impl<B: Backend, T: Tile, E: CoordinateEncoder> RenderGroup<B, Resources> for DrawBackground<B, T, E> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        resources: &Resources,
    ) -> PrepareResult {
        #[cfg(feature = "profiler")]
        profile_scope!("prepare");

        self.env.process(factory, index, resources);
        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<'_, B>,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        _resources: &Resources,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("draw");

        let layout = &self.pipeline_layout;
        encoder.bind_graphics_pipeline(&self.pipeline);
        self.env.bind(index, layout, 0, &mut encoder);
        self.vertex.bind(index, 0, 0, &mut encoder);
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &Resources) {
        unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline);
            factory
                .device()
                .destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}

fn build_tiles_pipeline<B: Backend>(
    factory: &Factory<B>,
    subpass: hal::pass::Subpass<'_, B>,
    framebuffer_width: u32,
    framebuffer_height: u32,
    transparent: bool,
    layouts: Vec<&B::DescriptorSetLayout>,
) -> Result<(B::GraphicsPipeline, B::PipelineLayout), failure::Error> {
    let pipeline_layout = unsafe {
        factory
            .device()
            .create_pipeline_layout(layouts, None as Option<(_, _)>)
    }?;

    let shader_vertex = unsafe { TILES_VERTEX.module(factory).unwrap() };
    let shader_fragment = unsafe { TILES_FRAGMENT.module(factory).unwrap() };

    let pipes = PipelinesBuilder::new()
        .with_pipeline(
            PipelineDescBuilder::new()
                .with_vertex_desc(&[(SpriteArgs::vertex(), pso::VertexInputRate::Instance(1))])
                .with_input_assembler(pso::InputAssemblerDesc::new(hal::Primitive::TriangleStrip))
                .with_shaders(util::simple_shader_set(
                    &shader_vertex,
                    Some(&shader_fragment),
                ))
                .with_layout(&pipeline_layout)
                .with_subpass(subpass)
                .with_framebuffer_size(framebuffer_width, framebuffer_height)
                .with_blend_targets(vec![pso::ColorBlendDesc(
                    pso::ColorMask::ALL,
                    if transparent {
                        pso::BlendState::ALPHA
                    } else {
                        pso::BlendState::Off
                    },
                )])
                .with_depth_test(pso::DepthTest::On {
                    fun: pso::Comparison::Less,
                    write: !transparent,
                }),
        )
        .build(factory, None);

    unsafe {
        factory.destroy_shader_module(shader_vertex);
        factory.destroy_shader_module(shader_fragment);
    }

    match pipes {
        Err(e) => {
            unsafe {
                factory.device().destroy_pipeline_layout(pipeline_layout);
            }
            Err(e)
        }
        Ok(mut pipes) => Ok((pipes.remove(0), pipeline_layout)),
    }
}
