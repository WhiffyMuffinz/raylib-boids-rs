use alea::f64_less_than;
use alea::i32_less_than;

use nalgebra::Vector2;
use raylib::prelude::*;

mod boid;

use crate::boid::Boid;

const BOID_COUNT: u16 = 128;
const BOID_VIEW_DISTANCE: i32 = 12;
const WINDOW_DIMENSIONS: [i32; 2] = [1280, 720];
const BOID_SPEED: f64 = 20.0;
//const GRID_DIMENSIONS: [i8; 2] = [5, 5];
//const DEBUG: bool = false;

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
            speed: BOID_SPEED,
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
    let mut boids2 = boids.clone();

    let mut n = 0;
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        for i in 0..boids.len() {
            if n % 2 == 0 {
                boids[i].update(&boids2, WINDOW_DIMENSIONS, dt);
            } else {
                boids2[i].update(&boids, WINDOW_DIMENSIONS, dt);
            }
        }
        //for b in &mut boids {
        //    b.update(&boids2, WINDOW_DIMENSIONS, dt);
        //    if DEBUG && n < 1 {
        //        println!("{}, {}", b.position[0], b.position[1]);
        //    }
        //}

        //boids2 = boids.clone();
        println!("1: {}, {}", boids[1].position[0], boids[1].position[1]);
        println!("2: {}, {}", boids2[1].position[0], boids2[1].position[1]);

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::new(127, 127, 127, 255));
        for i in 0..boids.len() {
            if n % 2 == 0 {
                boids[i].render(&mut d);
            } else {
                boids2[i].render(&mut d);
            }
        }
        n += 1;
    }
}

//fn main2() {
//    let mut b: Boid = Boid {
//        colour: Color::BLACK,
//        num: 0,
//        position: [0.0, 0.0],
//        vector: Vector2::new(1.0, 1.0),
//        view_distance: BOID_VIEW_DISTANCE,
//    };
//    let mut b2: Boid = Boid {
//        colour: Color::BLACK,
//        num: 1,
//        position: [1.0, 1.0],
//        vector: Vector2::new(-1.0, -1.0),
//        view_distance: BOID_VIEW_DISTANCE,
//    };
//    println!(
//        "1: p:{},{} v:{},{}",
//        b.position[0], b.position[1], b.vector[0], b.vector[1]
//    );
//    println!(
//        "2: p:{},{} v:{},{}",
//        b2.position[0], b2.position[1], b2.vector[0], b2.vector[1]
//    );
//}
