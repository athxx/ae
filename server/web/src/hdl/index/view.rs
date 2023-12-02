use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "default/index.html")]
pub struct IndexTemplate {
    pub title: String,
    pub keywords: String,
    pub description: String,
    pub messages: Vec<String>,
}

// input message and render and return template
pub fn index(headline: String, msg: Vec<String>) -> String {
    let template = IndexTemplate {
        title: headline,
        keywords: "this is a keyword".to_owned(),
        description: "this is a description".to_owned(),
        messages: msg,
    };
    template.render_once().unwrap()
}
