#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod dal;

use std::sync::Mutex;

use commands::models::{BoradNames, HeadlineIdManager, IdManager};
use dal::board_object::create_db_tables;





use crate::commands::commands::*;
fn main() {
  
  create_db_tables().expect("Failed to create tables");
  let id_manager = IdManager::new().expect("Failed to initialize IdManager");
  let headline_id_manager = HeadlineIdManager::new().expect("Failed to initialize HeadlineIdManager");
  let db_board_name = BoradNames::new().expect("Failed to initialize BoradNames");
    tauri::Builder::default()
        .manage(Mutex::new(id_manager))
        .manage(Mutex::new(headline_id_manager))
        .manage(Mutex::new(db_board_name))
        .invoke_handler(tauri::generate_handler![
          get_note_id_command,
          get_headline_id_command,
          save_note_info_command,
          save_headline_info_command,
          get_note_info_command,
          get_headline_info_command,
          load_notes_from_screen_command,
          load_headlines_from_screen_command,
          list_screens_command,
          is_board_name_exist_command,
          create_screen_command,
          delete_note_command,
          delete_headline_command,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


