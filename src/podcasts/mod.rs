/* mod.rs contains the handlers for the podcasts api */

use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
mod db;

/* podcasts main, receives get request and returns json data for podcasts main page*/
#[get("/podcasts")]
pub fn podcasts_main() -> &'static str {
    "podcasts home page"
}


/* podcasts_add, receives a post request and returns a success message */
#[post("/podcasts", format = "json", data = "<req>")]
pub fn add_podcast_info(req: Json<db::AddPodcast>) ->  &'static str {
    let podcast = req.into_inner();
    println!("podcast: {:?}", podcast);
    let res = db::add_podcast_info_db(podcast);
    match res {
        Ok(v) => println!("added podcast"),
        Err(e) => println!("error adding podcast"),
    }
    "added podcast"
}

/* podcast info */

/* list podcast episodes */
