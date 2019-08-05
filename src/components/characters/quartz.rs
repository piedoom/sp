use super::Character;
use crate::resources::{MaterialResource, PrimitiveResource};
use amethyst::{
    assets::{AssetStorage, Handle, PrefabData},
    derive::PrefabData,
    ecs::{prelude::*, Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
    renderer::{palette::rgb::LinSrgba, Material, Mesh},
};

use serde::{Deserialize, Serialize};
use specs_physics::{
    colliders::Shape, nphysics::object::BodyStatus, PhysicsBody, PhysicsBodyBuilder,
    PhysicsCollider, PhysicsColliderBuilder,
};

#[derive(Clone, Default, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
pub struct Quartz;

impl Character for Quartz {
    fn attack(&mut self, world: &mut World) -> Entity {
        type SystemData<'a> = (
            Entities<'a>,
            WriteStorage<'a, PhysicsBody<f32>>,
            WriteStorage<'a, PhysicsCollider<f32>>,
            WriteStorage<'a, Handle<Mesh>>,
            WriteStorage<'a, Handle<Material>>,
            ReadExpect<'a, PrimitiveResource>,
        );
        println!("firing");
        // let materials_resource = world.read_resource::<MaterialResource>();
        //let mat = materials_resource.generate_material(world, LinSrgba::new(0.5, 0.0, 1.0, 1.0)).clone();
        world.exec(
            |(entities, mut bodies, mut colliders, mut meshes, mut materials, primitives): SystemData| {
                entities
                    .build_entity()
                    .with(
                        PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic).build(),
                        &mut bodies,
                    )
                    .with(
                        PhysicsColliderBuilder::<f32>::from(Shape::<f32>::Ball { radius: 1f32 })
                            .build(),
                        &mut colliders,
                    )
                    .with(primitives.sphere(), &mut meshes)
                   // .with(mat.clone(), &mut materials)
                    .build()
            },
        )
    }
}

impl Component for Quartz {
    type Storage = DenseVecStorage<Self>;
}
