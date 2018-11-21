#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;
use gfx::traits::FactoryExt;

use gfx::format::{DepthStencil, Rgba8};

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "coord2d",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<Rgba8> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.8, -0.8 ] },
    Vertex { pos: [  0.8, -0.8 ] },
    Vertex { pos: [  0.0,  0.8 ] }
];

fn main() {
    // Glutin event loop, passed to gfx_window_glutin init function
    let mut events_loop = glutin::EventsLoop::new();
    // Glutin window builder, passed to gfx_window_glutin init function
    let window_builder = glutin::WindowBuilder::new().with_title("Cubix".to_string());
    // Glutin's OpenGL context builder, also passed to gfx_window_glutin init function
    let context = glutin::ContextBuilder::new();
    let (window, mut device, mut factory, rtv, _stv) =
        gfx_window_glutin::init::<Rgba8, DepthStencil>(window_builder, context, &events_loop);

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory
        .create_pipeline_simple(
            include_bytes!("triangle_120.glslv"),
            include_bytes!("triangle_120.glslf"),
            pipe::new(),
        )
        .unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out:  rtv,
    };

    let mut running = true;
    while running {
        // fetch events
        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::KeyboardInput {
                        input:
                            glutin::KeyboardInput {
                                virtual_keycode: Some(glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => return,
                    _ => (),
                }
            }
        });

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}