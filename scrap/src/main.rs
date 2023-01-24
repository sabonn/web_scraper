extern crate reqwest;
extern crate scraper;

use std::fs::File;
use std::io::stdin;

fn main() {
    let mut inp = String::new();
    stdin().read_line(&mut inp).unwrap();//getting input from the user
    let inpclone = inp.clone();//cloning for later use
    let url = inp2url(inp);//changing the input to the url format
    let img_urls = get_html(&url);//get the images url
    for i in 0..img_urls.len() {
        let mut name = String::from(&inpclone);
        name.push_str(&i.to_string());//creating a name for each image
        get_image(&name, &img_urls[i]);//downloading the images
    }
}

fn inp2url(inp: String) -> String {
    //url format - https://www.google.com/search?q=[name+name]&tbm=isch&ved=2ahUKE/
    let mut url = String::from("https://www.google.com/search?q=");
    for i in inp.chars() {
        if i == ' ' { url.push('+'); }
        else if i == '\n' { continue;}
        url.push(i);
    }
    url.push_str("&tbm=isch&ved=2ahUKE/");//adding the end of the url format.
    return url;
}

fn get_html(url:&str) -> Vec<String>{
    let mut url_vec:Vec<String> = Vec::new();
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();//from this line until line 39 is just for getting the scraped html of the url
    let doc = scraper::Html::parse_document(&res);
    let selec = scraper::Selector::parse("div").unwrap();
    let temp = doc.select(&selec).map(| i | i.inner_html());
    temp.for_each(|i| if i.contains("<img src=") { url_vec.push(i) });//checking if the html contains the image url
    url_vec = get_url(url_vec);//scraping the urls from the html
    return  url_vec;
}

fn get_image(name: &str, url: &str) {
    let mut path = String::from("../pic/");
    path.push_str(&name);
    path.push_str(".jpg");//craeting a file at the pic folder
    let mut file = File::create(path).unwrap();
    reqwest::blocking::get(url).unwrap().copy_to(&mut file).unwrap();//copying the image to the file from the url
}

fn get_url(img_url: Vec<String>) -> Vec<String> {
    let mut url:Vec<String> = Vec::new();
    if img_url.is_empty() { return url; }
    for img in img_url {
        let mut img_clone = img.clone();//cloning the item for saftey reasons
        while img_clone.matches("https://encrypted-tbn0.gstatic.com/images?q=tbn:").count() > 1{
            let temp = img_clone.find("https://encrypted-tbn0.gstatic.com/images?q=tbn:").unwrap();//getting the index of the closest match to the url format
            let index = img_clone[temp..].find(";s").unwrap() + temp;//getting the index of the end of the url format
            
            if index <= temp { img_clone.replace_range(index..index+2, ""); }
            
            let txt = &img_clone[temp..index+2];//copying the string from the indexs that I got
            if !url.contains(&txt.to_string())/*checking if I already copied the url*/ {
                url.push(txt.to_string());
                img_clone.replace_range(temp..index+2, "");//removing the url from the string so we could search for more
            }
        }
    }
    return url;
}