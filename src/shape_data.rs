pub struct ShapeData {

}

impl ShapeData {
    pub fn get_cube_vertices() -> Vec<f32> {
        let length = 1.0;
        let vertices: Vec<f32> = vec![
            // 4 - 0 - 3 - 7 Left
            -length, length, -length, 0.0, 1.0, // 0 1 2  2 3 0
            -length, -length, -length, 0.0, 0.0,
            -length, -length, length, 1.0, 0.0,
            -length, length, length, 1.0, 1.0,
            // 7 - 3 - 2 - 6 Front
            -length, length, length, 0.0, 1.0, // 4 5 6  6 7 4
            -length, -length, length, 0.0, 0.0,
            length, -length, length, 1.0, 0.0,
            length, length, length, 1.0, 1.0,
            // 6 - 2 - 1 - 5 Right
            length, length, length, 0.0, 1.0, // 8 9 10  10 11 8
            length, -length, length, 0.0, 0.0,
            length, -length, -length, 1.0, 0.0,
            length, length, -length, 1.0, 1.0,
            // 5 - 1 - 0 - 4 Back
            length, length, -length, 0.0, 1.0, // 12 13 14  14 15 12
            length, -length, -length, 0.0, 0.0,
            -length, -length, -length, 1.0, 0.0,
            -length, length, -length, 1.0, 1.0,
            // 4 - 7 - 6 - 5 Top
            -length, length, -length, 0.0, 1.0, // 16 17 18  18 19 16
            -length, length, length, 0.0, 0.0,
            length, length, length, 1.0, 0.0,
            length, length, -length, 1.0, 1.0,
            // 3 - 0 - 1 - 2 Bottom
            -length, -length, length, 0.0, 1.0, // 20 21 22  22 23 20
            -length, -length, -length, 0.0, 0.0,
            length, -length, -length, 1.0, 0.0,
            length, -length, length, 1.0, 1.0,
        ];
        vertices
    }

    pub fn get_cube_indices() -> Vec<u32> {
        let mut indices: Vec<u32> = vec![];
        for index in 0..6 {
            indices.push(4*index);
            indices.push(4*index + 1);
            indices.push(4*index + 2);
            indices.push(4*index + 2);
            indices.push(4*index + 3);
            indices.push(4*index);
        }
        indices
    }
}

/*let temp_vertices: Vec<f32> = vec![
        -length, -length, -length,  // 0
        length, -length, -length,   // 1
        length, -length, length,    // 2
        -length, -length, length,   // 3
        -length, length, -length,   // 4
        length, length, -length,    // 5
        length, length, length,     // 6
        -length, length, length,    // 7

    ];
    let indices: Vec<u32> = vec![
        4, 0, 3, 3, 7, 4,   // left
        7, 3, 2, 2, 6, 7,   // front
        6, 2, 1, 1, 5, 6,   // right
        5, 1, 0, 0, 4, 5,   // back
        4, 7, 6, 6, 5, 4,   // top
        3, 0, 1, 1, 2, 3,   // bottom
    ];*/