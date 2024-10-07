//! `rust_training_tool` is a tool that makes game dev with eframe easy.
//! Warning: eframe is not made for game dev. If you are thinking about using
//! this crate to create a game: don't.  

pub mod collision;
pub mod gui;

// Re-export all useful libraries:
pub use {eframe, eframe::egui};
