mod auth;
mod task;

use actix_web::web::ServiceConfig;

use task::init_task_view;

pub fn init_view(app: &mut ServiceConfig) {
    init_task_view(app);
}
