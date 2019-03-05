use actix_web::{http::ContentEncoding, HttpRequest, HttpResponse};
use maud::{html, Markup, PreEscaped, DOCTYPE};

use data::plot;
use AppState;

fn render(req: &HttpRequest<AppState>, body: Markup) -> HttpResponse {
  let doc = html! {
    (DOCTYPE)
    html {
      head {
        meta charset="utf-8";
        meta http-equiv="X-UA-Compatible" content="IE=edge";

        title { "tosbook" }

        link rel="stylesheet" href="//cdnjs.cloudflare.com/ajax/libs/normalize/8.0.1/normalize.min.css";
        style { (PreEscaped(String::from(req.state().styles.lock().unwrap().clone()))) }
      }

      body { (body) }
    }
  };

  HttpResponse::Ok()
    .content_type("text/html")
    .content_encoding(ContentEncoding::Br)
    .body(doc.into_string())
}

pub fn index(req: &HttpRequest<AppState>) -> HttpResponse {
  let export = req.state().export.lock().unwrap().clone();
  let chart = plot(&export).unwrap();

  render(
    req,
    html! {
      section {
        (PreEscaped(chart))
      }

      section {
        @for res in &export {
          div {
            span { (res.date) ", " (res.time) }
            span { (res.amount) }
            span { (res.fees) }
            span { (res.balance) }
          }
        }
      }
    },
  )
}
