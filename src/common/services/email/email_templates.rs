use askama::Template;

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeEmail<'a> {
   pub name: &'a str,
   pub verify_email_url: &'a str
}
