use cgmath::Quaternion;
use cgmath::Vector3;
use crate::camera::CameraPos;
use crate::instance::Instance;

#[derive(Debug)]
pub struct ShapeDesc {
    pub index_buffer_index: usize,
    pub vertex_buffer_index: usize,
    pub index_buffer_len: usize,
    pub vertex_buffer_len: usize,
    pub instance_buffer_index: usize,
    pub instance_buffer_len: usize,
}

#[derive(Debug)]
pub struct SerializedGame {
    pub index_buffer: Vec<u16>,
    pub vertex_buffer: Vec<Vertex>,
    pub instance_buffer: Vec<Instance>,
    pub shapes: Vec<ShapeDesc>,
    pub camera: CameraPos
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3]
}

impl Vertex {
    pub fn new(pos: [f32; 3]) -> Self {
        Self {
            pos
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }    
}

#[derive(Clone)]
pub enum GameState {
    Lobby,
    InGame,
    GameOver
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Lobby
    }
}

#[derive(Clone)]
pub struct Triangle {
    pub a_len: f32,
    pub b_len: f32,
    pub c_len: f32,
    pub a_thick: f32,
    pub b_thick: f32,
    pub c_thick: f32,

    pub material: Material,
    pub rotation: Quaternion<f32>,
    pub position: Vector3<f32>,
}

pub struct Shape {
    pub vertexes: Vec<Vertex>,
    pub indexes: Vec<u16>,
}

#[derive(Clone)]
pub enum Material {
    Wood,
    Stone
}


#[derive(Clone)]
pub struct Player {
    pub position: Vector3<f32>,
    pub rotation: Quaternion<f32>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(0.0, 0.0, 0.0, 0.0)
        }
    }
}

#[derive(Default, Clone)]
pub struct PlayerState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub primary: bool,
    pub secondary: bool,
    pub third: bool,
}
