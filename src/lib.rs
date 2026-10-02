use crate::{
    code::{Code, EvalError},
    piston::submit_code,
};

mod code;
mod piston;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Evaluate code
///
/// Takes a codeblock annotated with the language to evaluate, \
/// and an optional second codeblock for stdin.
#[poise::command(
    prefix_command,
    track_edits,
    category = "evaluation",
    aliases("e", "eval")
)]
pub async fn evaluate(
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

    if let Some(stderr) = result.stderr
        && !stderr.is_empty()
    {
        response.push_str(format!("stderr: ```\n{stderr}\n```\n").as_str());
    }
    if let Some(stdout) = result.stdout
        && !stdout.trim().is_empty()
    {
        if stdout.contains('`') {
            response
                .push_str("Tried to break formatting. \n```diff\n- Invalid character: \"`\" ```");
        } else {
            response.push_str(format!("```\n{stdout}\n```").as_str());
        }
    } else {
        response.push_str("```\nNo output\n```");
    }

    ctx.say(response).await?;

    Ok(())
}
