mod create;
mod get; 
mod edit;

use actix_web::web::{get, post, scope, ServiceConfig};

pub fn init_task_view(app: &mut ServiceConfig) {
    app.service(
        scope("v1/item")
            .route("create/{title}", post().to(create::create))
			.route("edit", post().to(edit::edit))
            .route("get", get().to(get::get))
    );
}
