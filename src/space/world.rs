use super::orbit::OrbitTrails;
use super::planet::Planet;
use super::ship::SpaceShip;
use super::{Body, BodyId, Renderable};
use crate::physics::forces::nbody_step;

use piston_window::graphics::Context;
use piston_window::wgpu_graphics::WgpuGraphics;
enum Entity {
    Planet(Planet),
    Ship(SpaceShip),
}

pub struct World {
    bodies: Vec<Body>,
    space_objects: Vec<Entity>,
    orbit: OrbitTrails,
}

impl World {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            space_objects: Vec::new(),
            orbit: OrbitTrails::default(),
        }
    }
    fn create_body(
        &mut self,
        position: [f64; 2],
        velocity: [f64; 2],
        mass: f64,
        radius: f64,
    ) -> BodyId {
        self.bodies
            .push(Body::new(position, velocity, mass, radius));

        BodyId(self.bodies.len() - 1)
    }

    pub fn add_planet(&mut self, position: [f64; 2], velocity: [f64; 2], mass: f64, radius: f64) {
        let body_id = self.create_body(position, velocity, mass, radius);
        self.space_objects
            .push(Entity::Planet(Planet::new(body_id)));
        self.orbit.register(body_id, 100, 0.5);
    }

    pub fn add_ship(&mut self, position: [f64; 2], velocity: [f64; 2], mass: f64, radius: f64) {
        let body_id = self.create_body(position, velocity, mass, radius);
        self.space_objects
            .push(Entity::Ship(SpaceShip::new(body_id)));
        self.orbit.register(body_id, 70, 0.5);
    }

    pub fn render(&mut self, context: &Context, graphics: &mut WgpuGraphics) {
        self.orbit.draw(context, graphics);
        for space_object in self.space_objects.iter() {
            match space_object {
                Entity::Planet(planet) => planet.render(&self.bodies, context, graphics),
                Entity::Ship(space_ship) => space_ship.render(&self.bodies, context, graphics),
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        nbody_step(&mut self.bodies, 1.0, dt);
        self.orbit.step_trajectory(&self.bodies, dt);
    }
}
