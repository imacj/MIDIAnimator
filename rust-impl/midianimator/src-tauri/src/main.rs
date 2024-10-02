// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
#![allow(dead_code)]

use MIDIAnimator::ipc::start_server;
use MIDIAnimator::state::{WINDOW, update_state};
use MIDIAnimator::ui::menu;

use tauri::{generate_context, Manager};
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String
}

#[tokio::main]
async fn main() {
    // emit the global state to the frontend
    let context = generate_context!();

    let devtools = devtools::init(); // initialize the plugin as early as possible

    tauri::Builder::default()
    .plugin(devtools)
    .invoke_handler(MIDIAnimator::auto_commands::get_cmds())  // AUTO GENERATED BY build.rs
    .setup(|app| {
        // update the global state with the window
        let window = app.get_window("main").unwrap();
        *WINDOW.lock().unwrap() = Some(window);
        
        // start server and update the state
        tauri::async_runtime::spawn(async move {
            update_state();  // send update to frontend for initial state
            start_server();  // start the IPC server
        });
        
        return Ok(())
    })
    .menu(menu::build_menu(&context.package_info().name))
    .on_menu_event(|event| {
        menu::handle_menu_event(event);
    })
    .run(context)
    .expect("error while launching MIDIAnimator!");
}