use actix_web::HttpResponse;
use super::content_loader::{read_file, add_component};

pub async fn items() -> HttpResponse {
	let mut html_data = read_file("./templates/main.html");
	let js_data = read_file("./templates/main.js");
	let css_base_data = read_file("./templates/css/base.css");
	let css_data = read_file("./templates/css/main.css");

	html_data = html_data.replace("{{JAVASCRIPT}}", &js_data);
	html_data = html_data.replace("{{BASE_CSS}}", &css_base_data);
	html_data = html_data.replace("{{CSS}}", &css_data);
	html_data = add_component(String::from("header"), html_data);
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html_data)
}
