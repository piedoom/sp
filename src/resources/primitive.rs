//! Create a mesh from the `Shape` enum

use std::collections::HashMap;
use std::fs::read_dir;

use crate::components::CharacterPrefab;
use amethyst::{
    assets::{AssetLoaderSystemData, Handle, ProgressCounter},
    ecs::World,
    renderer::{
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        types::Mesh,
    },
    utils::application_root_dir,
};

use super::GameResource;

use amethyst::renderer::shape::Shape;

#[derive(Default)]
pub struct PrimitiveResource {
    sphere: Option<Handle<Mesh>>,
    cone: Option<Handle<Mesh>>,
    cube: Option<Handle<Mesh>>,
    cylinder: Option<Handle<Mesh>>,
    torus: Option<Handle<Mesh>>,
    ico_sphere: Option<Handle<Mesh>>,
    plane: Option<Handle<Mesh>>,
    circle: Option<Handle<Mesh>>,
}

impl PrimitiveResource {
    pub fn sphere(&self) -> Handle<Mesh> {
        self.sphere.clone().unwrap()
    }
    pub fn cone(&self) -> Handle<Mesh> {
        self.cone.clone().unwrap()
    }
    pub fn cube(&self) -> Handle<Mesh> {
        self.cube.clone().unwrap()
    }
    pub fn cylinder(&self) -> Handle<Mesh> {
        self.cylinder.clone().unwrap()
    }
    pub fn torus(&self) -> Handle<Mesh> {
        self.torus.clone().unwrap()
    }
    pub fn ico_sphere(&self) -> Handle<Mesh> {
        self.ico_sphere.clone().unwrap()
    }
    pub fn plane(&self) -> Handle<Mesh> {
        self.plane.clone().unwrap()
    }
    pub fn circle(&self) -> Handle<Mesh> {
        self.circle.clone().unwrap()
    }
}

impl GameResource for PrimitiveResource {
    fn initialize(world: &mut World, progress_counter: &mut ProgressCounter) {
        let mut p = Self::default();
        world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            p.sphere = Some(
                loader.load_from_data(
                    Shape::Sphere(16, 16)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    &mut *progress_counter,
                ),
            );
            p.cone = Some(
                loader.load_from_data(
                    Shape::Cone(16)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    &mut *progress_counter,
                ),
            );
            p.cylinder = Some(
                loader.load_from_data(
                    Shape::Cylinder(16, None)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    &mut *progress_counter,
                ),
            );
            p.torus = Some(
                loader.load_from_data(
                    Shape::Torus(0.3, 1.0, 16, 16)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    &mut *progress_counter,
                ),
            );
            p.ico_sphere = Some(
                loader.load_from_data(
                    Shape::IcoSphere(None)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    &mut *progress_counter,
                ),
            );
            p.plane = Some(
                loader.load_from_data(
                    Shape::Plane(None)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    &mut *progress_counter,
                ),
            );
            p.circle = Some(
                loader.load_from_data(
                    Shape::Circle(16)
                        .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                        .into(),
                    &mut *progress_counter,
                ),
            );
        });
        world.add_resource(p);
    }
}
