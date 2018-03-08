use cgmath::*;

#[repr(C)]
pub struct Vertex {
    position: Vector3<f32>,
    color: RGB,
}

#[repr(C)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

pub static VERTEX_DATA: [Vertex; 8] = [
    Vertex {
        position: Vector3 {
            x: -0.5,
            y: -0.5,
            z: -0.5,
        },
        color: RGB {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        },
    },
    Vertex {
        position: Vector3 {
            x: 0.5,
            y: -0.5,
            z: -0.5,
        },
        color: RGB {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        },
    },
    Vertex {
        position: Vector3 {
            x: -0.5,
            y: 0.5,
            z: -0.5,
        },
        color: RGB {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        },
    },
    Vertex {
        position: Vector3 {
            x: 0.5,
            y: 0.5,
            z: -0.5,
        },
        color: RGB {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
    },
    Vertex {
        position: Vector3 {
            x: -0.5,
            y: -0.5,
            z: 0.5,
        },
        color: RGB {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        },
    },
    Vertex {
        position: Vector3 {
            x: 0.5,
            y: -0.5,
            z: 0.5,
        },
        color: RGB {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        },
    },
    Vertex {
        position: Vector3 {
            x: -0.5,
            y: 0.5,
            z: 0.5,
        },
        color: RGB {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        },
    },
    Vertex {
        position: Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
        color: RGB {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
    },
];

#[repr(C)]
pub struct Triangle(u32, u32, u32);

pub static ELEMENT_DATA: [Triangle; 2 * 6] = [
    // -X
    Triangle(0, 4, 6),
    Triangle(6, 2, 0),
    // +X
    Triangle(5, 1, 3),
    Triangle(3, 7, 5),
    // -Y
    Triangle(0, 1, 5),
    Triangle(5, 4, 0),
    // +Y
    Triangle(6, 7, 3),
    Triangle(3, 2, 6),
    // -Z
    Triangle(1, 0, 2),
    Triangle(2, 3, 1),
    // +Z
    Triangle(4, 5, 7),
    Triangle(7, 6, 4),
];
