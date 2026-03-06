use crate::space::Body;
use crate::space::BodyId;
use piston_window::graphics::Context;
use piston_window::graphics::line;
use piston_window::wgpu_graphics::WgpuGraphics;
use std::collections::HashMap;
use std::collections::VecDeque;

struct OrbitTrailEntry {
    points: VecDeque<(f64, f64)>,
    maximum_length: usize,
    sample_step: f64,
    pub count: f64,
}
impl OrbitTrailEntry {
    fn new(maximum_length: usize, sample_step: f64) -> Self {
        Self {
            points: VecDeque::with_capacity(maximum_length),
            maximum_length,
            sample_step,
            count: 0.0,
        }
    }
    fn add(&mut self, coords: (f64, f64)) {
        if self.points.len() == self.maximum_length {
            self.points.pop_front();
        }
        self.points.push_back(coords);
    }
}

#[derive(Default)]
pub struct OrbitTrails {
    trails: HashMap<BodyId, OrbitTrailEntry>,
}

impl OrbitTrails {
    pub fn register(&mut self, id: BodyId, maximum_length: usize, sample_step: f64) {
        self.trails
            .insert(id, OrbitTrailEntry::new(maximum_length, sample_step));
    }

    pub fn step_trajectory(&mut self, bodies: &[Body], dt: f64) {
        for (id, body) in bodies.iter().enumerate() {
            let body_id = BodyId(id);

            if let Some(trail) = self.trails.get_mut(&body_id) {
                trail.count += dt;

                if trail.count > trail.sample_step {
                    trail.add((body.cx, body.cy));
                    trail.count = 0.0;
                }
            }
        }
    }

    pub fn draw(&mut self, context: &Context, graphics: &mut WgpuGraphics) {
        for (_, orbit_trail) in &self.trails {
            if orbit_trail.points.len() < 2 {
                continue;
            }

            let mut alpha = 0.25;
            let alpha_step = alpha / orbit_trail.points.len() as f32;

            for i in 1..orbit_trail.points.len() {
                let (x1, y1) = orbit_trail.points[i - 1];
                let (x2, y2) = orbit_trail.points[i];

                let color = [0.0, 1.0, 1.0, alpha];

                line(
                    color,
                    2.0,
                    [x1 as f64, y1 as f64, x2 as f64, y2 as f64],
                    context.transform,
                    graphics,
                );

                alpha += alpha_step;
            }
        }
    }
}
#[cfg(test)]
mod ut {
    use super::*;
    #[test]
    fn orbit_trail_max_entry() {
        let mut orbital_trail_entry = OrbitTrailEntry::new(2, 1);
        orbital_trail_entry.add((1.0, 1.0));
        orbital_trail_entry.add((2.0, 2.0));
        orbital_trail_entry.add((3.0, 3.0));

        assert_eq!((2.0, 2.0), orbital_trail_entry.points[0]);
        assert_eq!((3.0, 3.0), orbital_trail_entry.points[1]);
    }
}
