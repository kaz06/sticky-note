use crate::dal::db::schema::boards;
use crate::dal::db::schema::sticky_notes;
use crate::dal::db::schema::board_objects;
use crate::dal::db::schema::headlines;

extern crate diesel;
use diesel::prelude::*;
use serde::Deserialize;
use serde::Serialize;

const BOARDS: &str = "boards";
const STIKY_NOTES: &str = "sticky_notes";
const BOARD_OBJECTS: &str = "board_objects";
const HEADLINES: &str = "headlines";


#[derive(Queryable, Insertable, Associations, AsChangeset, Debug, Serialize, Deserialize)]
#[belongs_to(parent = "BoardTable", foreign_key = "board_id")]
#[table_name="board_objects"]
pub struct BoardObjectTable {
  pub board_id: i32,
  pub left: String,
  pub top: String,
  pub width: String,
  pub height: String,
  pub object_type_number: i32,
  pub object_id: i32,
}

impl BoardObjectTable {
  pub fn table_name() -> String {
    BOARD_OBJECTS.to_string()
  }
  
}



#[derive(Queryable, Insertable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name="boards"]
pub struct BoardTable {
    pub id: i32,
    pub name: String,
}

impl  BoardTable {
  pub fn table_name() -> String {
    BOARDS.to_string()
  }
}

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name="sticky_notes"]
pub struct StickyNoteTable {
    pub id: i32,
    pub board_id: i32,
    pub memo: Option<String>,
}

impl StickyNoteTable {
  pub fn table_name() -> String {
    STIKY_NOTES.to_string()
  }
}

#[derive(Queryable, Insertable, Identifiable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name="headlines"]
pub struct HeadlineTable {
    pub id: i32,
    pub board_id: i32,
    pub headline: String,
}

impl HeadlineTable {
  pub fn table_name() -> String {
    HEADLINES.to_string()
  }
    
}