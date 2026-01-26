use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::LazyLock;

static DONE_READ_PATH: LazyLock<String> = LazyLock::new(|| {
    let home = std::env::var("HOME").expect("HOME env variable is not set");
    format!("{home}/.sfeed/done_read")
});

fn main() {
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
        return;
    }

    write_output(&new_items);

    append_new_done_items(&DONE_READ_PATH, &new_items);

    eprintln!("{} new item(s)", new_items.len());
}

// TODO: maybe treat as id hash of whole Item
fn load_already_read(path: &str) -> HashSet<String> {
    match File::open(path) {
        Ok(f) => BufReader::new(f)
            .lines()
            .filter_map(|x| {
                x.inspect_err(|e| eprintln!("[warn] Skipping invalid line in done_read file: {e}"))
                    .ok()
            })
            .filter(|s| !s.is_empty())
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
        .filter_map(|x| {
            x.inspect_err(|e| eprintln!("[warn] Skipping invalid line in stdin: {e}"))
                .ok()
        })
        .flat_map(|line| {
            Item::parse_from_line(&line)
                .inspect_err(|e| eprintln!("[warn] Parse error: {e:?} on line {line:?}"))
        })
        .collect()
}

fn write_output(items: &[Item]) {
    let mut rss_content = items
        .iter()
        .map(
            |Item {
                 title,
                 link,
                 content,
             }| {
                format!(
                    r#"<h3><a href=\"{link}\">{title}</a></h3>
                       <pre>{content}</pre>
                       <hr>"#
                )
            },
        )
        .collect::<Vec<String>>()
        .join("\n");
    // add style (it will go inside iframe)
    rss_content.push_str(
        "<style> body { max-width:900px; margin:2rem auto; }
                 pre { white-space: pre-wrap }
         </style>",
    );

    println!(
        r#"<!DOCTYPE html>
           <meta charset="utf-8">
           <title>Unread sfeed items</title>
           <body>"#
    );

    // put everything in iframe for better isolation (and disable js)
    // width and height 99% to prevent browser from showing scrollbars in body outside of iframe
    println!(
        r#"<iframe sandbox width="99%"
                    style="position: absolute; height: 99%; border: none"
                    srcdoc=""#
    );
    // the following is content of attribute srcdoc
    println!("{}", rss_content.replace('"', "&quot;"));
    // closing srcdoc and iframe
    println!("\"></iframe>");

    println!("</body></html>");
}

fn append_new_done_items(path: &str, done_items: &[Item]) {
    let mut done_links: Vec<&str> = done_items.iter().map(|x| x.link.as_ref()).collect();

    // trailing empty line to visually separate newly added dones
    done_links.push("");

    let out = done_links.join("\n");
    OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Can't open done_read file")
        .write_all(out.as_bytes())
        .expect("Error writing to done_read file");
}
