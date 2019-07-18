use amethyst::{
    ecs::{ReadExpect, Resources, SystemData},
    renderer::{
        palette::Srgb,
        pass::{
            DrawDebugLinesDesc, DrawPbrDesc,
            DrawPbrTransparentDesc, DrawSkyboxDesc,
        },
        rendy::{
            factory::Factory,
            graph::{
                present::PresentNode,
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{
                command::{ClearDepthStencil, ClearValue},
                format::Format,
                image,
            },
        },
        system::GraphCreator,
        types::Backend,
    },
    window::{ScreenDimensions, Window},
};

#[derive(Default)]
pub struct RenderGraph {
    dimensions: Option<ScreenDimensions>,
    surface_format: Option<Format>,
    dirty: bool,
}

impl<B: Backend> GraphCreator<B> for RenderGraph {
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    fn builder(&mut self, factory: &mut Factory<B>, res: &Resources) -> GraphBuilder<B, Resources> {
        self.dirty = false;

        let window = <(ReadExpect<'_, Window>)>::fetch(res);

        let surface = factory.create_surface(&window);

        // cache surface format to speed things up
        let surface_format = *self
            .surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));

        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = image::Kind::D2(
            dbg!(dimensions.width()) as u32,
            dimensions.height() as u32,
            1,
            1,
        );

        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.34, 0.36, 0.52, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let mut opaque_subpass = SubpassBuilder::new();
        let mut transparent_subpass = SubpassBuilder::new();

        opaque_subpass.add_group(DrawPbrDesc::skinned().builder());
        transparent_subpass.add_group(DrawPbrTransparentDesc::skinned().builder());

        let opaque = graph_builder.add_node(
            opaque_subpass
                .with_group(DrawDebugLinesDesc::new().builder())
                .with_group(
                    DrawSkyboxDesc::with_colors(
                        Srgb::new(0.82, 0.51, 0.50),
                        Srgb::new(0.18, 0.11, 0.85),
                    )
                    .builder(),
                )
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let transparent = graph_builder.add_node(
            transparent_subpass
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );

        let _present = graph_builder.add_node(
            PresentNode::builder(factory, surface, color)
                .with_dependency(opaque)
                .with_dependency(transparent),
        );

        graph_builder
    }
}