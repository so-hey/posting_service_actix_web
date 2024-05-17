mod handler;

use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use handler::{create, delete, destroy, edit, home, new, not_found, posts, show, update};
use tera::Tera;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 重要度の高い順に error, warn, info, debug, trace
    // ここで指定したもの以上の重要度のlogが表示される
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("http://127.0.0.1:8080");

    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*.html").unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .service(home)
            .service(posts)
            .service(new)
            .service(create)
            .service(update)
            .service(show) // showを最後に登録しないと，/posts/newとかがposts/{id}にマッチしてしまう.
            .service(edit)
            .service(delete)
            .service(destroy)
            .default_service(web::to(not_found))
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
