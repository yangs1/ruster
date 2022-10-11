use std::{str::FromStr, collections::HashMap};

use clap::{Parser, Subcommand, Args};
use colored::Colorize;
use mime::Mime;
use reqwest::{Url, Client, header, Response};
use anyhow::{anyhow, Result, Ok};
use syntect::{parsing::SyntaxSet, highlighting::{ThemeSet, Style}, easy::HighlightLines, util::{LinesWithEndings, as_24_bit_terminal_escaped}};

/// a naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd : Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// this is a Get request
    Get(Get),
    Post(Post),
}

/// feed get with an url an we will retrieve the response for you
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
struct  Get {
    #[arg(
        require_equals = false,
        value_name = "URL",
        ignore_case = true,
        // action = clap::ArgAction::Help
        value_parser = parse_url
    )]
    url : String
}
/// feed post with an rul and optional key=value pairs. We will post the data
/// as Json, and retrieve the response for you
#[derive(Debug, Args)]
struct Post {
    #[arg(
        long,
        short,
        // value_parser = parse_url,
    )]
    url : String,
    #[arg(
        value_parser = parse_body
    )]
    body : Vec<KvPair>
}

fn parse_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;
    
    Ok(url.into())
}


#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
struct KvPair {
    k : String,
    v : String
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");

        let err = || anyhow!("Failed to parse {}", s);

        Ok(Self {
            k : (split.next().ok_or_else(err)?).to_string(),
            v : (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_body(body: &str) -> Result<KvPair> {
    Ok(body.parse()?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    // print!("{:?}", opts);
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);


    let client = Client::builder().default_headers(headers).build()?;

    // client 发生 move, 如果 match 之后要用 client 需要使用 &
    let result = match opts.subcmd{
        Commands::Get(ref args) => get(client,args).await?,
        Commands::Post(ref args) => post(client, args).await?
    };


    Ok(result)
}


async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;

    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }

    let resp = client.post(&args.url).json(&body).send().await?;
    
    Ok(print_resp(resp).await?)
}

/** ===================*/

fn print_status(resp : &Response) {
    let status = format!("{:?} - {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_header(resp : &Response){
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON  => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML  => print_syntect(body, "html"),
        _ => println!("{}", body),  
            
    }
}

async fn print_resp(resp: Response)-> Result<()> {
    print_status(&resp);
    print_header(&resp);
    let mime: Option<Mime> = resp.headers().get(header::CONTENT_TYPE).map(|v| v.to_str().unwrap().parse().unwrap());
    let body = resp.text().await?;

    print_body(mime, &body);

    Ok(())
}



fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works(){
        assert!(parse_url("asd").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
    }

    #[test]
    fn parse_kv_pair_works(){
        assert!(parse_body("body").is_err());

        assert_eq!(
            parse_body("body=123").unwrap(),
            KvPair {
                k : "body".into(),
                v : "123".into()
            }
        )
    }
}


// cargo run post -u=https://httpbin.org/post greeting=hla name=tyr
// cargo run get https://httpbin.org/get 