use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct Diary{
  
   pub title:String,
   pub description:String,
   pub date:String
 
 
 }
 #[derive(Deserialize,Serialize)]
pub struct EntryResponse{
  pub id:i32,
  pub title:String,
  pub description:String,
  pub date:String

  

}

 #[derive(Deserialize,Serialize)]
 pub struct User{
   pub user_name:String,
   pub password:String
 }
 #[derive(Deserialize,Serialize)]
 pub struct UserRow{
  pub id:i32,
  pub password:String
 }