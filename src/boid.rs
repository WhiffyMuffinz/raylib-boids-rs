use nalgebra as na;
use nalgebra::Vector2;
use raylib::prelude::*;

pub struct Boid {
    pub position: [f64; 2],
    pub vector: Vector2<f64>,
    pub view_distance: i32,
    pub colour: Color,
    pub num: u8,
}

impl Boid {
    pub fn accumulate_forces(&mut self, boids: &Vec<Boid>) {
        let align = self.alignment(boids);
        //let cohes = self.cohesion(boids);
        let sepa = self.separation(boids);
        //TODO: sum vector components
        self.vector = na::base::Matrix::normalize(&self.vector);
    }
    pub fn render(&self, d: &mut RaylibDrawHandle, window: [i32; 2]) {
        d.draw_circle(
            self.position[0] as i32,
            self.position[1] as i32,
            5.0,
            self.colour,
        );
    }
    fn alignment(&mut self, boids: &Vec<Boid>) -> Vector2<f64> {
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
        if neighbour_count >= 0.0 {
            out[0] /= neighbour_count;
            out[1] /= neighbour_count;
            return na::base::Matrix::normalize(&out);
        } else {
            return Vector2::new(0.0, 0.0);
        }
    }
    //fn cohesion(&mut self, boids: &Vec<Boid>) -> Vector2<f64> {
    //    let mut neighbour_count = 0.0;
    //    let mut out = [0.0, 0.0];
    //    for b in boids {
    //        if b.num != self.num {
    //            if self.distance_from(b) <= self.view_distance as f64 {
    //                out[0] += b.position[0];
    //                out[1] += b.position[1];
    //                neighbour_count += 1.0;
    //            }
    //        }
    //    }
    //    if neighbour_count >= 0.0 {
    //        out[0] /= neighbour_count;
    //        out[1] /= neighbour_count;
    //        return na::base::Matrix::normalize(&Vector2::new(
    //            out[0] - self.vector[0],
    //            out[1] - self.vector[1],
    //        ));
    //    } else {
    //        return Vector2::new(0.0, 0.0);
    //    }
    //}
    fn separation(&mut self, boids: &Vec<Boid>) -> Vector2<f64> {
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
        if neighbour_count >= 0.0 {
            return na::base::Matrix::normalize(&out);
        } else {
            return out;
        }
    }
    fn distance_from(&mut self, other: &Boid) -> f64 {
        return ((self.position[0] + other.position[0]).powi(2)
            + (self.position[1] + other.position[1]).powi(2))
        .sqrt();
    }
}