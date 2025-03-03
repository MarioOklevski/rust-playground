mod api_docs;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Define the OpenAPI specification with paths
#[derive(OpenApi)]
#[openapi(paths(hello, echo, manual_hello))]
struct ApiDoc;

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
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(hello))
            .service(web::resource("/echo").to(echo))
            .service(
                web::scope("/docs").service(api_docs::openapi_json)
                // .service(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
                .service(
                    SwaggerUi::new("/swagger/{_:.*}").url("/docs/openapi.json", Default::default()),
                ),
            )
            .route("/hey", web::get().to(manual_hello))
            })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}