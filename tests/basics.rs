use buildlike_rs::{Designer, Vertex, World};

#[test]
pub fn basic(){
    let mut world = World::new();
    let mut designer = Designer::new();

    // make a triangle sector
    designer.push_vertex(Vertex::new(0.0, 0.0));
    designer.push_vertex(Vertex::new(1.0, 0.0));
    designer.push_vertex(Vertex::new(1.0, 1.0));
    designer.commit(&mut world);
    assert_eq!(world.vertices.len(), 3);
    assert_eq!(world.lines.len(), 3);

    
    world = World::new();
    designer.push_vertex(Vertex::new(0.0, 0.0));
    designer.push_vertex(Vertex::new(1.0, 0.0));
    designer.commit(&mut world);
    assert_eq!(world.lines.len(),  1);
    assert_eq!(world.vertices.len(), 2);
    designer.push_vertex(Vertex::new(0.5, -1.0));
    designer.push_vertex(Vertex::new(0.5, 1.0));
    designer.commit(&mut world);

    // 4 lines and 5 vertices, due to splitting
   // assert_eq!(world.lines.len(),  4);
    assert_eq!(world.vertices.len(), 5);
}