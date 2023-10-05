use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::io::Write;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

use super::common::{get_lc_dir, prompt_for_input};

#[derive(Debug, Clone, Parser, EnumString, IntoStaticStr, EnumIter)]
pub enum TagType {
    // #[strum(serialize = "stack", serialize = "<other name>")]
    #[strum(serialize = "stack", ascii_case_insensitive)]
    Stack,

    #[strum(serialize = "dp", ascii_case_insensitive)]
    DynamicProgramming,

    #[strum(serialize = "btree", ascii_case_insensitive)]
    BinaryTree,

    #[strum(serialize = "graph", ascii_case_insensitive)]
    Graph,

    #[strum(serialize = "backtracking", ascii_case_insensitive)]
    BackTracking,

    #[strum(serialize = "hashmap", ascii_case_insensitive)]
    HashMap,

    #[strum(serialize = "string", ascii_case_insensitive)]
    String,

    #[strum(serialize = "array", ascii_case_insensitive)]
    Array,

    #[strum(serialize = "math", ascii_case_insensitive)]
    Math,

    #[strum(serialize = "sorting", ascii_case_insensitive)]
    Sorting,

    #[strum(serialize = "greedy", ascii_case_insensitive)]
    Greedy,

    #[strum(serialize = "dfs", ascii_case_insensitive)]
    DepthFirstSearch,

    #[strum(serialize = "binarysearch", ascii_case_insensitive)]
    BinarySearch,

    #[strum(serialize = "database", ascii_case_insensitive)]
    Database,

    #[strum(serialize = "bfs", ascii_case_insensitive)]
    BreadthFirstSearch,

    #[strum(serialize = "tree", ascii_case_insensitive)]
    Tree,

    #[strum(serialize = "matrix", ascii_case_insensitive)]
    Matrix,

    #[strum(serialize = "twopointers", ascii_case_insensitive)]
    TwoPointers,

    #[strum(serialize = "bitmanip", ascii_case_insensitive)]
    BitManipulation,

    #[strum(serialize = "heap", ascii_case_insensitive)]
    Heap,

    #[strum(serialize = "prefixsum", ascii_case_insensitive)]
    PrefixSum,

    #[strum(serialize = "sim", ascii_case_insensitive)]
    Simulation,

    #[strum(serialize = "design", ascii_case_insensitive)]
    Design,

    #[strum(serialize = "counting", ascii_case_insensitive)]
    Counting,

    #[strum(serialize = "slidingwindow", ascii_case_insensitive)]
    SlidingWindow,

    #[strum(serialize = "unionfind", ascii_case_insensitive)]
    UnionFind,

    #[strum(serialize = "ll", ascii_case_insensitive)]
    LinkedList,

    #[strum(serialize = "orderedset", ascii_case_insensitive)]
    OrderedSet,

    #[strum(serialize = "enum", ascii_case_insensitive)]
    Enumeration,

    #[strum(serialize = "monotonicstack", ascii_case_insensitive)]
    MonotonicStack,

    #[strum(serialize = "trie", ascii_case_insensitive)]
    Trie,

    #[strum(serialize = "recursion", ascii_case_insensitive)]
    Recursion,

    #[strum(serialize = "divideandconquer", ascii_case_insensitive)]
    DivideAndConquer,

    #[strum(serialize = "numbertheory", ascii_case_insensitive)]
    NumberTheory,

    #[strum(serialize = "bitmask", ascii_case_insensitive)]
    Bitmask,

    #[strum(serialize = "queue", ascii_case_insensitive)]
    Queue,

    #[strum(serialize = "bst", ascii_case_insensitive)]
    BinarySearchTree,

    #[strum(serialize = "memo", ascii_case_insensitive)]
    Memoization,

    #[strum(serialize = "segmenttree", ascii_case_insensitive)]
    SegmentTree,

    #[strum(serialize = "geometry", ascii_case_insensitive)]
    Geometry,

    #[strum(serialize = "topologicalsort", ascii_case_insensitive)]
    TopologicalSort,

