extern crate reqwest;
extern crate scraper;
extern crate image;

use std::io::stdin;

//url format - https://www.google.com/search?q=[name+name]&tbm=isch&ved=2ahUKE/

fn main() {
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
    let mut url_vec:Vec<String> = Vec::new();
    let mut _img_url:Vec<String> = Vec::new();
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    let doc = scraper::Html::parse_document(&res);
    let selec = scraper::Selector::parse("div").unwrap();
    let temp = doc.select(&selec).map(| i | i.inner_html());
    temp.for_each(|i| if i.contains("<img src=") { url_vec.push(i) });
    _img_url = get_url(url_vec);
    println!("{:?},{}", _img_url, _img_url.len());
}
/*
fn get_image(name: &str, url: &str) {
    //one way to download images using rust
    let mut file = File::create(name).unwrap();
    reqwest::blocking::get(url).unwrap().copy_to(&mut file).unwrap();
    //second way to download images using rust
    let img_bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let _img = image::load_from_memory(&img_bytes);
}*/

fn get_url(img_url: Vec<String>) -> Vec<String> {
    let mut url:Vec<String> = Vec::new();
    for j in 0..img_url.len() {
        let mut temp = false;
        let mut txt = String::from("h");
        for i in 0..img_url[j].len() {
            if i < img_url[j].len() - 4 && img_url[j].chars().nth(i).unwrap() == 'h' && img_url[j].chars().nth(i+1).unwrap() == 't' && img_url[j].chars().nth(i+2).unwrap() == 't' && img_url[j].chars().nth(i+3).unwrap() == 'p' && img_url[j].chars().nth(i+4).unwrap() == 's' {
                temp = true;
            } else if i < img_url[j].len() - 1 && img_url[j].chars().nth(i).unwrap() == ';' && img_url[j].chars().nth(i+1).unwrap() == 's' {
                temp = false;
                if txt.contains("https://encrypted-tbn0.gstatic.com/images?q=tbn:") {
                    txt.push_str(";s");
                    url.push(txt.clone());
                }
                txt.clear();
                txt.push('h');
            } else if temp {
                txt.push(img_url[j].chars().nth(i).unwrap());
            }
        }
    }
    return url;
}