use std::collections::HashSet;
use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::sync::LazyLock;

static DONE_READ_PATH: LazyLock<String> = LazyLock::new(|| {
    let home = std::env::var("HOME").expect("HOME env variable is not set");
    format!("{home}/.sfeed/done_read")
});

fn main() -> io::Result<()> {
    let feeds_files: Vec<OsString> = std::env::args_os().skip(1).collect();
    if feeds_files.is_empty() {
        // TODO: can just read from stdin. Don't need pass list of files and reading them here
        eprintln!(
            "Usage: {} [file ...]",
            std::env::args().next().unwrap_or_default()
        );
        return Ok(());
    }

    let already_read = load_already_read(&DONE_READ_PATH);
    let new_items: Vec<Item> = read_items_from_files(feeds_files)
        .into_iter()
        .filter(|x| !already_read.contains(&x.link))
        .collect();
    let new_done_read = new_items.iter().map(|x| x.link.clone()).collect();

    if new_items.is_empty() {
        eprintln!("No new unread items");
        return Ok(());
    }

    write_output(&new_items)?;
    append_new_done_read(&DONE_READ_PATH, new_done_read);

    eprintln!("{} new item(s)", new_items.len());

    Ok(())
}

fn load_already_read(path: &str) -> HashSet<String> {
    if let Ok(f) = File::open(path) {
        BufReader::new(f)
            .lines()
            .filter_map(Result::ok)
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
            .collect()
    } else {
        eprintln!("[warn] Can't open done_read file. It does not exists yet?");
        HashSet::default()
    }
}
struct Item {
    title: String,
    link: String,
    content: String,
}

impl Item {
    fn parse_from_line(line: &str) -> Result<Self, String> {
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 7 {
            return Err("Not enough fields".into());
        }

        let _title = fields[1].to_string();
        let link = fields[2].to_string();
        let content = fields[3].to_string();
        let author = fields[6].to_string();

        // TODO: me hack, idk should it be in release or me do something other
        let title = author;

        Ok(Item {
            title,
            link,
            content,
        })
    }
}

fn read_items_from_files(feeds_files: Vec<OsString>) -> Vec<Item> {
    let mut res = Vec::new();

    for file_path in feeds_files {
        let file = File::open(&file_path).expect(&format!("Can't open feeds file {file_path:?}"));

        for line in BufReader::new(file).lines().filter_map(Result::ok) {
            match Item::parse_from_line(&line) {
                Ok(item) => res.push(item),
                Err(e) => eprintln!("[warn] Parsing error: {e:?} on line {line:?}"),
            };
        }
    }

    res
}

fn write_output(items: &[Item]) -> io::Result<()> {
    let mut f = std::io::stdout();

    writeln!(
        f,
        r#"<!DOCTYPE html>
<meta charset="utf-8">
<title>Unread sfeed items</title>
<body style="max-width:900px; margin:2rem auto; font:1.1rem/1.5 sans-serif;">"#
    )?;

    let esc = |s: &str| {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    };

    for item in items {
        writeln!(
            f,
            "<h3><a href=\"{}\">{}</a></h3>",
            esc(&item.link),
            esc(&item.title)
        )?;

        if item.content.is_empty() || item.content == "NULL" {
            writeln!(f, "<p>(no content)</p>")?;
        } else if item.content.trim_start().starts_with('<') {
            writeln!(f, "{}", item.content)?;
        } else {
            writeln!(f, "<pre>{}</pre>", esc(&item.content))?;
        }

        writeln!(f, "<hr>")?;
    }

    writeln!(f, "</body></html>")?;

    Ok(())
}

fn append_new_done_read(path: &str, links: Vec<String>) {
    let mut out = links.join("\n");
    // trailing empty line to visually separate newly added dones
    out.push_str("\n\n");
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Can't open file with done_read")
        .write_all(out.as_bytes())
        .expect("Error writing to done_read file");
}
