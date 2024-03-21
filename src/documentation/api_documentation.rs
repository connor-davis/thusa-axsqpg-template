use utoipa::OpenApi;

use crate::{
    documentation::api_security_addon::SecurityAddon,
    routes::{authentication, customers},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Thusa Managed Executive Reports API Documentation",
        version = "0.1.0",
        description = "The API documentation for the Thusa Managed Executive Reports API Documentation",
        contact(
            name = "Thusa",
            url = "https://thusa.co.za",
        ),
        license(
            name = "GPL-3.0",
        )
    ),
    paths(
        authentication::login::user,
        authentication::check::index,
        customers::view::customers,
        customers::view::customer
    ),
    components(
        schemas(
            authentication::login::LoginPayload
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Authentication", description = "Authentication routes."),
        (name = "Customers", description = "Customers routes.")
    ),
    servers(
        (
            url = "https://mer.thusacloud.co.za",
            description = "Production server",
        ),
        (
            url = "http://localhost:4000",
            description = "Development server",
        )
    )
)]
pub struct ApiDoc;
