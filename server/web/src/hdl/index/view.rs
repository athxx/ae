use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "default/index.html")]
pub struct IndexTemplate {
    title: String,
    meta : Vec<String>
    pub messages: Vec<String>
}

// input message and render and return template
pub fn index(title: String, msg: Vec<String>) -> String {
    let template = IndexTemplate { title: title, messages: msg };
    template.render_once().unwrap()
}