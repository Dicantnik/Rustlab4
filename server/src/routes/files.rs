use std::path::Path;
use rocket::fs::NamedFile;

#[get("/<file_name>")]
pub async fn download_file(file_name: String) -> Option<NamedFile> {
    let file_path = Path::new("./uploads").join(file_name);

    if file_path.exists() {
        NamedFile::open(file_path).await.ok()
    } else {
        None
    }
}