    #[strum(serialize = "binaryindexedtree", ascii_case_insensitive)]
    BinaryIndexedTree,

    #[strum(serialize = "gametheory", ascii_case_insensitive)]
    GameTheory,

    #[strum(serialize = "hashfunction", ascii_case_insensitive)]
    HashFunction,

    #[strum(serialize = "shortestpath", ascii_case_insensitive)]
    ShortestPath,

    #[strum(serialize = "combinatorics", ascii_case_insensitive)]
    Combinatorics,

    #[strum(serialize = "interactive", ascii_case_insensitive)]
    Interactive,

    #[strum(serialize = "stringmatching", ascii_case_insensitive)]
    StringMatching,

    #[strum(serialize = "datastream", ascii_case_insensitive)]
    DataStream,

    #[strum(serialize = "rollinghash", ascii_case_insensitive)]
    RollingHash,

    #[strum(serialize = "brainteaser", ascii_case_insensitive)]
    Brainteaser,

    #[strum(serialize = "randomized", ascii_case_insensitive)]
    Randomized,

    #[strum(serialize = "monotonicqueue", ascii_case_insensitive)]
    MonotonicQueue,

    #[strum(serialize = "mergesort", ascii_case_insensitive)]
    MergeSort,

    #[strum(serialize = "iterator", ascii_case_insensitive)]
    Iterator,

    #[strum(serialize = "concurrency", ascii_case_insensitive)]
    Concurrency,

    #[strum(serialize = "2ll", ascii_case_insensitive)]
    DoublyLinkedList,

    #[strum(serialize = "prob", ascii_case_insensitive)]
    ProbabilityAndStatistics,

    #[strum(serialize = "quickselect", ascii_case_insensitive)]
    QuickSelect,

    #[strum(serialize = "bucketsort", ascii_case_insensitive)]
    BucketSort,

    #[strum(serialize = "suffixarray", ascii_case_insensitive)]
    SuffixArray,

    #[strum(serialize = "minimumspanningtree", ascii_case_insensitive)]
    MinimumSpanningTree,

    #[strum(serialize = "countingsort", ascii_case_insensitive)]
    CountingSort,

    #[strum(serialize = "shell", ascii_case_insensitive)]
    Shell,

    #[strum(serialize = "linesweep", ascii_case_insensitive)]
    LineSweep,

    #[strum(serialize = "reservoirsampling", ascii_case_insensitive)]
    ReservoirSampling,

    #[strum(serialize = "stronglyconnectedcomponent", ascii_case_insensitive)]
    StronglyConnectedComponent,

    #[strum(serialize = "euleriancircuit", ascii_case_insensitive)]
    EulerianCircuit,

    #[strum(serialize = "radixsort", ascii_case_insensitive)]
    RadixSort,

    #[strum(serialize = "rejectionsampling", ascii_case_insensitive)]
    RejectionSampling,

    #[strum(serialize = "biconnectedcomponent", ascii_case_insensitive)]
    BiconnectedComponent,
}

#[derive(Debug, Clone, Parser)]
pub enum TagCommand {
    /// Add a tag to an existing problem
    Add,
    /// Remove a tag from an existing problem
    Remove,
    /// Edit the tags of a specified problem
    Edit,
    /// Search for all problems with a specific tag
    Search,
    /// List all the available tags for usage
    List,
}

impl ValueEnum for TagCommand {
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Self::Add => clap::builder::PossibleValue::new("add"),
            Self::Remove => clap::builder::PossibleValue::new("remove"),
            Self::Edit => clap::builder::PossibleValue::new("edit"),
            Self::Search => clap::builder::PossibleValue::new("search"),
            Self::List => clap::builder::PossibleValue::new("list"),
        })
    }
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Add,
            Self::Remove,
            Self::Edit,
            Self::Search,
            Self::List,
        ]
    }
}

