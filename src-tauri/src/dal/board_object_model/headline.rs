use crate::dal::db::connection::*;
use crate::dal::board_object::{BoardObject, ObjectType};
use diesel::result::Error;
use diesel::{Connection, SqliteConnection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Headline {
  pub id: i32,
  pub board_id: i32,
  pub left: String,
  pub top: String,
  pub width: String,
  pub height: String,
  pub headline: Option<String>,
}

impl BoardObject for Headline {
  const OBJECT_TYPE: ObjectType = ObjectType::Headline;

    fn create_table_if_not_exists(db_name: &str) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      create_headline_table_if_not_exists(&mut conn)
    }

    fn exists_id_in_db(db_name: &str, board_id:i32, headline_id: i32) -> bool {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      exists_headline_id(&mut conn, board_id, headline_id)
    }

    fn create(self, db_name: &str) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      create_headline(&mut conn, self.id, self.board_id, self.left, self.top, self.width, self.height, self.headline, Self::OBJECT_TYPE.to_number())
    }

    fn update(self, db_name: &str) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      update_headline(&mut conn, self.id, self.board_id, self.left, self.top, self.width, self.height, self.headline, Self::OBJECT_TYPE.to_number())
    }

    fn save(self, db_name: &str) -> Result<usize, Error> {
    
      if Self::exists_id_in_db(db_name, self.board_id, self.id)
      {
       return  self.update(db_name);
      } 
      self.create(db_name)
      
    }
    fn delete(db_name: &str, board_id: i32, board_object_id: i32) -> Result<usize, Error> {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      delete_object(&mut conn, board_id, Self::OBJECT_TYPE.to_number(), board_object_id)
    }

  fn get(db_name: &str, board_id: i32, headline_id: i32) -> Result<Headline, Error> {
    let mut conn = SqliteConnection::establish(db_name).unwrap();
    let object_info = get_object_info(&mut conn, board_id, Self::OBJECT_TYPE.to_number(),  headline_id)?;
    let headline_info = get_headline_info(&mut conn, board_id, headline_id)?;
    let headline = Headline {
      id: headline_id,
      board_id,
      left: object_info.left,
      top: object_info.top,
      width: object_info.width,
      height: object_info.height,
      headline: Some(headline_info.headline),
    };
    Ok(headline)
  }

  fn get_all_in_board(db_name: &str, board_id: i32) -> Result<Vec<Headline>, Error> {
    let mut conn = SqliteConnection::establish(db_name).unwrap();
    let mut headlines = Vec::new();
    let object_infos = get_object_infos_in_board(&mut conn, board_id, Self::OBJECT_TYPE.to_number())?;
    for object_info in object_infos {
      let headline_info = get_headline_info(&mut conn, board_id, object_info.object_id)?;
      let headline = Headline {
        id: object_info.object_id,
        board_id,
        left: object_info.left,
        top: object_info.top,
        width: object_info.width,
        height: object_info.height,
        headline: Some(headline_info.headline),
      };
      headlines.push(headline);
    }
    Ok(headlines)
  }

    fn get_max_object_id_on_board(db_name: &str, board_id: i32) -> i32 {
      let mut conn = SqliteConnection::establish(db_name).unwrap();
      get_max_object_id_on_board(&mut conn, board_id, Self::OBJECT_TYPE.to_number())
    } 

}


#[cfg(test)]
mod headline_test{


use crate::dal::{board::create_board, board_object_model::headline::*};
  
  #[test]
  fn test_save_note(){

    let test_db = "test_save_headline.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = Headline::create_table_if_not_exists(test_db);

    let id = 1;
    let left = "100".to_string();
    let top = "200".to_string();
    let width = "300".to_string();
    let height = "400".to_string();
    let test_headline = "Test headline".to_string();
    let query_board_id = 1;
    let test_headline = Headline {
      id,
      board_id: query_board_id,
      left,
      top,
      width,
      height,
      headline: Some(test_headline.clone()),
    };

    let save_result = test_headline.save(test_db);
    match save_result {
      Ok(_) => {},
      Err(e) => panic!("Error: {}", e)
    }

    let test_headline = Headline::get(test_db, query_board_id, id).unwrap();
    assert_eq!(test_headline.id, 1);
    assert_eq!(test_headline.left, "100");
    assert_eq!(test_headline.top, "200");
    assert_eq!(test_headline.width, "300");
    assert_eq!(test_headline.height, "400");
    assert_eq!(test_headline.headline.unwrap(), "Test headline");
    drop(conn);

    let _ = std::fs::remove_file(test_db);
  }

  

#[test]
fn test_get_all_board_names() {
  
  let test_db = "test_get_all_board_names.sqlite";
  let mut conn = SqliteConnection::establish(test_db).unwrap();
  let _ = create_board_table_if_not_exists(&mut conn);
  let _ = create_board_object_table_if_not_exists(&mut conn);
  let _ = Headline::create_table_if_not_exists(test_db);
  let test_board_name = "test_get_all_board_names";

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
  fn test_get_all_headline(){

    let test_db = "test_get_all_headline.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = Headline::create_table_if_not_exists(test_db);
    let test_board_name = "test_get_all_headline";
    let _ = create_board(test_db, test_board_name);
    let board_id = get_board_id(&mut conn, test_board_name).unwrap();
    let headlines = Headline::get_all_in_board(test_db, board_id).unwrap();
    assert_eq!(headlines.len(), 0);
    let id = 1;
    let left = "100".to_string();
    let top: String = "200".to_string();
    let width = "300".to_string();
    let height = "400".to_string();
    let test_headline_text = "Test headline".to_string();
    let test_headline = Headline {
      id,
      board_id,
      left: left.clone(),
      top: top.clone(),
      width: width.clone(),
      height: height.clone(),
      headline: Some(test_headline_text.clone()),
    };
    let _ = test_headline.save(test_db);

    let headlines = Headline::get_all_in_board(test_db, board_id).unwrap();
    assert_eq!(headlines.len(), 1);
    assert_eq!(headlines[0].id, 1);
    assert_eq!(headlines[0].left, "100");
    assert_eq!(headlines[0].top, "200");
    assert_eq!(headlines[0].width, "300");
    assert_eq!(headlines[0].height, "400");
    assert_eq!(headlines[0].headline.as_ref().unwrap(), "Test headline");

    drop(conn);
    let _ = std::fs::remove_file(test_db);
  }


}