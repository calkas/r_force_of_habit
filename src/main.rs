use piston_window::{
    graphics::{Context, Transformed, rectangle},
    wgpu_graphics::WgpuGraphics,
    *,
};

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

fn main() {
    let mut window: PistonWindow = WindowSettings::new("R Force-of-habit", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut object = ForceObject::new(200.0, 10.0);
    let mut mouse_coords = (0.0, 0.0);
    while let Some(e) = window.next() {
        e.update(|args| {
            //println!("dt: {}", args.dt);
            object.update(args.dt);
        });

        e.mouse_cursor(|pos| {
            //println!("Mouse pos: {:?}", args)
            mouse_coords = (pos[0], pos[1]);
            if object.marked {
                object.x = mouse_coords.0 - 25.0;
                object.y = mouse_coords.1 - 25.0;
            }
        });
        e.press(|args| match args {
            Button::Mouse(MouseButton::Left) => {
                let inside_x = mouse_coords.0 >= object.x && mouse_coords.0 <= object.x + 50.0;
                let inside_y = mouse_coords.1 >= object.y && mouse_coords.1 <= object.y + 50.0;

                if inside_x && inside_y {
                    object.marked = true;
                    object.vx = 0.0;
                    object.vy = 0.0;
                }
            }
            _ => {}
        });
        e.release(|button| {
            if button == Button::Mouse(MouseButton::Left) {
                object.marked = false;
            }
        });

        window.draw_2d(&e, |c, g, _| {
            use graphics::*;
            clear([0.5, 0.5, 0.5, 1.0], g);
            object.render(&c, g);
        });
    }
}
