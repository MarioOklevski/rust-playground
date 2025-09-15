mod api_docs;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use dotenvy::dotenv;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn establish_connection() -> DbPool {
    dotenv().ok(); // load .env

    let database_url = dotenvy::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}
// Define the hello route
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Returns a greeting message", body = String)
    )
)]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

// Define the echo route
#[utoipa::path(
    post,
    path="/echo",
    request_body = String,
    responses(
        (status = 200, description = "Echoes the input string", body = String)
    )
)]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// Define the manual_hello route
#[utoipa::path(
    get,
    path="/hey",
    responses(
        (status = 200, description = "Returns a custom greeting", body = String)
    )
)]
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    establish_connection();
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(hello))
            .service(web::resource("/echo").to(echo))
            .route("/hey", web::get().to(manual_hello))
            // Serve raw OpenAPI JSON (from api_docs.rs)
            .service(web::scope("/docs").service(api_docs::openapi_json))
            // Serve Swagger UI
            .service(
                SwaggerUi::new("/swagger/{_:.*}")
                    .url("/docs/openapi.json", api_docs::ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}