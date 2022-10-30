use std::fs::{read_to_string, File};
use std::io::Write;

const HEADER: &str = "\
{| class=\"wikitable\"
! File name !! Japanese !! English translation
";

const FOOTER: &str = "\
|}
";

fn main() {
    let entries: Vec<Entry> = read_to_string("transcription.tsv")
        .unwrap()
        .split('\n')
        .skip(1)
        .map(Entry::from)
        .collect();

    let mut output = String::new();
    output.push_str(HEADER);
    for entry in entries {
        let string = format!(
            "|-\n| {} || {} || {}\n",
            entry.filename,
            format_text(&entry.jp_title, &entry.jp_text),
            format_text(&entry.en_title, &entry.en_text),
        );
        output.push_str(&string);
    }
    output.push_str(FOOTER);

    let mut file = File::create("output.txt").unwrap();
    write!(file, "{output}").unwrap()
}

fn format_text(title: &str, body: &str) -> String {
    format!("'''{title}'''<br>{body}")
}

#[derive(Debug)]
struct Entry {
    filename: String,
    jp_title: String,
    jp_text: String,
    en_title: String,
    en_text: String,
}

impl From<&str> for Entry {
    fn from(contents: &str) -> Self {
        let cells: Vec<&str> = contents.split('\t').collect();

        let en_text = *cells.iter().rev().find(|s| !s.trim().is_empty()).unwrap();

        Self {
            filename: cells[0].to_owned(),
            jp_title: cells[1].to_owned(),
            jp_text: cells[2].to_owned(),
            en_title: cells[3].to_owned(),
            en_text: en_text.to_owned(),
        }
    }
}
