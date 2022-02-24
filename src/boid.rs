use nalgebra as na;
use nalgebra::Vector2;
use raylib::prelude::*;

use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Copy, Clone)]
pub struct Boid {
    pub position: [f64; 2],
    pub vector: Vector2<f64>,
    pub view_distance: i32,
    pub colour: Color,
    pub num: u8,
    pub speed: f64,
    pub align_strength: f64,
    pub sepa_strength: f64,
    pub cohes_strength: f64,
}

impl Boid {
    pub fn update(&mut self, boids: &Vec<Boid>, window: [i32; 2], dt: f32, log: bool) {
        let vector = self.accumulate_forces(boids, window);
        self.vector = vector;
        let x_comp = vector[0];
        let y_comp = vector[1];
        if log && (self.num == 0 || self.num == 1) {
            let mut name: String = "log".to_owned();
            let num = self.num as i32;
            name = name + &num.to_string();
            name = name + ".txt";
            if !(Path::new(&name).exists()) {
                let mut f = File::create(&name).expect("unable to create file");
                println!("Created new file");
                write!(
                    f,
                    "x: {}, {}, {}, {}",
                    (self.speed * x_comp * dt as f64),
                    self.speed,
                    x_comp,
                    dt
                )
                .expect("Access is Denied.");
                write!(
                    f,
                    "\ty: {}, {}, {}, {}",
                    (self.speed * x_comp * dt as f64),
                    self.speed,
                    y_comp,
                    dt
                )
                .expect("Access is Denied.");
                write!(f, "\n").expect("Access is Denied.");
            } else {
                let mut f = OpenOptions::new()
                    .write(true)
                    .read(true)
                    .open(&name)
                    .expect("Unable to Open File");
                let tmp = read_to_string(&name).expect("Access is Denied");
                write!(f, "{}", tmp).expect("Access is Denied");
                write!(
                    f,
                    "x: {}, {}, \t{}, {}",
                    (self.speed * x_comp * dt as f64),
                    self.speed,
                    x_comp,
                    dt
                )
                .expect("Access is Denied.");
                write!(
                    f,
                    "\ty: {}, {}, \t{}, {}",
                    (self.speed * x_comp * dt as f64),
                    self.speed,
                    y_comp,
                    dt
                )
                .expect("Access is Denied.");
                write!(f, "\n").expect("Access is Denied.");
            }
        }
        self.position[0] =
            (self.position[0] + (self.speed * x_comp * dt as f64)) % window[0] as f64;
        self.position[1] =
            (self.position[1] + (self.speed * y_comp * dt as f64)) % window[1] as f64;
    }

    pub fn accumulate_forces(&self, boids: &Vec<Boid>, window: [i32; 2]) -> Vector2<f64> {
        let align = self.alignment(boids);
        let cohes = self.cohesion(boids);
        let sepa = self.separation(boids);
        let awall = self.avoid_walls(window);
        let mut out = self.vector + align + cohes + sepa + awall;
        out = na::base::Matrix::normalize(&out);
        out
    }
    pub fn render(&self, d: &mut RaylibDrawHandle, debug: bool) {
        d.draw_fps(10, 10);
        if debug {
            d.draw_line(
                self.position[0] as i32,
                self.position[1] as i32,
                (self.position[0] + (self.speed * self.vector[0])) as i32,
                (self.position[1] + (self.speed * self.vector[1])) as i32,
                Color::GREEN,
            );
            if self.num == 0 {
                d.draw_circle_lines(
                    self.position[0] as i32,
                    self.position[1] as i32,
                    self.view_distance as f32,
                    Color::GREEN,
                )
            }
        }
        //d.draw_rectangle_pro(
        //    ffi::Rectangle {
        //        x: (self.position[0] - 25.0) as f32,
        //        y: (self.position[1] - 2.5) as f32,
        //        width: 50.0,
        //        height: 5.0,
        //    },
        //    ffi::Vector2 {
        //        x: self.position[0] as f32,
        //        y: self.position[1] as f32,
        //    },
        //    60.0,
        //    self.colour,
        //);
        d.draw_circle(
            self.position[0] as i32,
            self.position[1] as i32,
            5.0,
            self.colour,
        );
    }
    pub fn alignment(&self, boids: &Vec<Boid>) -> Vector2<f64> {
        let mut neighbour_count = 0.0;
        let mut out: Vector2<f64> = Vector2::new(0.0, 0.0);
        for b in boids {
            if b.num != self.num {
                if self.distance_from(b) <= self.view_distance as f64 {
                    out[0] += b.vector[0];
                    out[1] += b.vector[1];
                    neighbour_count += 1.0;
                }
            }
        }
        if neighbour_count > 0.0 {
            out[0] /= neighbour_count;
            out[1] /= neighbour_count;
            return na::base::Matrix::normalize(&out);
        } else {
            return Vector2::new(0.0, 0.0);
        }
    }
    pub fn cohesion(&self, boids: &Vec<Boid>) -> Vector2<f64> {
        let mut neighbour_count = 0.0;
        let mut out = [0.0, 0.0];
        for b in boids {
            if b.num != self.num {
                if self.distance_from(b) <= self.view_distance as f64 {
                    out[0] += b.position[0];
                    out[1] += b.position[1];
                    neighbour_count += 1.0;
                }
            }
        }
        if neighbour_count > 0.0 {
            out[0] /= neighbour_count;
            out[1] /= neighbour_count;
            return na::base::Matrix::normalize(&Vector2::new(
                out[0] - self.vector[0],
                out[1] - self.vector[1],
            ));
        } else {
            return Vector2::new(0.0, 0.0);
        }
    }
    pub fn separation(&self, boids: &Vec<Boid>) -> Vector2<f64> {
        let mut neighbour_count = 0.0;
        let mut out = Vector2::new(0.0, 0.0);
        for b in boids {
            if b.num != self.num {
                if self.distance_from(b) <= self.view_distance as f64 {
                    neighbour_count += 1.0;
                    out[0] += b.vector[0] - self.vector[0];
                    out[1] += b.vector[1] - self.vector[1];
                }
            }
        }
        out[0] *= -1.0;
        out[1] *= -1.0;
        if neighbour_count > 0.0 {
            return na::base::Matrix::normalize(&out);
        } else {
            return out;
        }
    }
    pub fn avoid_walls(&self, window: [i32; 2]) -> Vector2<f64> {
        let center = [(window[0] / 2) as f64, (window[1] / 2) as f64];
        let x = -(self.position[0] - center[0]);
        let y = -(self.position[1] - center[1]);
        return na::base::Matrix::normalize(&Vector2::new(x, y));
    }
    pub fn distance_from(&self, other: &Boid) -> f64 {
        let out = ((self.position[0] - other.position[0]).powi(2)
            + (self.position[1] - other.position[1]).powi(2))
        .sqrt();
        return out;
    }
}
