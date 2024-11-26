#[derive(Debug)]
struct StrSplit<'a> {
    remainder: &'a str,
    delim: &'a str,
}

impl<'a> StrSplit<'a> {
    fn new(input_str: &'a str, delim: &'a str) -> Self {
        Self {
            remainder: input_str,
            delim,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delim) {
            let until_delim = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delim.len())..];
            Some(until_delim)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = "";
            Some(rest)
        }
    }
}

#[test]
fn it_works() {
    let input_str = "a b c d e";
    let letters: Vec<_> = StrSplit::new(input_str, " ").collect();
    // let expected: Vec<&str> = vec!["a", "b", "c", "d", "e"];
    // assert_eq!(letters.collect::<Vec<_>>(), expected);
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"])
}

#[test]
fn tail() {
    let input_str = "a b c d ";
    let letters: Vec<_> = StrSplit::new(input_str, " ").collect();
    // let expected: Vec<&str> = vec!["a", "b", "c", "d", "e"];
    // assert_eq!(letters.collect::<Vec<_>>(), expected);
    assert_eq!(letters, vec!["a", "b", "c", "d", " "])
}
