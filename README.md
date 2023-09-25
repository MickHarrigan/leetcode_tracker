# LeetCode CLI Tool

## Usage Examples

### New Problem
Create a new problem with the interactive startup
```bash
$ lc new
```
or use flags to specify information right to the exe
```bash
$ lc new -l <link> -p [problem number] -n [problem name] -f [function name] -a [function arguments] -r [function return] -e [extra code needed]
```

Overall the preferred way to do this is with just `lc new` and inputting either the information that way, or just using a link that the information can be obtained from
in the first place.

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

### Submitting a Problem
This utility allows the user to submit the problem as a response and see output.

### Finishing a Problem
This will tag a problem internally as completed and as such will have its `main.rs` used as the solution.

### Removing a Problem
This will remove the problem from the listings and remove its finish status if it has it.

### Failing (Hiding) a Problem
This is for the case that a problem has been attempted but given up on temporarily.
Tagged with failed or deferred.
