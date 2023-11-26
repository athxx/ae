mod tpl;

pub async fn index() -> &'static str {
    let ctx = tpl::HelloTemplate {
        messages: vec![String::from("foo"), String::from("bar")],
    };
    format!("{}", ctx.render_once().unwrap())
}

pub async fn topic() -> &'static str {
    "pong"
}

pub async fn post() -> &'static str {
    "dd"
}
