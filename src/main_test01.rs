//https://github.com/PistonDevelopers/piston_window/blob/master/examples/hello_piston.rs
extern crate piston_window;
extern crate specs;

use piston_window::*;
use specs::prelude::*;

// A component contains data which is
// associated with an entity.

struct Vel(f32);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

struct SysA;

impl<'a> System<'a> for SysA {
    // These are the resources required for execution.
    // You can also define a struct and `#[derive(SystemData)]`,
    // see the `full` example.
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        // The `.join()` combines multiple components,
        // so we only access those entities which have
        // both of them.

        // This joins the component storages for Position
        // and Velocity together; it's also possible to do this
        // in parallel using rayon's `ParallelIterator`s.
        // See `ParJoin` for more.
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
        println!("run loop system...");
    }
}


/// Stores application state of inner event loop.
pub struct InnerApp {
    pub title: &'static str,
    pub exit_button: Button,
}

impl InnerApp {
    pub fn run(&mut self, window: &mut PistonWindow) {
        window.set_title(self.title.into());
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([0.5, 0.5, 1.0, 1.0], g);
                ellipse(
                    [1.0, 0.0, 0.0, 1.0],
                    [50.0, 50.0, 100.0, 100.0],
                    c.transform,
                    g,
                );
            });
            if let Some(button) = e.press_args() {
                if button == self.exit_button {
                    break;
                }
            }
        }
    }
}

fn main() {

    // The `World` is our
    // container for components
    // and other resources.

    let mut world = World::new();
    world.register::<Pos>();
    world.register::<Vel>();
    // An entity may or may not contain some component.

    world.create_entity().with(Vel(2.0)).with(Pos(0.0)).build();
    world.create_entity().with(Vel(4.0)).with(Pos(1.6)).build();
    world.create_entity().with(Vel(1.5)).with(Pos(5.4)).build();

    // This entity does not have `Vel`, so it won't be dispatched.
    world.create_entity().with(Pos(2.0)).build();

    // This builds a dispatcher.
    // The third parameter of `add` specifies
    // logical dependencies on other systems.
    // Since we only have one, we don't depend on anything.
    // See the `full` example for dependencies.
    let mut dispatcher = DispatcherBuilder::new().with(SysA, "sys_a", &[]).build();

    // This dispatches all the systems in parallel (but blocking).
    //dispatcher.dispatch(&mut world);


    let title = "Hello Piston! (press any key to enter inner loop)";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.5, 1.0, 0.5, 1.0], g);
            rectangle(
                [1.0, 0.0, 0.0, 1.0],
                [50.0, 50.0, 100.0, 100.0],
                c.transform,
                g,
            );
        });

        if e.press_args().is_some() {
            InnerApp {
                title: "Inner loop (press X to exit inner loop)",
                exit_button: Button::Keyboard(Key::X),
            }
            .run(&mut window);
            window.set_title(title.into());
        }
        //dispatcher.dispatch(&mut world);
    }
}

