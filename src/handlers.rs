use actix_web::*;
pub mod Handlers{
    use std::{path::PathBuf, process::id, sync::{Arc, Mutex}};
    use actix_files::NamedFile;
    use bcrypt::*;
    

    use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
    
  
    use rusqlite::{params, Connection, Row};
    use serde_json::json;

    use crate::models::{Entry, EntryResponse, User, UserRow};


#[get("/")]
async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("static/index.html").parse().unwrap();
    Ok(NamedFile::open(path)?)
}


#[post("/c/user")]
pub async fn create_user(
    conn: web::Data<Arc<Mutex<Connection>>>,
    user: web::Json<User>,
) -> HttpResponse {
    let conn = conn.lock().unwrap();
    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let user_name = &user.user_name;

    let res = conn.execute(
        "INSERT INTO users (username, password_hash) VALUES (?, ?)",
        params![user_name, hashed_password],
    );

    match res {
        
        Ok(_) => HttpResponse::Created().json(json!("User added successfully")),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add user"),
    }
}
#[post("/login")]
pub async fn user_login(conn:web::Data<Arc<Mutex<Connection>>>,user:web::Json<User>)->HttpResponse{
    let conn=conn.lock().unwrap();
    let mut stmt=conn.prepare("SELECT id, password_hash FROM users WHERE username= ?").unwrap();
    let user_row=stmt.query_row(params![&user.user_name],|row|{
       Ok(
        UserRow{
            id:row.get(0)?,
            password:row.get(1)?
        }
       )
    });
 
    match user_row {
        Ok(user_row) => {
            if verify(&user.password, &user_row.password).unwrap() {
                return HttpResponse::Ok().json(json!(user_row.id))
            } else {
                return HttpResponse::Unauthorized().json("Invalid credentials")
            }
        },
        Err(_) => {
            return HttpResponse::NotFound().json(json!("Error:User does not exist"))
        }
        
    }
}
#[post("{user_id}/upload")]
pub async fn write_new(path:web::Path<i32>,conn:web::Data<Arc<Mutex<Connection>>>,entry:web::Json<Entry>)->HttpResponse{
    let con=conn.lock().unwrap();
    let user_id=path.into_inner();
    let qres=con.execute("INSERT INTO diaries(user_id,title,description,date) VALUES(?,?,?,?)",params![user_id,entry.title,entry.description,entry.date]);
    match qres {
        Ok(_)=>HttpResponse::Created().json("sucess"),
        Err(e)=>{
            eprintln!("Database error: {:?}", e);
            HttpResponse::InternalServerError().json("Fail")
        }
        
    }
}
#[get("{user_id}/all")]
pub async fn get_all(path:web::Path<i32>,conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
    let conn=conn.lock().unwrap();
    let user_id=path.into_inner();
    let mut stmt=conn.prepare("SELECT id,title,description,date from diaries WHERE user_id=?").unwrap();
    let entries=stmt.query_map([user_id], |row|{
        Ok(EntryResponse{
            id:row.get(0)?,
            title:row.get(1)?,
            description:row.get(2)?,
            date:row.get(3)?

        })
    }).unwrap();
    let entries:Vec<EntryResponse>=entries.filter_map(Result::ok).collect();


    HttpResponse::Ok().json(entries)
}

#[delete("{user_id}/entry/{entry_id}")]
pub async fn delete_entry(
    path: web::Path<(i32, i32)>,
    conn: web::Data<Arc<Mutex<Connection>>>,
) -> HttpResponse {
    let (user_id, entry_id) = path.into_inner();
    let conn = conn.lock().unwrap();
    let result=conn.execute("DELETE FROM diaries WHERE id=? AND user_id=?", params![entry_id,user_id]);
    match result {
        Ok(u ) if u>0 =>{
            HttpResponse::Ok().json("Deleted")

        }
        Ok(_)=>HttpResponse::NotFound().json("not found"),
        Err(e)=>{
            eprintln!("{:?}",e);
            HttpResponse::InternalServerError().json("fail")
        }
        
    }
}
#[put("/edit/{user_id}/{entry_id}")]
pub async fn update_entry(ids: web::Path<(i32,i32)>, entry: web::Json<Entry>,conn:web::Data<Arc<Mutex<Connection>>>)->HttpResponse{
    let (user_id,entry_id)=ids.into_inner();
    let conn=conn.lock().unwrap();
    let res=conn.execute("UPDATE diaries SET  title=?,description=?,date=? WHERE user_id=? AND id= ?  ", params![entry.title,entry.description,entry.date,user_id,entry_id]);
    match res {
        Ok(rows) => {
            if rows == 0 {
                HttpResponse::NotFound().json("Task not found")
            } else {
                HttpResponse::Ok().json("Status updated successfully")
            }
        }
        Err(e) => {
            eprintln!("Failed to update status: {}", e);
            HttpResponse::InternalServerError().json("Failed ")
        }
    }
    
}
}
