use crate::*;


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
        // find or insert all vertices
        let mut vertices = Vec::new();
        for p in self.vertices.drain(..) {
            let v = world.vertices.find_or_insert(p);
            vertices.push(v);
        }

        // find or insert all lines
        let mut lines = Vec::new();
        for i in 0..vertices.len() {
            if let (Some(v1), Some(v2)) = (vertices.get(i), vertices.get((i + 1) % vertices.len())) {
                if *v1 != *v2 {
                    let line = Line {
                        start:v1.clone(),
                        end:v2.clone()
                    };

                    let index = world.lines.find_or_insert(line);
                    lines.push(index);
                }
            }
        }

        // split lines which are now intersecting
        loop {
            let mut no_more_intersections = true;
            let mut added_lines = Vec::new();
            for index in lines.iter() { 
                if let Some((other_index, intersection)) = find_first_intersecting_line(*index, &world.vertices, &world.lines) {
                    if let Some(line) = world.lines.get_mut(*index) {
                        let new_line = line.split(intersection, &mut world.vertices);
                        let n = world.lines.find_or_insert(new_line);
                        added_lines.push(n);
                    }
                    if let Some(line) = world.lines.get_mut(other_index) {
                        let new_line = line.split(intersection, &mut world.vertices);
                        let n = world.lines.find_or_insert(new_line);
                        added_lines.push(n);
                    }

                    no_more_intersections = false;
                }
            }

            for line in added_lines.drain(..) {
                lines.push(line);
            }

            if no_more_intersections {
                break;
            }
        }
        
       
       /* let mut vertices = Vec::new();
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
       */

        self.vertices.clear();
    }
}
