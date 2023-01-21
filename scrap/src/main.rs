extern crate reqwest;
extern crate scraper;
extern crate image;
use std::io::stdin;

//url format - https://www.google.com/search?q=[name+name]&tbm=isch&ved=2ahUKE/

fn main() {
    scrap();
}

fn scrap() {
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();
    let url = inp2url(inp);
    println!("{}",url);
    get_html(&url);
}

fn inp2url(inp: String) -> String {
    let mut url = String::from("https://www.google.com/search?q=");
    for i in inp.chars() {
        match i {
            ' ' => url.push('+'),
            '\n' => continue,
            _ => url.push(i)
        }
    }
    url.push_str("&tbm=isch&ved=2ahUKE/");
    return url;
}

fn get_html(url:&str) {
    let mut img_url:Vec<String> = Vec::new();
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    let doc = scraper::Html::parse_document(&res);
    let selec = scraper::Selector::parse("div").unwrap();
    let temp = doc.select(&selec).map(| i | i.inner_html());
    temp.for_each(|i| if i.contains("<img src=") { img_url.push(i) });
    println!("{:?}", img_url);
}
