use clap::Parser;

#[tokio::main]
async fn main() {
    // parse arguments
    let args = Args::parse();

    // build message
    let mut message = announce_lib::Message {
        text: args.message.as_deref(),
        file_path: args.file.as_deref(),
        hints: vec![],
    };

    // add hints
    if let Some(link) = args.description.as_deref() {
        message.hints.push(announce_lib::Hint::Description(&link));
    }

    if let Some(link) = args.link.as_deref() {
        message.hints.push(announce_lib::Hint::Link(&link));
    }

    // send to urls
    let urls = args
        .urls
        .iter()
        .map(|url| url::Url::parse(url).expect("url missformed"))
        .collect();
    let ann = announce_lib::Announce::new()
        .await
        .expect("init announce failed");
    ann.announce(urls, &message)
        .await
        .expect("error while announcing");
}

/// By specifing a url you can send messages and files to supported services
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// String to be send to urls
    #[arg(short, long)]
    message: Option<String>,

    /// Path to a file for sending
    #[arg(short, long)]
    file: Option<String>,

    /// Description for the file
    #[arg(short, long, requires = "file")]
    description: Option<String>,

    /// Target(s) of notification in url format
    urls: Vec<String>,

    /// Notify with a link. Some services highlight links
    #[arg(short, long)]
    link: Option<String>,
}
