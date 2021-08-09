use anyhow::Result;
use actix_web::{web, HttpServer};

mod cfg;
mod routes;

pub struct AppState {
    config: cfg::AppConfig,
}

pub struct App;

impl App {
    pub fn new() -> Self {
        App
    }

    pub fn run(&self) -> Result<()> {
        let config = cfg::instance()?;

        let app_state = web::Data::new(AppState { config });

        let state = app_state.clone();

        actix_web::rt::System::new("main")
            .block_on(async move {
                HttpServer::new(move || {
                    actix_web::App::new()
                        .app_data(app_state.clone())
                        .configure(routes::user::configure)
                })
                .bind(&state.config.external_base_url)?
                .run()
               .await
        
            })?;

        Ok(())    
    }
}

// fn url_to_enpoint(&str) -> Result<String> {
//     let url = url::Url::parse(str)?;

//     url.host_str()
// }