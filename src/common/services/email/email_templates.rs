use askama::Template;

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeMail<'a> {
   pub name: &'a str,
   pub login_url: &'a str
}
#[derive(Template)]
#[template(path = "email_verification_code.html")]
pub struct EmailVerificationCodeMail<'a> {
   pub code: &'a str,
   pub expiry: &'a str,
}
