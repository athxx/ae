use sailfish::TemplateOnce;

pub struct BaseTpl {
    pub title: String,
    pub keywords: String,
    pub description: String,
}

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
#[template(rm_whitespace = true)]
pub struct IndexTemplate {
    pub base: BaseTpl,
    pub messages: Vec<String>,
}

// input message and render and return template
pub fn index(headline: String, msg: Vec<String>) -> String {
    let tpl = IndexTemplate {
        base: BaseTpl {
            title: headline,
            keywords: "this is a keyword".to_owned(),
            description: "this is a description".to_owned(),
        },

        messages: msg,
    };
    tpl.render_once().unwrap()
}
