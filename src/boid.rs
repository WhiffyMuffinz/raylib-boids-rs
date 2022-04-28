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
    pub size: f32,
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
        let align = self.alignment(boids) * self.align_strength;
        let cohes = self.cohesion(boids) * self.cohes_strength;
        let sepa = self.separation(boids) * self.sepa_strength;
        let awall = self.avoid_walls(window);
        let mut out = self.vector + align + cohes + sepa; //+ awall;
        out = na::base::Matrix::normalize(&out);
        out
    }

    fn get_triangle_points(&self) -> [raylib::core::math::Vector2; 3] {
        //returns an array of three vectors representing the points of a triangle
        //takes into acount the direction the velocity vector is heading, and points the triangle in that direction
        let mut angle = (self.vector[1] / self.vector[0]).atan() as f32;
        if self.vector[0] < 0.0 {
            angle += std::f32::consts::PI;
        }

        let point_1 = raylib::core::math::Vector2::new(
            // point along the velocity vector
            self.position[0] as f32 + (self.size + self.size * 0.1) * angle.cos(),
            self.position[1] as f32 + (self.size + self.size * 0.1) * angle.sin(),
        );
        let point_2 = raylib::core::math::Vector2::new(
            // point at the right of the velocity vector
            self.position[0] as f32 + self.size * (angle + 2.0943951023931953).cos(),
            self.position[1] as f32 + self.size * (angle + 2.0943951023931953).sin(),
        );
        let point_3 = raylib::core::math::Vector2::new(
            // point at the left of the velocity vector
            self.position[0] as f32 + self.size * (angle - 2.0943951023931953).cos(),
            self.position[1] as f32 + self.size * (angle - 2.0943951023931953).sin(),
        );
        let points = [point_1, point_2, point_3];
        // println!("{:?}, {:?}", self.position, points);
        points
    }
    pub fn render(&self, d: &mut RaylibDrawHandle, debug: bool) {
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
        let points = self.get_triangle_points();
        d.draw_triangle_lines(points[0], points[1], points[2], self.colour);
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
