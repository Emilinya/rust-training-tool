use eframe::{egui, egui::Key};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

pub trait MainFunction: FnMut(Context) {}
impl<T: FnMut(Context)> MainFunction for T {}

pub struct Context {
    pub dt: Duration,
    pub key_map: HashSet<Key>,
    pub painter: egui::Painter,
    pub drawable_area: egui::Rect,
}

struct Gui<F: MainFunction> {
    main_loop: F,
    last_update: Instant,
}

impl<F: MainFunction> Gui<F> {
    fn new(main_loop: F) -> Self {
        Self {
            main_loop,
            last_update: Instant::now(),
        }
    }
}

pub fn run<F>(options: eframe::NativeOptions, main_loop: F) -> eframe::Result
where
    F: MainFunction,
{
    eframe::run_native(
        "rustout",
        options,
        Box::new(|_cc| Ok(Box::new(Gui::new(main_loop)))),
    )
}

impl<F: MainFunction> eframe::App for Gui<F> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // update dt
        let dt = self.last_update.elapsed();
        self.last_update = Instant::now();

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                // create painter
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), egui::Sense::hover());

                let key_map = ctx.input(|i| i.keys_down.clone());

                (self.main_loop)(Context {
                    dt,
                    key_map,
                    drawable_area: response.rect,
                    painter,
                });
            });

        // update at 30 fps
        ctx.request_repaint_after_secs(1.0 / 30.0);
    }
}
