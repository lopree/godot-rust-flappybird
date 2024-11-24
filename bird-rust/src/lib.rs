mod game_manager;
mod player;
mod bg_pipe;
mod coin;
use godot::prelude::*;

pub struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
