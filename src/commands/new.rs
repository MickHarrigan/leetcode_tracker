use std::{io::Write, mem::forget, str::FromStr};

use super::common::{GQL_ENDPOINT, SESSION, TOKEN};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::Url;
use serde::{Deserialize, Serialize};

const LEETCODE_HOST: &str = "leetcode.com";

pub async fn run(link: &String) -> Result<()> {
    let link = sanitize_lc_link(link)?;
    println!("New command with link: {}", link);
    let client = generate_request_client(&link)?;

    // this goes inside of the query that is sent
    let title_slug = get_title_slug(&link)?;

    // with the title-slug I can now query the GQL endpoint for information like the
    // question title, question number, code snippets, etc.
    // put title-slug inside of the query

    // data should contain specifically
    // question number
    // question title
    // code snippets
    // and a capability to add more

    let query = serde_json::json!({"query":"\n    query questionEditorData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    title\n    codeSnippets {\n      lang\n      langSlug\n      code\n    }\n    envInfo\n    enableRunCode\n    hasFrontendPreview\n    frontendPreviews\n  }\n}\n    ","variables":{"titleSlug":title_slug},"operationName":"questionEditorData"});
    let data = query_endpoint(&GQL_ENDPOINT.to_string(), &query, &client).await?;

    // parse the data into a single struct that can be converted to json and stored in the
    // repo itself
    let mut problem_data = parse_from_json_to_problem(data)?;
    // link is not set yet
    problem_data.link = link.to_string();

    // create the directory things inside the repo from problem data
    match create_entry(problem_data) {
        Ok(()) => {
            // return a good note
            println!("Successfully created the problem!");
            Ok(())
        }
        Err(e) => {
            // return an error of what the issue was
            println!("Failed to create the directory: {e}");
            Err(e)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    number: usize,
    // is Some(a) when it DOES NOT match number
    number_backend: Option<usize>,
    snippet: String,
    title: String,
    link: String,
}

pub fn sanitize_lc_link(link: &String) -> Result<Url> {
    let link = link.parse::<Url>()?;
    match link.host_str() {
        Some(a) if !a.eq_ignore_ascii_case(LEETCODE_HOST) => {
            return Err(anyhow::Error::msg(format!(
                "Host in link is incorrect: {a}"
            )))
        }
        None => {
            return Err(anyhow::Error::msg(format!(
                "Incorrect or unexpected link: {link}"
            )))
        }
        // nop if the host is good
        _ => {}
    };

    // check path next
    let re = Regex::new(r"^(/[^/]+/[^/]+/)").unwrap();
    let path = match re.captures(link.path()) {
        Some(caps) => caps[0].to_owned(),
        None => {
            return Err(anyhow::Error::msg(format!(
                "Path to problem was incorrect: {}",
                link.path()
            )))
        }
    };

    // return a sanitized version of the link that has the title-slug at the end
    let url = Url::from_str(format!("https://{}{}", LEETCODE_HOST, path).as_str()).unwrap();
    Ok(url)
}

pub fn get_title_slug(link: &Url) -> Result<String> {
    // retrieves the title-slug from the URL
    let re = Regex::new(r"/([^/]+)/$").unwrap();
    match re.captures(link.as_str()) {
        Some(caps) => Ok(caps[0].trim_matches('/').to_owned()),
        None => Err(anyhow::Error::msg(format!(
            "Failed to retrieve the title-slug from the link: {link}"
        ))),
    }
}

pub async fn query_endpoint(
    endpoint: &String,
    query: &serde_json::Value,
    client: &reqwest::Client,
) -> Result<serde_json::Value> {
    //     .json(&serde_json::json!({
    //         // replace two-sum with whatever question is in the link
    //         // "query":"\n    query questionTitle($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    title\n    titleSlug\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionTitle"
    //         //
    //         // below is how to get function signature and other code
    //         // output["data"]["question"]["codeSnippets"][15]["code"] == code that is
    //         // provided
    //         // "query":"\n    query questionEditorData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    codeSnippets {\n      lang\n      langSlug\n      code\n    }\n    envInfo\n    enableRunCode\n    hasFrontendPreview\n    frontendPreviews\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionEditorData"
    //         //
    //         // submissions work
    //         "lang":"rust","question_id":"1","typed_code":"impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        use std::collections::HashMap;\n        // hash each number with the index as their value\n        let mut hash: HashMap<i32, i32> = HashMap::new();\n        for (k, v) in nums.iter().zip(0..) {\n            match hash.get(&(target - k)) {\n                Some(i) => return vec![v, *i],\n                None => hash.insert(*k, v),\n            };\n        }\n        vec![]\n    }\n}"
    //     }))
    let resp: serde_json::Value = client
        .post(endpoint)
        .json(query)
        .send()
        .await?
        .json()
        .await?;
    Ok(resp)
}

pub fn generate_request_client(sanitized_link: &Url) -> Result<reqwest::Client> {
    use reqwest::header;
    let cookies = format!("LEETCODE_SESSION={};csrftoken={}", SESSION, TOKEN);

    let mut headers = header::HeaderMap::new();

    let cookie = header::HeaderValue::from_str(cookies.as_str())?;
    let referer = header::HeaderValue::from_str(sanitized_link.as_str())?;
    let csrf = header::HeaderValue::from_str(TOKEN)?;
    let content = header::HeaderValue::from_str("application/json")?;
    let accept = header::HeaderValue::from_str("application/json")?;

    headers.insert(header::COOKIE, cookie);
    headers.insert(header::REFERER, referer);
    headers.insert(header::CONTENT_TYPE, content);
    headers.insert(header::ACCEPT, accept);
    headers.insert(header::HeaderName::from_static("x-csrftoken"), csrf);

    reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117")
        .default_headers(headers)
        .cookie_store(true)
        .build()
        .map_err(|e| anyhow::Error::msg(e.to_string()))
}

pub fn parse_from_json_to_problem(json: serde_json::Value) -> Result<Problem> {
    // data should contain specifically
    // question number
    // question title
    // code snippets
    // and a capability to add more
    let question = &json["data"]["question"];
    let number = match question["questionFrontendId"].as_str() {
        Some(a) => match a.parse::<usize>() {
            Ok(a) => a,
            Err(e) => return Err(e.into()),
        },
        None => {
            return Err(anyhow::Error::msg(
                "Could not get the questionFrontendId from JSON",
            ))
        }
    };
    // if backend number != frontend number then backend number = Some(backend)
    // otherwise its None
    let number_backend = match question["questionId"].as_str() {
        Some(a) => match a.parse::<usize>() {
            Ok(a) if a == number => None,
            Ok(a) => Some(a),
            Err(e) => return Err(e.into()),
        },
        None => return Err(anyhow::Error::msg("Could not get the questionId from JSON")),
    };
    let len = match question["codeSnippets"].as_array() {
        Some(a) => a.len(),
        None => {
            return Err(anyhow::Error::msg(
                "Could not get the codeSnippets array from JSON",
            ))
        }
    };
    let mut snippet: String = "".to_string();
    for i in 0..len {
        match question["codeSnippets"][i]["langSlug"].as_str() {
            Some(a) => {
                if a == "rust" {
                    match question["codeSnippets"][i]["code"].as_str() {
                        Some(a) => snippet = a.to_string(),
                        None => {
                            return Err(anyhow::Error::msg(
                                "Could not get the codeSnippet from JSON",
                            ))
                        }
                    }
                }
            }
            None => {
                return Err(anyhow::Error::msg(
                    "Could not get the codeSnippet from JSON",
                ))
            }
        }
    }
    if snippet == "".to_string() {
        return Err(anyhow::Error::msg(
            "Could not get the codeSnippet from JSON",
        ));
    }
    let title = match question["title"].as_str() {
        Some(a) => a.to_string(),
        None => return Err(anyhow::Error::msg("Could not get the title from JSON")),
    };
    Ok(Problem {
        number,
        number_backend,
        snippet,
        title,
        link: "".to_owned(),
    })
}

pub fn create_entry(prob: Problem) -> Result<()> {
    // this should do all of the OS things like making a directory and editing files
    // reference the old bash script for this
    //
    use std::env;
    let key = "LEETCODE_DIR";
    // val is the top level directory for the leetcode directory
    let lc_dir = env::var(key)?;

    // first check if the problem exists already in the Cargo.toml
    let cargo = std::fs::read_to_string(format!("{}{}", lc_dir, "/Cargo.toml"))?;
    let re = Regex::new(format!(r#"\[\[bin\]\]\nname = \"{}\""#, prob.number).as_str()).unwrap();
    if let Some(_a) = re.captures(cargo.as_str()) {
        return Err(anyhow::Error::msg(format!(
            "Problem already exists in repo! e: {}",
            prob.number
        )));
    }

    // then get the function name

    let re = Regex::new(r"impl Solution.*\n\s+pub\sfn\s(?<func>\w+)\s*\(").unwrap();
    let func = match re.captures(prob.snippet.as_str()) {
        Some(caps) => caps["func"].to_owned(),
        None => {
            return Err(anyhow::Error::msg(format!(
                "Could not find function name: {}",
                prob.snippet
            )))
        }
    };

    // then set up the main function to be added in
    let main = format!(
        "\n\nfn main() {{
    let sol = Solution::{}();
    println!(\"{{:?}}\", sol);\n}}",
        func
    );

    let code = format!("{}{}", prob.snippet, main);
    let readme = format!(
        "# {}. {}\n\n[Here]({}) is the link to the problem.",
        prob.number, prob.title, prob.link
    );

    // then make the prob.number directory in src
    std::fs::create_dir_all(format!("{}{}{}{}", lc_dir, "src/", prob.number, "/src/"))?;
    // then make its write the code in main.rs
    std::fs::write(
        format!("{}{}{}{}", lc_dir, "src/", prob.number, "/src/main.rs"),
        code,
    )?;
    // README.md
    std::fs::write(
        format!("{}{}{}{}", lc_dir, "src/", prob.number, "/README.md"),
        readme,
    )?;
    // TAGS
    std::fs::write(
        format!("{}{}{}{}", lc_dir, "src/", prob.number, "/TAGS"),
        "",
    )?;
    Ok(())
}
