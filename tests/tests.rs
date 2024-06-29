use std::process;

fn test_search(filename: &str, pattern: &str, func_start: Option<&str>, expected_out: Vec<&str>) {
    let mut cmd = process::Command::new("target/debug/codegrep");
    cmd.arg(pattern).arg(filename);

    if let Some(func_start) = func_start {
        cmd.arg(format!("-n={}", func_start));
    }

    let bytes = cmd.output().unwrap().stdout;

    let str_out = String::from_utf8_lossy(&bytes).to_string();

    let mut lines: Vec<&str> = str_out.split("\n").collect();
    lines.pop();

    assert_eq!(lines.len(), expected_out.len());
    for (line, expected) in lines.iter().zip(expected_out.iter()) {
        assert_eq!(line.contains(expected), true);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn empty() {
    //     let results = search("nonexistent");
    //     assert_eq!(results.len(), 0);
    // }

    #[test]
    fn same_file() {
        test_search(
            "data/single-file.js",
            "pin",
            Some("foo"),
            vec![
                "let pin = a;",
                "let pin = b;",
                "abc(pin);",
                "const abc = (pin) => {",
                "let pin = c;",
                "//pin",
                "moo(pin);",
            ],
        );
    }

    #[test]
    fn comments() {
        test_search("data/comments.js", "pin", Some("foo"), vec!["pin = bar;"]);
    }

    #[test]
    fn multi_file() {
        test_search(
            "data/import-test.js",
            "baz",
            Some("foo"),
            vec![
                "bazz();",
                "function baz(obj) {",
                "obj.baz = 1;",
                "function baz2(obj) {",
                "obj.baz = 2;",
            ],
        );
    }

    // #[test]
    // fn single_destructure_import() {
    //     let results = search("obj.fixed");
    //     assert_eq!(results.len(), 1);
    // }

    // #[test]
    // fn multi_destructure_import() {
    //     let results = search("obj.qux");
    //     assert_eq!(results.len(), 1);
    // }

    // #[test]
    // fn multi_nl_destructure_import() {
    //     let results = search("obj.double");
    //     assert_eq!(results.len(), 1);
    // }

    // #[test]
    // fn default_import() {
    //     let results = search("obj.lar");
    //     assert_eq!(results.len(), 1);
    // }

    // #[test]
    // fn single_nest() {
    //     let results = search("obj.baz");
    //     assert_eq!(results.len(), 1);
    // }
}
