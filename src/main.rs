use alea::f64_less_than;
use alea::i32_less_than;

use nalgebra::Vector2;
use raylib::prelude::*;

mod boid;

use crate::boid::Boid;

const BOID_COUNT: u16 = 128;
const BOID_VIEW_DISTANCE: i32 = 12;
//const GRID_DIMENSIONS: [i8; 2] = [5, 5];
const WINDOW_DIMENSIONS: [i32; 2] = [1280, 720];

fn create_boids(number: u16) -> Vec<Boid> {
    let mut out: Vec<Boid> = Vec::new();
    for i in 0..number {
        out.push(Boid {
            colour: Color::new(
                i32_less_than(255) as u8,
                i32_less_than(255) as u8,
                i32_less_than(255) as u8,
                i32_less_than(255) as u8,
            ),
            position: [
                f64_less_than(WINDOW_DIMENSIONS[0] as f64),
                f64_less_than(WINDOW_DIMENSIONS[1] as f64),
            ],
            vector: Vector2::new(f64_less_than(1.0), f64_less_than(1.0)),
            view_distance: BOID_VIEW_DISTANCE,
            num: i as u8,
        });
    }
    out
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_DIMENSIONS[0], WINDOW_DIMENSIONS[1])
        .title("hello world")
        .build();
    let mut boids = create_boids(BOID_COUNT);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(127, 127, 127, 255));
        for b in &boids {
            b.render(&mut d);
        }
    }
}
