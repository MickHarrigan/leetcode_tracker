# LeetCode TUI Tool

This is the crate with the front end contents for the TUI tool that I am developing.


# Notes

## Get Question List from LC
```json
{"query":"\n    query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {\n  problemsetQuestionList: questionList(\n    categorySlug: $categorySlug\n    limit: $limit\n    skip: $skip\n    filters: $filters\n  ) {\n    total: totalNum\n    questions: data {\n      acRate\n      difficulty\n      freqBar\n      frontendQuestionId: questionFrontendId\n      isFavor\n      paidOnly: isPaidOnly\n      status\n      title\n      titleSlug\n      topicTags {\n        name\n        id\n        slug\n      }\n      hasSolution\n      hasVideoSolution\n    }\n  }\n}\n    ","variables":{"categorySlug":"","skip":0,"limit":50,"filters":{}},"operationName":"problemsetQuestionList"}
```
The above query gets the first 50 problems and their information.
Within each of the questions is a field `status` that is one of 3 values: `"ac", "notac", null`.
These correspond to completed, attempted, and not attempted, respectively.

The next query gets the question of the day
```json
{"query":"\n    query questionOfToday {\n  activeDailyCodingChallengeQuestion {\n    date\n    userStatus\n    link\n    question {\n      acRate\n      difficulty\n      freqBar\n      frontendQuestionId: questionFrontendId\n      isFavor\n      paidOnly: isPaidOnly\n      status\n      title\n      titleSlug\n      hasVideoSolution\n      hasSolution\n      topicTags {\n        name\n        id\n        slug\n      }\n    }\n  }\n}\n    ","variables":{},"operationName":"questionOfToday"}
```

Next is the query for getting the description of the question.
```json
{"query":"\n    query questionContent($titleSlug: String!) {\n  question(titleSlug: $titleSlug) {\n    content\n    mysqlSchemas\n    dataSchemas\n  }\n}\n    ","variables":{"titleSlug":"jump-game-ii"},"operationName":"questionContent"}
```
This is in HTML and needs to be parsed or cleaned or something to make it more readable.

### HTML Tags and their replacements with ratatui
`<code>` should use a slightly lighter background font and the same text color.

`<em>` should just be italics.

`<li>` should be a bulleted list item or a numbered list item.

`<p>` is a paragragh in ratatui(?).

`<pre>` may have to be indented and highlighted somehow.

`<strong>` is just bold.

`<strong class=example>` is a strong with a slightly larger font maybe?

`<sup>` must be converted to just using the '^' character.

`<ul>` must be converted to correct formatting of lines.

### Images
Using the amazing crate [ratatui-image](https://github.com/benjajaja/ratatui-image) these can just be used inside the description window!

### UTF-8
The icons that I am planning on using right now for the Accepted, Attempted (failed), Attempted (no submission), Nothing will be:
```
"\u{1FBB1}"
"\u{1FBC0}"
"\u{1FBC4}"
```
where accepted is the check, Attempted (failed) is the cross, and unattempted is the question mark.
Note that I may make unattempted nothing and make non-submitted as question mark.
