mod utils;

use insta::assert_snapshot;
use moon_utils::test::{create_fixtures_sandbox, create_moon_command_in, get_assert_output};
use predicates::prelude::*;
use serial_test::serial;
use std::fs::read_to_string;
use utils::{append_workspace_config, get_path_safe_output, update_version_workspace_config};

#[test]
fn runs_package_managers() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:npm")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn runs_standard_script() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:standard")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn runs_cjs_files() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:cjs")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn runs_mjs_files() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:mjs")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn supports_top_level_await() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:topLevelAwait")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn handles_process_exit_zero() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:processExitZero")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn handles_process_exit_nonzero() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:processExitNonZero")
        .assert();

    if cfg!(windows) {
        assert.code(1);
    } else {
        assert_snapshot!(get_assert_output(&assert));
    }
}

#[test]
fn handles_process_exit_code_zero() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:exitCodeZero")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn handles_process_exit_code_nonzero() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:exitCodeNonZero")
        .assert();

    if cfg!(windows) {
        assert.code(1);
    } else {
        assert_snapshot!(get_assert_output(&assert));
    }
}

#[test]
fn handles_throw_error() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:throwError")
        .assert();
    let output = get_assert_output(&assert);

    // Output contains file paths that we cant snapshot
    assert!(predicate::str::contains("Error: Oops").eval(&output));
}

#[test]
fn handles_unhandled_promise() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:unhandledPromise")
        .assert();

    if cfg!(windows) {
        assert.code(1);
    } else {
        assert_snapshot!(get_path_safe_output(&assert, fixture.path()));
    }
}

#[test]
fn passes_args_through() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:passthroughArgs")
        .arg("--")
        .arg("-aBc")
        .arg("--opt")
        .arg("value")
        .arg("--optCamel=value")
        .arg("foo")
        .arg("'bar baz'")
        .arg("--opt-kebab")
        .arg("123")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn sets_env_vars() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:envVars")
        .assert();

    assert_snapshot!(get_assert_output(&assert));
}

#[test]
fn inherits_moon_env_vars() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:envVarsMoon")
        .assert();

    assert_snapshot!(get_path_safe_output(&assert, fixture.path()));
}

#[test]
fn runs_from_project_root() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:runFromProject")
        .assert();

    assert_snapshot!(get_path_safe_output(&assert, fixture.path()));
}

#[test]
fn runs_from_workspace_root() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:runFromWorkspace")
        .assert();

    assert_snapshot!(get_path_safe_output(&assert, fixture.path()));
}

#[test]
fn retries_on_failure_till_count() {
    let fixture = create_fixtures_sandbox("cases");

    let assert = create_moon_command_in(fixture.path())
        .arg("run")
        .arg("node:retryCount")
        .assert();
    let output = get_assert_output(&assert);

    assert!(predicate::str::contains("Process ~/.moon/tools/node/16.0.0").eval(&output));
}

mod install_deps {
    use super::*;

    #[test]
    fn installs_on_first_run() {
        let fixture = create_fixtures_sandbox("cases");

        assert!(!fixture.path().join("node_modules").exists());

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .env_remove("MOON_TEST_HIDE_INSTALL_OUTPUT")
            .assert();
        let output = get_assert_output(&assert);

        assert!(fixture.path().join("node_modules").exists());

        assert!(predicate::str::contains("added 7 packages").eval(&output));
    }

    #[test]
    fn doesnt_reinstall_on_second_run() {
        let fixture = create_fixtures_sandbox("cases");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .env_remove("MOON_TEST_HIDE_INSTALL_OUTPUT")
            .assert();
        let output1 = get_assert_output(&assert);

        assert!(predicate::str::contains("added 7 packages").eval(&output1));

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .env_remove("MOON_TEST_HIDE_INSTALL_OUTPUT")
            .assert();
        let output2 = get_assert_output(&assert);

        assert!(!predicate::str::contains("added 7 packages").eval(&output2));
    }

