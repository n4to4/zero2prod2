use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Submit New Issue</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>Title
            <input
                type="text"
                placeholder="Enter issue title"
                name="title"
            >
        </label>
        <br>
        <label>HTML Content
            <input
                type="text"
                placeholder="Enter HTML content"
                name="content_html"
            >
        </label>
        <br>
        <label>Text Content
            <input
                type="text"
                placeholder="Enter text content"
                name="content_text"
            >
        </label>
        <button type="submit">Submit new issue</button>
    </form>
    <p><a href="/admin/dashboard">&gt;- Back</a></p>
</body>
</html>"#,
        ))
}
