#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Latex {
    Text(String),
    Italic(Vec<Latex>),
    Bold(Vec<Latex>),
    Highlight(Vec<Latex>),
    Href { url: String, text: String },
    LineBreak,
}

peg::parser!(
    grammar basic_tex() for str {
        rule eof()
            = ![_] {}

        rule syntax()
            = "}" / r"\textit" / r"\textbf" / r"\texthl" / r"\myhref" / r"\\" {}

        rule char() -> char
            = !(syntax() / eof()) c:[_] {
                c
        }

        rule chars() -> String
            = s:char()+ {
            String::from_iter(s.into_iter())
        }

        // lazily collect anything that is not syntax + one more
        rule text() -> Latex
            = s:chars() {
            Latex::Text(s)
        }

        pub rule italic() -> Latex
            = r"\textit{" l:item()* "}" {
            Latex::Italic(l)
        }

        pub rule bold() -> Latex
            = r"\textbf{" l:item()* "}" {
            Latex::Bold(l)
        }

        pub rule highlight() -> Latex
            = r"\texthl{" l:item()* "}" {
            Latex::Highlight(l)
        }

        pub rule href() -> Latex
            = r"\myhref{" url:chars() "}{" text:chars() "}" {
            Latex::Href{url, text}
        }

        pub rule line_break() -> Latex
            = r"\\" {
            Latex::LineBreak
        }

        rule item() -> Latex
            = i:(italic() / bold() / highlight() / href() / line_break() / text()) {
            i
        }

        pub rule line() -> Vec<Latex>
            = l:item()* ![_] {
            l
        }
    }
);

pub fn parse_line(line: &str) -> Vec<Latex> {
    basic_tex::line(line).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Latex::*;

    fn text(s: &'static str) -> Latex {
        Latex::Text(s.to_owned())
    }

    #[test]
    fn line_with_text() {
        let input = "hello";
        let tree = basic_tex::line(input).unwrap();
        assert_eq!(tree, vec![text("hello")])
    }

    #[test]
    fn italic() {
        let input = r"\textit{italic text works}";
        let tree = basic_tex::italic(input).unwrap();
        assert_eq!(tree, Italic(vec![text("italic text works")]))
    }

    #[test]
    fn nested_italics() {
        let input = r"\textit{italic \textit{text} works}";
        let tree = basic_tex::italic(input).unwrap();
        assert_eq!(
            tree,
            Italic(vec![
                text("italic "),
                Italic(vec![text("text")]),
                text(" works")
            ])
        )
    }

    #[test]
    fn myhref() {
        let input = r"\myhref{https://davidsk.dev/thesis}{GovFs}";
        let tree = basic_tex::href(input).unwrap();
        assert_eq!(
            tree,
            Href {
                url: "https://davidsk.dev/thesis".to_owned(),
                text: "GovFs".to_owned()
            }
        )
    }

    #[test]
    fn italic_in_line() {
        let input = r"\textit{italic text works}";
        let italic = basic_tex::line(input).unwrap().remove(0);
        assert_eq!(italic, Italic(vec![text("italic text works")]))
    }

    #[test]
    fn italic_and_bold_in_line() {
        let input = r"\textit{italic} \textbf{bold}";
        let line = basic_tex::line(input).unwrap();
        assert_eq!(
            line,
            vec![
                Italic(vec![text("italic")]),
                text(" "),
                Bold(vec![text("bold")])
            ]
        )
    }

    #[test]
    fn myhref_in_line() {
        let input = r"\myhref{https://davidsk.dev/thesis}{GovFs}";
        let tree = basic_tex::line(input).unwrap();
        assert_eq!(
            tree,
            vec![Href {
                url: "https://davidsk.dev/thesis".to_owned(),
                text: "GovFs".to_owned()
            }]
        )
    }

    #[test]
    fn cv_entry_description() {
        let input = r"Focused on cloud and distributed computing, explored other fields including AI, robotics and CS-theory \\ \textit{Thesis: \myhref{https://davidsk.dev/thesis}{GovFs}, a highly scalable consistent distributed file system}";
        let tree = basic_tex::line(input).unwrap();
        assert_eq!(
            tree,
            vec![
                text(
                    r"Focused on cloud and distributed computing, explored other fields including AI, robotics and CS-theory "
                ),
                LineBreak,
                text(" "),
                Italic(vec![
                    text("Thesis: "),
                    Href {
                        url: "https://davidsk.dev/thesis".to_owned(),
                        text: "GovFs".to_owned()
                    },
                    text(", a highly scalable consistent distributed file system")
                ])
            ]
        )
    }
}
