use buildlike_rs::{Designer, Vertex, World};
use eframe::{egui::{CentralPanel, Color32, Key, Pos2, Rect, Stroke, Vec2}, epi};


struct App {
    pub designer:Designer,
    pub world:World
}

impl Default for App {
    fn default() -> Self {
        let mut world = World::default();
        let mut designer = Designer::default();

        designer.push_vertex(Vertex::new(100.0, 100.0));
        designer.push_vertex(Vertex::new(300.0, 100.0));
        designer.push_vertex(Vertex::new(300.0, 300.0));

        designer.commit(&mut world);

        Self { 
            designer, 
            world 
        }
    }
}

impl epi::App for App {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui|{
            if let Some(pos) = ui.input().pointer.hover_pos() {
                ui.label(format!("{:?}", pos));
            }
            
            let world = &mut self.world;
            let designer = &mut self.designer;

            // clicked
            if ui.input().pointer.any_pressed() {
                if let Some(pos) = ui.input().pointer.press_origin() {
                    designer.push_vertex(Vertex::new(pos.x, pos.y));
                }
            }

            // commit
            if ui.input().key_pressed(Key::Space) {
                designer.commit(world);
            }

            let p = ui.painter();
            // world
            // paint vertices
            for (_, vertex) in world.vertices.iter() {
                p.circle(Pos2::new(vertex.x, vertex.y), 3.0, Color32::WHITE, Stroke::none());
            }

            // world
            // paint lines
            for (_, line) in world.lines.iter() {
                if let (Some(v1), Some(v2)) = (world.vertices.get(line.start), world.vertices.get(line.end)) {
                    p.line_segment([Pos2::new(v1.x, v1.y), Pos2::new(v2.x, v2.y)], Stroke::new(1.0, Color32::WHITE));
                }
            }

            // designer
            // paint vertices in designer
            for v in designer.vertices.iter() {
                p.circle(Pos2::new(v.x, v.y), 3.0, Color32::RED, Stroke::none());
            }
        });
    }

    fn name(&self) -> &str {
        "EGUI Buildlike"
    }
}


pub fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(App::default()), options);
}