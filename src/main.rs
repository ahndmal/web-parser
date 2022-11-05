mod r_files;

use std::fs;
use std::fs::File;
use html_parser::{Dom, Node, Element};
use scraper::{Html, Selector};
use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};

fn main() {


    let html_str = r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Html parser</title>
                </head>
                <body>
                    <h1 id="a" class="b c">Hello world</h1>
                    <div id="div1">one</div>
                    <div>two</div>
                </body>
            </html>"#;

    let doc = Html::parse_document(html_str);
    let divs_delector = Selector::parse("div").unwrap();
    for elem in doc.select(&divs_delector) {
        let elem_value = elem.value();
        if elem_value.attr("id") != None {
            println!("{:?}", elem_value.attr("id"));
        }
    }
}

fn core_html(html_str: &str) {
    //https://docs.rs/html_parser/latest/html_parser/enum.Node.html
    let dom: Result<Dom, html_parser::Error> = Dom::parse(html_str);
    let nodes = dom.unwrap().children;
    for node in nodes {
        match node {
            Node::Text(text) => println!("text"),
            Node::Comment(comment) => println!("comment is {comment}"),
            Node::Element(element) => {
                let body: &Node = &element.children[1];
                if let Node::Element(element) = body {
                    let div2 = &element.children[2];
                    println!("{:?}", div2)
                }
            },
        }
    }
}
