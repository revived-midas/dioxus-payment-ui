//! trunk only lets main.rs, not any binary
//!
//!
use myproject::*;

#[cfg(target_arch = "wasm32")]
fn main() {
    dioxus_web::launch(app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    dioxus_desktop::launch(app);
}