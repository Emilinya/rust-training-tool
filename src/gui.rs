//! Gui and main loop functionality.

use eframe::{egui, egui::Key};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

/// The main function where your game logic is ran each frame.
/// Takes in a `Context`.
pub trait MainFunction: FnMut(Context) {}
impl<T: FnMut(Context)> MainFunction for T {}

/// The frame context the main logic is called with. Contains
/// everything you need to update entities and render a frame.
pub struct Context {
    /// The time elapsed since last frame.
    pub dt: Duration,
    /// A set of all keys currently held down.
    pub key_map: HashSet<Key>,
    /// A painter which lets you draw shapes and text to the window.
    pub painter: egui::Painter,
    /// The size of the drawable region og the window in pixels.
    pub drawable_area: egui::Rect,
}

struct Gui<F: MainFunction> {
    main_function: F,
    last_update: Instant,
}

impl<F: MainFunction> Gui<F> {
    fn new(main_function: F) -> Self {
        Self {
            main_function,
            last_update: Instant::now(),
        }
    }
}

/// The function which starts a window and calls the `main_function` each frame.
/// `options` are passed to  `eframe::run_native`.
///
/// ### Example
///
/// ```
/// # let _ = "
/// use rust_training_tool::gui::run;
/// # ";
/// # use rust_training_tool::gui::__test_run as run;
/// use rust_training_tool::{eframe, egui};
///
/// let options = eframe::NativeOptions {
///     viewport: egui::ViewportBuilder::default()
///         .with_inner_size([480.0, 360.0]),
///     ..Default::default()
/// };
/// run(options, |ctx| {
///     ctx.painter.circle_filled(
///         ctx.drawable_area.center(),
///         ctx.drawable_area.width() * 0.1,
///         egui::Color32::ORANGE,
///     );
/// })
/// .expect("Can't start game!");
/// ```
pub fn run<F>(options: eframe::NativeOptions, main_function: F) -> eframe::Result
where
    F: MainFunction,
{
    eframe::run_native(
        "rustout",
        options,
        Box::new(|_cc| Ok(Box::new(Gui::new(main_function)))),
    )
}

/// Fake run function for use in doctests
pub fn __test_run<F>(_options: eframe::NativeOptions, _main_function: F) -> eframe::Result
where
    F: MainFunction,
{
    Ok(())
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

                (self.main_function)(Context {
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
