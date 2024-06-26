use std::sync::Mutex;
use tauri::{command, State};
use crate::dal::board::{create_board, get_board_id, list_screen};


use crate::commands::models::{IdManager, BoradNames};
use crate::dal::board_object::BoardObject;
use crate::dal::board_object_model::headline::Headline;
use crate::dal::board_object_model::sticky_note::StickyNote;
use crate::dal::board_object::get_database_path;


use super::models::HeadlineIdManager;

#[command]
pub fn get_note_id_command(id_manager: State<'_, Mutex<IdManager>>) -> i64 {
  let mut id_manager = id_manager.lock().unwrap();
  id_manager.get_next_id()
}


#[command]
pub fn get_headline_id_command(headline_id_manager: State<'_, Mutex<HeadlineIdManager>>) -> i64 {
  let mut id_manager = headline_id_manager.lock().unwrap();
  id_manager.get_next_id()
}

#[command]
pub fn save_note_info_command(
  db_board_name: State<'_, Mutex<BoradNames>>,
  id: String,
  left: String, 
  top: String, 
  width: String, 
  height: String, 
  memo: String) -> String {
    let binding = db_board_name.lock().unwrap();
    let board_name = &binding.board_name;
    let db_name = get_database_path();
    let board_id = get_board_id(&db_name, board_name).unwrap();
    let sticky_note = StickyNote {
      id: id.parse::<i32>().unwrap(),
      board_id,
      left,
      top,
      width,
      height,
      memo: Some(memo),
    };
    let db_name = get_database_path();
    match sticky_note.save(&db_name){
      Ok(_) => "success".to_string(),
      Err(e) => format!("error: {}", e)
    }
}

#[command]
pub fn save_headline_info_command(
  db_board_name: State<'_, Mutex<BoradNames>>,
  id: String,
  left: String, 
  top: String, 
  width: String, 
  height: String, 
  headline: String) -> String {
    let binding = db_board_name.lock().unwrap();
    let board_name = &binding.board_name;
    let db_name = get_database_path();
    let board_id = get_board_id(&db_name, board_name).unwrap();
    let head_line = Headline {
      id: id.parse::<i32>().unwrap(),
      board_id,
      left,
      top,
      width,
      height,
      headline: Some(headline),
    };

    let db_name = get_database_path();
    match head_line.save(&db_name){
      Ok(_) => "success".to_string(),
      Err(e) => format!("error: {}", e)
    }
}


#[command]
pub fn get_note_info_command(db_board_name: State<'_, Mutex<BoradNames>> , id: i32) -> StickyNote {
  let binding = db_board_name.lock().unwrap();
  let board_name = &binding.board_name;
  let db_name = get_database_path();
  let board_id = get_board_id(&db_name, board_name).unwrap();
  let db_name = get_database_path();
  let result = StickyNote::get(&db_name, board_id, id);
  match result {
    Ok(sticky_note) => {
      sticky_note
    },
    Err(e) => panic!("Error: {}", e)
  }
}


#[command]
pub fn get_headline_info_command(db_board_name: State<'_, Mutex<BoradNames>> , id: i32) -> Headline {
  let binding = db_board_name.lock().unwrap();
  let board_name = &binding.board_name;
  let db_name = get_database_path();
  let board_id = get_board_id(&db_name, board_name).unwrap();
  let result = Headline::get(&db_name, board_id, id);
  match result {
    Ok(sticky_note) => {
      sticky_note
    },
    Err(e) => panic!("Error: {}", e)
  }
}

#[command]
pub fn delete_note_command(db_board_name: State<'_, Mutex<BoradNames>> , id: i32) -> String {
  let binding = db_board_name.lock().unwrap();
  let board_name = &binding.board_name;
  let db_name = get_database_path();
  let board_id = get_board_id(&db_name, board_name).unwrap();
  match StickyNote::delete(&db_name, board_id, id) {
    Ok(_) => "success".to_string(),
    Err(e) => format!("error: {}", e)
  }
}

#[command]
pub fn delete_headline_command(db_board_name: State<'_, Mutex<BoradNames>> , id: i32) -> String {
  let binding = db_board_name.lock().unwrap();
  let board_name = &binding.board_name;
  let db_name = get_database_path();
  let board_id = get_board_id(&db_name, board_name).unwrap();
  
  match Headline::delete(&db_name, board_id, id) {
    Ok(_) => "success".to_string(),
    Err(e) => format!("error: {}", e)
  }
}

#[command]
pub fn create_screen_command(board: String) -> String {
  let db_name = get_database_path();
  match create_board(&db_name, &board) {
    Ok(_) => "success".to_string(),
    Err(e) => format!("error: {}", e)
  }
}

#[command]
pub fn list_screens_command() -> Vec<String>{
  let db_name = get_database_path();
  list_screen(&db_name)
}



#[command]
pub fn load_notes_from_screen_command(
  db_board_name: State<'_, Mutex<BoradNames>>,  id_manager: State<'_, Mutex<IdManager>>, board: &str) -> Vec<StickyNote> {
  let _ = id_manager.lock().unwrap().change_table(board);
  let _ = db_board_name.lock().unwrap().change_table(board);
  let binding = db_board_name.lock().unwrap();
  let board_name = &binding.board_name;
  let db_name = get_database_path();
  let board_id = get_board_id(&db_name, board_name).unwrap();
  let result = StickyNote::get_all_in_board(&db_name, board_id);
  match result {
    Ok(sticky_notes) => sticky_notes,
    Err(e) => panic!("Error: {}", e)
  }
}

#[command]
pub fn load_headlines_from_screen_command(
  db_board_name: State<'_, Mutex<BoradNames>>,  headline_id_manager: State<'_, Mutex<HeadlineIdManager>>, board: &str) -> Vec<Headline> {
  let _ = headline_id_manager.lock().unwrap().change_table(board);
  let _ = db_board_name.lock().unwrap().change_table(board);
  let binding = db_board_name.lock().unwrap();
  let board_name = &binding.board_name;
  let db_name = get_database_path();
  let board_id = get_board_id(&db_name, board_name).unwrap();
  let result = Headline::get_all_in_board(&db_name, board_id);
  match result {
    Ok(headline) => headline,
    Err(e) => panic!("Error: {}", e)
  }
}

#[command]
pub fn is_board_name_exist_command(board: &str) -> bool {
  let db_name = get_database_path();
  let result = get_board_id(&db_name, board);
  result.is_ok()
}



