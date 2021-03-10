use super::shape::{Position, Normal, Shape};
use glium::Surface;

pub struct RenderObject {
    positions: glium::VertexBuffer<Position>,
    normals: glium::VertexBuffer<Normal>,
    indicies: glium::IndexBuffer<u32>,
}

impl RenderObject {
    pub fn new(display: &glium::Display, shape: &Shape) -> Self {
        let positions = glium::VertexBuffer::new(display, &shape.positions).unwrap();
        let normals = glium::VertexBuffer::new(display, &shape.normals).unwrap();
        let indicies = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &shape.indicies).unwrap();
        Self {
            positions,
            normals,
            indicies
        }
    }

    pub fn draw<U>(&self, target: &mut glium::Frame, program: &glium::Program, uniforms: &U) 
    where U: glium::uniforms::Uniforms {
        target.draw(&self.positions, &self.indicies, program, uniforms, &Default::default()).unwrap();
    }
}