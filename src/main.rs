// Copyright 2022 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rand::prelude::*;
use rand_pcg::Pcg64;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::{build, get, launch, routes};
use rocket_sync_db_pools::diesel::prelude::*;
use rocket_sync_db_pools::diesel::sql_query;
use rocket_sync_db_pools::{database, diesel};
use serde::{Deserialize, Serialize};
use std::io::Result;

const API_WAIT_SECONDS: i32 = 1;

#[database("postgres")]
struct Database(diesel::PgConnection);

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct ApiResponse {
    id: u64,
    token: String,
}

#[get("/api/<id>")]
async fn api(conn: Database, id: u64) -> Json<ApiResponse> {
    conn.run(|c| {
        sql_query("SELECT pg_sleep($1)")
            .bind::<diesel::sql_types::Integer, _>(API_WAIT_SECONDS)
            .execute(c)
            .unwrap();
    })
    .await;
    let mut rng: Pcg64 = Pcg64::seed_from_u64(id);
    let mut token: String = String::new();
    for _ in 0..64 {
        token.push(rng.gen_range('a'..='z'));
    }
    Json(ApiResponse { id, token })
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct PingResponse {
    message: String,
}

#[get("/ping")]
fn ping() -> Json<PingResponse> {
    Json(PingResponse {
        message: "pong".to_string(),
    })
}

#[get("/")]
async fn index() -> Result<NamedFile> {
    NamedFile::open("files/index.html").await
}

#[launch]
fn rocket() -> _ {
    build()
        .attach(Database::fairing())
        .mount("/", routes![index, ping, api])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::{Client, LocalResponse};
    use std::fs::read_to_string;
    use std::time::{Duration, Instant};

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response: LocalResponse = client.get("/").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(
            response.content_type(),
            Some(rocket::http::ContentType::HTML)
        );
        assert_eq!(
            response.into_string().unwrap(),
            read_to_string("files/index.html").unwrap()
        );
    }

    #[test]
    fn test_ping() {
        let client = Client::tracked(rocket()).unwrap();
        let response: LocalResponse = client.get("/ping").dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
        assert_eq!(
            response.into_json::<PingResponse>().unwrap(),
            PingResponse {
                message: "pong".to_string(),
            }
        );
    }

    #[test]
    fn test_api_proper() {
        let client: Client = Client::tracked(rocket()).unwrap();
        let expected_responses: Vec<ApiResponse> = vec![
            ApiResponse {
                id: 10,
                token: "hjupifwjnzholhbcehxlmdgaayihhjfbsnkmaecvmumzcmyfqueruzayamxhpflo"
                    .to_string(),
            },
            ApiResponse {
                id: 11,
                token: "yasjymdhhvasuqowyidxvsuzxrusynlzbxhoulctrknnljohnqidzekeisqbrcyn"
                    .to_string(),
            },
            ApiResponse {
                id: 18446744073709551615,
                token: "hfrickgjqfuupnkigfaurvmylyoldzyyagvmkutmlotzsewkrqakhtdjldvnfrni"
                    .to_string(),
            },
        ];
        for expected_response in expected_responses {
            let start: Instant = Instant::now();
            let response: LocalResponse = client
                .get(format!("/api/{}", expected_response.id))
                .dispatch();
            let end: Instant = Instant::now();
            assert_eq!(response.status(), rocket::http::Status::Ok);
            assert_eq!(
                response.into_json::<ApiResponse>().unwrap(),
                expected_response
            );
            assert!(Duration::from_secs(API_WAIT_SECONDS.abs() as u64) < end.duration_since(start));
        }
    }
}
