use piston_window::{
    graphics::{Context, Transformed, ellipse, rectangle},
    wgpu_graphics::WgpuGraphics,
    *,
};

use crate::space::{Renderable, planet::Planet, world};

use crate::space::world::World;

mod physics;
mod space;

const GRAVITY: f64 = 100.0;
struct ForceObject {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    marked: bool,
}
impl ForceObject {
    fn new(pos_x: f64, pos_y: f64) -> Self {
        Self {
            x: pos_x,
            y: pos_y,
            vx: 0.0,
            vy: 0.0,
            marked: false,
        }
    }

    fn update(&mut self, time: f64) {
        if !self.marked {
            let acceleration = GRAVITY;
            let velocity = acceleration * time;
            self.vy += velocity;
            let dumping = 0.4;
            //println!("velocity: {}", self.vy);

            self.x += self.vx * time;
            self.y += self.vy * time;

            // Collision
            if self.y >= 512.0 - 50.0 {
                self.y = 512.0 - 50.0;

                // Bounce
                if self.vy > 0.0 {
                    self.vy = -self.vy * dumping;
                }
                if self.vy.abs() < 1.0 {
                    self.vy = 0.0;
                }
            }
        }
    }

    fn render(&mut self, c: &Context, g: &mut WgpuGraphics<'_>) {
        rectangle(
            [1.0, 0.0, 0.0, 1.0],   // red
            [0.0, 0.0, 50.0, 50.0], // rectangle
            c.transform.trans(self.x, self.y),
            g,
        );
    }
}

struct NewtonGravityObject {
    x: f64,
    y: f64,
    angle: f64,
    r: f64,
    mass: f64,
    orbit: Vec<(f64, f64)>,
}

impl NewtonGravityObject {
    fn new() -> Self {
        Self {
            y: 0.0,
            x: 0.0,
            angle: 0.0,
            r: 150.0,
            mass: 10.0,
            orbit: Vec::new(),
        }
    }

    fn update(&mut self, dt: f64) {
        let cx = 200.0;
        let cy = 200.0;

        let angular_speed = 1.0;
        //predkosc katowa
        // angle = w * dt

        self.angle += angular_speed * dt;
        //println!("angle {}", self.angle);

        self.x = cx + self.r * self.angle.cos();
        self.y = cy - self.r * self.angle.sin();

        if self.orbit.len() < 376 {
            self.orbit.push((self.x, self.y));
        }
    }

    fn render(&mut self, c: &Context, g: &mut WgpuGraphics<'_>) {
        ellipse(
            [0.0, 1.0, 0.0, 1.0],       // red
            [-25.0, -25.0, 50.0, 50.0], // rectangle
            c.transform.trans(200.0, 200.0),
            g,
        );
        ellipse(
            [1.0, 0.0, 0.0, 1.0],       // red
            [-25.0, -25.0, 50.0, 50.0], // rectangle
            c.transform.trans(self.x, self.y),
            g,
        );

        for e in self.orbit.iter() {
            ellipse(
                [0.0, 0.0, 1.0, 0.25],  // red
                [-2.5, -2.5, 5.0, 5.0], // rectangle
                c.view.trans(e.0, e.1),
                g,
            );
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("R Force-of-habit", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));
    let mut game_world = world::World::new();

    game_world.add_planet([200.0, 200.0], [0.0, 0.0], 10000.0, 20.0);
    game_world.add_planet([300.0, 200.0], [0.0, 10.0], 1.0, 20.0);
    //game_world.add_planet([100.0, 100.0], [0.0, 00.0], 10000.0, 20.0);
    let mut mouse_coords = (0.0, 0.0);
    while let Some(e) = events.next(&mut window) {
        e.update(|args| {
            //println!("dt: {}", args.dt);
            game_world.update(args.dt);
        });

        e.mouse_cursor(|pos| {
            //println!("Mouse pos: {:?}", args)
            // mouse_coords = (pos[0], pos[1]);
            // if object.marked {
            //     object.x = mouse_coords.0 - 25.0;
            //     object.y = mouse_coords.1 - 25.0;
            // }
        });
        e.press(|args| match args {
            Button::Mouse(MouseButton::Left) => {
                // let inside_x = mouse_coords.0 >= object.x && mouse_coords.0 <= object.x + 50.0;
                // let inside_y = mouse_coords.1 >= object.y && mouse_coords.1 <= object.y + 50.0;

                // if inside_x && inside_y {
                //     object.marked = true;
                //     object.vx = 0.0;
                //     object.vy = 0.0;
                // }
            }
            _ => {}
        });
        e.release(|button| {
            // if button == Button::Mouse(MouseButton::Left) {
            //     object.marked = false;
            // }
        });

        window.draw_2d(&e, |c, g, _| {
            use graphics::*;
            clear([0.5, 0.5, 0.5, 1.0], g);
            game_world.render(&c, g);
        });
    }
}
