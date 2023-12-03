use sailfish::TemplateOnce;

pub struct BaseTpl {
    pub title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(TemplateOnce)]
#[template(path = "default/index.html")]
pub struct IndexTemplate {
    pub base: BaseTpl,
    pub messages: Vec<String>,
}

// input message and render and return template
pub fn index(headline: String, msg: Vec<String>) -> String {
    let template = IndexTemplate {
        base: BaseTpl {
            title: headline,
            keywords: "this is a keyword".to_owned(),
            description: "this is a description".to_owned(),
        },

        messages: msg,
    };
    template.render_once().unwrap()
}
