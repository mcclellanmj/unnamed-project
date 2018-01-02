use na;

use specs::{Component, System, VecStorage, Fetch, ReadStorage, WriteStorage, World, DispatcherBuilder};
use ecs::DeltaTime;

#[derive(Debug)]
pub struct Position(pub na::Vector2<f32>);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity(pub na::Vector2<f32>);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Rotate(pub f32);

impl Component for Rotate {
    type Storage = VecStorage<Self>;
}

pub struct Move;
impl<'a> System<'a> for Move {
    type SystemData = (
        Fetch<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta, vel, mut pos): Self::SystemData) {
        use specs::Join;

        let DeltaTime(delta) = *delta;

        for (&Velocity(vel), &mut Position(mut pos)) in (&vel, &mut pos).join() {
            pos += vel * delta;
        }
    }
}