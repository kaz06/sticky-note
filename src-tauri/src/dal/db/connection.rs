extern crate diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::sql_query;
use diesel::RunQueryDsl;
use crate::dal::db::schema::boards::dsl::*;
use crate::dal::db::schema::board_objects::dsl::*;
use crate::dal::db::schema::sticky_notes::dsl::*;
use crate::dal::db::schema::headlines::dsl::*;



use super::models::BoardObjectTable;
use super::models::BoardTable;
use super::models::HeadlineTable;
use super::models::StickyNoteTable;





pub fn create_board_table_if_not_exists(conn: &mut SqliteConnection) -> QueryResult<usize> {
  let create_table_query = format!(
      "CREATE TABLE IF NOT EXISTS {} (
          id INTEGER PRIMARY KEY,
          name TEXT NOT NULL
      )", BoardTable::table_name());
  sql_query(create_table_query).execute(conn)
}

pub fn create_sticky_note_table_if_not_exists(conn: &mut SqliteConnection) -> QueryResult<usize> {
  let create_table_query = format!(
      "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER NOT NULL,
        board_id INTEGER NOT NULL,
        memo TEXT,
        UNIQUE(id, board_id)
      )", StickyNoteTable::table_name());
  sql_query(create_table_query).execute(conn)
}

pub fn create_board_object_table_if_not_exists(conn: &mut SqliteConnection) -> QueryResult<usize> {
  let create_table_query = format!(
      "CREATE TABLE IF NOT EXISTS {} (
        board_id INTEGER NOT NULL,
        left TEXT NOT NULL,
        top TEXT NOT NULL,
        width TEXT NOT NULL,
        height TEXT NOT NULL,
        object_type_number INTEGER NOT NULL,
        object_id INTEGER NOT NULL,
        UNIQUE(board_id, object_type_number, object_id)
      )", BoardObjectTable::table_name());
  sql_query(create_table_query).execute(conn)
}

pub fn create_headline_table_if_not_exists(conn: &mut SqliteConnection) -> QueryResult<usize> {
  let create_table_query = format!(
      "CREATE TABLE IF NOT EXISTS {} (
        id INTEGER NOT NULL,
        board_id INTEGER NOT NULL,
        headline TEXT,
        UNIQUE(id, board_id)
      )", HeadlineTable::table_name());
  sql_query(create_table_query).execute(conn)
}

pub fn insert_board_to_boards(conn: &mut SqliteConnection,  board: &str) -> QueryResult<usize> {
  let insert_board_query = format!(
    "INSERT INTO {} (name) VALUES ('{}')", BoardTable::table_name(), board);
    sql_query(insert_board_query).execute(conn)
}

pub fn create_sticky_note(
  conn: &mut SqliteConnection, 
  query_id: i32,
  query_board_id: i32,
  query_left: String,
  query_top: String,
  query_width: String,
  query_height: String,
  query_memo: Option<String>,
  object_number: i32) -> QueryResult<usize> {
  let board_object_table = BoardObjectTable {
    board_id: query_board_id,
    object_type_number: object_number,
    object_id: query_id,
    left: query_left,
    top: query_top,
    width: query_width,
    height: query_height,
  };
  let sticky_note_table = StickyNoteTable {
    id: query_id,
    board_id: query_board_id,
    memo: query_memo,
  };
  diesel::insert_into(board_objects)
    .values(&board_object_table) 
    .execute(conn)
    .and_then(|_| {
      diesel::insert_into(sticky_notes)
        .values(&sticky_note_table)
        .execute(conn)
    })
}

pub fn create_headline(
  conn: &mut SqliteConnection, 
  query_id: i32,
  query_board_id: i32,
  query_left: String,
  query_top: String,
  query_width: String,
  query_height: String,
  query_headline: Option<String>,
  object_number: i32) -> QueryResult<usize> {
  let board_object_table = BoardObjectTable {
    board_id: query_board_id,
    object_type_number: object_number,
    object_id: query_id,
    left: query_left,
    top: query_top,
    width: query_width,
    height: query_height,
  };
  let headline_table = HeadlineTable {
    id: query_id,
    board_id: query_board_id,
    headline: query_headline.unwrap(),
  };
  diesel::insert_into(board_objects)
    .values(&board_object_table) 
    .execute(conn)
    .and_then(|_| {
      diesel::insert_into(headlines)
        .values(&headline_table)
        .execute(conn)
    })
}


