
use crate::dal::db::connection::*;
use crate::dal::board_object::{BoardObject, ObjectType};
use diesel::result::Error;
use diesel::{Connection, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StickyNote {
  pub id: i32,
  pub board_id: i32,
  pub left: String,
  pub top: String,
  pub width: String,
  pub height: String,
  pub memo: Option<String>,
}

impl BoardObject for StickyNote {
  const OBJECT_TYPE: ObjectType = ObjectType::StickyNote;

    fn create_table_if_not_exists(db_name: &str) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      create_sticky_note_table_if_not_exists(&mut conn)
    }

    fn exists_id_in_db(db_name: &str, board_id:i32, sticky_note_id: i32) -> bool {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      exists_sticky_note_id(&mut conn, board_id, sticky_note_id)
    }

    fn create(self, db_name: &str) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      create_sticky_note(&mut conn, self.id, self.board_id, self.left, self.top, self.width, self.height, self.memo, Self::OBJECT_TYPE.to_number())
    }

    fn update(self, db_name: &str) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      update_sticky_note(&mut conn, self.id, self.board_id, self.left, self.top, self.width, self.height, self.memo, Self::OBJECT_TYPE.to_number())
    }

    fn save(self, db_name: &str) -> Result<usize, Error> {
    
      if Self::exists_id_in_db(db_name, self.board_id, self.id)
      {
       return self.update(db_name)
      } 
      self.create(db_name)
      
    }
    fn delete(db_name: &str, board_id: i32, board_object_id: i32) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      delete_object(&mut conn, board_id, Self::OBJECT_TYPE.to_number(), board_object_id)
    }

  fn get(db_name: &str, board_id: i32, sticky_note_id: i32) -> Result<StickyNote, Error> {
    let mut conn = SqliteConnection::establish(db_name).unwrap();
    let object_info = get_object_info(&mut conn, board_id, Self::OBJECT_TYPE.to_number(),  sticky_note_id)?;
    let sticky_note_info = get_sticky_note_info(&mut conn, board_id, sticky_note_id)?;
    let sticky_note = StickyNote {
      id: sticky_note_id,
      board_id,
      left: object_info.left,
      top: object_info.top,
      width: object_info.width,
      height: object_info.height,
      memo: sticky_note_info.memo,
    };
    Ok(sticky_note)
  }

  fn get_all_in_board(db_name: &str, board_id: i32) -> Result<Vec<StickyNote>, Error> {
    let mut conn = SqliteConnection::establish(db_name).unwrap();
    let mut sticky_notes = Vec::new();
    let object_infos = get_object_infos_in_board(&mut conn, board_id, Self::OBJECT_TYPE.to_number())?;
    for object_info in object_infos {
      let sticky_note_info = get_sticky_note_info(&mut conn, board_id, object_info.object_id)?;
      let sticky_note = StickyNote {
        id: object_info.object_id,
        board_id,
        left: object_info.left,
        top: object_info.top,
        width: object_info.width,
        height: object_info.height,
        memo: sticky_note_info.memo,
      };
      sticky_notes.push(sticky_note);
    }
    Ok(sticky_notes)
  }

    
    fn get_max_object_id_on_board(db_name: &str, board_id: i32) -> i32 {
      let mut conn: SqliteConnection = SqliteConnection::establish(db_name).unwrap();
      get_max_object_id_on_board(&mut conn, board_id, Self::OBJECT_TYPE.to_number())
    } 

}


  #[cfg(test)]
  mod sticky_note_test{


use crate::dal::board::create_board;

use super::*;
    
    #[test]
    fn test_save_note(){
  
      let test_db = "test_save_note_position.sqlite";
      let mut conn = SqliteConnection::establish(test_db).unwrap();
      let _ = create_board_table_if_not_exists(&mut conn);
      let _ = create_board_object_table_if_not_exists(&mut conn);
      let _ = StickyNote::create_table_if_not_exists(test_db);
  
      let id = 1;
      let left = "100".to_string();
      let top = "200".to_string();
      let width = "300".to_string();
      let height = "400".to_string();
      let memo = "Test memo".to_string();
      let query_board_id = 1;
      let sticky_note = StickyNote {
        id,
        board_id: query_board_id,
        left,
        top,
        width,
        height,
        memo: Some(memo.clone()),
      };

      let save_result = sticky_note.save(test_db);
      match save_result {
        Ok(_) => {},
        Err(e) => panic!("Error: {}", e)
      }
  
      let note = StickyNote::get(test_db, query_board_id, id).unwrap();
      assert_eq!(note.id, 1);
      assert_eq!(note.left, "100");
      assert_eq!(note.top, "200");
      assert_eq!(note.width, "300");
      assert_eq!(note.height, "400");
      assert_eq!(note.memo.unwrap(), "Test memo");
      drop(conn);

      let _ = std::fs::remove_file(test_db);
    }
  
    
  
  #[test]
  fn test_get_all_borad_names() {

    let test_db = "test_get_all_board_names.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = StickyNote::create_table_if_not_exists(test_db);
    let test_board_name = "test_get_all_borad_names";
  
    let board_names = get_all_board_names(&mut conn).unwrap();
    assert_eq!(board_names.len(), 0);
    let _ = create_board(test_db, test_board_name);
    let board_names = get_all_board_names(&mut conn).unwrap();
    assert_eq!(board_names.len(), 1);
    assert_eq!(board_names[0], test_board_name);

    drop(conn);
    let _ = std::fs::remove_file(test_db);
  }
  
  
    #[test]
    fn test_get_all_note_position(){
      let test_db = "test_get_all_note.sqlite";
      let mut conn = SqliteConnection::establish(test_db).unwrap();
      let _ = create_board_table_if_not_exists(&mut conn);
      let _ = create_board_object_table_if_not_exists(&mut conn);
      let _ = StickyNote::create_table_if_not_exists(test_db);
      let test_board_name = "test_get_all_note_board";
      let _ = create_board(test_db, test_board_name);
      let board_id = get_board_id(&mut conn, test_board_name).unwrap();
      let notes = StickyNote::get_all_in_board(test_db, board_id).unwrap();
      assert_eq!(notes.len(), 0);
      let id = 1;
      let left = "100".to_string();
      let top: String = "200".to_string();
      let width = "300".to_string();
      let height = "400".to_string();
      let memo = "Test memo".to_string();
      let sticky_note = StickyNote {
        id,
        board_id,
        left: left.clone(),
        top: top.clone(),
        width: width.clone(),
        height: height.clone(),
        memo: Some(memo.clone()),
      };
      let _ = sticky_note.save(test_db);
  
      let notes = StickyNote::get_all_in_board(test_db, board_id).unwrap();
      assert_eq!(notes.len(), 1);
      assert_eq!(notes[0].id, 1);
      assert_eq!(notes[0].left, "100");
      assert_eq!(notes[0].top, "200");
      assert_eq!(notes[0].width, "300");
      assert_eq!(notes[0].height, "400");
      assert_eq!(notes[0].memo.as_ref().unwrap(), "Test memo");
  
      drop(conn);
      let _ = std::fs::remove_file(test_db);
    }
  
  
  }