    #[test]
    fn creates_workspace_state_cache() {
        let fixture = create_fixtures_sandbox("cases");

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .assert();

        assert!(fixture
            .path()
            .join(".moon/cache/workspaceState.json")
            .exists());
    }
}

mod engines {
    use super::*;

    #[test]
    fn adds_engines_constraint() {
        let fixture = create_fixtures_sandbox("cases");

        append_workspace_config(
            &fixture.path().join(".moon/workspace.yml"),
            r#"  addEnginesConstraint: true"#,
        );

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .assert();

        assert_snapshot!(read_to_string(fixture.path().join("package.json")).unwrap());
    }

    #[test]
    fn doesnt_add_engines_constraint() {
        let fixture = create_fixtures_sandbox("cases");

        append_workspace_config(
            &fixture.path().join(".moon/workspace.yml"),
            r#"  addEnginesConstraint: false"#,
        );

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .assert();

        assert_snapshot!(read_to_string(fixture.path().join("package.json")).unwrap());
    }
}

mod version_manager {
    use super::*;

    #[test]
    fn adds_no_file_by_default() {
        let fixture = create_fixtures_sandbox("cases");

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .assert();

        assert!(!fixture.path().join(".nvmrc").exists());
        assert!(!fixture.path().join(".node-version").exists());
    }

    #[test]
    fn adds_nvmrc_file() {
        let fixture = create_fixtures_sandbox("cases");

        append_workspace_config(
            &fixture.path().join(".moon/workspace.yml"),
            r#"  syncVersionManagerConfig: nvm"#,
        );

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .assert();

        assert!(fixture.path().join(".nvmrc").exists());

        assert_eq!(
            read_to_string(fixture.path().join(".nvmrc")).unwrap(),
            "16.0.0"
        );
    }

    #[test]
    fn adds_nodenv_file() {
        let fixture = create_fixtures_sandbox("cases");

        append_workspace_config(
            &fixture.path().join(".moon/workspace.yml"),
            r#"  syncVersionManagerConfig: nodenv"#,
        );

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .assert();

        assert!(fixture.path().join(".node-version").exists());

        assert_eq!(
            read_to_string(fixture.path().join(".node-version")).unwrap(),
            "16.0.0"
        );
    }

    #[test]
    fn errors_for_invalid_value() {
        let fixture = create_fixtures_sandbox("cases");

        append_workspace_config(
            &fixture.path().join(".moon/workspace.yml"),
            r#"  syncVersionManagerConfig: invalid"#,
        );

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("node:standard")
            .assert();

        let output = get_assert_output(&assert);

        assert!(predicate::str::contains(
            "unknown variant: found `invalid`, expected ``nodenv` or `nvm``"
        )
        .eval(&output));
    }
}

mod sync_depends_on {
    use super::*;

    #[test]
    fn syncs_as_dependency_to_package_json() {
        let fixture = create_fixtures_sandbox("cases");

        append_workspace_config(
            &fixture.path().join(".moon/workspace.yml"),
            "  syncProjectWorkspaceDependencies: true",
        );

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("dependsOn:standard")
            .assert();

        // deps-c does not have a `package.json` on purpose
        assert_snapshot!(read_to_string(fixture.path().join("depends-on/package.json")).unwrap());
    }

    #[test]
    fn syncs_as_reference_to_tsconfig_json() {
        let fixture = create_fixtures_sandbox("cases");

        append_workspace_config(
            &fixture.path().join(".moon/workspace.yml"),
            "typescript:\n  syncProjectReferences: true",
        );

        create_moon_command_in(fixture.path())
            .arg("run")
            .arg("dependsOn:standard")
            .assert();

        // root
        assert_snapshot!(read_to_string(fixture.path().join("tsconfig.json")).unwrap());

        // project
        // deps-a does not have a `tsconfig.json` on purpose
        assert_snapshot!(read_to_string(fixture.path().join("depends-on/tsconfig.json")).unwrap());
    }
}

