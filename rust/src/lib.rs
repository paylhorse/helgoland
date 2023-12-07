// ------ GODOT ENTRY POINT ------
use godot::prelude::*;
use godot_typst::TypstTextureRect;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

// ------ MODULE IMPORT ------
mod blackboard;
mod hagoromo;
