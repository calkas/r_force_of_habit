pub mod planet;
pub mod ship;
pub mod world;

use piston_window::graphics::Context;
use piston_window::wgpu_graphics::WgpuGraphics;
pub trait Renderable {
    fn render(&self, bodies: &Vec<Body>, context: &Context, graphics: &mut WgpuGraphics);
}

#[derive(Copy, Clone, Debug)]
pub struct BodyId(pub usize);

/// # Body - Physics
/// (cx, cy) - centered coords required for physics
#[derive(Clone, Debug)]
pub struct Body {
    pub cx: f64,
    pub cy: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
    pub radius: f64,
}
impl Body {
    pub fn new(position: [f64; 2], velocity: [f64; 2], mass: f64, radius: f64) -> Self {
        Self {
            cx: position[0],
            cy: position[1],
            vx: velocity[0],
            vy: velocity[1],
            mass,
            radius,
        }
    }
}