impl std::fmt::Display for TagType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn tag_subcommands(cmd: &TagCommand) -> Result<()> {
    match cmd {
        TagCommand::Add => {
            let lc_dir = get_lc_dir()?;
            let (_input_tag, tag) = prompt_for_input::<TagType>("Enter Tag to add: ")?;

            let (_input_num, num) =
                prompt_for_input::<usize>("Enter Problem Number to add Tag to: ")?;

            let problem_dir = format!("{}{}{}", lc_dir, "src/", num);
            let tag_path = format!("{}{}", problem_dir, "/TAGS");
            let tag_file = std::fs::read_to_string(tag_path.clone())?;

            // search for that tag in the problem, if it already exists, return an error of that
            let re = Regex::new(format!(r"{}", tag).as_str()).unwrap();
            if let Some(_a) = re.captures(tag_file.as_str()) {
                return Err(anyhow::Error::msg(format!(
                    "Tag `{tag}` already exists for this problem!"
                )));
            } else {
                let mut file = std::fs::OpenOptions::new().append(true).open(tag_path)?;
                writeln!(file, "{}", tag.to_string())?;
            }

            println!("Tag: {tag:?} was added to Problem: {num}");
            Ok(())
        }
        TagCommand::Remove => {
            let lc_dir = get_lc_dir()?;

            let (_input_num, num) =
                prompt_for_input::<usize>("Enter Problem Number to remove a Tag from: ")?;

            let prompt = format!("Enter Tag to remove from Problem {}: ", num);
            let (_input_tag, tag) = prompt_for_input::<TagType>(prompt.as_str())?;

            let problem_dir = format!("{}{}{}", lc_dir, "src/", num);
            let tag_path = format!("{}{}", problem_dir, "/TAGS");
            let tag_file = std::fs::read_to_string(tag_path.clone())?;

            // file contents without the tag that is to be removed
            let mut buf = Vec::new();
            for line in tag_file.lines() {
                if line.to_string() != tag.to_string() {
                    buf.push(line);
                }
            }
            let buf = buf.join("\n");

            let re = Regex::new(format!(r"{}", tag).as_str()).unwrap();

            if let Some(_a) = re.captures(tag_file.as_str()) {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(tag_path)?;
                writeln!(file, "{}", buf)?;
            } else {
                return Err(anyhow::Error::msg(format!(
                    "Tag `{tag}` doesn't exist for this problem!"
                )));
            }
            println!("Tag: {tag:?} was removed from Problem: {num}");
            Ok(())
        }
        TagCommand::Edit => {
            // should just take a number and then give a list of all tags for that problem that
            // the user can adjust
            println!("Under Construction!");
            Ok(())
        }
        TagCommand::Search => {
            // given a tag, finds all problems that have that tag
            let (_input_tag, tag) = prompt_for_input::<TagType>("Enter the tag to search for: ")?;

            // read all TAGS files in the system to find those with said tag
            let mut out = Vec::new();
            let lc_dir = get_lc_dir()?;
            let paths = std::fs::read_dir(format!("{}{}", lc_dir, "src/"))?;
            paths.into_iter().for_each(|path| {
                if let Ok(path) = path {
                    let tag_file = format!("{}{}", path.path().display(), "/TAGS");
                    if let Ok(file) = std::fs::read_to_string(tag_file.clone()) {
                        // find the tag within the file
                        let re = Regex::new(format!(r"{}", tag).as_str()).unwrap();
                        match re.captures(file.as_str()) {
                            Some(_a) => out.push(path.file_name()),
                            None => {}
                        }
                    } else {
                        // this should be put into an error log
                        // println!("Couldn't find the TAGS file in {:?}", path.file_name());
                    }
                }
            });
            out.iter().for_each(|path| {
                println!("Problems with {}: {}", tag, path.to_str().unwrap_or(""))
            });
            Ok(())
        }
        TagCommand::List => {
            list_tags();
            Ok(())
        }
    }
}

pub fn list_tags() {
    for tag in TagType::iter() {
        println!("Tag: {tag}");
    }
}
