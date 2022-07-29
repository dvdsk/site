use crate::parser::Latex;

pub fn latex_to_markdown(latex: Vec<Latex>) -> String {
    latex.into_iter().map(|l| l.as_markdown()).collect()
}

impl Latex {
    fn as_markdown(&self) -> String {
        use Latex::*;

        let mut res = String::new();
        match self {
            Text(s) => res.push_str(&s),
            Italic(latex) => {
                let s: String = latex.into_iter().map(|l| l.as_markdown()).collect();
                res.push('*');
                res.push_str(&s);
                res.push('*');
            }
            Bold(latex) => {
                let s: String = latex.into_iter().map(|l| l.as_markdown()).collect();
                res.push_str("**");
                res.push_str(&s);
                res.push_str("**");
            }
            Highlight(latex) => {
                let s: String = latex.into_iter().map(|l| l.as_markdown()).collect();
                res.push_str("**");
                res.push_str(&s);
                res.push_str("**");
            }
            Href { url, text } => res.push_str(&format!("[{text}]({url})")),
            LineBreak => res.push_str("  \n"),
        }

        res
    }
}
