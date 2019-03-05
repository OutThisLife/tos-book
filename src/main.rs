#![feature(proc_macro_hygiene)]
#![recursion_limit = "256"]

extern crate actix_web;
extern crate chrono;
extern crate csv;
extern crate listenfd;
extern crate maud;
extern crate minifier;
extern crate plotlib;
extern crate regex;

use actix_web::{fs::StaticFiles, http::Method, server, App};
use listenfd::ListenFd;
use minifier::css::minify;

use std::sync::Arc;
use std::sync::Mutex;

mod data;
mod routes;

pub struct AppState {
  styles: Arc<Mutex<String>>,
  export: Arc<Mutex<Vec<data::Record>>>,
}

fn main() {
  let mut listenfd = ListenFd::from_env();

  let styles = minify(data::read_file("./static/style.css").unwrap().as_str()).unwrap();
  let export = data::read_export().unwrap();

  let mut app = server::new(move || {
    App::with_state(AppState {
      styles: Arc::new(Mutex::new(styles.clone())),
      export: Arc::new(Mutex::new(export.clone())),
    })
    .resource("/", |r| r.method(Method::GET).f(routes::index))
    .handler(
      "/static",
      StaticFiles::new("./static").unwrap().show_files_listing(),
    )
    .finish()
  });

  app = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    app.listen(l)
  } else {
    app.bind("127.0.0.1:3000").unwrap()
  };

  println!("Started http server: [::1]:3000");
  let _ = app.run();
}
