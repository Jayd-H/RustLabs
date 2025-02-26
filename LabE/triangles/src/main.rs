#[macro_use]
extern crate glium;
extern crate winit;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("Triangles")
        .build(&event_loop);

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.05, -0.0288] };
    let vertex2 = Vertex { position: [ 0.00,  0.0577] };
    let vertex3 = Vertex { position: [ 0.05, -0.0288] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut delta_t: f32 = -0.5;

    #[allow(deprecated)] 
    let _ = event_loop.run(move |event, window_target| {
        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => window_target.exit(),

                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },

                winit::event::WindowEvent::RedrawRequested => {

                    let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
                    winit::event_loop::ControlFlow::WaitUntil(next_frame_time);

                    // Begin render loop

                    // Animation counter
                    delta_t += 0.005;
                    if delta_t > 0.7 {
                        delta_t = -1.4;
                    }

                    // Create a drawing target
                    let mut target = display.draw();

                    // Clear the screen to black
                    target.clear_color(0.0, 0.0, 0.0, 1.0);

                    // Iterate over the 10 triangles
                    for i in 0 .. 10 {

                        let x = rand::random::<f32>();
                        let y = rand::random::<f32>();

                        // Calculate the position of the triangle
                        let pos_x : f32 = delta_t + ((i as f32) * 0.1 * x);
                        let pos_y : f32 = delta_t + ((i as f32) * 0.1 * y);
                        let pos_z : f32 = 0.0;
                        
                        // Create a 4x4 matrix to store the position and orientation of the triangle
                        let uniforms = uniform! {
                            matrix: [
                                [1.0, 0.0, 0.0, 0.0],
                                [0.0, 1.0, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [pos_x, pos_y, pos_z, 1.0],
                            ]
                        };

                        // Draw the triangle
                        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
                    }

                    // Display the completed drawing
                    target.finish().unwrap();

                    // End render loop
                },
                _ => (),
            },                
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        };
    });
}