pub fn get_object_info(conn: &mut SqliteConnection, query_board_id: i32, query_object_type_number: i32, query_object_id: i32) -> QueryResult<BoardObjectTable> {
  use crate::dal::db::schema::board_objects::dsl::board_id;

  board_objects
    .filter(board_id.eq(query_board_id).and(object_type_number.eq(query_object_type_number)).and(object_id.eq(query_object_id)))
    .first(conn)
}

pub fn get_object_infos_in_board(conn: &mut SqliteConnection, query_board_id: i32, query_object_type_number: i32) -> QueryResult<Vec<BoardObjectTable>> {
  use crate::dal::db::schema::board_objects::dsl::board_id;
  board_objects
    .filter(board_id.eq(query_board_id).and(object_type_number.eq(query_object_type_number)))
    .load::<BoardObjectTable>(conn)
}

pub fn get_sticky_note_info(conn: &mut SqliteConnection, query_board_id: i32, query_sticky_note_id: i32) -> QueryResult<StickyNoteTable> {
  use crate::dal::db::schema::sticky_notes::dsl::id;
  use crate::dal::db::schema::sticky_notes::dsl::board_id;
  sticky_notes
    .filter(id.eq(query_sticky_note_id).and(board_id.eq(query_board_id)))
    .first(conn)
}


pub fn get_headline_info(conn: &mut SqliteConnection, query_board_id: i32, query_headline_id: i32) -> QueryResult<HeadlineTable> {
  use crate::dal::db::schema::headlines::dsl::board_id;
  use crate::dal::db::schema::headlines::dsl::id;
  headlines
    .filter(id.eq(query_headline_id).and(board_id.eq(query_board_id)))
    .first(conn)
}

pub fn get_board_id(conn: &mut SqliteConnection, query_board_name: &str) -> QueryResult<i32> {
  use crate::dal::db::schema::boards::dsl::id;
  boards
    .filter(name.eq(query_board_name))
    .select(id)
    .first(conn)
}

pub fn get_all_board_names(conn: &mut SqliteConnection) -> QueryResult<Vec<String>> {
  boards
    .select(name)
    .load::<String>(conn)
}

pub fn get_max_object_id_on_board(conn: &mut SqliteConnection, query_board_id: i32, object_number: i32) -> i32 {
  use crate::dal::db::schema::board_objects::dsl::board_id;
  let result = board_objects
    .filter(board_id.eq(query_board_id).and(object_type_number.eq(object_number)))
    .select(object_id)
    .load::<i32>(conn);
  match result {
    Ok(result_notes) => {
      let mut max_id = 0;
      for result_id in result_notes {
        if result_id > max_id {
          max_id = result_id;
        }
      }
      max_id
    },
    Err(_) => 0,
  }
}

pub fn delete_object(conn: &mut SqliteConnection, query_board_id: i32, object_number: i32, query_id: i32) -> QueryResult<usize> {
  diesel::delete(board_objects
    .filter(
      crate::dal::db::schema::board_objects::dsl::board_id.eq(query_board_id)
      .and(object_type_number.eq(object_number))
      .and(object_id.eq(query_id))
    )
  )
  .execute(conn)
}

pub fn exists_sticky_note_id(conn: &mut SqliteConnection, query_board_id:i32, query_id: i32) -> bool {
  use crate::dal::db::schema::sticky_notes::dsl::id;
  use crate::dal::db::schema::sticky_notes::dsl::board_id;

  let result = sticky_notes
    .filter(board_id.eq(query_board_id))
    .filter(id.eq(query_id))
    .load::<StickyNoteTable>(conn);
  match result {
    Ok(result_notes) => !result_notes.is_empty(),
    Err(_) => false,
  }
}

