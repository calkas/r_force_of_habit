mod physics;
mod space;

use crate::graphics::clear;
use crate::space::world::World;
use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("R Force-of-habit", [512; 2])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new().ups(60).max_fps(60));
    let mut game_world = World::new();

    game_world.add_planet([200.0, 200.0], [0.0, 0.0], 10000.0, 20.0);
    game_world.add_planet([300.0, 200.0], [0.0, 10.0], 1.0, 20.0);
    game_world.add_planet([350.0, 200.0], [-5.0, 5.0], 1.0, 20.0);
    //game_world.add_ship([100.0, 100.0], [0.0, 8.0], 1.0, 10.0);
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
            clear([0.5, 0.5, 0.5, 1.0], g);
            game_world.render(&c, g);
        });
    }
}
