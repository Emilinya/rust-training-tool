//! `rust_training_tool` is a tool that makes game dev with eframe easy.
//! Warning: eframe is not made for game dev. If you are thinking about using
//! this crate to create a game: don't.  
//!
//! ## Feature flags
//!
//! To enable the use of images through `egui::Image`, enable the `images` feature by replacing
//! `rust-training-tool = "x.y.z"` with `rust-training-tool = { version = "x.y.z", features = ["images"] }`

pub mod collision;
pub mod gui;

// Re-export all useful libraries:
pub use {eframe, eframe::egui};
