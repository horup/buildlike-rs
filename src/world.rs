use crate::*;

#[derive(Default)]
pub struct World {
    pub vertices: Collection<Vertex>,
    pub lines: Collection<Line>
}

impl World {
    pub fn new() -> Self {
        Self {
            vertices: Collection::default(),
            lines: Collection::default(),
        }
    }
}