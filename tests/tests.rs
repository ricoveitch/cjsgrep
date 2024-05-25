use std::process;

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
        let bytes = process::Command::new("target/debug/codegrep")
            .output()
            .unwrap()
            .stdout;

        let str_out = String::from_utf8_lossy(&bytes).to_string();
        let mut lines: Vec<&str> = str_out.split("\n").collect();
        lines.pop();

        assert_eq!(lines.len(), 3);
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
