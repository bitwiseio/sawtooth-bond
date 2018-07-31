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

// 'print_literal' lint disabled due to an issue in Rocket
// https://github.com/SergioBenitez/Rocket/issues/698
#![cfg_attr(feature = "cargo-clippy", allow(print_literal))]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod errors;
mod route_handlers;

use route_handlers::{bonds, holdings, orders, organizations, participants, quotes, settlements};

fn main() {
    rocket::ignite()
        .catch(catchers![errors::not_found])
        .mount(
            "/",
            routes![
                bonds::create_bond,
                bonds::list_bonds,
                bonds::retrieve_bond,
                holdings::create_holding,
                holdings::delete_holding,
                participants::login,
                organizations::create_organization,
                organizations::list_organizations,
                organizations::update_organization,
                organizations::retrieve_organization,
                organizations::update_organization_auth,
                participants::create_participant,
                participants::list_participants,
                participants::update_participant,
                participants::retrieve_participant,
                quotes::create_quote,
                quotes::list_quotes,
                quotes::retrieve_quote,
                orders::create_order,
                orders::list_orders,
                orders::retrieve_order,
                settlements::create_settlement,
                settlements::list_settlements
            ],
        )
        .launch();
}
