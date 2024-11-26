#[derive(Debug)]
struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delim: &'a str,
}

impl<'a> StrSplit<'a> {
    fn new(input_str: &'a str, delim: &'a str) -> Self {
        Self {
            remainder: Some(input_str),
            delim,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delim) {
                let until_delim = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delim.len())..];
                Some(until_delim)
            } else {
                self.remainder.take()
            }
        } else {
             None
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
