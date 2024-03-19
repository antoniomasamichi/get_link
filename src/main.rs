use reqwest;
use soup::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = get("https://www.seikyoonline.com/").await?;

    let soup = Soup::new(&html);

    let title = soup.tag("title").find().expect("can'T find title");

    println!("{:?}", title.text());

    let mut url_list: Vec<String> = vec![];

    for  (_i,link) in soup.tag("a").find_all().enumerate(){

        let href= link.get("href");
        match href{
            Some(x)=> if x.contains("article") { url_list.push(x)},
            None => continue
        }
    }

    url_list.iter_mut().for_each(|x| x.replace_range(0..2,""));

    println!("{:?}",url_list);
    Ok(())
}

async fn get(url:&str) -> Result<String, Box<dyn std::error::Error>>{

    let url =url;
    let contents = reqwest::get(url).await?.text().await?;

    Ok(contents)

} 