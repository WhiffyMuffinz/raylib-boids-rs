use nalgebra as na;
use nalgebra::Vector2;
use raylib::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Boid {
    pub position: [f64; 2],
    pub vector: Vector2<f64>,
    pub view_distance: i32,
    pub colour: Color,
    pub num: u8,
    pub speed: f64,
}

impl Boid {
    pub fn update(&mut self, boids: &Vec<Boid>, window: [i32; 2], dt: f32) {
        let vector = self.accumulate_forces(boids, window);
        self.vector = vector;
        self.position = [
            (self.position[0] + (self.speed * self.vector[0] as f64 * dt as f64))
                % window[0] as f64,
            (self.position[1] + (self.speed * self.vector[1] as f64 * dt as f64))
                % window[1] as f64,
        ];
    }

    pub fn accumulate_forces(&self, boids: &Vec<Boid>, window: [i32; 2]) -> Vector2<f64> {
        let align = self.alignment(boids);
        let cohes = self.cohesion(boids);
        let sepa = self.separation(boids);
        //let awall = self.avoid_walls(window);
        let mut out = self.vector + align + cohes + sepa; //+ awall;
        out = na::base::Matrix::normalize(&out);
        out
    }
    pub fn render(&self, d: &mut RaylibDrawHandle, debug: bool) {
        d.draw_circle(
            self.position[0] as i32,
            self.position[1] as i32,
            5.0,
            self.colour,
        );
        if debug {
            d.draw_line(
                self.position[0] as i32,
                self.position[1] as i32,
                (self.position[1] + self.vector[1]) as i32,
                (self.position[0] + self.vector[0]) as i32,
                Color::GREEN,
            );
            //d.draw_circle_lines(
            //    self.position[0] as i32,
            //    self.position[1] as i32,
            //    self.view_distance as f32,
            //    Color::GREEN,
            //)
        }
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
        let y = -(self.position[0] - center[0]);
        return na::base::Matrix::normalize(&Vector2::new(x, y));
    }
    pub fn distance_from(&self, other: &Boid) -> f64 {
        return ((self.position[0] + other.position[0]).powi(2)
            + (self.position[1] + other.position[1]).powi(2))
        .sqrt();
    }
}
