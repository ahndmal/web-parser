use html_parser::{Dom, Node};

fn main() {
    //https://docs.rs/html_parser/latest/html_parser/enum.Node.html
    let html_str = r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Html parser</title>
                </head>
                <body>
                    <h1 id="a" class="b c">Hello world</h1>
                    </h1> <!-- comments & dangling elements are ignored -->
                </body>
            </html>"#;

    let dom: Result<Dom, html_parser::Error> = Dom::parse(html_str);
    let nodes = dom.unwrap().children;
    for node in nodes {
        match node {
            Node::Text(text) => println!("text"),
            Node::Comment(comment) => println!("comment"),
            Node::Element(element) => {
                println!("{:?}", element)
            },
        }
    }
}
