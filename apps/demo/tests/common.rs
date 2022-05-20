use actix_web::{web, Scope};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

use demo::{auth_handler, invitation_handler, models, register_handler};

pub fn setup() -> (Scope, models::Pool, String) {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "simple-auth-server=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    // let conn = pool.get().unwrap();

    let domain = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let scope = web::scope("/api")
        .service(
            web::resource("/invitation").route(web::post().to(invitation_handler::post_invitation)),
        )
        .service(
            web::resource("/register/{invitation_id}")
                .route(web::post().to(register_handler::register_user)),
        )
        .service(
            web::resource("/auth")
                .route(web::post().to(auth_handler::login))
                .route(web::delete().to(auth_handler::logout))
                .route(web::get().to(auth_handler::get_me)),
        );

    (scope, pool, domain)
}

pub fn generate_invitation(
    eml: String,
    pool: &models::Pool,
) -> Result<models::Invitation, demo::errors::ServiceError> {
    use demo::schema::invitations::dsl::invitations;

    let new_invitation: models::Invitation = eml.into();
    let conn = &pool.get().unwrap();

    let inserted_invitation = diesel::insert_into(invitations)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}
