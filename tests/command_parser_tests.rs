use crater_ohos_bot::bot::BotCommand;

#[test]
fn test_parse_run_command() {
    let cmd = BotCommand::parse("@crater-bot run stable beta", "@crater-bot").unwrap();
    if let Some(BotCommand::Run { toolchains }) = cmd {
        assert_eq!(toolchains, vec!["stable", "beta"]);
    } else {
        panic!("Expected Some(BotCommand::Run {{ .. }})");
    }
}

#[test]
fn test_parse_run_command_with_nightly() {
    let cmd = BotCommand::parse("@crater-bot run nightly-2024-01-01 stable", "@crater-bot").unwrap();
    if let Some(BotCommand::Run { toolchains }) = cmd {
        assert_eq!(toolchains, vec!["nightly-2024-01-01", "stable"]);
    } else {
        panic!("Expected Some(BotCommand::Run {{ .. }})");
    }
}

#[test]
fn test_parse_multiple_toolchains() {
    let cmd = BotCommand::parse("@crater-bot run stable beta nightly", "@crater-bot").unwrap();
    if let Some(BotCommand::Run { toolchains }) = cmd {
        assert_eq!(toolchains, vec!["stable", "beta", "nightly"]);
    } else {
        panic!("Expected Some(BotCommand::Run {{ .. }})");
    }
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
fn test_parse_empty_command() {
    let cmd = BotCommand::parse("@crater-bot", "@crater-bot").unwrap();
    assert_eq!(cmd, Some(BotCommand::Help));
}

#[test]
fn test_parse_whitespace_after_trigger() {
    let cmd = BotCommand::parse("@crater-bot    status", "@crater-bot").unwrap();
    assert_eq!(cmd, Some(BotCommand::Status));
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

#[test]
fn test_parse_run_command_no_args() {
    let result = BotCommand::parse("@crater-bot run", "@crater-bot");
    assert!(result.is_err());
}

#[test]
fn test_case_insensitive_commands() {
    let cmd = BotCommand::parse("@crater-bot STATUS", "@crater-bot").unwrap();
    assert_eq!(cmd, Some(BotCommand::Status));
    
    let cmd = BotCommand::parse("@crater-bot Help", "@crater-bot").unwrap();
    assert_eq!(cmd, Some(BotCommand::Help));
}
