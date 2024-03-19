use reqwest;
use soup::prelude::*;
use clap::Parser;



/// get links from web page
#[derive(Parser)]
#[clap(
    name="get_url",
    author="masamichi",
    version="v0.0.1",
    about="get url links from  webpage"
)]
struct Args {
    /// target URL
    url: String,

    /// filter string from URL 
    #[arg(short, long)]
    filter: Option<String>,

    /// remove number of characters from each head
    #[arg(short, long)]
    substring: Option<u8>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let arg: Args = Args::parse();

    // HTMLソースの取得
    let html = get_html_contents(&arg.url).await?;

    // ソースをパースしてリンクを取得
    // フィルタで文字列、サブストリング設定
    let list = get_url_list(html, arg.filter, arg.substring);

    println!("{:?}", list);


    Ok(())
}

fn get_url_list(html:String, filter:Option<String> , substring: Option<u8>) -> Vec<String>{

    let soup = Soup::new(&html);
    let mut url_list: Vec<String> = vec![];

    for  (_i,link) in soup.tag("a").find_all().enumerate(){

        // リンク取得
        let href= link.get("href");

        // フィルタ文字列を取得　デフォは/
        let filter_char = &filter.clone().unwrap_or(String::from("/"));

        // リンクがNoneならスキップ
        // 値があれば、リストに追加
        match href{
            Some(x)=> if x.contains(filter_char) { url_list.push(x)},
            None => continue
        }

    }

    // サブストオプションを確認　
    if substring.is_some(){
        // rangeにするためにu8をusizeにキャスト 
        let char_count:usize = (substring.unwrap() ).into();

        // その数、冒頭から文字を削除
        url_list.iter_mut().for_each(|x| x.replace_range(0..char_count,""));
    }

    url_list

}

async fn get_html_contents(url:&str) -> Result<String, Box<dyn std::error::Error>>{

    // htmlソースの取得
    let url =url;
    let contents = reqwest::get(url).await?.text().await?;

    Ok(contents)

} 