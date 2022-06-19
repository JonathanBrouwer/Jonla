use std::collections::HashMap;
use jonla_macros::grammar;
use jonla_macros::grammar::{GrammarFile, RuleBody};
use jonla_macros::parser::parser_core::ParserState;
use jonla_macros::parser::parser_result::ParseResult;
use jonla_macros::parser::parser_rule::PR;

macro_rules! parse_test {
    (name: $name:ident syntax: $syntax:literal passing tests: $($input_pass:literal)* failing tests: $($input_fail:literal)*) => {
        #[test]
        fn $name() {
            let syntax: &'static str = $syntax;
            let grammar: GrammarFile = match grammar::grammar_def::toplevel(&syntax) {
                Ok(ok) => ok,
                Err(err) => {
                    panic!("{}", err);
                }
            };
            let rules: HashMap<&'static str, RuleBody<'static>> =
                grammar.rules.iter().map(|r| (r.name, r.body.clone())).collect();

            $(
            let input: &'static str = $input_pass;
            println!("== Parsing (should be ok): {}", input);
            let mut state: ParserState<'static, 'static, PR<'static>> = ParserState::new(input);
            let result: ParseResult<'static, PR<'static>> =
                state.parse_full_input(|s, p| s.parse_rule(p, &rules, "start"));
            assert!(result.is_ok());
            )*

            $(
            let input: &'static str = $input_fail;
            println!("== Parsing (should be fail): {}", input);
            let mut state: ParserState<'static, 'static, PR<'static>> = ParserState::new(input);
            let result: ParseResult<'static, PR<'static>> =
                state.parse_full_input(|s, p| s.parse_rule(p, &rules, "start"));
            assert!(!result.is_ok());
            )*
        }
    }
}

parse_test! {
name: literal
syntax: r#"
    rule start -> Input {
        "lol"
    }
    "#
passing tests:
    "lol"
failing tests:
    "lolz"
    "loll"
    "lol "
    ""
    "l"
    "lo"
    " lol"
    "lo\nn"
}

parse_test! {
name: literal_indirect
syntax: r#"
    rule start -> Input {
        lol
    }
    rule lol -> Input {
        "lol"
    }
    "#
passing tests:
    "lol"
failing tests:
    "lolz"
    "loll"
    "lol "
    ""
    "l"
    "lo"
    " lol"
    "lo\nn"
}

parse_test! {
name: charclass
syntax: r#"
    rule start -> Input {
        $([ 'w'-'z' | '8' | 'p'-'q' ])
    }
    "#
passing tests:
    "8"
    "w"
    "x"
    "y"
    "z"
    "p"
    "q"

failing tests:
    "a"
    "b"
    "v"
    "7"
    "9"
    "o"
    "r"
    " "
    "w8"
    "8w"
}

parse_test! {
name: repeat_star
syntax: r#"
    rule start -> Input {
        $([ 'w'-'z' | '8' | 'p'-'q' ]*)
    }
    "#
passing tests:
    "8"
    "w"
    "x"
    "y"
    "z"
    "p"
    "q"
    ""
    "8w"
    "w8"
    "wxyz8pqpq8wz"

failing tests:
    "a"
    "b"
    "v"
    "7"
    "9"
    "o"
    "r"
    " "
    "wxya"
    "w8 "
}

parse_test! {
name: sequence
syntax: r#"
    rule start -> Input {
        "a" ['w'-'y'] "q"
    }
    "#
passing tests:
    "awq"
    "axq"
    "ayq"

failing tests:
    "a"
    "aw"
    "ax"
    "ay"
    "aqq"
    "aaq"
    "bwq"
    ""
    "awqq"
}

parse_test! {
name: choice
syntax: r#"
    rule start -> Input {
        "a" / ['w'-'y'] / "q"
    }
    "#
passing tests:
    "a"
    "w"
    "y"
    "q"

failing tests:
    "aw"
    ""
    "b"
    "z"
    "wy"
    "wq"
}

parse_test! {
name: action
syntax: r#"
    rule start -> Input {
        "a" c:['w'-'y'] d:"q" { c }
    }
    "#
passing tests:
    "awq"
    "axq"
    "ayq"

failing tests:
    "a"
    "aw"
    "ax"
    "ay"
    "aqq"
    "aaq"
    "bwq"
    ""
    "awqq"
}