/// Utilities for experiment name management
use crate::error::{BotError, Result};

/// Generate an experiment name from a project path and issue ID
/// Format: {project with / replaced by --}--{issue_id}
/// Example: "user/repo" + 123 -> "user--repo--123"
/// This uses double-dash to avoid ambiguity with single dashes in owner/repo names
pub fn generate_experiment_name(project: &str, issue_id: u64) -> String {
    format!("{}-{}", project.replace('/', "--"), issue_id)
}

/// Parse an experiment name to extract project path and issue ID
/// Reverses the format created by generate_experiment_name
pub fn parse_experiment_name(experiment_name: &str) -> Result<(String, u64)> {
    // Split from the right to get the last component (issue_id)
    let parts: Vec<&str> = experiment_name.rsplitn(2, '-').collect();
    
    if parts.len() != 2 {
        return Err(BotError::Internal(format!(
            "Invalid experiment name format: {}",
            experiment_name
        )));
    }

    let issue_id = parts[0].parse::<u64>().map_err(|_| {
        BotError::Internal(format!(
            "Invalid issue ID in experiment name: {}",
            parts[0]
        ))
    })?;

    // parts[1] contains the project with double-dashes, convert back to slashes
    // Format: owner--repo becomes owner/repo
    let project_with_dashes = parts[1];
    let project = project_with_dashes.replace("--", "/");

    Ok((project, issue_id))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_experiment_name() {
        assert_eq!(
            generate_experiment_name("user/repo", 123),
            "user--repo-123"
        );
        assert_eq!(
            generate_experiment_name("org/project-name", 456),
            "org--project-name-456"
        );
        assert_eq!(
            generate_experiment_name("my-org/my-repo", 789),
            "my-org--my-repo-789"
        );
    }

    #[test]
    fn test_parse_experiment_name() {
        let (project, issue_id) = parse_experiment_name("user--repo-123").unwrap();
        assert_eq!(project, "user/repo");
        assert_eq!(issue_id, 123);

        let (project, issue_id) = parse_experiment_name("org--project-name-456").unwrap();
        assert_eq!(project, "org/project-name");
        assert_eq!(issue_id, 456);
    }

    #[test]
    fn test_parse_experiment_name_with_dashes() {
        // Test project names with dashes in owner and repo names
        let (project, issue_id) = parse_experiment_name("my-org--my-cool-project-789").unwrap();
        assert_eq!(project, "my-org/my-cool-project");
        assert_eq!(issue_id, 789);
    }

    #[test]
    fn test_roundtrip() {
        let test_cases = vec![
            ("user/repo", 123),
            ("org/project-name", 456),
            ("owner/my-cool-project", 789),
            ("my-org/my-repo", 999),
            ("complex-owner/complex-repo-name", 111),
        ];

        for (original_project, original_issue_id) in test_cases {
            let experiment_name = generate_experiment_name(original_project, original_issue_id);
            let (parsed_project, parsed_issue_id) = parse_experiment_name(&experiment_name).unwrap();

            assert_eq!(parsed_project, original_project, "Roundtrip failed for project: {}", original_project);
            assert_eq!(parsed_issue_id, original_issue_id);
        }
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!(parse_experiment_name("invalid").is_err());
        assert!(parse_experiment_name("no-number-abc").is_err());
    }
}
