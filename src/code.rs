use regex::RegexBuilder;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Code {
    language: Language,
    body: String,
    stdin: Option<String>,
}

impl Code {
    pub fn new(
        language: &str,
        body: impl Into<String>,
        stdin: Option<String>,
    ) -> Result<Self, EvalError> {
        Ok(Self {
            language: language.parse()?,
            body: body.into(),
            stdin,
        })
    }

    pub fn get_language(&self) -> Language {
        self.language
    }

    pub fn get_code(&self) -> String {
        self.body.clone()
    }

    pub fn get_stdin(&self) -> Option<String> {
        self.stdin.clone()
    }

    pub fn from_code_blocks(input: &str) -> Result<Self, EvalError> {
        let pattern =
            r"```+\s*(?P<language>\S*)\n(?P<code>.*?)```+(?:\s*```+\s*\n?(?P<stdin>.*?)```+)?";

        let re = RegexBuilder::new(pattern)
            .dot_matches_new_line(true)
            .build()?;

        let caps = re.captures(input).ok_or(EvalError::ParseFailed)?;

        let language = caps
            .name("language")
            .map_or_else(|| "invalid", |m| m.as_str());

        let code = caps
            .name("code")
            .map_or_else(String::new, |m| m.as_str().to_string());

        let stdin = caps.name("stdin").map(|m| m.as_str().to_string());

        Self::new(language, code, stdin)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    Python,
    Go,
    Brainfuck,
    Rust,
    C,
    Cpp,
    Bash,
    Java,
    Lua,
    JavaScript,
}

impl Language {
    pub fn name(self) -> String {
        let name = match self {
            Self::Python => "python",
            Self::Go => "go",
            Self::Brainfuck => "brainfuck",
            Self::Rust => "rust",
            Self::C => "c",
            Self::Cpp => "cpp",
            Self::Bash => "bash",
            Self::Java => "java",
            Self::Lua => "lua",
            Self::JavaScript => "javascript",
        };
        name.to_string()
    }
}

impl FromStr for Language {
    type Err = EvalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &s.to_lowercase();
        match s.as_str() {
            "python" | "py" => Ok(Self::Python),
            "rust" | "rs" => Ok(Self::Rust),
            "golang" | "go" => Ok(Self::Go),
            "brainfuck" | "bf" => Ok(Self::Brainfuck),
            "c" => Ok(Self::C),
            "cpp" => Ok(Self::Cpp),
            "bash" => Ok(Self::Bash),
            "java" => Ok(Self::Java),
            "lua" => Ok(Self::Lua),
            "js" | "javascript" => Ok(Self::JavaScript),
            unsuported => Err(EvalError::UnsupportedLanguage(unsuported.to_string())),
        }
    }
}

#[derive(Debug, Error)]
pub enum EvalError {
    #[error("Used unsuported language: {0}")]
    UnsupportedLanguage(String),
    #[error("Failed to parse code block")]
    ParseFailed,
    #[error("Failed to build regex")]
    BuildFailed(#[from] regex::Error),
    #[error("Correct usage example: \n!e \'\'\'py\n\nprint('Hello, world!')")]
    EmptyCodeBlock,
}
