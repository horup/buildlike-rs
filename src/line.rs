use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: Index,
    pub end: Index,
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end) || (self.start == other.end && self.end == other.start) 
    }
}

impl Line {

    pub fn split(&mut self, mid:Vertex, vertices:&mut Collection<Vertex>) -> Line {
        let end = self.end;
        let mid = vertices.find_or_insert(mid);
        self.end = mid;
        Line { start: mid, end: end }
    }
    
    pub fn vertices(&self, vertices:&Collection<Vertex>) -> Option<(Vertex, Vertex)> {
        if let [Some(v1), Some(v2)] = [vertices.get(self.start), vertices.get(self.end)] {
            return Some((*v1, *v2));
        }

        return None;
    }

    pub fn as_geo_line(&self, vertices:&Collection<Vertex>) -> Option<geo::Line<f32>> {
        if let Some((v1, v2)) = self.vertices(vertices) {
            return Some(geo::Line {
                start:geo::Coordinate {
                    x: v1.x,
                    y: v1.y,
                },
                end:geo::Coordinate {
                    x: v2.x,
                    y: v2.y,
                }
            })
        }
        None
    }

    pub fn to_geo_line(&self, vertices:&Collection<Vertex>) -> Option<geo::Line<f32>> {
        if let Some((v1, v2)) = self.vertices(vertices) {
            let line = geo::Line {
                start: geo::Coordinate {x:v1.x, y:v1.y},
                end:   geo::Coordinate {x:v2.x, y:v2.y}
            };

            return Some(line);
        }

        None
    }

    pub fn intersects(&self, other:&Line, vertices:&Collection<Vertex>) -> Option<Vertex> {
        if let (Some(line1), Some(line2)) = (self.to_geo_line(vertices), other.to_geo_line(vertices)) {
            if let Some(intersection) = geo::line_intersection::line_intersection(line1, line2) {
                match intersection {
                    geo::line_intersection::LineIntersection::SinglePoint { intersection, is_proper } => {
                        if is_proper {
                            return Some(Vertex::new(intersection.x, intersection.y));
                        }
                    },
                    geo::line_intersection::LineIntersection::Collinear { intersection:_ } => {

                    },
                }
            }
        }
        None
    } 
}

pub fn find_first_intersecting_line(line_index:Index, vertices:&Collection<Vertex>, lines:&Collection<Line>) -> Option<(Index, Vertex)> {
    if let Some(line) = lines.get(line_index) {
        for (other_index, other_line) in lines.iter() {
            if line_index != other_index {
                if let Some(res) = line.intersects(other_line, &vertices) {
                    return Some((other_index, res)); 
                }
            }
        }
    }
    
    None
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut vertices:Collection<Vertex> = Collection::default();
        let mut lines:Collection<Line> = Collection::default();
        let mut line1 = Line {
            start: vertices.insert(Vertex::new(0.0, 0.0)),
            end: vertices.insert(Vertex::new(10.0, 10.0))
        };

        let line2 = Line {
            start: vertices.insert(Vertex::new(0.0, 10.0)),
            end: vertices.insert(Vertex::new(10.0, 0.0))
        };

        let line3 = Line {
            start: vertices.insert(Vertex::new(20.0, 20.0)),
            end: vertices.insert(Vertex::new(20.0, 30.0)),
        };

        //lines.insert(value)

        let res = line1.intersects(&line2, &vertices);
        assert_eq!(res.is_some(), true);

        let res = line1.intersects(&line3, &vertices);
        assert_eq!(res.is_some(), false);

        let res = line1.intersects(&line2, &vertices);
        let res = res.unwrap();

        line1.split(Vertex::new(5.0, 5.0), &mut vertices);
        assert_eq!(vertices.len(), 7);
    }
}
