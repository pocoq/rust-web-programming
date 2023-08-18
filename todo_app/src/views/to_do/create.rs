use actix_web::{HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::{
	diesel,
    database::DB,
    json_serialization::to_do_items::ToDoItems,
    models::item::{item::Item, new_item::NewItem},
    schema::to_do,
};

pub async fn create(req: HttpRequest, db: DB) -> HttpResponse {
	// let connection = establish_connection();

    let title = req.match_info().get("title").unwrap().to_string();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&db.connection);
    }
    return HttpResponse::Ok().json(ToDoItems::get_state());
}
