#[derive(Clone, Copy)]
pub struct BoundingBox {
    min_x: f32,
    min_y: f32,
    min_z: f32,
    max_x: f32,
    max_y: f32,
    max_z: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32) -> Self {
        BoundingBox {
            min_x: x,
            min_y: y,
            min_z: z,
            max_x: x + width,
            max_y: y + height,
            max_z: z + depth,
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min_x < other.max_x &&
            self.max_x > other.min_x &&
            self.min_y < other.max_y &&
            self.max_y > other.min_y &&
            self.min_z < other.max_z &&
            self.max_z > other.min_z
    }
}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &Self) -> bool {
        self.min_x == other.min_x &&
            self.min_y == other.min_y &&
            self.min_z == other.min_z &&
            self.max_x == other.max_x &&
            self.max_y == other.max_y &&
            self.max_z == other.max_z
    }
}

#[derive(Clone, Copy)]
pub struct Player {
    pub bounding_box: BoundingBox,
    pub velocity: (f32, f32, f32)
}

impl Player {
    pub fn new(x: f32, y: f32, z: f32, width: f32, height: f32, depth: f32) -> Self {
        Player {
            bounding_box: BoundingBox::new(x, y, z, width, height, depth),
            velocity: (0.0, 0.0, 0.0)
        }
    }

    pub fn update(&mut self, blocks: &[BoundingBox], delta_time: f32) {
        let (vx, vy, vz) = self.velocity;
        let new_x = self.bounding_box.min_x + vx * delta_time;
        let new_y = self.bounding_box.min_y + vy * delta_time;
        let new_z = self.bounding_box.min_z + vz * delta_time;

        let mut new_box = self.bounding_box;
        new_box.min_x = new_x;
        new_box.max_x = new_x + (self.bounding_box.max_x - self.bounding_box.min_x);
        new_box.min_y = new_y;
        new_box.max_y = new_y + (self.bounding_box.max_y - self.bounding_box.min_y);
        new_box.min_z = new_z;
        new_box.max_z = new_z + (self.bounding_box.max_z - self.bounding_box.min_z);

        let mut collision = false;
        for block in blocks {
            if new_box.intersects(block) {
                collision = true;
                break;
            }
        }

        if !collision {
            println!("COLLISION NOT DETECTED");
            self.bounding_box = new_box;
        } else {
            println!("COLLISION DETECTED");
            self.velocity = (0.0, 0.0, 0.0);
        }
    }
}
