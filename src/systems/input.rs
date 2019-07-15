use crate::components::{Thruster, Player};
use crate::util::{GameBindings, Axis};
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::BindingTypes;
use amethyst::input::InputHandler;

#[derive(Default, Debug)]
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Thruster>,
        Read<'a, InputHandler<GameBindings>>,
    );

    fn run(&mut self, (mut players, mut thrusters, input): Self::SystemData) {
        // Loop through all players and assign direction
        for (thruster, _) in (&mut thrusters, &mut players).join() {
            thruster.rotation_control = input.axis_value(&Axis::Horizontal).unwrap();
            thruster.thrust_control = input.axis_value(&Axis::Vertical).unwrap();
        }

        // loop through all weapons systems and assign firing states
        // for (_, manager) in (&mut players, &mut managers).join() {
        //     manager.wants_to_fire = input
        //         .action_is_down(&Action::Fire)
        //         .expect("Error reading action");
        // }
    }
}
