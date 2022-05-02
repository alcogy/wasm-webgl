pub struct Cube {
    pub vertex: [f32; 72],
    pub color: [f32; 96],
    pub index: [u8; 36],
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            vertex: [ 
                -0.5, -0.5, 0.5, 0.5, -0.5, 0.5, 0.5, 0.5, 0.5, -0.5, 0.5, 0.5,          // Front
                 0.5, -0.5, 0.5, 0.5, -0.5, -0.5, 0.5,  0.5, -0.5, 0.5, 0.5, 0.5,        // Right
                -0.5, -0.5, -0.5, -0.5,  0.5, -0.5, 0.5,  0.5, -0.5, 0.5, -0.5, -0.5,    // Back
                -0.5, -0.5, 0.5, -0.5,  0.5, 0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5,    // Left    
                -0.5, -0.5, 0.5, -0.5, -0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.5,      // Bottom
                -0.5,  0.5, 0.5, 0.5,  0.5, 0.5, 0.5,  0.5, -0.5, -0.5,  0.5, -0.5       // Top
                ],

            color: [
                1.0,  1.0,  1.0,  1.0,    // Front face: white
                1.0,  1.0,  1.0,  1.0,    // Front face: white
                1.0,  1.0,  1.0,  1.0,    // Front face: white
                1.0,  1.0,  1.0,  1.0,    // Front face: white
                1.0,  0.0,  0.0,  1.0,    // Right face: red
                1.0,  0.0,  0.0,  1.0,    // Right face: red
                1.0,  0.0,  0.0,  1.0,    // Right face: red
                1.0,  0.0,  0.0,  1.0,    // Right face: red
                0.0,  1.0,  0.0,  1.0,    // Back face: green
                0.0,  1.0,  0.0,  1.0,    // Back face: green
                0.0,  1.0,  0.0,  1.0,    // Back face: green
                0.0,  1.0,  0.0,  1.0,    // Back face: green
                0.0,  0.0,  1.0,  1.0,    // Left face: blue
                0.0,  0.0,  1.0,  1.0,    // Left face: blue
                0.0,  0.0,  1.0,  1.0,    // Left face: blue
                0.0,  0.0,  1.0,  1.0,    // Left face: blue
                1.0,  1.0,  0.0,  1.0,    // Bottom face: yellow
                1.0,  1.0,  0.0,  1.0,    // Bottom face: yellow
                1.0,  1.0,  0.0,  1.0,    // Bottom face: yellow
                1.0,  1.0,  0.0,  1.0,    // Bottom face: yellow
                1.0,  0.0,  1.0,  1.0,    // Top face: purple
                1.0,  0.0,  1.0,  1.0,    // Top face: purple
                1.0,  0.0,  1.0,  1.0,    // Top face: purple
                1.0,  0.0,  1.0,  1.0,    // Top face: purple
            ],

            index: [
                0, 1, 2, 0, 2, 3,
                4, 5, 6, 4, 6, 7,
                8, 9, 10, 8, 10, 11,
                12, 13, 14, 12, 14, 15,
                16, 17, 18, 16, 18, 19,
                20, 21, 22, 20, 22, 23
            ],
        }
        
    }

}