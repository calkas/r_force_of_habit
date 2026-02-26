use super::{Body, BodyId, Renderable};
use piston_window::graphics::Context;
use piston_window::graphics::Transformed;
use piston_window::graphics::rectangle;
use piston_window::wgpu_graphics::WgpuGraphics;

pub struct SpaceShip {
    pub body_id: BodyId,
}

impl Renderable for SpaceShip {
    fn render(&self, bodies: &Vec<Body>, context: &Context, graphics: &mut WgpuGraphics) {
        let r = bodies[self.body_id.0].radius;
        let (cx, cy) = (bodies[self.body_id.0].cx, bodies[self.body_id.0].cy);

        let rect = [0.0, 0.0, 2.0 * r, 2.0 * r];
        let color = [1.0, 1.0, 0.0, 1.0];

        rectangle(
            color,
            rect,
            context.transform.trans(cx - r, cy - r),
            graphics,
        );
    }
}

impl SpaceShip {
    pub fn new(body_id: BodyId) -> Self {
        Self { body_id }
    }
}
