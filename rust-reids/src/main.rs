use redis::{Client, aio::Connection, AsyncCommands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut conn = get_connect("redis://127.0.0.1:6379").await?;
    let pong:String = redis::cmd("PING").query_async(&mut conn).await?;

    println!("{}", pong);

    conn.set("sad", 12).await?;

    let res:String = match conn.get("sad2").await {
        Ok(s) => s,
        Err(_) => {
            println!("this key is not available");
            "".to_string()
        }
    };

    conn.xadd("stream_key", "*", &[("a","aa1"),("b","bb1"),("c","cc1"),("d","dd1")]).await?;
    conn.xadd("stream_key", "*", &[("a","aa2"),("b","bb2"),("c","cc2"),("d","dd2")]).await?;
    conn.xadd("stream_key", "*", &[("a","aa3"),("b","bb3"),("c","cc3"),("d","dd3")]).await?;
    conn.xadd("stream_key", "*", &[("a","aa4"),("b","bb4"),("c","cc4"),("d","dd4")]).await?;


    let len:usize = conn.xlen("stream_key").await?;

    println!("{}", len);



    conn.del("stream_key").await?;
    conn.del("sad").await?;

    println!("{}", res);

    Ok(())
}

async fn get_connect(path: &str) -> Result<Connection, Box<dyn std::error::Error>>{
    let client = Client::open(path)?;
    let conn = client.get_tokio_connection().await?;

    Ok(conn)
}

