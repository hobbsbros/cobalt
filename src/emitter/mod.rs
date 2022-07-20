//! Provides a simple Cobalt-to-HTML emitter.

use std::path::Path;

use crate::{
    parser::Expression,
    error::{throw, Error},
    Config,
};


/// Abstracts over the HTML processed in `Emitter::emit()`.
pub struct Html {
    site: String,
    page: String,
    head: String,
    body: String,
}

impl Html {
    /// Constructs a new instance of `Html`.
    pub fn new() -> Self {
        Self {
            site: String::new(),
            page: String::new(),
            head: String::new(),
            body: String::new(),
        }
    }

    /// Sets the site name.
    pub fn set_site(&mut self, site: String) {
        self.site = site;
    }

    /// Sets the page name.
    pub fn set_page(&mut self, page: String) {
        self.page = page;
    }

    /// Pushes to the head.
    pub fn push_head(&mut self, s: &str) {
        self.head.push_str(s);
    }

    /// Pushes to the body.
    pub fn push_body(&mut self, s: &str) {
        self.body.push_str(s);
    }

    /// Gets the head.
    pub fn get_head(&self) -> String {
        self.head.to_owned()
    }

    /// Gets the body.
    pub fn get_body(&self) -> String {
        self.body.to_owned()
    }

    /// Gets the site & page name based on the provided protocol.
    pub fn get_name(&self, protocol: &str) -> String {
        match protocol {
            "page" => format!("<title>{}</title>\n", &self.page),
            "site" => format!("<title>{}</title>\n", &self.site),
            "page | site" => format!("<title>{} | {}</title>\n", &self.page, &self.site),
            "site | page" => format!("<title>{} | {}</title>\n", &self.site, &self.page),
            _ => throw(Error::InvalidConfig (protocol.to_string())),
        }
    }
}


/// Provides the `emit()` method to transform Cobalt syntax trees into HTML.
pub struct Emitter {
    config: Config,
}

impl Emitter {
    /// Constructs a new emitter from a parser.
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    /// Emits an expression, accounting for class and ID.
    fn emit_class_id(&self, name: &str, tag: &str, arg: &str, class: Option<String>, id: Option<String>) -> String {
        match class {
            Some(c) => match id {
                Some(i) => {
                    // Class and ID
                    format!(
                        "<{} class=\"{}\" id=\"{}\" {}=\"{}\">",
                        &name,
                        &c,
                        &i,
                        &tag,
                        &arg,
                    )
                },
                None => {
                    // Class, no ID
                    format!(
                        "<{} class=\"{}\" {}=\"{}\">",
                        &name,
                        &c,
                        &tag,
                        &arg,
                    )
                },
            },
            None => match id {
                Some(i) => {
                    // No Class but ID
                    format!(
                        "<{} id=\"{}\" {}=\"{}\">",
                        &name,
                        &i,
                        &tag,
                        &arg,
                    )
                },
                None => {
                    format!(
                        "<{} {}=\"{}\">",
                        &name,
                        &tag,
                        &arg,
                    )
                },
            }
        }
    }

    /// Emits an expression into an optional page name, head code, body code.
    fn emit_expr(&self, expr: Expression) -> (Option<String>, String, String) {
        let mut site: Option<String> = None;
        #[allow(unused_mut)]
        let mut head = String::new();
        let mut body = String::new();

        match expr {
            Expression::Ctrl {
                keyword: k,
                class: c,
                id: i,
                argument: a,
            } => match k.as_str() {
                "pagename" => {
                    site = Some(a);
                },
                "image" => body.push_str(&self.emit_class_id(
                    "img",
                    "src",
                    &a,
                    c,
                    i,
                )),
                "script" => {
                    body.push_str(&self.emit_class_id(
                        "script",
                        "src",
                        &a,
                        c,
                        i,
                    ));
                    body.push_str("</script>");
                },
                "download" => {
                    body.push_str(&self.emit_class_id(
                        "a",
                        "href",
                        &a,
                        Some("download".to_string()),
                        i,
                    ));
                    body.push_str("Download</a>");
                },
                _ => throw(Error::InvalidCtrlSequence (k)),
            },
            Expression::Paragraph (s) => body.push_str(&format!("<p>{}</p>", &s)),
            Expression::Hyperlink {
                text: t,
                href: h,
            } => body.push_str(&format!("<a href={}>{}</a>", &h, &t)),
            Expression::H1 (s) => body.push_str(&format!("<h1>{}</h1>\n", &s)),
            Expression::H2 (s) => body.push_str(&format!("<h2>{}</h2>\n", &s)),
            Expression::H3 (s) => body.push_str(&format!("<h3>{}</h3>\n", &s)),
            Expression::H4 (s) => body.push_str(&format!("<h4>{}</h4>\n", &s)),
            Expression::H5 (s) => body.push_str(&format!("<h5>{}</h5>\n", &s)),
            Expression::H6 (s) => body.push_str(&format!("<h6>{}</h6>\n", &s)),
        };

        (site, head, body)
    }

    /// Emits a vector of expressions into a `String`.
    pub fn emit(&self, expressions: Vec<Expression>, root_directory: &Path) -> String {
        let header = "\
        <!DOCTYPE html>\n\
        <html>\n\
            <head>\n\
        ".to_string();
        let footer = "\
        </html>\n\
        ".to_string();
        let mut html = Html::new();

        html.set_site(self.config.site.name.to_owned());

        for expr in expressions {
            let emitted = self.emit_expr(expr);
            html.push_head(&emitted.1);
            html.push_body(&emitted.2);

            // Set the site name, if applicable.
            if let Some(s) = emitted.0 {
                html.set_page(s);
            }
        }

        // Push configuration data to the head
        let title_protocol = match &self.config.site.title {
            Some(s) => s,
            None => "page",
        };
        html.push_head(&html.get_name(title_protocol));

        // Emit primary stylesheet and external stylesheets.
        let stylesheet_path = root_directory.join(self.config.style.default.to_owned());
        let stylesheet = match stylesheet_path.into_os_string().into_string() {
            Ok(s) => s,
            Err(_) => throw(Error::CouldNotOpenFile (self.config.style.default.to_owned())),
        };
        let stylesheet_link = format!(
            "<link rel=\"stylesheet\" href=\"{}\">",
            &stylesheet,
        );
        html.push_head(&stylesheet_link);

        if let Some(s) = &self.config.style.external {
            let mut stylesheets = String::new();
            for stylesheet in s {
                let stylesheet_link = format!(
                    "<link rel=\"stylesheet\" href=\"{}\">",
                    &stylesheet,
                );
                stylesheets.push_str(&stylesheet_link);
            }
            html.push_head(&stylesheets);
        }

        html.push_head("\
        </head>\n\
        <body>\n\
        ");

        html.push_body("\
        </body>\n\
        ");

        let mut output = String::new();
        output.push_str(&header);
        output.push_str(&html.get_head());
        output.push_str(&html.get_body());
        output.push_str(&footer);

        output
    }
}