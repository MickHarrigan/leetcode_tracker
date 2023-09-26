mod commands;
use commands::{common::*, finish::*, hide::*, info::*, new::*, search::*, submit::*, tag::*};

use anyhow::Result;
use clap::Parser;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    #[cfg(not(feature = "test"))]
    match &args.command {
        Commands::New { link } => {
            println!("New command with link: {}", link);
            // this should now parse the link such that it can read all the info that it needs from
            // it.

            // make a request here
            let session = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiMjcyOTYwMyIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImFsbGF1dGguYWNjb3VudC5hdXRoX2JhY2tlbmRzLkF1dGhlbnRpY2F0aW9uQmFja2VuZCIsIl9hdXRoX3VzZXJfaGFzaCI6IjlkMmI3NjMzMTJiMjAwNjAwNDE1NWM1ODI4NWUzM2M2MTQ2MDJmMzAiLCJpZCI6MjcyOTYwMywiZW1haWwiOiJtaGFycmlnYW4zMjhAZ21haWwuY29tIiwidXNlcm5hbWUiOiJtaGFycmlnMSIsInVzZXJfc2x1ZyI6Im1oYXJyaWcxIiwiYXZhdGFyIjoiaHR0cHM6Ly9zMy11cy13ZXN0LTEuYW1hem9uYXdzLmNvbS9zMy1sYy11cGxvYWQvYXNzZXRzL2RlZmF1bHRfYXZhdGFyLmpwZyIsInJlZnJlc2hlZF9hdCI6MTY5NTY4MTA2MywiaXAiOiI2Ny4xNjQuMTI1LjYwIiwiaWRlbnRpdHkiOiI2ZTUzMWYwNmJmM2ZjMjZmMzZiM2MyODg5NzhlOWZjNCIsInNlc3Npb25faWQiOjQ1OTg0Nzc1LCJfc2Vzc2lvbl9leHBpcnkiOjEyMDk2MDB9.hasC5lHN2_jFX3bUtjbgAjnI9UBGKhXhPxuFLS49fe4";
            let token = "aYcsgdAMmffTwhkAEICVGvuj4eR1sZgvrrtCcb2g5LQXPZrXFyXmY7TuDecxYetZ";
            let cookies = format!("LEETCODE_SESSION={};csrftoken={}", session, token);
            let jar = reqwest::cookie::Jar::default();
            let url = "https://leetcode.com/problems/two-sum/submit/"
                .parse::<reqwest::Url>()
                .unwrap();
            jar.add_cookie_str(cookies.as_str(), &url);
            let mut headers = header::HeaderMap::new();
            let head = header::HeaderValue::from_str(cookies.as_str())?;
            let referer = header::HeaderValue::from_str("https://leetcode.com/problems/two-sum/")?;
            let csrf = header::HeaderValue::from_str(token)?;
            let content = header::HeaderValue::from_str("application/json")?;
            let accept = header::HeaderValue::from_str("application/json")?;
            headers.insert(header::COOKIE, head);
            headers.insert(header::REFERER, referer);
            headers.insert(header::CONTENT_TYPE, content);
            headers.insert(header::ACCEPT, accept);
            headers.insert(header::HeaderName::from_static("x-csrftoken"), csrf);
            let client: serde_json::Value = reqwest::Client::builder().user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117").default_headers(headers).cookie_store(true).build()?
                // .post("https://leetcode.com/graphql/")
                .post("https://leetcode.com/problems/two-sum/submit/")
                .json(&serde_json::json!({
                    // replace two-sum with whatever question is in the link
                    // "query":"\n    query questionTitle($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    title\n    titleSlug\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionTitle"
                    // 
                    // below is how to get function signature and other code
                    // output["data"]["question"]["codeSnippets"][15]["code"] == code that is
                    // provided
                    // "query":"\n    query questionEditorData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    codeSnippets {\n      lang\n      langSlug\n      code\n    }\n    envInfo\n    enableRunCode\n    hasFrontendPreview\n    frontendPreviews\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionEditorData"
                    //
                    // submissions work
                    "lang":"rust","question_id":"1","typed_code":"impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        use std::collections::HashMap;\n        // hash each number with the index as their value\n        let mut hash: HashMap<i32, i32> = HashMap::new();\n        for (k, v) in nums.iter().zip(0..) {\n            match hash.get(&(target - k)) {\n                Some(i) => return vec![v, *i],\n                None => hash.insert(*k, v),\n            };\n        }\n        vec![]\n    }\n}"
                }))
                .send()
                .await?
                .json()
                .await?;
            println!("{:#?}", client);
        }

        Commands::Tag { cmd } => match cmd {
            TagCommand::Add => {
                // FLAGS
                // ************************************************************
                // Add should take a number and a tag/[tags]
                // to apply to a problem referenced by the number provided
                //
                // PROMPTS
                // ************************************************************
                // should prompt for a tag and a problem number
                let (_input_tag, tag) =
                    prompt_for_input::<TagType>("Enter Tag to add: ".to_string())?;

                let (_input_num, num) =
                    prompt_for_input::<usize>("Enter Problem Number to add Tag to: ".to_string())?;
                println!("Tag: {tag:?} was added to Problem: {num}");
            }
            TagCommand::Remove => {
                // should tame a number and a tag/[tags] to remove the tags from said problem
            }
            TagCommand::Edit => {
                // should just take a number and then give a list of all tags for that problem that
                // the user can adjust
            }
            TagCommand::Search => {
                // given a tag, finds all problems that have that tag
            }
        },
        Commands::Info { num } => {
            // takes a number and prints a bunch of info about the problem
        }
        Commands::Search { cmd } => {
            // given any of (name, number, tag(s)) will find what you are searching for
        }
        Commands::Hide { num } => {
            // given a number will tag this as a hidden problem that has been attempted but not
            // completed. This should maybe be pushed somewhere else or just not tracked.
        }
        Commands::Submit { num } => {
            // given a number should aim to send the code to LeetCode, but I have no idea on how
            // to actually send this to them and receive the response. HTTP? GraphQL? I have no
            // clue.
        }
        Commands::Finish { num } => {
            // Ceremoniously tags the problem as completed and with whichever solution was used.
            // Maybe this can track multiple solutions as well to be able to compare them.
            //
            // Hopefully this can also have a way to see the time/space complexities and
            // explanations of functions but we will see.
        }
        #[allow(unreachable_patterns)]
        a => println!("Input was: {a:?}"),
    }
    Ok(())
}
