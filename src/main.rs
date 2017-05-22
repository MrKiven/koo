extern crate iron;
#[macro_use]
extern crate tera;
#[macro_use]
extern crate lazy_static;

extern crate koo;

use iron::prelude::Iron;
use tera::{Tera, Context};
use koo::routers::router_generator;

// 延迟渲染模版
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = compile_templates!("templates/**/*");
        tera
    };
}

fn main () {
    let _server = Iron::new(router_generator()).http("localhost:3000").unwrap();
    println!("Server on :3000");
}
