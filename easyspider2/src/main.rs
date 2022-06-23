use serde::{Serialize, Deserialize};

#[derive(Debug,Deserialize, Serialize)]
struct Posts{
    #[serde(rename= "userId")]
    user_id : Option<usize>,
    id : Option<usize>,
    title : String,
    body : String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    
    let client = reqwest::Client::new();

    //解析单个
    let posts_get_one = client
    .get("https://jsonplaceholder.typicode.com/posts/1")
    .send().await?.json::<Posts>().await?;

    println!("{:?}", posts_get_one);


    let posts_get_array = client
    .get("https://jsonplaceholder.typicode.com/posts")
    .send().await?.json::<Vec<Posts>>().await?;

    println!("{:#?}", posts_get_array);

    

    let create_posts = Posts{
        user_id:None,
        id : None,
        title : "shaoxia_test".to_string(),
        body : "shaoxia_test body".to_string()
    };

    let create_posts_resp = client
    .post("https://jsonplaceholder.typicode.com/posts")
    .json(&create_posts)
    .send()
    .await?
    .json::<Posts>()
    .await?;

    println!("{:#?}", create_posts_resp);

    


    //================ end 结构体请求 ===================


    

    Ok(())
}
