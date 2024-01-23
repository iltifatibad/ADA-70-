use actix_multipart::{
    form::{
        tempfile::{TempFile, TempFileConfig},
        MultipartForm,
        text::Text
        
    },
    Multipart,
};
use actix_session::Session;
use actix_web::{HttpRequest, Responder, get};
use actix_web::http::header::CONTENT_LENGTH;
use actix_web::{Error, error, http, HttpResponse, web};
use sqlx::Row;
use tera::{Context, Tera};
use serde::*;
use validator::{Validate};
use bcrypt::bcrypt;
use bcrypt::DEFAULT_COST;
use futures::{StreamExt, TryStreamExt, future::err};
use std::fs::{self, File};
use std::io::Write;
use actix_web::{middleware, App, HttpServer};
use mime::{ Mime, PDF };
use rand::{self, Rng};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};


#[derive(Deserialize, Validate)]
pub struct LoginUser{
    #[validate(email)]
    email: String,
    password: String
}
#[derive(Debug, Deserialize, Validate)]
pub struct SigninUser{
    #[validate(length(min=5))]
    name: String,
    #[validate(length(min=5))]
    surname: String,
    #[validate(email)]
    email: String,
    #[validate(length(min=6))]
    password: String,
    
    // password2: String
}


#[derive(Deserialize, Validate)]
pub struct AstroCoursesPost {
    name : String,
    surname : String,
    phone : String,
    email : String
}



#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User{
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct SMTailSaw {
    name: String,
    imgpath: String,
    datapath: String
}


// #[get("/test/{id}")]
// async fn test(tmpl: web::Data<Tera>,id: web::Path<String>,connp: web::Data<sqlx::SqlitePool>,) -> Result<HttpResponse, Error> {
//     let mut ctx = Context::new();
//     let a = tmpl.render("login.html", &ctx).map_err(error::ErrorInternalServerError)?;
//     Ok(HttpResponse::Ok().body(a))
// }


