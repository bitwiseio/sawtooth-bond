// Copyright 2018 Bitwise IO
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

use rocket::http::ContentType;
use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::{Json, Value};
use std::io::Cursor;

#[catch(404)]
pub fn not_found() -> Json<Value> {
    Json(json!({
        "error": {
            "status": 404,
            "message": "Not found"
        }
    }))
}

#[derive(Debug)]
pub enum ApiError {
    /// Defines the HTTP Errors that the API can return.
    NotImplemented,
}

impl<'r> Responder<'r> for ApiError {
    /// JSON responder for ApiErrors.
    fn respond_to(self, _req: &Request) -> Result<Response<'r>, Status> {
        match self {
            ApiError::NotImplemented => Response::build()
                .header(ContentType::JSON)
                .status(Status::NotImplemented)
                .sized_body(Cursor::new(
                    json!({
                        "error": {
                            "status": 501,
                            "message": "Not implemented",
                        }
                    }).to_string(),
                ))
                .ok(),
        }
    }
}
