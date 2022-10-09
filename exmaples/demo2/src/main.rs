use clap::{Parser, Subcommand, Args};

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
        long,
        require_equals = true,
        value_name = "WHEN",
        // value_parser = parse_url
    )]
    url : String
}
/// feed post with an rul and optional key=value pairs. We will post the data
/// as Json, and retrieve the response for you
#[derive(Debug, Args)]
struct Post {
    url : String,
    body : Vec<String>
}

// #[allow(unused)]
// fn parse_url(url: &str) -> Result<String> {

//     Ok("asd".to_string())
// }


fn main() {
    let opts: Opts = Opts::parse();
    print!("{:?}", opts);
}