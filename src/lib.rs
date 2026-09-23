use crate::{
    code::{Code, EvalError},
    piston::submit_code,
};
use poise::serenity_prelude as serenity;

mod code;
mod piston;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn hello(
    ctx: Context<'_>,
    #[description = "User to greet"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("Hello! {}", user.name);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(prefix_command)]
pub async fn eval(
    ctx: Context<'_>,
    #[description = "Code to be executed"]
    #[rest]
    code: Option<String>,
) -> Result<(), Error> {
    let code = code
        .ok_or(EvalError::EmptyCodeBlock)
        .and_then(|code| Code::from_code_blocks(code.as_str()))?;

    let language = code.get_language().name();
    let client = &reqwest::Client::new();
    let result = submit_code(client, language, code.get_code(), code.get_stdin()).await?;

    let mut response = format!(
        "Your eval has returned with code: {}\n\n",
        result.code.unwrap_or(1),
    );

    if let Some(stdout) = result.stdout
        && !stdout.is_empty()
    {
        response.push_str("```\n{}\n```");
    } else {
        response.push_str("```\nNo output\n```");
    }

    if let Some(stderr) = result.stderr
        && !stderr.is_empty()
    {
        response.push_str(format!("\nstderr: ```\n{stderr}\n```").as_str());
    }

    ctx.say(response).await?;

    Ok(())
}
