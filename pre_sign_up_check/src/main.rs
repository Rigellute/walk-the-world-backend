use aws_lambda_events::event::cognito::CognitoEventUserPoolsPreSignup;
use lambda_runtime::{error::HandlerError, lambda, Context};
use simple_error::bail;

fn main() {
    lambda!(handler)
}

fn handler(
    event: CognitoEventUserPoolsPreSignup,
    _: Context,
) -> Result<CognitoEventUserPoolsPreSignup, HandlerError> {
    let email_attribute = event.request.user_attributes.get("email").unwrap();

    if !email_attribute.contains("ucl.ac.uk") {
        bail!("Email is not allowed");
    }

    Ok(event)
}
