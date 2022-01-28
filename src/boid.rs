use nalgebra::{norm, Vector2};

pub struct Boid {
    pub position: [f64; 2],
    pub vector: Vector2,
    pub view_distance: i32,
    pub colour: [f32; 4],
    pub num: u8,
}

impl Boid {
    pub fn accumulate_forces(&mut self, boids: &Vec<Boid>) {
        let align = self.alignment(boids);
        let cohes = self.cohesion(boids);
        let sepa = self.separation(boids);
        self.vector[0] += align[0] + cohes[0] + sepa[0];
        self.vector[1] += align[1] + cohes[1] + sepa[1];
        self.vector = vm::vec2_normalized(self.vector);
    }
    fn alignment(&mut self, boids: &Vec<Boid>) -> vm::Vector2<f64> {
        let mut neighbour_count = 0.0;
        let mut out: vm::Vector2<f64> = [0.0, 0.0];
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
            return vm::vec2_normalized(out);
        } else {
            return [0.0, 0.0];
        }
    }
    fn cohesion(&mut self, boids: &Vec<Boid>) -> vm::Vector2<f64> {
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
        if neighbour_count >= 0.0 {
            out[0] /= neighbour_count;
            out[1] /= neighbour_count;
            return vm::vec2_normalized([out[0] - self.vector[0], out[1] - self.vector[1]]);
        } else {
            return [0.0, 0.0];
        }
    }
    fn separation(&mut self, boids: &Vec<Boid>) -> vm::Vector2<f64> {
        let mut neighbour_count = 0.0;
        let mut out = [0.0, 0.0];
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
            return vm::vec2_normalized(out);
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
