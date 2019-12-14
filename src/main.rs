use std::fs::OpenOptions;
use std::io::prelude::*;

use chrono::prelude::*;

use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Responce {
    items: Vec<Lib>,
}

#[derive(Deserialize, Debug)]
struct Lib {
    name: String,
    html_url: String,
    stargazers_count: u32,
    open_issues: u32,
    description: String,
}

const PREAMBULE: &'static str = "# Rust http library list

A list of Rust http libs inspired by https://github.com/mingrammer/go-web-framework-stars


|Name|Stars|Issues|Description|
|:--:|:---:|:--:|:----:|
";

fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let mut response = client
        .get("https://api.github.com/search/repositories?q=language:rust+topic:http&sort=stars&order=desc")
        .send()?;
    let libs = response.json::<Responce>()?.items;

    let mut read_me = String::from(PREAMBULE);
    for lib in libs {
        read_me = format!(
            "{base}|[{name}]({url})|{stars}|{issues}|{desc}|\n",
            base = read_me,
            name = lib.name,
            url = lib.html_url,
            stars = lib.stargazers_count,
            issues = lib.open_issues,
            desc = lib.description,
        );
    }

    read_me = format!(
        "{base}\nLast Automatic Update: {date}",
        base = read_me,
        date = Utc::now()
    );
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("README.md")
        .unwrap();
    file.write_all(read_me.as_bytes())
        .expect("Could not write to file");
    Ok(())
}
