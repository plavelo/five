#[allow(dead_code)]
pub struct Register {
    x: [u64; 32],
}

#[allow(dead_code)]
impl Register {
    pub fn new() -> Self {
        Self { x: [0; 32] }
    }
}
