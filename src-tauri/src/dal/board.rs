use crate::dal::db::connection::*;

use diesel::result::Error;
use diesel::{Connection, SqliteConnection};

pub fn create_board(db_name: &str, board_name: &str) -> Result<usize, Error> {
    let mut conn = SqliteConnection::establish(db_name).unwrap();
    insert_board_to_boards(&mut conn, board_name)
}

pub fn get_board_id(db_name: &str, table_name: &str) -> Result<i32, Error> {
    let mut conn = SqliteConnection::establish(db_name).unwrap();
    crate::dal::db::connection::get_board_id(&mut conn, table_name)
}

pub fn list_screen(db_name: &str) -> Vec<String>{
    let mut conn = SqliteConnection::establish(db_name).unwrap();
    get_all_board_names(&mut conn).unwrap()
  }
