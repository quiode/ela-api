use rocket::serde::uuid::Uuid;

#[macro_use]
extern crate rocket;

#[post("/ping/<uuid>")]
fn ping(uuid: Uuid) -> String {
    uuid.to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![ping])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn ping() {
        const UUID: &str = "dc1a46a3-6e0f-4a9b-9e37-22291471e8e5";

        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(format!("/api/ping/{}", UUID)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), UUID);
    }
}
