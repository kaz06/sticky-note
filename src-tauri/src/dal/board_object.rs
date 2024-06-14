
use diesel::{result::Error, Connection, SqliteConnection};

use crate::dal::board_object_model::{headline::Headline, sticky_note::StickyNote};

pub const DBNAME: &str = "./src/dal/db/file/sticky_note.sqlite";
pub trait BoardObject {
    const OBJECT_TYPE: ObjectType ;

    fn create_table_if_not_exists(dbname: &str) -> Result<usize, Error>;
    fn exists_id_in_db(dbname: &str, board_id: i32, board_object_id: i32) -> bool;
    fn get(dbname: &str, board_id: i32, board_object_id: i32) -> Result<Self, Error> where Self: std::marker::Sized;
    fn get_all_in_board(db_name: &str, board_id: i32) -> Result<Vec<Self>, Error> where Self: std::marker::Sized;
    fn get_max_object_id_on_board(dbname: &str, board_id: i32) -> i32;
    fn create(self, dbname: &str) -> Result<usize, Error>;
    fn update(self, dbname: &str) -> Result<usize, Error>;
    fn save(self, dbname: &str) -> Result<usize, Error>;
    fn delete( dbname: &str, board_id: i32, board_object_id: i32) -> Result<usize, Error>;
}


pub enum ObjectType {
    StickyNote,
    Headline,
}
impl ObjectType {
    pub fn to_number(&self) -> i32 {
        match self {
            ObjectType::StickyNote => 1,
            ObjectType::Headline => 2,
        }
    }
}

pub fn create_db_tables() -> Result<usize, Error> {
    use crate::dal::db::connection::*;
    let mut conn = SqliteConnection::establish(DBNAME).unwrap();
    let _ = create_board_object_table_if_not_exists(&mut conn);
    let _ = StickyNote::create_table_if_not_exists(DBNAME);
    let _ = Headline::create_table_if_not_exists(DBNAME);
    create_board_table_if_not_exists(&mut conn)
}