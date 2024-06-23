
use diesel::result::Error;
use crate::dal::{board::get_board_id, board_object::BoardObject, board_object_model::{headline::Headline, sticky_note::StickyNote}};
use crate::dal::board_object::get_database_path;

pub struct IdManager {
    current_id: i64
  }
  
  impl IdManager {
    pub fn new() -> Result<Self, Error>{
      Ok(IdManager {current_id: 0})
    }
    pub fn change_table(&mut self, table_name: &str) -> Result<(), Error>{
      let db_name = get_database_path();
      let board_id = get_board_id(&db_name, table_name)?;
      let max_id = StickyNote::get_max_object_id_on_board(&db_name, board_id) as i64;
      self.current_id = max_id;
      Ok(())
    }
    pub fn get_next_id(&mut self) -> i64 {
      self.current_id += 1;
      self.current_id
    }
  }
  

pub struct HeadlineIdManager {
    current_id: i64
  }
  impl HeadlineIdManager {
    pub fn new() -> Result<Self, Error>{
      Ok(HeadlineIdManager {current_id: 0})
    }
    pub fn change_table(&mut self, table_name: &str) -> Result<(), Error>{
      let db_name = get_database_path();
      let board_id = get_board_id(&db_name, table_name)?;
      let max_id = Headline::get_max_object_id_on_board(&db_name, board_id) as i64;
      self.current_id = max_id;
      Ok(())
    }
    pub fn get_next_id(&mut self) -> i64 {
      self.current_id += 1;
      self.current_id
    }
  }


  pub struct BoradNames{
    pub board_name: String
  }
  impl BoradNames{
    pub fn new() -> Result<Self, Error> {
      Ok(BoradNames{board_name: "".to_string()})
    }
    pub fn change_table(&mut self, board_name: &str) -> Result<(), Error> {
      self.board_name = board_name.to_string();
      Ok(())
    }
  }
  