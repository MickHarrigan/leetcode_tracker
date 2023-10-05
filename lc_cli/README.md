# LeetCode CLI Tool

## Repo model
The model that the problems should have is 
```
 src
├──  <problem1>
│  ├──  README.md
│  ├──  src
│  │  ├──  main.rs
│  │  └──  [test.rs]
│  └──  TAGS
├──  <problem2>
│  ├──  README.md
│  ├──  src
│  │  ├──  main.rs
│  │  └──  [test.rs]
│  └──  TAGS
├──  <problem3>
│  ├──  README.md
│  ├──  src
│  │  ├──  main.rs
│  │  └──  [test.rs]
│  └──  TAGS
│  ...
└──  <problemN>
   ├──  README.md
   ├──  src
   │  ├──  main.rs
   │  └──  [test.rs]
   └──  TAGS
```


## Usage Examples

### New Problem
Create a new problem with a provided link.
```bash
$ lc new <link>
```

Read the title-slug and then request data from the server to then build the directory structure

### Inspecting a Problem
This allows the user to look at the problem and see information about it.
The problem must be one that is already attempted (for now) so that it can show the tags and relevant
user added data.

In the future this may be something that allows the user to look for and read about a problem in the first place.

### Tagging a Problem
This allows the users to attribute tags to a question for easier lookup and distinction of what each problem teaches.

```bash
$ lc tag
```
Using `tag` has a question pop up about what to do with tags either add, remove, edit, or search for tags.

#### Usage
The goal is to make no subcommands deeper than `tag add` or similar. This should be handled by
either the prompts after running this command, or a set of flags for more depth.

### Submitting a Problem
This utility allows the user to submit the problem as a response and see output.

### Finishing a Problem
This will tag a problem internally as completed and as such will have its `main.rs` used as the solution.

### Removing a Problem
This will remove the problem from the listings and remove its finish status if it has it.

### Failing (Hiding) a Problem
This is for the case that a problem has been attempted but given up on temporarily.
Tagged with failed or deferred.

## Problems
There are some problems that are both seen and unseen.

### Seen
- `lc tag search` and `lc search tag` These need to be reconciled such that they either overlap completely or one is removed and has its functionality taken into the other.


## Notes for API interaction

All use /graphql/ unless otherwise specified.

### Question number
The number on the page shown to users is not the internal number all the time.
To get the internal number there is a GraphQL request and response that works.

Request:
```json
{"query":"\n    query consolePanelConfig($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    questionTitle\n    enableDebugger\n    enableRunCode\n    enableSubmit\n    enableTestMode\n    exampleTestcaseList\n    metaData\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"consolePanelConfig"}
```

Response:
```json
{"data":{"question":{"questionId":"1","questionFrontendId":"1","questionTitle":"Two Sum","enableDebugger":true,"enableRunCode":true,"enableSubmit":true,"enableTestMode":false,"exampleTestcaseList":["[2,7,11,15]\n9","[3,2,4]\n6","[3,3]\n6"],"metaData":"{\n  \"name\": \"twoSum\",\n  \"params\": [\n    {\n      \"name\": \"nums\",\n      \"type\": \"integer[]\"\n    },\n    {\n      \"name\": \"target\",\n      \"type\": \"integer\"\n    }\n  ],\n  \"return\": {\n    \"type\": \"integer[]\",\n    \"size\": 2\n  },\n  \"manual\": false\n}"}}}
```

#### Alternative

```json
{"query":"\n    query questionTitle($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    title\n    titleSlug\n    isPaidOnly\n    difficulty\n    likes\n    dislikes\n    categoryTitle\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionTitle"}
```

```json
{"data":{"question":{"questionId":"1","questionFrontendId":"1","title":"Two Sum","titleSlug":"two-sum","isPaidOnly":false,"difficulty":"Easy","likes":51833,"dislikes":1696,"categoryTitle":"Algorithms"}}}
```

or 

Below has been tested and working.

```json
{"query":"\n    query questionNote($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    note\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionNote"}
```

```json
{"data":{"question":{"questionId":"1","note":""}}}
```

where `questionId` is the value from what I can deduce.
This also can be used when trying to get the number and name when given the link as `title-slug` is in the link itself.

### Provided Code
The code that is provided to the user is at this endpoint
```json
{"query":"\n    query questionEditorData($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    questionId\n    questionFrontendId\n    codeSnippets {\n      lang\n      langSlug\n      code\n    }\n    envInfo\n    enableRunCode\n    hasFrontendPreview\n    frontendPreviews\n  }\n}\n    ","variables":{"titleSlug":"two-sum"},"operationName":"questionEditorData"}
```

and what I want is in `codeSnippets` where `lang` is Rust.

### Title vs Title-Slug
Title is the well formed name of the problem whereas Title-slug is that which is used in the endpoints.

### Submission
This is how the system has submitted
@ /problems/$title-slug/submit/ send the query as stated below.

```json
{"lang":"rust","question_id":"1","typed_code":"impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        use std::collections::HashMap;\n        // hash each number with the index as their value\n        let mut hash: HashMap<i32, i32> = HashMap::new();\n        for (k, v) in nums.iter().zip(0..) {\n            match hash.get(&(target - k)) {\n                Some(i) => return vec![v, *i],\n                None => hash.insert(*k, v),\n            };\n        }\n        vec![]\n    }\n}"}
```
if there is an error I will find out about it later.
