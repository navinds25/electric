#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
//use rocket::fs::FileServer; // version 0.5
use rocket_contrib::serve::StaticFiles;

mod podcasts;

#[catch(404)]
fn not_found(req: &rocket::Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    let media_dir = "/media";
    rocket::ignite()
        .mount(
            "/v1",
            routes![index, podcasts::podcasts_main, podcasts::add_podcast_info],
        )
        .mount("/media", StaticFiles::from(media_dir))
        .register(catchers![not_found])
        .launch();
}

//pub fn rocket() -> rocket::Rocket {
//    dotenv().ok();
//    rocket::custom(config::from_env())
//        .mount(
//            "/api",
//            routes![
//                podcasts::podcasts_main,
//            ],
//        )
//        .attach(db::Conn::fairing())
//        .attach(cors_fairing())
//        .attach(config::AppState::manage())
//        .register(catchers![not_found])
//}
//
//fn main() {
//    rocket().launch();
//}
