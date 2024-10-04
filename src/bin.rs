use eframe::{egui, egui::Key};
use rust_training_tool::{gui, gui::Context};

struct Player {
    pos: egui::Vec2,
}

impl Player {
    pub fn draw(&mut self, ctx: &Context) {
        let scale = ctx.drawable_area.size().min_elem();
        let dt = ctx.dt.as_secs_f32();

        if ctx.key_map.contains(&Key::A) {
            self.pos.x -= 0.5 * scale * dt;
        }
        if ctx.key_map.contains(&Key::D) {
            self.pos.x += 0.5 * scale * dt;
        }
        if ctx.key_map.contains(&Key::W) {
            self.pos.y -= 0.5 * scale * dt;
        }
        if ctx.key_map.contains(&Key::S) {
            self.pos.y += 0.5 * scale * dt;
        }

        ctx.painter.circle_filled(
            ctx.drawable_area.center() + self.pos,
            scale * 0.1,
            egui::Color32::ORANGE,
        );
    }
}

fn main() {
    let mut player = Player {
        pos: egui::Vec2::new(0.0, 0.0),
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 360.0]),
        ..Default::default()
    };
    gui::run(options, |ctx| {
        player.draw(&ctx);
    })
    .unwrap();
}
