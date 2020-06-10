use crate::config::Config;

mod config;
mod execute;
mod query;

fn main() {
    let mut args = std::env::args();
    if args.len() != 3 {
        return help();
    }
    args.next();
    let req_type = args.next().unwrap().to_lowercase();
    let req_content = args.next().unwrap();
    if req_content.trim().is_empty() {
        return;
    }
    if req_type == "-q" || req_type == "--query" {
        let identifier = std::env::var("IDENTIFIER").expect("Cannot find IDENTIFIER from env");
        let output = std::env::var("OUTPUT").expect("Cannot find OUTPUT from env");
        let settings = std::env::var("SETTINGS").expect("Cannot find SETTINGS from env");
        let config: Config =
            serde_json::from_str(&settings).expect("Settings cannot be deserialized to config");
        if let Some(response) = query::query(req_content.trim(), config.lowercased(), &identifier) {
            std::fs::write(output, response).expect("Failed to write response");
        }
    } else if req_type == "-x" || req_type == "--execute" {
        execute::execute(req_content.trim())
    } else if req_type == "-X" || req_type == "--alter-execute" {
        execute::preview(req_content.trim())
    } else {
        help()
    }
}

fn help() {
    println!(
        r#"
Simple Web Service

Options:
-q, --query          <QUERY>     List quick open urls
-x, --execute        <ID>        Open given URL in your default browser
-X, --alter-execute  <ID>        Preview given URL in the quick look 
    "#
    )
}
