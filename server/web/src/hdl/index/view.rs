use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "default/index.html")]
pub struct IndexTemplate<T> {
    title: String,
    keywords: String,
    description: String,
    pub messages: Vec<T>,
}

// input message and render and return template
pub fn index<T>(header: String, msg: Vec<T>) -> String {
    let template = IndexTemplate {
        title: header,
        keywords: "".to_owned(),
        description: "".to_owned(),
        messages: msg,
    };
    template.render_once().unwrap()
}
