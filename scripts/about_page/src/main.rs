use regex::Regex;
use std::fs;
use std::ops::Range;
use std::str::CharIndices;

fn re_format(line: &str) -> String {
    let re = Regex::new(r"\\text((hl)|(bf))\{(?P<text>.+?)\}").unwrap();
    let line = re.replace_all(line, "**$text**");

    let re = Regex::new(r"\\textit\{(?P<text>.+?)\}").unwrap();
    let line = re.replace_all(&line, "*$text*");

    let re = Regex::new(r"\\myhref\{(?P<url>.+?)\}\{(?P<name>.+?)\}").unwrap();
    let line = re.replace_all(&line, "[$name]($url)");

    let line = line.replace(r"\\", "  \n");
    let line = line.replace(r"\smallskip", "");

    // unescape characters
    let line = line.replace(r"\_", "_");

    line.to_string()
}

fn find_matching_bracket(line: &mut CharIndices) -> Option<Range<usize>> {
    let mut depth = 0;
    let mut start = None;
    for (i, char) in line {
        match (char, depth) {
            ('{', 1..) => depth += 1,
            ('{', 0) => {
                start = Some(i);
                depth += 1;
            }
            ('}', 2..) => depth -= 1,
            ('}', 1) => {
                let start = start.expect("cant reach depth 1 without passing 0 and setting start");
                return Some(start..i);
            }
            ('}', 0) => panic!("improperly closed bracket"),
            (_, _) => continue,
        }
    }
    None
}

fn bracket_enclosed_to_vec(line: &str) -> Vec<&str> {
    let mut indices = line.char_indices();
    let list: Vec<_> = std::iter::from_fn(move || match find_matching_bracket(&mut indices) {
        Some(range) => Some(&line[range]),
        None => None,
    })
    .collect();
    list
}

fn format_open_source_main(line: &str) -> String {
    let items = bracket_enclosed_to_vec(line);
    let link = re_format(items[0]);
    format!("- {link} ({}),   \n_{}_\n", items[1], items[2])
}

#[derive(Debug)]
struct CvEntry<'a> {
    date: &'a str,
    what: &'a str,
    location: &'a str,
    note: &'a str,
    description: String,
}

impl<'a> TryFrom<&'a str> for CvEntry<'a> {
    type Error = ();
    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let items = bracket_enclosed_to_vec(line);
        dbg!(&items);
        Ok(dbg!(Self {
            date: items[0],
            what: items[1],
            location: items[2],
            note: items[4],
            description: re_format(items[5]),
        }))
    }
}

impl CvEntry<'_> {
    fn to_md(self) -> String {
        let CvEntry {
            date,
            what,
            location,
            note,
            description,
        } = self;
        let note = re_format(note);
        let mut res = format!("- {date} **{what}** _{location}_ {note}  \n");
        if !description.is_empty() {
            res = res + &format!("_{description}_\n");
        }
        res
    }
}

fn main() {
    let mut output = String::from(
        r#"---
title: "About me"
seoTitle: "David Kleingeld"
draft: false
menu: "main"
---"#,
    );
    let input = fs::read_to_string("body.tex").unwrap();
    let lines = input.lines();

    let summary: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\section{Summary}"))
        .skip(1)
        .take_while(|line| !line.starts_with(r"\section{Education}"))
        .collect();
    output.push_str("Summary\n=======\n");
    output.push_str(&summary);
    output.push_str("\n\n");

    let education: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\section{Education}"))
        .skip(1)
        .take_while(|line| !line.starts_with(r"\section{Skills}"))
        .filter(|line| !line.is_empty())
        .map(CvEntry::try_from)
        .map(Result::unwrap)
        .map(CvEntry::to_md)
        .collect();

    output.push_str("Education\n=======\n");
    output.push_str(&education);
    output.push_str("\n\n");

    let software_engineering: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\subsection{Software Engineering}"))
        .skip(1)
        .take_while(|line| !line.starts_with(r"\subsection{Programming Languages}"))
        .map(re_format)
        .collect();
    output.push_str("Skills\n=======\n");
    output.push_str("### Software Engineering\n\n");
    output.push_str(&software_engineering);
    output.push_str("\n\n");

    let languages: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\subsection{Programming Languages}"))
        .skip(2)
        .take_while(|line| !line.starts_with(r"\end{itemize}"))
        .map(re_format)
        .map(|line| line.trim_start().strip_prefix(r"\item").unwrap().to_owned())
        .map(|line| "-".to_owned() + &line + "\n\n")
        .collect();

    output.push_str("### Programming Languages\n\n");
    output.push_str(&languages);
    output.push_str("\n\n");

    let devops: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\subsection{DevOps}"))
        .skip(2)
        .take_while(|line| !line.starts_with(r"\end{itemize}"))
        .map(re_format)
        .map(|line| line.trim_start().strip_prefix(r"\item").unwrap().to_owned())
        .map(|line| "-".to_owned() + &line + "\n\n")
        .collect();

    output.push_str("### DevOps\n\n");
    output.push_str(&devops);
    output.push_str("\n\n");

    let oss_list: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\subsection{Open Source}"))
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(format_open_source_main)
        .collect();

    output.push_str("Experience\n=======\n");
    output.push_str("### Open Source\n\n");
    output.push_str(&oss_list);
    output.push_str("\n\n");

    let oss_text: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\subsection{Open Source}"))
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .take_while(|line| !line.starts_with(r"\subsection"))
        .map(re_format)
        .collect();

    output.push_str("\n");
    output.push_str(&oss_text);
    output.push_str("\n");

    let other: String = lines
        .clone()
        .skip_while(|line| !line.starts_with(r"\subsection{Other}"))
        .skip(1)
        .take_while(|line| !line.starts_with(r"\end{document}"))
        .map(re_format)
        .collect();

    output.push_str("### Other\n");
    output.push_str(&other);

    fs::write("output.md", &output).unwrap();
}
