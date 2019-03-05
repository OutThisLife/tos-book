#![feature(proc_macro_hygiene)]
#![recursion_limit = "256"]

mod data;

extern crate chrono;
extern crate csv;
extern crate maud;
extern crate minifier;
extern crate plotlib;
extern crate regex;

use maud::{html, PreEscaped};

fn render() -> maud::Markup {
  let styles = data::read_styles().unwrap();
  let export = data::read_export().unwrap();
  let chart = data::plot(&export).unwrap();

  html! {
    (maud::DOCTYPE)
    html {
      head {
        meta charset="utf-8";
        meta http-equiv="X-UA-Compatible" content="IE=edge";

        title { "tosbook" }

        link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css";
        style { (PreEscaped(String::from(styles))) }
      }

      body {
        section {
          (PreEscaped(chart))
        }

        section {
          @for res in &export {
            div {
              span { (res.balance) }
              span { (res.amount) }
              span { (res.fees) }
              span { (res.date) }
            }
          }
        }
      }
    }
  }
}

fn main() {
  let build_dir = std::path::Path::new("./.out");

  std::fs::create_dir_all(build_dir).unwrap();

  match std::fs::write(
    build_dir.join("index.html"),
    String::from(render().into_string()).as_bytes(),
  ) {
    Ok(_) => println!("Export success"),
    Err(e) => panic!(e),
  }
}
