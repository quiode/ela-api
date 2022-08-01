use chrono::Utc;
use database_interactions::database_interactions::Db;
use rocket::{
    fs::{relative, FileServer},
    serde::uuid::Uuid,
};
use rocket_db_pools::Database;

mod database_interactions;

#[macro_use]
extern crate rocket;

#[post("/ping/<uuid>")]
fn ping(uuid: Uuid) -> String {
    format!("{}: {}", uuid, Utc::now())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![ping])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;

    #[test]
    fn ping() {
        const UUID: &str = "dc1a46a3-6e0f-4a9b-9e37-22291471e8e5";

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(format!("/api/ping/{}", UUID)).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn website() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::HTML);
    }
}
