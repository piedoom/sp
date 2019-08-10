//! Primitive materials for shading development objects
use amethyst::ecs::World;
use amethyst::{
    assets::{AssetLoaderSystemData, Handle, ProgressCounter},
    renderer::{
        mtl::Material, palette::rgb::LinSrgba, rendy::texture::palette::load_from_linear_rgba,
        types::Texture, MaterialDefaults,
    },
};

use super::GameResource;

#[derive(Default)]
pub struct MaterialResource {
    default_material: Option<Material>,
    pub diffuse_white: Option<Handle<Material>>,
}

impl GameResource for MaterialResource {
    fn initialize(world: &mut World, progress_counter: &mut ProgressCounter) {
        let mut mr = Self::default();
        // Get the resource for the material defaults
        mr.default_material = Some(world.read_resource::<MaterialDefaults>().0.clone());
        // Set the white material
        mr.diffuse_white = Some(mr.generate_material(world, LinSrgba::new(1.0, 1.0, 1.0, 1.0)));
        world.add_resource::<MaterialResource>(mr);
    }
}

impl MaterialResource {
    pub fn generate_material(&self, world: &mut World, rgba: LinSrgba) -> Handle<Material> {
        let albedo = self.generate_albedo(world, rgba);
        // Create material
        world.exec(
            |(mtl_loader, tex_loader): (
                AssetLoaderSystemData<'_, Material>,
                AssetLoaderSystemData<'_, Texture>,
            )| {
                // Create a material with that albedo
                mtl_loader.load_from_data(
                    Material {
                        albedo: albedo.clone(),
                        ..self.default_material.clone().unwrap()
                    },
                    (),
                )
            },
        )
    }

    fn generate_albedo(&self, world: &mut World, rgba: LinSrgba) -> Handle<Texture> {
        world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data(load_from_linear_rgba(rgba).into(), ())
        })
    }
}
