use iron::status;
use iron::prelude::*;

use models;

pub fn koo_show(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Koo list")))
}

pub fn koo_add(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Add page")))
}

pub fn koo_delete(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "delete page")))
}
