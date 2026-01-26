use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::sync::LazyLock;

static DONE_READ_PATH: LazyLock<String> = LazyLock::new(|| {
    let home = std::env::var("HOME").expect("HOME env variable is not set");
    format!("{home}/.sfeed/done_read")
});

fn main() -> io::Result<()> {
    let already_read = load_already_read(&DONE_READ_PATH);
    let new_items: Vec<Item> = {
        let items = read_items_from_stdin();
        items
            .into_iter()
            .filter(|x| !already_read.contains(&x.link))
            .collect()
    };

    if new_items.is_empty() {
        eprintln!("No new unread items");
        return Ok(());
    }

    write_output(&new_items)?;

    append_new_done_items(&DONE_READ_PATH, &new_items);

    eprintln!("{} new item(s)", new_items.len());

    Ok(())
}

fn load_already_read(path: &str) -> HashSet<String> {
    match File::open(path) {
        Ok(f) => BufReader::new(f)
            .lines()
            .filter_map(Result::ok)
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
            .collect(),
        Err(e) => {
            eprintln!("[warn] Can't open done_read file: {path:?}. Error: {e}");
            HashSet::default()
        }
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

fn read_items_from_stdin() -> Vec<Item> {
    std::io::stdin()
        .lines()
        // this will crash on invalid UTF-8, for example
        .map(|line| line.expect("Error on reading line from stdin"))
        .flat_map(|line| {
            Item::parse_from_line(&line)
                .inspect_err(|e| eprintln!("[warn] Parse error: {e:?} on line {line:?}"))
        })
        .collect()
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

fn append_new_done_items(path: &str, done_items: &Vec<Item>) {
    let mut done_links: Vec<&str> = done_items.into_iter().map(|x| x.link.as_ref()).collect();

    // trailing empty line to visually separate newly added dones
    done_links.push("");

    let out = done_links.join("\n");
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Can't open file with done_read")
        .write_all(out.as_bytes())
        .expect("Error writing to done_read file");
}
