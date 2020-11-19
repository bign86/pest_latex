use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "latex-math-grammar.pest"]
pub struct LaTeXParser;


fn printout(tk: pest::iterators::Pair<Rule>, l: usize) {
    let spaces: String = " |  ".repeat(l);
    let rule_str = [spaces.clone(), "Rule: ".to_string()].join("");
    println!("{}{:?}  {:?}", rule_str, tk.as_rule(), tk.as_str());
    for inner_tk in tk.into_inner() {
        printout(inner_tk, l+1);
    }
}


fn main() {
    let input = "latex_test.tex";
    let latex = fs::read_to_string(input).expect("Cannot open file");

    let parse = LaTeXParser::parse(Rule::intro, &latex)
                            .expect("").next().unwrap();

    printout(parse, 0);
}



