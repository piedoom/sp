use crate::components::*;
use crate::util::{Action, Axis, GameBindings};
use amethyst::core::Time;
use amethyst::ecs::prelude::*;
use amethyst::input::InputHandler;

#[derive(Default, Debug)]
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Thruster>,
        ReadStorage<'a, Character>,
        WriteStorage<'a, CharacterState>,
        Read<'a, InputHandler<GameBindings>>,
        Read<'a, Time>,
    );

    fn run(
        &mut self,
        (mut players, mut thrusters, characters, mut states, input, time): Self::SystemData,
    ) {
        // Loop through all players and assign direction
        for (thruster, _) in (&mut thrusters, &mut players).join() {
            thruster.rotation_control = input.axis_value(&Axis::Horizontal).unwrap();
            thruster.thrust_control = input.axis_value(&Axis::Vertical).unwrap();
        }

        // Assign a firing state to any player that is attached to a character
        for (state, _, _) in (&mut states, &characters, &mut players).join() {
            state.attack = input
                .action_is_down(&Action::Fire)
                .expect("Error reading action");
        }
    }
}