pub async fn index(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    if let Some(user) = session.get::<String>("name")?{
        if let Some(duty) = session.get::<String>("duty")?{
            ctx.insert("name", &user);
            ctx.insert("duty", &duty);
        }
    }

    let a = tmpl.render("index.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}


// pub async fn articles(tmpl : web::Data<Tera>, connp: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse, Error> {
//     let mut ctx = Context::new();
//     let mut vecimgpath = Vec::new();
//     let mut vecdatapath = Vec::new();
//     let mut vecname = Vec::new();

    
//     for i in 0..10 {
//         let (name,imgpath, datapath,): (String, String, String) = sqlx::query_as(" SELECT * FROM articledata ORDER BY RANDOM() LIMIT 1 ").fetch_one(&**connp).await.expect("AA");
//         vecimgpath.push(imgpath);
//         vecdatapath.push(datapath);
//         vecname.push(name);

//     }
//     ctx.insert("imgpath", &vecimgpath);
//     ctx.insert("datapath", &vecdatapath);
//     ctx.insert("name", &vecname);
//     log::info!("");
//     let a = tmpl.render("articles.html", &ctx).map_err(error::ErrorInternalServerError)?;
//     Ok(HttpResponse::Ok().body(a))
    
// }

#[derive(Debug, Validate,MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
    name:Text<String>,

}

// pub async fn save_files(MultipartForm(form): MultipartForm<UploadForm>,conn: web::Data<sqlx::SqlitePool>) -> Result<impl Responder, Error> {
//     for f in form.files {
//         let mut fname:String =  (*form.name.clone()).to_string();
//         fs::create_dir(&format!("../static/articles/{}/", fname));
//         let path = format!("../static/articles/{}/{}", *form.name, f.file_name.unwrap());
//         let pathdb = format!("../static/articles/{}/", *form.name);
//         log::info!("saving to {}", &path);
//         f.file.persist(&path).unwrap();
//         let add_data = sqlx::query("insert into info(id,imgpath,datapath,name) values($1,$2,$3,$4)")
//             .bind(&bcrypt::hash(rand::thread_rng().gen_range(100000000..1000000000).to_string(),DEFAULT_COST).expect("Şifreleme hatalı"))
//             .bind(&path)
//             .bind(pathdb)
//             .execute(&**conn).await;
//     }

//     Ok(HttpResponse::Ok())
// }

pub async fn membershipcard(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("membershipcard.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn movienights(tmpl : web::Data<Tera> , conn : web::Data<sqlx::SqlitePool>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut vecimgpath = Vec::new();
    let mut vecname = Vec::new();

    for i in 0..10 {
        let (name,imgpath, ): (String, String) = sqlx::query_as(" SELECT * FROM moviedata ORDER BY RANDOM() LIMIT 1 ").fetch_one(&**conn).await.expect("AA");
        vecimgpath.push(imgpath);
        vecname.push(name);
        }
        ctx.insert("imgpath", &vecimgpath);
        ctx.insert("name", &vecname);
        let a = tmpl.render("movienights.html", &ctx).map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().body(a))
}


pub async fn articles(tmpl : web::Data<Tera>, conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut vecname = Vec::new();
    let mut vecimgpath = Vec::new();
    
    for i in 0..10 {
        let (name,imgpath): (String, String) = sqlx::query_as(" SELECT * FROM articledata ORDER BY RANDOM() LIMIT 1 ").fetch_one(&**conn).await.expect("AA");
        vecname.push(name);
        vecimgpath.push(imgpath);
    }
    ctx.insert("imgpath", &vecimgpath);
    ctx.insert("name", &vecname);
    log::info!("");
    let a = tmpl.render("scientificarticles.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
    
}

#[get("/ScientificArticles/{name}")]
async fn scientificarticlestailsaw(name: web::Path<String>,tmpl: web::Data<Tera>) -> Result<HttpResponse , Error> {
    let mut magazinename = format!("{}", name);
    let mut ctx = Context::new();
    ctx.insert("name", &magazinename);

    // ctx.insert("imgpath", &imgpath);
    // ctx.insert("datapath", &datapath);

    let a = tmpl.render("scientificarticlestailsaw.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}








pub async fn schoolarships(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("schoolarship.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}







pub async fn spacemagazineupload(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("spacemagazineupload.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}
pub async fn post_spacemagazineupload(tmpl : web::Data<Tera> , MultipartForm(form): MultipartForm<UploadForm> ) -> Result<impl Responder, Error> {
    let mut ctx = Context::new();

    let a = tmpl.render("spacemagazineupload.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(redirect("/"))
}











pub async fn post_schoolarships(tmpl : web::Data<Tera> , form : web::Form<AdvocateService> ) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut emailctx =format!(
        "Name : {} , Surname : {} , Email : {} , Phonenumber : {}", 
        form.0.name, 
        form.0.surname, 
        form.0.email, 
        form.0.phone,
      );
    let smtp_key : &str = "xsmtpsib-9eadffaef474428d38bfa7f1ae67f43a75dbbc0b7fee011fdae7eda3c9718518-QYjMTWN1Zpz9rEtO";
    let from_email : &str = "thegalacticsociety1@gmail.com";
    let host : &str = "smtp-relay.sendinblue.com";
    let to_email : &str = "iltifatibad@gmail.com";
    
    let email : Message = Message::builder()
    .from(from_email.parse().unwrap())
    .to(to_email.parse().unwrap())
    .subject(" Schoolarship Register")
    .body(emailctx.to_string())
    .unwrap();

    let mailer : SmtpTransport = SmtpTransport::relay(&host)
    .unwrap()
    .credentials(Credentials::new(from_email.to_string(), smtp_key.to_string(),)).build();
    
    match mailer.send(&email) {
        Ok(_) => println!("Email sent succesfully"),
        Err(e) => println!("Could not send email")
    }
    
    // let a = tmpl.render("advocateservice.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(redirect("/"))
}

pub async fn astroeducation(tmpl : web::Data<Tera>, conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let (url,):(String,) = sqlx::query_as(" SELECT * FROM astrourl ").fetch_one(&**conn).await.expect("AA");
    ctx.insert("url", &url);
    log::info!("");
    let a = tmpl.render("astronomyeducation.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
    
}

pub async fn post_astroeducation(tmpl : web::Data<Tera> , form : web::Form<AdvocateService> ) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut emailctx =format!(
        "Name : {} , Surname : {} , Email : {} , Phonenumber : {}", 
        form.0.name, 
        form.0.surname, 
        form.0.email,
        form.0.phone,
      );
    let smtp_key : &str = "xsmtpsib-9eadffaef474428d38bfa7f1ae67f43a75dbbc0b7fee011fdae7eda3c9718518-QYjMTWN1Zpz9rEtO";
    let from_email : &str = "thegalacticsociety1@gmail.com";
    let host : &str = "smtp-relay.sendinblue.com";
    let to_email : &str = "iltifatibad@gmail.com";
    
    let email : Message = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(" AstroEducation Register ")
        .body(emailctx.to_string())
        .unwrap();

    let mailer : SmtpTransport = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(Credentials::new(from_email.to_string(), smtp_key.to_string(),)).build();
    
    match mailer.send(&email) {
        Ok(_) => println!("Email sent succesfully"),
        Err(e) => println!("Could not send email")
    }
    
    // let a = tmpl.render("advocateservice.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(redirect("/"))
}

pub async fn events(tmpl : web::Data<Tera>, conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut vecname = Vec::new();
    let mut vecimgpath = Vec::new();
    
    for i in 0..10 {
        let (name,imgpath): (String, String) = sqlx::query_as(" SELECT * FROM events ORDER BY RANDOM() LIMIT 1 ").fetch_one(&**conn).await.expect("AA");
        vecname.push(name);
        vecimgpath.push(imgpath);
    }
    ctx.insert("imgpath", &vecimgpath);
    ctx.insert("name", &vecname);
    log::info!("");
    let a = tmpl.render("events.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

#[get("/Events/{name}")]
async fn eventstailview(name: web::Path<String>,tmpl: web::Data<Tera>,conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse , Error> {
    let mut magazinename = format!("{}", name);
    let mut ctx = Context::new();
    let data: SMTailSaw = sqlx::query_as("select * from events where name = $1")
            .bind(&magazinename)
            .fetch_one(&**conn).await.expect("AA");
    ctx.insert("name", &data.name);
    ctx.insert("imgpath", &data.imgpath);
    ctx.insert("datapath", &data.datapath);

    let a = tmpl.render("eventstailsaw.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

pub async fn excursion(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("excursion.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn spacemagazine(tmpl : web::Data<Tera>, conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut vecname = Vec::new();
    let mut vecimgpath = Vec::new();
    
    for i in 0..10 {
        let (name,imgpath): (String, String) = sqlx::query_as(" SELECT * FROM spacemagazine ORDER BY RANDOM() LIMIT 1 ").fetch_one(&**conn).await.expect("AA");
        vecname.push(name);
        vecimgpath.push(imgpath);
    }
    ctx.insert("imgpath", &vecimgpath);
    ctx.insert("name", &vecname);
    log::info!("");
    let a = tmpl.render("spacemagazine.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
    
}

#[get("/SpaceMagazine/{name}")]
async fn spacemagazinetailview(name: web::Path<String>,tmpl: web::Data<Tera>,conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse , Error> {
    let mut magazinename = format!("{}", name);
    let mut ctx = Context::new();
    let data: SMTailSaw = sqlx::query_as("select * from spacemagazine where name = $1")
            .bind(&magazinename)
            .fetch_one(&**conn).await.expect("AA");
    ctx.insert("name", &data.name);
    ctx.insert("imgpath", &data.imgpath);
    ctx.insert("datapath", &data.datapath);

    // ctx.insert("imgpath", &imgpath);
    // ctx.insert("datapath", &datapath);

    let a = tmpl.render("spacemagazinetailsaw.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}


#[get("/Articles/{name}")]
async fn articlespdfview(name: web::Path<String>,tmpl: web::Data<Tera>,conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse , Error> {
    let mut pdfname = format!("./{}", name);
    let mut ctx = Context::new();
    ctx.insert("pdfname", &pdfname);

    let a = tmpl.render("articlespdf.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}






pub async fn astrocourses(tmpl : web::Data<Tera> ) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("courses.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn post_astrocourses(tmpl : web::Data<Tera> , form : web::Form<AstroCoursesPost>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    
    let a = tmpl.render("courses.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn careerdev(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("careerdevelopment.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn conferences(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("conferences.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn spacelibrary(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("library.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn post_conferences(tmpl : web::Data<Tera> , form : web::Form<AdvocateService> ) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut emailctx =format!(
        "Name : {} , Surname : {} , Email : {} , Phonenumber : {}", 
        form.0.name, 
        form.0.surname, 
        form.0.email,
        form.0.phone,
      );
    let smtp_key : &str = "xsmtpsib-9eadffaef474428d38bfa7f1ae67f43a75dbbc0b7fee011fdae7eda3c9718518-QYjMTWN1Zpz9rEtO";
    let from_email : &str = "thegalacticsociety1@gmail.com";
    let host : &str = "smtp-relay.sendinblue.com";
    let to_email : &str = "iltifatibad@gmail.com";
    
    let email : Message = Message::builder()
        .from(from_email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(" AstroEducation Register ")
        .body(emailctx.to_string())
        .unwrap();

    let mailer : SmtpTransport = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(Credentials::new(from_email.to_string(), smtp_key.to_string(),)).build();
    
    match mailer.send(&email) {
        Ok(_) => println!("Email sent succesfully"),
        Err(e) => println!("Could not send email")
    }
    
    // let a = tmpl.render("advocateservice.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(redirect("/"))
}


#[derive(Deserialize, Validate)]
pub struct AdvocateService {
    name : String,
    surname : String,
    phone : String,
    email : String
}

pub async fn advocateservice(tmpl : web::Data<Tera> ) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();   
    let a = tmpl.render("advocateservice.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

// pub async fn certificates(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
//     let mut ctx = Context::new();
//     let a = tmpl.render("certificates.html", &ctx).map_err(error::ErrorInternalServerError)?;
//     //.map_err("Err");
//     Ok(HttpResponse::Ok().body(a))
// }


pub async fn post_advocateservice(tmpl : web::Data<Tera> , form : web::Form<AdvocateService> ) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let mut emailctx =format!(
        "Name : {} , Surname : {} , Email : {} , Phonenumber : {}", 
        form.0.name, 
        form.0.surname, 
        form.0.email, 
        form.0.phone,
      );
    let smtp_key : &str = "xsmtpsib-9eadffaef474428d38bfa7f1ae67f43a75dbbc0b7fee011fdae7eda3c9718518-QYjMTWN1Zpz9rEtO";
    let from_email : &str = "thegalacticsociety1@gmail.com";
    let host : &str = "smtp-relay.sendinblue.com";
    let to_email : &str = "iltifatibad@gmail.com";
    
    let email : Message = Message::builder()
    .from(from_email.parse().unwrap())
    .to(to_email.parse().unwrap())
    .subject(" Advocate Service Register")
    .body(emailctx.to_string())
    .unwrap();

    let mailer : SmtpTransport = SmtpTransport::relay(&host)
    .unwrap()
    .credentials(Credentials::new(from_email.to_string(), smtp_key.to_string(),)).build();
    
    match mailer.send(&email) {
        Ok(_) => println!("Email sent succesfully"),
        Err(e) => println!("Could not send email")
    }
    
    // let a = tmpl.render("advocateservice.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(redirect("/"))
}

pub async fn certificates(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("certificates.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn galaxymeetingusa(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("galaxymeetingusa.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn galacticfamily(tmpl : web::Data<Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    let a = tmpl.render("galacticfamily.html", &ctx).map_err(error::ErrorInternalServerError)?;
    //.map_err("Err");
    Ok(HttpResponse::Ok().body(a))
}

pub async fn signin(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error>{
    if let Some(_) = session.get::<String>("name")?{
        return Ok(redirect("/"))
    }
    let ctx = Context::new();
    let a = tmpl.render("signin.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}



pub async fn post_signin(
    tmpl: web::Data<Tera>,
    form: web::Form<SigninUser>,
    session: Session,
    conn : web::Data<sqlx::SqlitePool>
    ) -> Result<HttpResponse, Error>{
    let mut ctx = Context::new();

    let add_user = sqlx::query("insert into users (id,name,surname,email,password,duty) values($1,$2,$3,$4,$5,$6)")
        // .bind(&id)
        .bind(&form.name)
        .bind(&form.surname)
        .bind(&form.email)
        .bind(&form.password)
        .bind("member").execute(&**conn).await.unwrap();
    session.insert("name", &form.name);
    session.insert("duty","member");
    Ok(redirect("/"))
}


pub async fn login(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error>{
    if let Some(_) = session.get::<String>("name")?{
        return Ok(redirect("/"))
    }
    if let Some(_) = session.get::<String>("email")?{
        return Ok(redirect("/"))
    }
    let ctx = Context::new();
    let a = tmpl.render("login.html", &ctx).map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(a))
}

pub async fn post_login(tmpl: web::Data<Tera>, form: web::Form<LoginUser>, session: Session, conn: web::Data<sqlx::SqlitePool>) -> Result<HttpResponse, Error>{
    let login_form = form.into_inner();
    let mut ctx = Context::new();
    if let Ok(_) = login_form.validate(){
        let user: sqlx::Result<User> = sqlx::query_as("select * from users where email = $1")
                .bind(&login_form.email)
                .fetch_one(&**conn).await;
            if let Ok(user)= user{
                let user: User = sqlx::query_as("select * from users where email = $1")
                .bind(&login_form.email)
                .fetch_one(&**conn).await.expect("AA");
    
            if (user.password == login_form.password) {
                ctx.insert("name", &user.name);
                return Ok(redirect("/"))
            } else {
                return Ok(redirect("/signin"));
            }
        }

    }
    Ok(redirect("/login"))
}

pub async fn postcards(tmpl: web::Data<Tera>, session: Session) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    if let Some(user) = session.get::<String>("name")?{
        let a = tmpl.render("postcards.html", &ctx).map_err(error::ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().body(a))
    } else {
        Ok(redirect("/membership"))
    }

    //.map_err("Err");
    
}

pub async fn onlinecoursescreate(MultipartForm(form): MultipartForm<UploadForm>) -> Result<impl Responder, Error> {
    for f in form.files {
        let mut fname:String =  (*form.name.clone()).to_string();
        fs::create_dir(&format!("../static/onlinecourses/{}/", fname));
        let path = format!("../static/onlinecourses/{}/{}", *form.name, f.file_name.unwrap());
        log::info!("saving to {path}");
        f.file.persist(path).unwrap();
    }

    Ok(HttpResponse::Ok())

}


fn redirect(location: &str)-> HttpResponse{
    HttpResponse::Found()
        .append_header((http::header::LOCATION, location))
        .finish()
}


