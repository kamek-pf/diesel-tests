#[macro_use]
extern crate diesel;
extern crate shared_types;

use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Bool;

use shared_types::model::postgres::schema::{self, accounts};
use shared_types::model::{Position, DeviceType, OperatingSystem};

#[derive(Debug, Queryable)]
pub struct Place {
    pub id: i32,
    pub wifi: String,
    pub address: String,
    pub position: Position
}

#[derive(Debug, Queryable)]
pub struct Dev {
    pub account: i32,
    pub os: Option<OperatingSystem>,
    pub device_type: Option<DeviceType>
}

#[derive(Debug, QueryableByName)]
#[table_name = "accounts"]
struct User {
    id: i32,
    marketing: bool,
}

fn main() {
    let res = find_users();

    match res {
        Ok(v) => println!("User ! {:?}", v),
        Err(e) => println!("Nope :( {:?}", e),
    }

    let res = find_one();

    match res {
        Ok(v) => println!("Place ! {:?}", v),
        Err(e) => println!("Nope :( {:?}", e),
    }

    let res = find_devices();

    match res {
        Ok(v) => println!("Devices ! {:?}", v),
        Err(e) => println!("Nope :( {:?}", e),
    }
}

fn find_one() -> Result<Place, diesel::result::Error> {
    use schema::places::dsl::*;
    let cn = establish_connection();
    let res: Place = places
        .select((id, wifi_name, timezone, position))
        .first(&cn)?;
    Ok(res)
}

fn find_devices() -> Result<Dev, diesel::result::Error> {
    use schema::devices::dsl::*;
    let cn = establish_connection();
    let res: Dev = devices
        .select((account, os, device_type))
        .first(&cn)?;
    Ok(res)
}

fn find_users() -> Result<User, diesel::result::Error> {
    let cn = establish_connection();
    let users: User = sql_query(include_str!("test.sql"))
        .bind::<Bool, _>(false)
        .get_result(&cn)?;
    Ok(users)
}

pub fn establish_connection() -> PgConnection {
    let database_url = "connection string here";

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
