use moon_cli::enums::TouchedStatus;
use moon_cli::queries::projects::QueryProjectsResult;
use moon_cli::queries::touched_files::QueryTouchedFilesResult;
use moon_utils::string_vec;
use moon_utils::test::{
    create_fixtures_sandbox, create_moon_command_in, get_assert_output, run_git_command,
};

mod projects {
    use super::*;

    #[test]
    fn returns_all_by_default() {
        let fixture = create_fixtures_sandbox("projects");

        let assert = create_moon_command_in(fixture.path())
            .arg("query")
            .arg("projects")
            .assert();

        let json: QueryProjectsResult = serde_json::from_str(&get_assert_output(&assert)).unwrap();
        let ids: Vec<String> = json.projects.iter().map(|p| p.id.clone()).collect();

        assert_eq!(
            ids,
            string_vec![
                "advanced",
                "bar",
                "basic",
                "baz",
                "emptyConfig",
                "foo",
                "noConfig",
                "tasks"
            ]
        );
    }

    #[test]
    fn can_filter_by_id() {
        let fixture = create_fixtures_sandbox("projects");

        let assert = create_moon_command_in(fixture.path())
            .arg("query")
            .arg("projects")
            .args(["--id", "ba(r|z)"])
            .assert();

        let json: QueryProjectsResult = serde_json::from_str(&get_assert_output(&assert)).unwrap();
        let ids: Vec<String> = json.projects.iter().map(|p| p.id.clone()).collect();

        assert_eq!(ids, string_vec!["bar", "baz"]);
        assert_eq!(json.options.id.unwrap(), "ba(r|z)".to_string());
    }

    #[test]
    fn can_filter_by_source() {
        let fixture = create_fixtures_sandbox("projects");

        let assert = create_moon_command_in(fixture.path())
            .arg("query")
            .arg("projects")
            .args(["--source", "config$"])
            .assert();

        let json: QueryProjectsResult = serde_json::from_str(&get_assert_output(&assert)).unwrap();
        let ids: Vec<String> = json.projects.iter().map(|p| p.id.clone()).collect();

        assert_eq!(ids, string_vec!["emptyConfig", "noConfig"]);
        assert_eq!(json.options.source.unwrap(), "config$".to_string());
    }

    #[test]
    fn can_filter_by_tasks() {
        let fixture = create_fixtures_sandbox("projects");

        let assert = create_moon_command_in(fixture.path())
            .arg("query")
            .arg("projects")
            .args(["--tasks", "lint"])
            .assert();

        let json: QueryProjectsResult = serde_json::from_str(&get_assert_output(&assert)).unwrap();
        let ids: Vec<String> = json.projects.iter().map(|p| p.id.clone()).collect();

        assert_eq!(ids, string_vec!["tasks"]);
        assert_eq!(json.options.tasks.unwrap(), "lint".to_string());
    }
}

mod touched_files {
    use super::*;

    #[test]
    fn can_change_options() {
        let fixture = create_fixtures_sandbox("cases");

        run_git_command(fixture.path(), "Failed to create branch", |cmd| {
            cmd.args(["checkout", "-b", "branch"]);
        });

        let assert = create_moon_command_in(fixture.path())
            .arg("query")
            .arg("touched-files")
            .args([
                "--base",
                "master",
                "--head",
                "branch",
                "--status",
                "deleted",
                "--upstream",
            ])
            .assert();

        let json: QueryTouchedFilesResult =
            serde_json::from_str(&get_assert_output(&assert)).unwrap();

        assert_eq!(json.options.base, "master".to_string());
        assert_eq!(json.options.head, "branch".to_string());
        assert_eq!(json.options.status, TouchedStatus::Deleted);
        assert!(json.options.upstream);
    }
}
