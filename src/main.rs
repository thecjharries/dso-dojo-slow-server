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

use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::{build, get, launch, routes};
use serde::{Deserialize, Serialize};
use std::io::Result;

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
    build().mount("/", routes![index, ping])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::{Client, LocalResponse};
    use std::fs::read_to_string;

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
}
