pub struct TestStruct {
    pub name: String,
    pub number: u32,
}

impl TestStruct {
    pub fn print_all(&self) {
        println!("name: {0}", self.name);
        println!("number: {0}", self.number);
    }
}

pub struct Vector {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl Vector {
    pub fn normalise(&self) -> Vector {
        let norm: u32 = u32::isqrt((self.x) ^ 2 + (self.y) ^ 2 + (self.z) ^ 2);
        Vector {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }
    pub fn print_all(&self) {
        println!("x: {0}", self.x);
        println!("y: {0}", self.y);
        println!("z: {0}", self.z);
    }
}
