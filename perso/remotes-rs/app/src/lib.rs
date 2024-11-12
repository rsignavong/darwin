mod app;

use app::App;

use log::Level;
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn main(parent_id: Option<String>, app_name: Option<String>) -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    if let Err(_) = console_log::init_with_level(Level::Trace) {
        log::warn!("console.log already initialized");
    }

    let gizmo = Gizmo::from(App::try_from(app_name)?);
    let view = View::from(gizmo.view_builder());

    if let Some(id) = parent_id {
        if let Some(parent) = utils::document().get_element_by_id(&id) {
            view.run_in_container(&parent)
        } else {
            Ok(())
        }
    } else {
        Err(JsValue::from_str("parent_id is missing"))
    }
}
