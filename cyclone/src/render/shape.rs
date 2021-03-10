use tobj;

#[derive(Clone, Copy, Debug)]
pub struct Position {
  position: (f32, f32, f32)
}
glium::implement_vertex!(Position, position);

impl Position {
  pub fn new(position: &[f32]) -> Self {
    Self {
      position: (position[0], position[1], position[2])
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct Normal {
  normal: (f32, f32, f32)
}
glium::implement_vertex!(Normal, normal);

impl Normal {
  pub fn new(normal: &[f32]) -> Self {
    Self {
      normal: (normal[0], normal[1], normal[2])
    }
  }
}

#[derive(Debug)]
pub struct Shape {
  pub positions: Vec<Position>,
  pub normals: Vec<Normal>,
  pub indicies: Vec<u32>,
}

impl Shape {
  pub fn new(filename: &str) -> Self {
    println!("{}", filename);
    let load = tobj::load_obj(filename, true);
    assert!(load.is_ok());
    let (models, _) = load.unwrap();
    assert!(models.len() > 0);
    
    let mesh = &models[0].mesh;

    let vertex_cnt = mesh.positions.len() / 3;
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    positions.reserve(vertex_cnt);
    normals.reserve(vertex_cnt);

    for n in 0..vertex_cnt {
      let start_idx = n*3;
      let end_idx = start_idx + 3;
      positions.push(Position::new(&mesh.positions[start_idx..end_idx]));
      normals.push(Normal::new(&mesh.normals[start_idx..end_idx]));
    }
    
    Self {
      positions,
      normals,
      indicies: mesh.indices.clone()
    }
  }
}