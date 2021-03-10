use glium::{glutin, Surface};
use cyclone::render;
use cyclone::datatypes::{Vector3, Matrix4x4};

fn main() {
  let window_info = render::renderloop::WindowInfo {
    title: "Cyclone - Rust",
    screen_size: (1280, 720)
  };
  let render_info = render::renderloop::init(window_info);
  let event_loop = render_info.0;
  let display = render_info.1;

  let cube = render::shape::Shape::new("cyclone/resources/cube.obj");
  let cube_render_object = render::RenderObject::new(&display, &cube);

  let vertex_shader_src = r#"
  #version 430

  in vec3 position;

  uniform mat4 matrix;

  void main() {
    gl_Position = matrix * vec4(position, 1.0);
  }
  "#;

  let fragment_shader_src = r#"
  #version 430

  out vec4 color;

  void main() {
    color = vec4(1.0, 0.0, 0.0, 1.0);
  }
  "#;

  let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).expect("failed to compile shader program");
  
  let mut t:f32 = 0.0;
  event_loop.run(move |event, _, control_flow| {
    t += 0.001;

    let model = Matrix4x4::translate(Vector3::new(0.0, 0.0, 0.0)) * Matrix4x4::scale(Vector3::new(1.0, 1.0, 1.0));
    let view = Matrix4x4::look_at(Vector3::new(t.cos()*10.0, 5.0, t.sin()*10.0), Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
    let projection = Matrix4x4::perspective(90.0, 1280.0/720.0, 0.1, 1000.0);
    let mut matrix = projection * view * model;
    matrix.transpose();

    let uniform = glium::uniform! {
      matrix: matrix.value
    };

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    cube_render_object.draw(&mut target, &program, &uniform);
    render::renderloop::draw(&display);
    target.finish().unwrap();

    match event {
      glutin::event::Event::WindowEvent {event, ..} => match event {
        glutin::event::WindowEvent::CloseRequested => {
          *control_flow = glutin::event_loop::ControlFlow::Exit;
          return;
        },
        _ => return,
      },
      _ => (),
    }

  });
}
