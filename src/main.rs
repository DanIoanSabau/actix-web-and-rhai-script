extern crate actix_web;
extern crate rhai;

use actix_web as actix;

const SERVER_IP_ADDRESS: &'static str = "127.0.0.1";
const SERVER_PORT: u16 = 8080;

#[actix::get("/multiply/{first}/{second}")]
async fn multiply(path: actix::web::Path<(i64, i64)>) -> impl actix::Responder {
    let (first, second) = path.into_inner();

    let mut engine = rhai::Engine::new();

    engine.register_fn("first", move || first);
    engine.register_fn("second", move || second);

    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();

    format!("{result}")
}

#[actix::get("/add/{first}/{second}")]
async fn add(path: actix::web::Path<(i64, i64)>) -> impl actix::Responder {
    let (first, second) = path.into_inner();

    let mut engine = rhai::Engine::new();

    engine.register_fn("first", move || first);
    engine.register_fn("second", move || second);

    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();

    format!("{result}")
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    let server = actix::HttpServer::new(|| {
        actix::App::new()
            .service(multiply)
            .service(add)
    });

    let server = server.bind((SERVER_IP_ADDRESS, SERVER_PORT)).unwrap();

    server.run().await?;

    Ok(())
}
