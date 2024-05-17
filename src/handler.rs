pub mod data;

use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::{DateTime, Local};
use data::Message;
use log::info;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

#[get("/")]
pub async fn home(tmpl: web::Data<Tera>) -> impl Responder {
    info!("Called posts");
    let all_posts = data::get_all();
    let mut context = Context::new();
    if !all_posts.is_empty() {
        context.insert("posts", &all_posts);
    }
    let body_str = tmpl.render("home.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts")]
pub async fn posts(tmpl: web::Data<Tera>) -> impl Responder {
    info!("Called posts");
    let all_posts = data::get_all();
    let mut context = Context::new();
    if !all_posts.is_empty() {
        context.insert("posts", &all_posts);
    }
    let body_str = tmpl.render("home.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[get("/posts/{id}")]
pub async fn show(info: web::Path<i32>, tmpl: web::Data<Tera>) -> impl Responder {
    info!("Called show");
    let id = info.into_inner();
    let post = data::get(id);
    let mut context = Context::new();
    context.insert("post", &post);
    let body_str = tmpl.render("show.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[derive(Serialize, Deserialize)]
struct PostingForm<'a> {
    action: &'a str,
    post: Message,
    button: &'a str,
}

#[get("/posts/new")]
pub async fn new(tmpl: web::Data<Tera>) -> impl Responder {
    info!("Called new");
    let mut context = Context::new();
    let form = PostingForm {
        action: "create",
        post: Message::default(),
        button: "登録",
    };
    context.insert("form", &form);

    let body_str = tmpl.render("posting.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}

#[post("/posts/create")]
pub async fn create(params: web::Form<Message>) -> impl Responder {
    info!("Called create");
    let now: DateTime<Local> = Local::now();
    let mut message = params.into_inner();
    message.posted = now.format("%Y/%m/%d  %H:%M:%S").to_string();
    message = data::create(message);

    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}

#[get("/posts/{id}/edit")]
pub async fn edit(info: web::Path<i32>, tmpl: web::Data<Tera>) -> impl Responder {
    info!("Called edit");
    let mut context = Context::new();
    let id = info.into_inner();
    let form = PostingForm {
        action: "update",
        post: data::get(id),
        button: "更新",
    };
    context.insert("form", &form);

    let body_str = tmpl.render("posting.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; utf-8")
        .body(body_str)
}

#[post("/posts/update")]
pub async fn update(params: web::Form<Message>) -> impl Responder {
    info!("Called create");
    let now: DateTime<Local> = Local::now();
    let mut message = params.into_inner();
    println!("{:?}", message);
    message.posted = now.format("%Y/%m/%d  %H:%M:%S").to_string();
    println!("{:?}", message);
    data::update(&message);

    web::Redirect::to(format!("/posts/{}", message.id)).see_other()
}

#[get("/posts/{id}/delete")]
pub async fn delete(info: web::Path<i32>, tmpl: web::Data<Tera>) -> impl Responder {
    info!("Called delete");
    let id = info.into_inner();
    let form = PostingForm {
        action: "destroy",
        post: data::get(id),
        button: "削除",
    };
    let mut context = Context::new();
    context.insert("form", &form);
    let body_str = tmpl.render("posting.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; utf-8")
        .body(body_str)
}

#[post("/posts/destroy")]
pub async fn destroy(params: web::Form<Message>) -> impl Responder {
    info!("Called destroy");
    let id = params.into_inner().id;
    data::delete(id);

    web::Redirect::to("/posts").see_other()
}

pub async fn not_found(tmpl: web::Data<Tera>) -> impl Responder {
    info!("Called not_found");
    let context = Context::new();
    let body_str = tmpl.render("not_found.html", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body_str)
}
