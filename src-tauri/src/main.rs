#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use mysql::{params, prelude::Queryable, Conn};
use serde::{Deserialize, Serialize};
use tauri::{async_runtime::Mutex, State};

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: i32,
    text: String,
    complete: i32,
}
//#[macro_use]
//extern crate lazy_static;

//lazy_static! {
//static ref CONNECTION: Mutex<Conn> =
//Mutex::new(
//Conn::new("mysql://root:root@127.0.0.1:3306/db_todos?prefer_socket=false").unwrap()
//);
//}
pub struct InnerDBConnection {
    conn: mysql::Conn,
}
pub struct DBConnection(pub Mutex<InnerDBConnection>);

#[tauri::command]
async fn get_all_task(state: State<'_, DBConnection>) -> Result<String, String> {
    let mut db = state.0.lock().await;
    let map: Result<Vec<Todo>, mysql::error::Error> = db.conn.exec_map(
        "SELECT id,text,complete FROM task
",
        (),
        |(id, text, complete)| Todo { id, text, complete },
    );
    match map {
        Ok(res) => Ok(serde_json::to_string(&res).unwrap()),
        Err(err) => return Err(err.to_string()),
    }
}
//#[tauri::command]
//async fn get_all_task2()->String{
//let mut m=CONNECTION.lock().await;
//let map:Result<Vec<todo>,mysql::error::Error> =
//m.exec_map("SELECT id,text,complete FROM task
//" , (),|(id,text,complete)|todo{
//id,
//text,
//complete,
//}
//);
//let x=map.expect("data is null");
//serde_json::to_string(&x).unwrap()
//}
#[tauri::command]
async fn insert_new_task(text: &str, state: State<'_, DBConnection>) -> Result<(), ()> {
    let mut db = state.0.lock().await;
    let res = db.conn.exec_drop(
        r"
    insert into task
    (text,complete)
    values
    (:text,:complete)    
    ",
        params! {
           "text"=>text.clone(),
           "complete"=>0
        },
    );
    match res {
        Ok(r) => Ok(()),
        Err(err) => return Err(()),
    }
}
//#[tauri::command]
//async fn insert_new_task(text: &str)->Result<(),()>{
//let mut m=CONNECTION.lock().await;
//let r=m.exec_drop(r"
//insert into task
//(text,complete)
//values
//(:text,:complete)
//",
//params! {
//"text"=>text.clone(),
//"complete"=>0
//});
//Ok(())
//}
#[tauri::command]
async fn update_the_task(id: i32, complete: i32, state: State<'_, DBConnection>) -> Result<(), ()> {
    let mut db = state.0.lock().await;
    let stmt = db
        .conn
        .prep("update task set complete=:complete where id=:id")
        .unwrap();
    let res = db.conn.exec_drop(
        &stmt,
        params! {
            "id" => id,
            "complete" => complete
        },
    );
    match res {
        Ok(r) => Ok(()),
        Err(err) => return Err(()),
    }
}

#[tauri::command]
async fn dalete_the_task(id: i32, state: State<'_, DBConnection>) -> Result<(), ()> {
    let mut db = state.0.lock().await;
    let stmt = db.conn.prep("delete from task where id=:id").unwrap();
    let res = db.conn.exec_drop(
        &stmt,
        params! {
            "id"=>id
        },
    );
    match res {
        Ok(r) => Ok(()),
        Err(err) => return Err(()),
    }
}
fn main() {
    tauri::Builder::default()
        .manage(DBConnection(Mutex::new(InnerDBConnection {
            conn: Conn::new("mysql://root:root@127.0.0.1:3306/db_todos?prefer_socket=false")
                .unwrap(),
        })))
        .invoke_handler(tauri::generate_handler![
            get_all_task,
            insert_new_task,
            update_the_task,
            dalete_the_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