mod npm {
    use super::*;

    #[test]
    #[serial]
    fn installs_correct_version() {
        let fixture = create_fixtures_sandbox("node-npm");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("npm:version")
            .assert();

        assert_snapshot!(get_assert_output(&assert));
    }

    // NOTE: This fails on Windows for some reason...
    #[cfg(not(windows))]
    #[test]
    #[serial]
    fn installs_correct_version_using_corepack() {
        let fixture = create_fixtures_sandbox("node-npm");

        // Corepack released in v16.9
        update_version_workspace_config(fixture.path(), "16.1.0", "16.10.0");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("npm:version")
            .assert();

        assert_snapshot!(get_assert_output(&assert));
    }

    #[test]
    #[serial]
    fn can_install_a_dep() {
        let fixture = create_fixtures_sandbox("node-npm");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("npm:installDep")
            .assert();

        assert.success();
    }
}

mod pnpm {
    use super::*;

    #[test]
    #[serial]
    fn installs_correct_version() {
        let fixture = create_fixtures_sandbox("node-pnpm");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("pnpm:version")
            .assert();

        assert_snapshot!(get_assert_output(&assert));
    }

    #[test]
    #[serial]
    fn installs_correct_version_using_corepack() {
        let fixture = create_fixtures_sandbox("node-pnpm");

        // Corepack released in v16.9
        update_version_workspace_config(fixture.path(), "16.2.0", "16.11.0");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("pnpm:version")
            .assert();

        assert_snapshot!(get_assert_output(&assert));
    }

    #[test]
    #[serial]
    fn can_install_a_dep() {
        let fixture = create_fixtures_sandbox("node-pnpm");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("pnpm:installDep")
            .assert();

        assert.success();
    }
}

mod yarn1 {
    use super::*;

    #[test]
    #[serial]
    fn installs_correct_version() {
        let fixture = create_fixtures_sandbox("node-yarn1");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("yarn:version")
            .assert();

        assert_snapshot!(get_assert_output(&assert));
    }

    #[test]
    #[serial]
    fn installs_correct_version_using_corepack() {
        let fixture = create_fixtures_sandbox("node-yarn1");

        // Corepack released in v16.9
        update_version_workspace_config(fixture.path(), "16.3.0", "16.12.0");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("yarn:version")
            .assert();

        assert_snapshot!(get_assert_output(&assert));
    }

    #[test]
    #[serial]
    fn can_install_a_dep() {
        let fixture = create_fixtures_sandbox("node-yarn1");

        let assert = create_moon_command_in(fixture.path())
            .arg("run")
            .arg("yarn:installDep")
            .assert();

        assert.success();
    }
}

// TODO: This fails in CI for some reason, but not locally...
// mod yarn {
//     use super::*;

//     #[test]
//     #[serial]
//     fn installs_correct_version() {
//         let fixture = create_fixtures_sandbox("node-yarn");

//         let assert = create_moon_command_in(fixture.path())
//             .arg("run")
//             .arg("yarn:version")
//             .assert();

//         assert_snapshot!(get_assert_output(&assert));
//     }

//     #[test]
//     #[serial]
//     fn installs_correct_version_using_corepack() {
//         let fixture = create_fixtures_sandbox("node-yarn");

//         // Corepack released in v16.9
//         update_version_workspace_config(fixture.path(), "16.4.0", "16.13.0");

//         let assert = create_moon_command_in(fixture.path())
//             .arg("run")
//             .arg("yarn:version")
//             .assert();

//         assert_snapshot!(get_assert_output(&assert));
//     }

//     #[test]
//     #[serial]
//     fn can_install_a_dep() {
//         let fixture = create_fixtures_sandbox("node-yarn");

//         let assert = create_moon_command_in(fixture.path())
//             .arg("run")
//             .arg("yarn:installDep")
//             .assert();

//         assert.success();
//     }
// }
