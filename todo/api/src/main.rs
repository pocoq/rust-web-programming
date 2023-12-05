mod entities;
mod views;
mod processes;
mod state;
mod models;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() ->  std::io::Result<()>{
	HttpServer::new(|| {
		let app = App::new().configure(views::init_view);
		return app
	})
	.bind("127.0.0.1:8080")?
	.run()
	.await
}
