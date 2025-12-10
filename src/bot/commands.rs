use crate::error::{BotError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BotCommand {
    Run {
        toolchains: Vec<String>,
    },
    Status,
    Abort,
    Help,
    List,
}

impl BotCommand {
    pub fn parse(text: &str, trigger_prefix: &str) -> Result<Option<Self>> {
        let text = text.trim();
        
        // Check if this is a bot command
        if !text.starts_with(trigger_prefix) {
            return Ok(None);
        }

        // Remove the trigger prefix
        let command_text = text[trigger_prefix.len()..].trim();
        
        if command_text.is_empty() {
            return Ok(Some(BotCommand::Help));
        }

        let parts: Vec<&str> = command_text.split_whitespace().collect();
        
        match parts[0].to_lowercase().as_str() {
            "run" => {
                if parts.len() < 3 {
                    return Err(BotError::InvalidCommand(
                        "run 命令需要至少两个工具链参数。用法: @crater-bot run <toolchain1> <toolchain2>".to_string()
                    ));
                }
                let toolchains = parts[1..].iter().map(|s| s.to_string()).collect();
                Ok(Some(BotCommand::Run { toolchains }))
            }
            "status" => Ok(Some(BotCommand::Status)),
            "abort" => Ok(Some(BotCommand::Abort)),
            "help" => Ok(Some(BotCommand::Help)),
            "list" => Ok(Some(BotCommand::List)),
            _ => Err(BotError::InvalidCommand(format!(
                "未知命令: {}. 使用 'help' 查看可用命令",
                parts[0]
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_run_command() {
        let cmd = BotCommand::parse("@crater-bot run stable beta", "@crater-bot").unwrap();
        assert_eq!(
            cmd,
            Some(BotCommand::Run {
                toolchains: vec!["stable".to_string(), "beta".to_string()]
            })
        );
    }

    #[test]
    fn test_parse_run_command_with_nightly() {
        let cmd = BotCommand::parse("@crater-bot run nightly-2024-01-01 stable", "@crater-bot").unwrap();
        assert_eq!(
            cmd,
            Some(BotCommand::Run {
                toolchains: vec!["nightly-2024-01-01".to_string(), "stable".to_string()]
            })
        );
    }

    #[test]
    fn test_parse_status_command() {
        let cmd = BotCommand::parse("@crater-bot status", "@crater-bot").unwrap();
        assert_eq!(cmd, Some(BotCommand::Status));
    }

    #[test]
    fn test_parse_abort_command() {
        let cmd = BotCommand::parse("@crater-bot abort", "@crater-bot").unwrap();
        assert_eq!(cmd, Some(BotCommand::Abort));
    }

    #[test]
    fn test_parse_help_command() {
        let cmd = BotCommand::parse("@crater-bot help", "@crater-bot").unwrap();
        assert_eq!(cmd, Some(BotCommand::Help));
    }

    #[test]
    fn test_parse_list_command() {
        let cmd = BotCommand::parse("@crater-bot list", "@crater-bot").unwrap();
        assert_eq!(cmd, Some(BotCommand::List));
    }

    #[test]
    fn test_parse_non_bot_comment() {
        let cmd = BotCommand::parse("This is a regular comment", "@crater-bot").unwrap();
        assert_eq!(cmd, None);
    }

    #[test]
    fn test_parse_invalid_command() {
        let result = BotCommand::parse("@crater-bot invalid", "@crater-bot");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_run_command_insufficient_args() {
        let result = BotCommand::parse("@crater-bot run stable", "@crater-bot");
        assert!(result.is_err());
    }
}
