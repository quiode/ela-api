use std::{str::FromStr, time::Duration};

use chrono::{serde::ts_seconds, DateTime, NaiveDateTime, Utc};
use rocket::serde::{json::Json, Serialize};
use rocket::{
    fs::{relative, FileServer},
    http::Status,
    serde::uuid::Uuid,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_db_pools::{
    sqlx::{self, Row},
    Connection, Database,
};

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TimeSpan {
    #[serde(with = "ts_seconds")]
    start: DateTime<chrono::Utc>,
    #[serde(with = "ts_seconds")]
    end: DateTime<chrono::Utc>,
    duration: Duration,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct History {
    user: Uuid,
    online_history: Vec<TimeSpan>,
}

fn int_to_datetime(int: i64) -> DateTime<Utc> {
    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(int, 0), Utc)
}

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::SqlitePool);

/// saves the ping in the database
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
        Err(_) => {
            result = Err(Status::InternalServerError);
        }
    }

    result
}

/// returns the duration for each device when it was turned on and how long
#[get("/data")]
async fn get_data(mut db: Connection<Db>) -> Json<Vec<History>> {
    let mut histories: Vec<History> = Vec::new();

    let mut all_users: Vec<String> = Vec::new();

    // get each unique user
    let fetched_users = sqlx::query("SELECT uuid from pings GROUP BY uuid")
        .fetch_all(&mut *db)
        .await
        .unwrap();

    for user in fetched_users {
        let uuid: String = user.get("uuid");
        all_users.push(uuid)
    }
    // calculate each time span for each user
    for user in all_users {
        let mut timespans: Vec<TimeSpan> = vec![];

        let mut timestamps: Vec<i64> = vec![];

        // get all pings, sorted
        let fetched_timestamps =
            sqlx::query("SELECT timestamp from pings WHERE uuid == ? ORDER BY timestamp")
                .bind(&user)
                .fetch_all(&mut *db)
                .await
                .unwrap();

        for timestamp in fetched_timestamps {
            timestamps.push(timestamp.get("timestamp"));
        }

        // iterate through each ping and check with previous one, if time difference is too big, create new timespan
        if timestamps.len() <= 4 {
            continue;
        }

        let timestamp = TimeSpan {
            start: int_to_datetime(*timestamps.get(0).unwrap()),
            end: int_to_datetime(*timestamps.get(0).unwrap()),
            duration: Duration::from_secs(0),
        };

        timespans.push(timestamp);

        let mut last_timestamp = *timestamps.get(0).unwrap();

        for timestamp in timestamps {
            if timestamp - last_timestamp >= 60 * 5 {
                let mut last_timespan = timespans.pop().unwrap();
                last_timespan.end = int_to_datetime(last_timestamp);
                last_timespan.duration = Duration::from_secs(
                    (last_timespan.end.timestamp() - last_timespan.start.timestamp())
                        .try_into()
                        .unwrap(),
                );
                timespans.push(last_timespan);

                let new_timespan = TimeSpan {
                    start: int_to_datetime(timestamp),
                    end: int_to_datetime(timestamp),
                    duration: Duration::from_secs(0),
                };

                timespans.push(new_timespan);
            } else {
                let mut last_timespan = timespans.pop().unwrap();
                last_timespan.end = int_to_datetime(timestamp);
                last_timespan.duration = Duration::from_secs(
                    (last_timespan.end.timestamp() - last_timespan.start.timestamp())
                        .try_into()
                        .unwrap(),
                );
                timespans.push(last_timespan);
            }

            last_timestamp = timestamp;
        }

        match Uuid::from_str(&user) {
            Ok(user) => {
                let history = History {
                    user: user,
                    online_history: timespans,
                };

                histories.push(history);
            }
            Err(_) => continue,
        }
    }

    // TODO: export all the histories to json and return them
    return Json(histories);
}

#[launch]
fn rocket() -> _ {
    // cors
    let allowed_origins = AllowedOrigins::All;

    let cors_options = CorsOptions {
        allowed_origins,
        ..Default::default()
    };

    let cors = cors_options.to_cors().unwrap();

    rocket::build()
        .attach(Db::init())
        .attach(cors)
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![ping, get_data])
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
