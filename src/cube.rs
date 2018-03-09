use cgmath::*;

#[repr(C)]
pub struct Vertex {
    ver_pos: Vector3<f32>,
    tex_pos: Vector2<f32>,
}

pub static VERTEX_DATA: [Vertex; 4*6 - 4] = [
    // 00: -X 00
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: -0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 0.0 },
    },
    // 01: -X 10
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: -0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 0.0 },
    },
    // 02: -X 01
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: 0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 1.0 },
    },
    // 03: -X 11
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: 0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 1.0 },
    },
    // 04: +X 00
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: -0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 0.0 },
    },
    // 05: +X 10
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: -0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 0.0 },
    },
    // 06: +X 01
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 1.0 },
    },
    // 07: +X 11
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: 0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 1.0 },
    },
    // 08: -Z 00
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: -0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 0.0 },
    },
    // 09: -Z 10
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: -0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 0.0 },
    },
    // 10: -Z 01
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: 0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 1.0 },
    },
    // 11: -Z 11
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: 0.5,
            z: -0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 1.0 },
    },
    // 12: +Z 00
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: -0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 0.0 },
    },
    // 13: +Z 10
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: -0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 0.0 },
    },
    // 14: +Z 01
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: 0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 1.0 },
    },
    // 15: +Z 11
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 1.0 },
    },
    // 00: -Y 00 = -X 00
    // 05: -Y 10 = +X 10
    // 16: -Y 01
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: -0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 1.0 },
    },
    // 17: -Y 11
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: -0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 1.0 },
    },
    // 18: +Y 00
    Vertex {
        ver_pos: Vector3 {
            x: -0.5,
            y: 0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 0.0, y: 0.0 },
    },
    // 19: +Y 10
    Vertex {
        ver_pos: Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
        tex_pos: Vector2 { x: 1.0, y: 0.0 },
    },
    // 02: +Y 01 = -X 01
    // 07: +Y 11 = +X 11
];

#[repr(C)]
pub struct Triangle(u32, u32, u32);

pub static ELEMENT_DATA: [Triangle; 2 * 6] = [
    // -X
    Triangle(00, 01, 03),
    Triangle(03, 02, 00),
    // +X
    Triangle(04, 05, 07),
    Triangle(07, 06, 04),
    // -Z
    Triangle(08, 09, 11),
    Triangle(11, 10, 08),
    // +Z
    Triangle(12, 13, 15),
    Triangle(15, 14, 12),
    // -Y
    Triangle(00, 05, 17),
    Triangle(17, 16, 00),
    // +Y
    Triangle(18, 19, 07),
    Triangle(07, 02, 18),
];
