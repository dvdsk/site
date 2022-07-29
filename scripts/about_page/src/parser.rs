#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Latex {
    Text(String),
    Italic(Vec<Latex>),
}

peg::parser!(
    grammar basic_tex() for str {
        rule eof()
            = ![_] {}

        rule syntax()
            = "}" / "/textit" {}

        rule char() -> char
            = !(syntax() / eof()) c:[_] {
                c
        }

        // lazily collect anything that is not syntax + one more
        rule text() -> Latex
            = s:char()+ {
            let chars = s.into_iter();
            Latex::Text(String::from_iter(chars))
        }

        pub rule italic() -> Latex
            = r"/textit{" l:item()* "}" {
            Latex::Italic(l)
        }

        rule item() -> Latex
            = i:(italic() / text()) {
            i
        }

        pub rule line() -> Vec<Latex>
            = l:item()* ![_] {
            l
        }
    }
);

#[cfg(test)]
mod tests {
    use super::*;
    use Latex::*;

    #[test]
    fn text() {
        let input = "hello";
        let tree = basic_tex::line(input).unwrap();
        assert_eq!(tree, vec![Text("hello".to_owned())])
    }

    #[test]
    fn italic() {
        let input = r"/textit{italic text works}";
        let tree = basic_tex::italic(input).unwrap();
        assert_eq!(tree, Italic(vec![Text("italic text works".to_owned())]))
    }

    #[test]
    fn nested_italics() {
        let input = r"/textit{italic /textit{text} works}";
        let tree = basic_tex::italic(input).unwrap();
        assert_eq!(
            tree,
            Italic(vec![
                Text("italic ".to_owned()),
                Italic(vec![Text("text".to_owned())]),
                Text(" works".to_owned())
            ])
        )
    }
}