pub fn exists_headline_id(conn: &mut SqliteConnection, query_board_id:i32, query_id: i32) -> bool {
  use crate::dal::db::schema::headlines::dsl::id;
  use crate::dal::db::schema::headlines::dsl::board_id;

  let result = headlines
    .filter(board_id.eq(query_board_id))
    .filter(id.eq(query_id))
    .load::<HeadlineTable>(conn);
  match result {
    Ok(result_notes) => !result_notes.is_empty(),
    Err(_) => false,
  }
}

pub fn update_sticky_note(
  conn: &mut SqliteConnection, 
  query_id: i32,
  query_board_id: i32,
  query_left: String,
  query_top: String,
  query_width: String,
  query_height: String,
  query_memo: Option<String>,
  object_number: i32) -> QueryResult<usize> {
  let board_object_table = BoardObjectTable {
    board_id: query_board_id,
    object_type_number: object_number,
    object_id: query_id,
    left: query_left,
    top: query_top,
    width: query_width,
    height: query_height,
  };
  let sticky_note_table = StickyNoteTable {
    id: query_id,
    board_id: query_board_id,
    memo: query_memo,
  };
  diesel::update(board_objects
    .filter(
      crate::dal::db::schema::board_objects::dsl::board_id.eq(query_board_id)
      .and(object_type_number.eq(object_number))
      .and(object_id.eq(query_id))
    )
  )
    .set(board_object_table)
    .execute(conn)
    .and_then(|_| {
      diesel::update(sticky_notes
        .filter(crate::dal::db::schema::sticky_notes::dsl::id.eq(query_id)
        .and(crate::dal::db::schema::sticky_notes::dsl::board_id.eq(query_board_id))))
        .set(sticky_note_table)
        .execute(conn)
    })
}

pub fn update_headline(
  conn: &mut SqliteConnection, 
  query_id: i32,
  query_board_id: i32,
  query_left: String,
  query_top: String,
  query_width: String,
  query_height: String,
  query_headline: Option<String>,
  object_number: i32) -> QueryResult<usize> {
  let board_object_table = BoardObjectTable {
    board_id: query_board_id,
    object_type_number: object_number,
    object_id: query_id,
    left: query_left,
    top: query_top,
    width: query_width,
    height: query_height,
  };
  let headline_table = HeadlineTable {
    id: query_id,
    board_id: query_board_id,
    headline: query_headline.unwrap(),
  };
  diesel::update(board_objects
    .filter(
      crate::dal::db::schema::board_objects::dsl::board_id.eq(query_board_id)
      .and(object_type_number.eq(object_number))
      .and(object_id.eq(query_id))
    )
  )
    .set(board_object_table)
    .execute(conn)
    .and_then(|_| {
      diesel::update(headlines
        .filter(crate::dal::db::schema::headlines::dsl::id.eq(query_id)
        .and(crate::dal::db::schema::headlines::dsl::board_id.eq(query_board_id))))
        .set(headline_table)
        .execute(conn)
    })
}


// DAMP
#[cfg(test)]
mod db_connection_test{
  use super::*;
  use diesel::sql_query;
  use diesel::sql_types::Text;

  #[test]
  fn test_create_board_table_if_not_exists() {
    
    let test_db = "test_create_board_table_if_not_exists.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    
    let exists_table_query = format!(
          "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'", BoardTable::table_name()
      );
    #[derive(QueryableByName, Debug)]
    #[table_name = "sqlite_master"]
    struct TableName {
        #[column_name = "name"]
        #[sql_type = "Text"]
        table_name: String,
    }
    let results: Vec<TableName> = sql_query(exists_table_query)
      .load(&mut conn)
      .unwrap();
    assert!(results.iter().last().unwrap().table_name == BoardTable::table_name());
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }

  #[test]
  fn test_create_board_object_table_if_not_exists(){
    let test_db = "test_create_board_object_table_if_not_exists.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_object_table_if_not_exists(&mut conn);
    
    let exists_table_query = format!(
          "SELECT name FROM sqlite_master WHERE type='table' AND name='{}'", BoardObjectTable::table_name()
      );
    #[derive(QueryableByName, Debug)]
    #[table_name = "sqlite_master"]
    struct TableName {
        #[column_name = "name"]
        #[sql_type = "Text"]
        table_name: String,
    }
    let results: Vec<TableName> = sql_query(exists_table_query)
      .load(&mut conn)
      .unwrap();
    assert!(results.iter().last().unwrap().table_name == BoardObjectTable::table_name());
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }

