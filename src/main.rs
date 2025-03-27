use parser::parser::parse_html;

mod parser;

fn main() {
    // Test data
    let html = r#"
        <html>
            <head>
                <title>My Title</title>
            </head>
            <body>
                <h1>Welcome to my website</h1>
                <p>This is a paragraph</p>
            </body>
        </html>
    "#;

    parse_html(html);
}
