#[macro_use]
extern crate glium;

use glium::{glutin, Display};
use glium::Surface;

// Vertex Implementation
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);


fn main() {
    // Event Loop Setup
    let mut event_loop = glutin::event_loop::EventLoop::new();
    // Window and Context Builders
    let window_builder = glutin::window::WindowBuilder::new();
    let context_builder = glutin::ContextBuilder::new();

    // Create the Display
    let display = Display::new(window_builder, context_builder, &event_loop).unwrap();

    // Event Handler Setup
    event_loop.run(move |event, _, control_flow| {
        let shape = vec![
            Vertex { position: [0.0, 0.8] },
            Vertex { position: [-0.5, -0.8] },
            Vertex { position: [0.5, -0.8] },
        ];

        // Shape
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        //  Shaders
        let vertex_shader = r#"
            #version 140
            in vec2 position;
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader = r#"
            #version 140
            out vec4 color;
            void main() {
                color = vec4(0.91, 0.23, 0.23, 1.0);
            }
        "#;

        // Program
        let program = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

        // Drawing
        let mut target = display.draw();
        target.clear_color(0.18, 0.13, 0.18, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        // Ensure Our Window Stays Open
        let next_frame = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => ()
        }
    });
}