  #[test]
  fn test_insert_board_to_boards(){
    let test_db = "test_insert_board_to_boards.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");
    
    let results: Vec<BoardTable> = boards.filter(name.eq("test_board")).load::<BoardTable>(&mut conn).unwrap();
    
    assert!(results.iter().last().unwrap().name == "test_board");
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }

  #[test]
  fn test_create_sticky_note(){

    use crate::dal::db::schema::sticky_notes::dsl::id;
    use crate::dal::db::schema::sticky_notes::dsl::board_id;

    let test_db = "test_create_sticky_note.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = create_sticky_note_table_if_not_exists(&mut conn);
    
    let _ = create_sticky_note(&mut conn, 1, 1, "0".to_string(), "0".to_string(), "100".to_string(), "100".to_string(), None, 1);

    let results: Vec<StickyNoteTable> = sticky_notes
      .filter(id.eq(1).and(board_id.eq(1)))
      .load::<StickyNoteTable>(&mut conn).unwrap();
    
      assert_eq!(results[0].id, 1);
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }
  
  #[test]
  fn test_get_sticky_note_info(){
    let test_db = "test_get_sticky_note_info.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = create_sticky_note_table_if_not_exists(&mut conn);
    
    let _ = create_sticky_note(&mut conn, 1, 1, "0".to_string(), "0".to_string(), "100".to_string(), "100".to_string(), None, 1);
    
    let result = get_sticky_note_info(&mut conn, 1, 1);
    
    assert_eq!(result.unwrap().id, 1);
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }

  #[test]
  fn test_get_board_id(){
    let test_db = "test_get_board.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");
    
    let result = get_board_id(&mut conn, "test_board");
    
    assert_eq!(result.unwrap(), 1);
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }

  #[test]
  fn test_get_all_board_names() {
    let test_db = "test_get_all_board_names.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");
    
    let result = get_all_board_names(&mut conn);
    
    assert_eq!(result.unwrap()[0], "test_board");
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }

  #[test]
  fn test_get_max_object_id_on_board() {
    let test_db = "test_get_max_object_id_on_board.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");

    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = create_sticky_note_table_if_not_exists(&mut conn);
    
    let _ = create_sticky_note(&mut conn, 1, 1, "0".to_string(), "0".to_string(), "100".to_string(), "100".to_string(), None, 1);
    
    let result = get_max_object_id_on_board(&mut conn, 1, 1);
    
    assert_eq!(result, 1);
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }

  #[test]
  fn test_exists_sticky_note_id() {
    let test_db = "test_exists_sticky_note_id.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = create_sticky_note_table_if_not_exists(&mut conn);
    
    let _ = create_sticky_note(&mut conn, 1, 1, "0".to_string(), "0".to_string(), "100".to_string(), "100".to_string(), None, 1);
    
    let result = exists_sticky_note_id(&mut conn, 1, 1);
    
    assert!(result);
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }
  
  #[test]
  fn test_update_note() {
    let test_db = "test_update_note.sqlite";
    let mut conn = SqliteConnection::establish(test_db).unwrap();
    let _ = create_board_table_if_not_exists(&mut conn);
    let _ = insert_board_to_boards(&mut conn, "test_board");
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = create_sticky_note_table_if_not_exists(&mut conn);
    
    let _ = create_sticky_note(&mut conn, 1, 1, "0".to_string(), "0".to_string(), "100".to_string(), "100".to_string(), None, 1);
    let _ = update_sticky_note(&mut conn, 1, 1, "10".to_string(), "0".to_string(), "100".to_string(), "100".to_string(), None, 1);
    
    let result = get_object_info(&mut conn, 1, 1, 1).unwrap();
    
    assert_eq!(result.left, "10");
    
    drop(conn);  
    let _ = std::fs::remove_file(test_db);
  }
}

