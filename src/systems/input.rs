use crate::components::{characters::Character, Player, Thruster};
use crate::util::{Axis, GameBindings};
use amethyst::core::Time;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::InputHandler;

#[derive(Default, Debug)]
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Thruster>,
        WriteStorage<'a, Character>,
        Read<'a, InputHandler<GameBindings>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut players, mut thrusters, mut characters, input, time): Self::SystemData) {
        // Loop through all players and assign direction
        for (thruster, _) in (&mut thrusters, &mut players).join() {
            thruster.rotation_control = input.axis_value(&Axis::Horizontal).unwrap();
            thruster.thrust_control = input.axis_value(&Axis::Vertical).unwrap();
        }

        // Assign a firing state to any player that is attached to a character
        for (character, _) in (&mut characters, &mut players).join() {
            character.fire()
        }
    }
}
