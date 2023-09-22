use clap::{arg, Parser};

/// CLI tool to create and manage LeetCode problems
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Link to the LeetCode page that has the problem
    #[arg(short, long, default_value = None)]
    link: Option<String>,

    /// Problem number on the LeetCode page
    #[arg(short, long, default_value = None)]
    number: Option<String>,

    /// Name of function in the problem given
    #[arg(short, long, default_value = None)]
    func_name: Option<String>,

    /// Name of the problem itself, next to the number
    #[arg(short, long, default_value = None)]
    prob_name: Option<String>,

    /// List of function arguments in the provided signature
    #[arg(short, long, default_value = None)]
    args_func: Option<String>,

    /// Function return type as provided
    #[arg(short, long, default_value = None)]
    ret_func: Option<String>,

    /// Extra code that is provided with the problem
    /// Examples being ListNode/TreeNode definitions
    #[arg(short, long, default_value = None)]
    extra: Option<String>,
}

impl Args {
    pub fn args(&self) -> impl Iterator<Item = Option<String>> {
        let fields = vec![
            self.link.clone(),
            self.number.clone(),
            self.func_name.clone(),
            self.prob_name.clone(),
            self.args_func.clone(),
            self.ret_func.clone(),
            self.extra.clone(),
        ];
        fields.into_iter()
    }
}
