mod async_manager;
mod cef;
mod chat;
mod entity_manager;
mod error;
mod helpers;
mod logger;
mod macros;
mod players;
mod plugin;
mod search;

use self::plugin::Plugin;
use classicube_sys::*;
use log::debug;
use std::{cell::Cell, os::raw::c_int, ptr};

extern "C" fn init() {
    color_backtrace::install_with_settings(
        color_backtrace::Settings::new().verbosity(color_backtrace::Verbosity::Full),
    );

    logger::initialize(true, false);

    time!("Plugin::initialize()", 10000, {
        Plugin::initialize();
    });
}

extern "C" fn free() {
    debug!("Free");

    time!("Plugin::shutdown()", 10000, {
        Plugin::shutdown();
    });
}

thread_local!(
    static CONTEXT_LOADED: Cell<bool> = Cell::new(false);
);

extern "C" fn on_new_map_loaded() {
    time!("Plugin::on_new_map_loaded()", 10000, {
        Plugin::on_new_map_loaded();
    });
}

#[no_mangle]
pub static Plugin_ApiVersion: c_int = 1;

#[no_mangle]
pub static mut Plugin_Component: IGameComponent = IGameComponent {
    // Called when the game is being loaded.
    Init: Some(init),
    // Called when the component is being freed. (e.g. due to game being closed)
    Free: Some(free),
    // Called to reset the component's state. (e.g. reconnecting to server)
    Reset: None,
    // Called to update the component's state when the user begins loading a new map.
    OnNewMap: None,
    // Called to update the component's state when the user has finished loading a new map.
    OnNewMapLoaded: Some(on_new_map_loaded),
    // Next component in linked list of components.
    next: ptr::null_mut(),
};
