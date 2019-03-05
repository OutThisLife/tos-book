use chrono::NaiveDateTime;
use csv::ReaderBuilder;
use minifier::css::minify;
use regex::bytes::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Record {
  id: String,
  pub amount: String,
  pub balance: f64,
  pub date: f64,
  pub desc: String,
  pub fees: String,
  pub time: String,
}

pub fn read_file(filename: &'static str) -> Result<String, Box<Error>> {
  let mut buf = String::new();
  let mut f = File::open(Path::new(filename))?;

  f.read_to_string(&mut buf)?;
  Ok(buf)
}

pub fn read_styles() -> Result<String, &'static str> {
  minify(read_file("./src/style.css").unwrap().as_str())
}

pub fn read_export<'a>() -> Result<Vec<Record>, Box<Error>> {
  let mut res = vec![];
  let buf = read_file("./src/export.csv")?;

  let re = Regex::new(r"Forex Statements(?P<cstr>[\s\S]+)Total Cash")?;
  let caps = re
    .captures_iter(buf.as_bytes())
    .map(|c| c.name("cstr").unwrap().as_bytes())
    .collect::<Vec<&[u8]>>();

  let mut rdr = ReaderBuilder::new().from_reader(caps[0]);

  for result in rdr.records() {
    let record = result?;

    let date = NaiveDateTime::parse_from_str(
      &vec![&record[1], &record[2]].join(" "),
      "%-m/%-d/%y %H:%M:%S",
    )?;

    let balance = &record[9][1..].replace(",", "");

    res.push(Record {
      id: record[4].parse()?,
      amount: record[7].parse()?,
      balance: balance.to_string().parse::<f64>()?,
      date: date.timestamp() as f64,
      time: record[2].parse()?,
      desc: record[2].parse()?,
      fees: record[6].parse()?,
    })
  }

  Ok(res)
}

pub fn plot(export: &Vec<Record>) -> Result<String, Box<Error>> {
  let data: Vec<(f64, f64)> = export.iter().map(|d| (d.date, d.balance)).collect();

  let l1 = plotlib::line::Line::new(&data).style(
    plotlib::style::LineStyle::new()
      .colour("#000000")
      .width(1.0),
  );

  let v = plotlib::view::ContinuousView::new()
    .add(&l1)
    .x_label("timestamp")
    .y_label("balance");

  Ok(
    plotlib::page::Page::empty()
      .add_plot(&v)
      .to_svg()
      .unwrap()
      .to_string(),
  )
}
