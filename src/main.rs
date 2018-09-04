extern crate glutin;

mod support;

use glutin::GlContext;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("A fantastic window!");
    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::GlThenGles{ opengl_version: (3,3), opengles_version: (3,2) });
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    let gl = support::load(&gl_window);

    let mut colors : Vec<([f32;4])> = Vec::new();
    colors.push([1.0,0.5,0.7,1.0]);
    colors.push([0.5,0.7,1.0,1.0]);
    colors.push([0.7,1.0,0.5,1.0]);

    let mut color_index : usize = 0;

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            use glutin::Event::WindowEvent;
            use glutin::WindowEvent::*;

            match event {
                WindowEvent { event, .. } => match event {
                    CloseRequested => running = false,
                    Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    MouseInput{ state, .. } => {
                        if state == glutin::ElementState::Pressed { step_color(&colors,&mut color_index); }
                    },
                    Touch(touch) => {
                        if touch.phase == glutin::TouchPhase::Started { step_color(&colors,&mut color_index); }
                    }
                    _ => (),
                },
                _ => ()
            }
        });

        gl.draw_frame(colors[color_index]);
        let _ = gl_window.swap_buffers();
    }
}

fn step_color(colors : &Vec<[f32;4]>, index : &mut usize) {
    *index += 1;
    if *index >= colors.len() { *index = 0; } 
}