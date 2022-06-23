use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct Posts{
    #[serde(rename= "userId")]
    user_id : Option<usize>,
    id : Option<usize>,
    title : String,
    body : String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get("https://top.baidu.com/board?platform=pc&sa=pcindex_entry")
    .unwrap().text().unwrap();


    let client = reqwest::Client::new();

    let post_get_one = client
    .get("https://jsonplaceholder.typicode.com/posts")
    .send().await?.text().await?;


    // let ip = reqwest::get("http://httpbin.org/ip")
    //      .await?
    //      .json::<Posts>()
    //      .await?;
         
    let post_get_one = client
    .get("https://jsonplaceholder.typicode.com/posts")
    .send().await?.json::<Posts>().await?;

    // let content = reqwest::blocking::get("http://httpbin.org/range/26")?.text()?;
    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("div.content-pos_1fT0H>div>div.c-single-text-ellipsis").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());

    titles.zip(1..101).for_each(|(item, number)| println!("{}. {}", number, item));

    Ok(())
}
