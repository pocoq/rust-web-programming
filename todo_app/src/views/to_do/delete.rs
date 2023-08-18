use crate::{
    database::DB,
    diesel,
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    models::item::item::Item,
    schema::to_do,
};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken, db: DB) -> HttpResponse {
	// let connection = establish_connection();

    let results = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();
    let _ = diesel::delete(&results[0]).execute(&db.connection);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
