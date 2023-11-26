use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "theme/default/index.html")]
pub struct HelloTemplate {
    pub messages: Vec<String>
}