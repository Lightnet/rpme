

extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", (640, 480))
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap_or_else(|e| { panic!("Failed to build PistonWindow: {}", e) });

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);

    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
        
    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            window.draw_2d(&e, |c, g, device| {
                clear([0.5, 1.0, 0.5, 1.0], g);
                /*
                let transform = c.transform.trans(10.0, 100.0);

                text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32).draw(
                    "Hello world!",
                    &mut glyphs,
                    &c.draw_state,
                    transform, g
                ).unwrap();

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
                */
            });
        }

        if let Some(args) = e.update_args() {
            //app.update(&args);
            //println!("update?");
        }
    }
}