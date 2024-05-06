// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
  }

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![greet])
  .setup(|app| {
      let win = app.get_window("main").unwrap();
      // win.set_resizable(false);
      // win.set_decorations(false);
      // win.set_always_on_top(true);
      // win.set_fullscreen(true);

      let app_handle = app.app_handle();

      // 监听窗口事件
      // win.on_window_event(move |event| match event {
      //     WindowEvent::Focused(focused) => {
      //         if !focused {
      //             // 当窗口失去焦点时，将其重新聚焦
      //             app_handle.get_window("main").unwrap().set_focus().unwrap();
      //         }
      //     }
      //     _ => {}
      // });

      Ok(())
  })
  .run(tauri::generate_context!())
  .expect("error while running tauri application");

    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
