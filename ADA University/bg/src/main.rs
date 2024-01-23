use std::env;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_session::CookieSession;
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use tera::{Tera};
use dotenv::dotenv;

// actix_web::types::Form;
use crate::app::*;
mod app;


#[actix_web::main]
async fn main()-> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let db = env::var("DATABASE_URL").expect("Database_url not found in .env");
    // let dbp = env::var("DATABASE_URLP").expect("Database_url not found in .env");
    let conn = sqlx::SqlitePool::connect(&db).await.unwrap();
    // let connp = sqlx::SqlitePool::connect(&dbp).await.unwrap();


    // std::fs::create_dir_all("./tmp")?;
    let secret_key = Key::generate();
    HttpServer::new(move || {
        use actix_files::Files;
        let mut templates = Tera::new("../templates/**/*").unwrap();
        templates.autoescape_on(vec!["tera"]);
        App::new()
            .wrap(Logger::default())
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .app_data(TempFileConfig::default().directory("./tmp"))
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(conn.clone()))
            .service(web::resource("/").route(web::get().to(index))) //Finished
            .service(web::resource("/signin").route(web::post().to(post_signin)).route(web::get().to(signin))) //Finished
            .service(web::resource("/login").route(web::post().to(post_login)).route(web::get().to(login))) //Finished
            .service(web::resource("/Scholarships").route(web::post().to(post_schoolarships)).route(web::get().to(schoolarships))) //Finished
            .service(web::resource("/MembershipCard").route(web::get().to(membershipcard)))
            .service(web::resource("/GalaxyMeetingUSA").route(web::get().to(galaxymeetingusa))) //Finished
            // .service(web::resource("/SpaceMuseum").route(web::get().to()))
            .service(web::resource("/AstroEducation").route(web::post().to(post_astroeducation)).route(web::get().to(astroeducation))) //Finished
            .service(web::resource("/Conferences").route(web::get().to(conferences))) //Finished
            .service(web::resource("/Excursion").route(web::get().to(excursion)))
            .service(web::resource("/SpaceMagazine").route(web::get().to(spacemagazine)))
            .service(web::resource("/Astro-Courses").route(web::post().to(post_astrocourses)).route(web::get().to(astrocourses)))
            .service(web::resource("/MovieNights").route(web::get().to(movienights)))
            .service(web::resource("/Events").route(web::get().to(events)))
            .service(web::resource("/CareerDev").route(web::get().to(careerdev)))
            .service(web::resource("/AdvocateService").route(web::post().to(post_advocateservice)).route(web::get().to(advocateservice))) //Finished
            .service(web::resource("/GalacticFamily").route(web::get().to(galacticfamily)))  //Finished
            // .service(web::resource("/Spacemagazineupload").route(web::post().to(post_spacemagazineupload)).route(web::get().to(spacemagazineupload))) //Finished
            // .service(web::resource("/ScientificArticles").route(web::get().to(articles)))
            // .service(web::resource("/Competitions").route(web::get().to()))
            .service(web::resource("/Certificates").route(web::get().to(certificates)))
            // .service(web::resource("/Spacevolunters").route(web::get().to()))
            // .service(web::resource("/SpaceArt").route(web::get().to()))
            // .service(web::resource("/InternationalProjects").route(web::get().to()))
            .service(web::resource("/SpaceLibrary").route(web::get().to(spacelibrary)))
            // .service(articlespdfview)
            .service(spacemagazinetailview)
            .service(eventstailview)
            .service(scientificarticlestailsaw)



            // .service(web::resource("/DiscussionPanel").route(web::get().to()))
            .service(Files::new("/static", "../static/").show_files_listing())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}