mod commands;
pub use commands::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use scraper::Html;

    use crate::common::{generate_request_client, GQL_ENDPOINT};

    #[test]
    fn combined_queries() {
        let client = generate_request_client(
            &reqwest::Url::from_str("https://leetcode.com/problems/all/").unwrap(),
        )
        .unwrap();
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .unwrap();
        // below is an optimized version of the query I am already using
        let c = serde_json::json!({
            "query":"query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
                problemsetQuestionList: questionList(
                    categorySlug: $categorySlug
                    limit: $limit
                    skip: $skip
                    filters: $filters
                    ) {
                        total: totalNum
                        questions: data {
                            acRate
                            difficulty
                            frontendQuestionId: questionFrontendId
                            status
                            title
                            titleSlug
                            topicTags {
                                name
                                id
                                slug
                            }
                        }
                }
            }",
            "variables":{"categorySlug":"","skip":0,"limit":50,"filters":{}},
            "operationName":"problemsetQuestionList"
        });
        let query = serde_json::json!({
            "query":"query questionEditorData($titleSlug: String!) {
                question(titleSlug: \"two-sum\") {
                    codeSnippets {
                        langSlug
                        code
                    }
                }
            }",
            "variables":{"titleSlug":"two-sum"},
            // "variables":{},
            "operationName":"questionEditorData"
        });
        let handle = rt.spawn(super::common::query_endpoint(
            GQL_ENDPOINT.to_string(),
            c,
            // query,
            client,
        ));

        let data = rt.block_on(handle).unwrap().unwrap();
        // this is a monstrosity to unwrap the data from the response
        let s = data
            .get("data")
            .and_then(|val| val.get("question"))
            .and_then(|val| val.get("codeSnippets"))
            .and_then(|val| {
                val.as_array().and_then(|list| {
                    list.iter()
                        .find(|snip| {
                            snip.get("langSlug").and_then(|lang| lang.as_str()) == Some("rust")
                        })
                        .and_then(|snippet| snippet.get("code").and_then(|code| code.as_str()))
                })
            });
        // println!("{:?}", s);
        assert!(false);
    }
}
