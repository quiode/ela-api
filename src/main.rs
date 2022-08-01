use rocket::{
    fs::{relative, FileServer},
    http::Status,
    serde::uuid::Uuid,
};
use rocket_db_pools::{
    sqlx::{self},
    Connection, Database,
};

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::SqlitePool);

#[macro_use]
extern crate rocket;

#[post("/ping/<uuid>")]
async fn ping(mut db: Connection<Db>, uuid: Uuid) -> Result<String, Status> {
    let timestamp = chrono::Utc::now().timestamp();
    let uuid = uuid.to_string();

    let result: Result<String, Status>;

    match sqlx::query("INSERT INTO pings (uuid, timestamp) VALUES (?,?)")
        .bind(&uuid)
        .bind(timestamp)
        .execute(&mut *db)
        .await
    {
        Ok(_) => result = Ok(format!("{}: {}", uuid, timestamp)),
        Err(_) => result = Err(Status::InternalServerError),
    }

    result
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
