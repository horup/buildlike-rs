mod collection;
pub use collection::*;
use collection::Collection;

use std::ops::{Deref, DerefMut};
use generational_arena::{Arena, Index};

#[derive(Debug, Clone, Copy)]
pub struct Sector {

}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub v1: Index,
    pub v2: Index,
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        (self.v1 == other.v1 && self.v2 == other.v2) || (self.v1 == other.v2 && self.v2 == other.v1) 
    }
}


#[derive(Clone, Copy, PartialEq, Default)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Sprite {}

pub struct World {
    pub vertices: Collection<Vertex>,
    pub lines: Collection<Line>,
    pub sectors: Collection<Sector>,
}

impl World {
    pub fn new() -> Self {
        Self {
            vertices: Collection::default(),
            lines: Collection::default(),
            sectors: Collection::default(),
        }
    }
}

pub struct Designer {
    pub vertices: Vec<Vertex>,
}

impl Designer {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn push_vertex(&mut self, v: Vertex) -> &mut Self {
        self.vertices.push(v);

        self
    }

    pub fn commit(&mut self, world: &mut World) {
        let mut vertices = Vec::new();
        for p in self.vertices.drain(..) {
            let v = world.vertices.find_or_insert(p);
            vertices.push(v);
        }

        let mut lines = Vec::new();
        for i in 0..vertices.len() {
            if let (Some(v1), Some(v2)) = (vertices.get(i), vertices.get((i + 1) % vertices.len())) {
                if *v1 != *v2 {
                    let line = Line {
                        v1:v1.clone(),
                        v2:v2.clone()
                    };

                    let index = world.lines.find_or_insert(line);
                    lines.push(index);
                }
            }
        }

        if lines.len() >= 2 {
            let sector = Sector {

            };
        }
    }
}
