use std::collections::{HashSet, VecDeque};
use std::rc::Rc;
use convert_case::{Case, Casing};
use scraper::{Selector, Html};
use reqwest::Error;
use tokio;
use std::io::Write as _;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let ctx = &mut Context::default();
    let enumerate = &mut (0..);

    /*
    // ctx.require_url("https://doc.rust-lang.org/std/", "index.html");
    // ctx.require_url("https://docs.rs/vulkano/latest/vulkano/", "index.html");
    let buf = std::fs::read_to_string("assets/unity-url.txt").unwrap();
    buf.lines().for_each(|line| {
        ctx.insert_word(line);
        ctx.require_url("https://docs.unity3d.com/ScriptReference/", line);
    });

    while let (Some(i), Some(url)) = (enumerate.next(), ctx.next()) {
        println!("{i}: {}", url);
        extract_element_unity(ctx, &url).await.unwrap();
    }*/
    let buf = std::fs::read_to_string("assets/unity-reflect.txt").unwrap();
    buf.lines().for_each(|line| {
        ctx.insert_word(line);
    });

    let mut words = ctx.words.drain().into_iter().collect::<Vec<_>>();
    words.sort();

    let mut file = std::fs::File::create("assets/unity.txt").unwrap();
    // let mut file = std::io::stdout();
    words.iter().for_each(|word| writeln!(file, "{}", word).unwrap());

    Ok(())
}

#[derive(Default)]
struct Context {
    words: HashSet<String>,
    // todo: rename
    urls_processed: HashSet<Rc<str>>,
    urls_to_process: VecDeque<Rc<str>>,
}

impl Context {
    fn require_url(&mut self, current: &str, href: &str) {
        let abs = reqwest::Url::parse(current).unwrap().join(href).unwrap();
        let url = Rc::<str>::from(abs.to_string());
        if self.urls_processed.insert(url.clone()) {
            self.urls_to_process.push_back(url);
        }
    }

    fn insert_word(&mut self, word: &str) {
        const PAT: &[char] = &['?', '\n', '\t', '-', ' ', ':', '@', '*', '(', ')', '!', '\"', '#', '$', '%', '&', '\'', '=', '^', '~', '|', '\\', '`', '[', ']', '{', '}', '+', ';', '.', ',', '<', '>', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        word.to_case(Case::Lower).split(PAT).filter(|s| !s.is_empty()).for_each(|word| {
            self.words.insert(word.to_owned());
        });
    }
}

impl Iterator for Context {
    type Item = Rc<str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.urls_to_process.pop_front()
    }
}

async fn fetch_html(url: &str) -> Result<String, Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

async fn extract_element_rust(ctx: &mut Context, url: &str) -> Result<(), Error> {
    let html = fetch_html(url).await?;
    let fragment = Html::parse_document(&html);
    {
        let ids = ["primitives", "modules", "functions", "macros", "keywords", "structs", "enums", "traits"];

        let href_selector = Selector::parse("div > a").unwrap();

        for id in ids {
            let list_selector = Selector::parse(&format!("#{} + ul > li", id)).unwrap();

            for element in fragment.select(&list_selector) {
                element.select(&href_selector).next().map(|a| {
                    a.value().attr("href").map(|href| {
                        ctx.require_url(url, href);
                    });
                    let text = a.text().next().unwrap();
                    ctx.insert_word(text);
                });
            }
        }
    }
    {
        // note: implementations つまりassociated consts & functions & methods
        let fn_selector = Selector::parse("a.constant, section.method > h4.code-header").unwrap();
        let selector = Selector::parse("#implementations-list").unwrap();
        fragment.select(&selector).next().map(|element| {
            element.select(&fn_selector).for_each(|a| {
                for text in a.text() {
                    ctx.insert_word(text);
                }
            });
        });
    }

    {
        // note: static関数の定義, traitの定義, structの定義, enumの定義
        let selector = Selector::parse("pre.item-decl > code").unwrap();
        fragment.select(&selector).next().map(|element| {
            for text in element.text() {
                ctx.insert_word(text);
            }
        });
    }

    Ok(())
}

async fn extract_element_unity(ctx: &mut Context, url: &str) -> Result<(), Error> {
    let html = fetch_html(url).await?;
    let fragment = Html::parse_document(&html);

    {
        let selector = Selector::parse("td.lbl > a[href]").unwrap();
        for element in fragment.select(&selector) {
            element.value().attr("href").map(|href| {
                ctx.insert_word(href);
                ctx.require_url(url, href);
            });
        }
    }
    {
        let selector = Selector::parse("h3 + table.list > tbody > tr > td.lbl").unwrap();
        for element in fragment.select(&selector) {
            let text = element.text().next().unwrap();
            ctx.insert_word(&format!("{text:?}"));
        }
    }

    Ok(())
}