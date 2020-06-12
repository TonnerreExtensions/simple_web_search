use crate::config::Config;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use serde_json::json;
use std::collections::VecDeque;
use std::str::Split;

const FRAGMENT: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'<')
    .add(b'>')
    .add(b'`')
    .add(b'&');

pub fn query(req: &str, config: Config, identifier: &str) -> Option<String> {
    let (keyword, content) = separate_keyword_content(req);
    let encoded_content = utf8_percent_encode(&content, FRAGMENT).to_string();
    let content_split = encoded_content.split(" ");
    let matched_urls = find_matched_urls(&config, &keyword);
    let filled_urls = matched_urls
        .into_iter()
        .filter_map(|(keyword, url)| Some((keyword, fill_matched_url(url, content_split.clone())?)))
        .collect::<Vec<_>>();
    let services = filled_urls
        .iter()
        .map(|(keyword, url)| {
            json!({
                "id": url,
                "subtitle": url,
                "title": format!("{} {}", keyword, if content.is_empty() {
                    "..."
                } else {
                    content
                })
            })
        })
        .collect::<Vec<_>>();
    let response = json!({
        "provider": identifier,
        "services": services
    });
    serde_json::to_string(&response).ok()
}

fn separate_keyword_content(req: &str) -> (String, &str) {
    let mut splits = req.splitn(2, " ");
    let keyword = splits.next().unwrap_or_default();
    let content = splits.next().unwrap_or_default();
    (keyword.to_lowercase(), content)
}

fn find_matched_urls<'a>(config: &'a Config, keyword: &str) -> Vec<(&'a String, &'a String)> {
    config
        .values()
        .iter()
        .filter(|(key, _)| key.starts_with(keyword))
        .collect()
}

fn fill_matched_url(url: &String, mut content: Split<&str>) -> Option<String> {
    let mut placeholders: VecDeque<_> = url.match_indices("{}").collect();
    let content_len = content.clone().count();
    let count = placeholders.len();
    if count == 0 {
        Some(url.to_owned())
    } else if count > content_len {
        None
    } else {
        let mut buffer = Vec::new();
        let mut ending = String::new();
        while let Some((end_index, _)) = placeholders.pop_front() {
            buffer.push(&url[..end_index]);
            if placeholders.is_empty() {
                ending = content.collect::<Vec<_>>().join(" ");
                break;
            } else {
                buffer.push(content.next().unwrap())
            }
        }
        buffer.push(&ending);
        Some(buffer.join(""))
    }
}
