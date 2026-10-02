use crate::{
    code::{Code, EvalError},
    piston::submit_code,
};

mod code;
mod piston;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

/// Evaluates code
///
/// Takes a codeblock annotated with the language to evaluate, \
/// and an optional second codeblock for stdin.
///
/// # Examples:
/// !e \`\`\`py
/// print(input("Say your name: "))
/// \`\`\`\
///
/// \`\`\`Kebab\`\`\`
#[poise::command(
    prefix_command,
    track_edits,
    category = "Utility",
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

    ctx.reply(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to be searched"] command: Option<String>,
) -> Result<(), Error> {
    let commands = &ctx.framework().options().commands;

    if let Some(cmd_searched) = command {
        if let Some(cmd_found) = commands
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(&cmd_searched))
        {
            let desc = cmd_found.description.as_deref().unwrap_or("No description");
            let help_text = cmd_found.help_text.as_deref().unwrap_or("No help text");
            let aliases = cmd_found.aliases.as_ref();
            let response = format!(
                "# **{}**\n-# *aliases: {}*\n\n{desc}\n{help_text}",
                cmd_found.name,
                aliases.join(", ")
            );
            ctx.reply(response).await?;
        } else {
            ctx.reply("Command not found").await?;
        }
    } else {
        let mut response = "Available commands:\n\n```diff\n".to_string();
        let mut util_category = "Utility:\n".to_string();
        let mut uncat_category = "Uncategorized:\n".to_string();

        for cmd in commands {
            let category = cmd.category.as_deref().unwrap_or("Uncategorized");
            if category == "Utility" {
                util_category.push_str(format!("+ {}\n", cmd.name).as_str());
            } else if category == "Uncategorized" {
                uncat_category.push_str(format!("- {}\n", cmd.name).as_str());
            }
        }
        response.push_str(format!("{util_category}\n").as_str());
        response.push_str(&uncat_category);

        response.push_str("```");
        ctx.reply(response).await?;
    }
    Ok(())
}
