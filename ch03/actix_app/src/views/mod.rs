/// The factory that defines all the views for the whole app

mod auth;
use actix_web::web::ServiceConfig;
use auth::auth_views_factory;

pub fn views_factory(app: &mut ServiceConfig){
	auth_views_factory(app);
}