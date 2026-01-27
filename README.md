# sfeed_bhtml

This is a little utility that produces single html page with content of
RSS/Atom feeds (content itself, not just titles), by parsing feeds items
prepared by [sfeed](https://codemadness.org/sfeed.html). This utility also
tracks finished read items so you don't have to.

### Why all content on single page?

I have a few feeds sources which have posting style of multiple short-text
posts in short period of time (generally they are channels on social networks
like Twitter/X, Mastodon, Telegram). I find it easier to quickly scroll through
all of them in single place instead of opening each one by one. Also this
approach gives context in case of multi posts in short period of time.

(To be honest I'm still confused why I haven't found many solutions for this
exact problem. Maybe I did bad research, who knows)

P.S. I still keep some short-text but rare-posting channels in regular feed
reader to have bigger attention span dedicated to them.

### Why [sfeed](https://codemadness.org/sfeed.html)?

It was chosen because of its format simplicity and its philosophical separation
of gathering feeds and displaying them.

In theory, you could prepare sfeed's format lines even from
[newsboat](https://github.com/newsboat/newsboat) or
[newsraft](https://codeberg.org/newsraft/newsraft/) by calling to their sqlite
database directly.

## Demo

TODO

## Usage

```shell
sfeed_update
cat ~/.sfeed/feeds/* | sfeed_bhtml > res.html
$BROWSER res.html
```

If you'll run `sfeed_bhtml` now once again, output will be empty, because it
tracks finished read items and outputs only new items.

If you want to see history of previous read items, check
[helper.sh](./helper.sh) for example of how you can keep previous generated
html pages.

### Input format

One item per line, fields separated by TAB character. Order of fields:

1. UNIX timestamp in UTC+0
2. title
3. link
4. content. TABs, newlines and `\` must be escaped with `\`, so it becomes:
   `\t`, `\n` and `\\`
5. *not used*
6. *not used*
7. author

## Configuration

Not intended and not planned.

At first it was meant to be self-contained script, but sadly
[cargo-script](https://github.com/rust-lang/rust-project-goals/issues/119) is
not yet stabilized. That being said, feel free to edit source code itself to
suit your needs (in best traditions of [suckless](https://suckless.org/)).

## Installation

```shell
cargo build --release
cp target/release/sfeed_bhtml /you/know/where/to/put/it
```
