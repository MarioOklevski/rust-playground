use std::env;

use actix_web::{get, HttpResponse, Responder};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    paths(),
    components (
        schemas ()
    ),
    tags(
        (name = "Auth", description = "A authentication routes"),
        (name = "Todo", description = "A todo routes"),
        (name = "Server Metadata", description = "A server metadata routes"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // Safety: This unwrap is safe because we know that the componets exists
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Bearer Token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        let mut contact = utoipa::openapi::Contact::new();
        contact.name = Some(env::var("API_CONTACT_NAME").expect("`API_CONTACT_NAME` must be set"));
        contact.url = Some(env::var("API_CONTACT_URL").expect("`API_CONTACT_URL` must be set"));
        contact.email =
            Some(env::var("API_CONTACT_EMAIL").expect("`API_CONTACT_EMAIL` must be set"));
        openapi.info.description = Some(include_str!("../api-desc.md").to_owned());
        openapi.info.title =
            env::var("API_TITLE").unwrap_or_else(|_| "RESTful Todo API documentation".to_owned());

        openapi.info.contact = Some(contact);
    }
}

/// Return a json OpenAPI document
#[get("/openapi.json")]
pub async fn openapi_json() -> impl Responder {
    let openapi = ApiDoc::openapi();
    HttpResponse::Ok().json(openapi)
}