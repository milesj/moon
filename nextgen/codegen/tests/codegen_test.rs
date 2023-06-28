use moon_codegen::CodeGenerator;
use moon_common::Id;
use moon_config::{FilePath, GeneratorConfig};
use starbase_sandbox::{create_empty_sandbox, create_sandbox};

mod codegen {
    use super::*;

    mod create_template {
        use super::*;

        #[test]
        #[should_panic(expected = "A template with the name standard already exists")]
        fn errors_if_already_exists() {
            let sandbox = create_empty_sandbox();
            sandbox.create_file("templates/standard/file", "");

            CodeGenerator::new(sandbox.path(), &GeneratorConfig::default())
                .create_template("standard")
                .unwrap();
        }

        #[test]
        fn creates_the_template() {
            let sandbox = create_empty_sandbox();

            let template = CodeGenerator::new(sandbox.path(), &GeneratorConfig::default())
                .create_template("new-template")
                .unwrap();

            assert!(sandbox.path().join("templates/new-template").exists());
            assert!(sandbox
                .path()
                .join("templates/new-template/template.yml")
                .exists());

            assert_eq!(template.id, "new-template");
            assert_eq!(template.root, sandbox.path().join("templates/new-template"));
        }

        #[test]
        fn creates_the_template_from_another_dir() {
            let sandbox = create_empty_sandbox();

            let template = CodeGenerator::new(
                sandbox.path(),
                &GeneratorConfig {
                    templates: vec![FilePath("./scaffolding".to_owned())],
                },
            )
            .create_template("new-template")
            .unwrap();

            assert!(sandbox.path().join("scaffolding/new-template").exists());
            assert!(sandbox
                .path()
                .join("scaffolding/new-template/template.yml")
                .exists());

            assert_eq!(template.id, "new-template");
            assert_eq!(
                template.root,
                sandbox.path().join("scaffolding/new-template")
            );
        }

        #[test]
        fn cleans_and_formats_the_name() {
            let sandbox = create_empty_sandbox();

            let template = CodeGenerator::new(sandbox.path(), &GeneratorConfig::default())
                .create_template("so&me temPlatE- with Ran!dom-Valu^es 123_")
                .unwrap();

            assert!(sandbox
                .path()
                .join("templates/so-me-temPlatE--with-Ran-dom-Valu-es-123_")
                .exists());
            assert!(sandbox
                .path()
                .join("templates/so-me-temPlatE--with-Ran-dom-Valu-es-123_/template.yml")
                .exists());

            assert_eq!(template.id, "so-me-temPlatE--with-Ran-dom-Valu-es-123_");
            assert_eq!(
                template.root,
                sandbox
                    .path()
                    .join("templates/so-me-temPlatE--with-Ran-dom-Valu-es-123_")
            );
        }
    }

    mod load_template {
        use super::*;

        #[test]
        fn loads_by_name() {
            let sandbox = create_sandbox("generator");

            let template = CodeGenerator::new(sandbox.path(), &GeneratorConfig::default())
                .load_template("one")
                .unwrap();

            assert_eq!(template.id, Id::raw("one"));
            assert_eq!(template.root, sandbox.path().join("templates/one"));
        }

        #[test]
        #[should_panic(expected = "No template with the name three could not be found")]
        fn errors_for_missing() {
            let sandbox = create_sandbox("generator");

            CodeGenerator::new(sandbox.path(), &GeneratorConfig::default())
                .load_template("three")
                .unwrap();
        }
    }
}
