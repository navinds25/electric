use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AddPodcast {
    name: String,
    url: String,
}

fn add_podcast_info_db(conn: &SqliteConnection, podcast: AddPodcast) -> Result<String, String> {
    return Ok("hello".to_string())
}