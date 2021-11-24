mod collection;
pub use collection::*;
use collection::Collection;
use geo::Coordinate;

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


#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

impl From<Vertex> for Coordinate<f32> {
    fn from(v: Vertex) -> Self {
        Self {
            x:v.x,
            y:v.y
        }
    }
}

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Sprite {}

#[derive(Default)]
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

#[derive(Default)]
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
        // locate all vertices
        let mut vertices = Vec::new();
        let mut geo_lines:Vec<geo::Line<f32>> = Vec::new();

        for i in 0..self.vertices.len() {
            if let [Some(v1), Some(v2)] = [self.vertices.get(i), self.vertices.get((i + 1) % self.vertices.len())] {
                geo_lines.push(geo::Line {
                    start: Coordinate::from(*v1),
                    end: Coordinate::from(*v2),
                });
            }
        }

        for geo_line in geo_lines.iter() {
            // check if intersects with lines in the world
            for (lineindex, line) in world.lines.iter() {

            }
        }

        
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
       

        self.vertices.clear();
    }
}
