use crate::router::login;
pub use crate::router::person;
use actix_web::{web, App, HttpServer};
pub struct Server_;

impl Server_ {

    #[actix_web::main]
    pub async fn run_server_(&self) -> std::io::Result<()> {
        println!("Iniciando servidor...");
        return HttpServer::new(|| {
                    App::new().service(
                        web::scope("/api/v1")
                            .configure(person::person_router)
                            .configure(login::login_router)
                    )
                }).bind(("127.0.0.1", 8080))?.run().await;
    }
}

