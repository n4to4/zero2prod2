use actix_web::http::header::ContentType;
use actix_web::HttpResponse;

pub async fn newsletter_form() -> HttpResponse {
    println!("GET newsletter form");
    HttpResponse::Ok().content_type(ContentType::html()).body(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
</head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Submit New Issue</title>
<body>
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
    )
}
