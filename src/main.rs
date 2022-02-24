use alea::f64_less_than;
use alea::i32_in_range;

use nalgebra::Vector2;
use raylib::prelude::*;

use rayon::prelude::*;

mod boid;

use crate::boid::Boid;

const BOID_COUNT: u16 = 5;
const BOID_VIEW_DISTANCE: i32 = 120;
const WINDOW_DIMENSIONS: [i32; 2] = [1280, 720];
const BOID_SPEED: f64 = 50.0;
//const GRID_DIMENSIONS: [i8; 2] = [5, 5];
const SEPARATION_PROPORTION: f64 = 0.5;
const COHESION_PROPORTION: f64 = 0.5;
const ALIGNMENT_PROPORTION: f64 = 0.5;
const DEBUG: bool = true;
const LOG: bool = false;

fn create_boids(number: u16) -> Vec<Boid> {
    let mut out: Vec<Boid> = Vec::new();
    for i in 0..number {
        out.push(Boid {
            colour: Color::new(
                i32_in_range(128, 255) as u8,
                i32_in_range(128, 255) as u8,
                i32_in_range(128, 255) as u8,
                255,
            ),
            position: [
                f64_less_than(WINDOW_DIMENSIONS[0] as f64),
                f64_less_than(WINDOW_DIMENSIONS[1] as f64),
            ],
            vector: Vector2::new(f64_less_than(1.0), f64_less_than(1.0)),
            view_distance: BOID_VIEW_DISTANCE,
            num: i as u8,
            speed: BOID_SPEED,
            align_strength: ALIGNMENT_PROPORTION,
            cohes_strength: COHESION_PROPORTION,
            sepa_strength: SEPARATION_PROPORTION,
        });
    }
    out
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_DIMENSIONS[0], WINDOW_DIMENSIONS[1])
        .title("Boids for Bakas")
        .build();
    let mut boids = create_boids(BOID_COUNT);
    let mut boids2 = boids.clone();

    let mut n = 0;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        for b in &mut boids {
            b.update(&boids2, WINDOW_DIMENSIONS, dt, LOG);
            if LOG && n < 1 {
                println!("{}, {}", b.position[0], b.position[1]);
            }
        }
        boids2 = boids.clone();

        //for i in 0..boids.len() {
        //    if n % 2 == 0 {
        //        boids[i].update(&boids2, WINDOW_DIMENSIONS, dt, LOG);
        //    } else {
        //        boids2[i].update(&boids, WINDOW_DIMENSIONS, dt, LOG);
        //    }
        //}
        //let i = boids.par_iter();
        //for b in i {
        //    b.update(&boids2, WINDOW_DIMENSIONS, dt, DEBUG);
        //}

        //boids2 = boids.clone();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(0, 0, 0, 255));
        for i in 0..boids.len() {
            if n % 2 == 100 {
                boids2[i].render(&mut d, DEBUG);
            } else {
                boids[i].render(&mut d, DEBUG);
            }
        }
        n += 1;
    }
}
