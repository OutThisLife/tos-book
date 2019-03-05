use csv::ReaderBuilder;
use regex::bytes::Regex;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Record {
  pub amount: String,
  pub balance: String,
  pub date: String,
  pub desc: String,
  pub fees: String,
  id: String,
  pub time: String,
}

pub fn read_file(filename: &'static str) -> Result<String, Box<Error>> {
  let mut buf = String::new();
  let mut f = File::open(Path::new(filename))?;

  f.read_to_string(&mut buf)?;
  Ok(buf)
}

pub fn read_export<'a>() -> Result<Vec<Record>, Box<Error>> {
  let mut res = vec![];
  let buf = read_file("./static/export.csv")?;

  let re = Regex::new(r"Forex Statements(?P<cstr>[\s\S]+)Total Cash")?;
  let caps = re
    .captures_iter(buf.as_bytes())
    .map(|c| c.name("cstr").unwrap().as_bytes())
    .collect::<Vec<&[u8]>>();

  let mut rdr = ReaderBuilder::new().from_reader(caps[0]);

  for result in rdr.records() {
    let record = result?;

    res.push(Record {
      amount: record[7].parse()?,
      balance: record[9].parse()?,
      date: record[1].parse()?,
      desc: record[2].parse()?,
      fees: record[6].parse()?,
      id: record[4].parse()?,
      time: record[2].parse()?,
    })
  }

  Ok(res)
}

pub fn plot(export: &Vec<Record>) -> Result<String, Box<Error>> {
  let mut data = vec![];
  let parse = chrono::NaiveDateTime::parse_from_str;

  for res in export {
    let mut date = res.date.to_string();
    let balance = res.balance[1..].replace(",", "").to_string();

    date.push_str(&" ".to_string());
    date.push_str(&res.time.to_string());

    let x = match parse(&date, "%-m/%-d/%y %H:%M:%S") {
      Ok(n) => n.timestamp() as f64,
      Err(_) => continue,
    };

    let y = match balance.parse::<f64>() {
      Ok(n) => n,
      Err(_) => continue,
    };

    data.push((x, y));
  }

  let l1 = plotlib::line::Line::new(&data).style(
    plotlib::style::LineStyle::new()
      .colour("#0000ee")
      .width(1.0),
  );

  let v = plotlib::view::ContinuousView::new().add(&l1);
  let p = plotlib::page::Page::empty().add_plot(&v);

  Ok(p.to_svg().unwrap().to_string())
}
