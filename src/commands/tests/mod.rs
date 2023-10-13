#[cfg(test)]
mod test {
    use std::{
        fs::{self, File},
        io::Read,
        path::Path,
    };

    use ic_wasm::metadata::list_metadata;

    use crate::{
        commands::{build, generate},
        lib::{environment::EnvironmentImpl, logger::create_root_logger},
    };

    // NOTE: Currently only one pattern of template pj
    const TEST_PROJECT_PATH: &str = "e2e/resources/template";

    const METADATA_LABELS: [&str; 6] = [
        "icp:public chainsight:label",
        "icp:public chainsight:component_type",
        "icp:public chainsight:description",
        "icp:public chainsight:tags",
        "icp:public chainsight:sources",
        "icp:public chainsight:intervalSec",
    ];
    const METADATA_LABEL_FOR_DESTINATION: &'static str = "icp:public chainsight:destination";

    pub struct TestComponent {
        pub id: &'static str,
        pub metadata_labels: Vec<String>,
    }
    impl TestComponent {
        pub fn component_path(&self) -> String {
            format!("components/{}.yaml", self.id)
        }
    }

    pub fn algorthm_indexer() -> TestComponent {
        TestComponent {
            id: "sample_algorithm_indexer",
            metadata_labels: METADATA_LABELS.iter().map(|&s| s.to_string()).collect(),
        }
    }

    // pub fn algorithm_lens() -> TestComponent {
    //     TestComponent {
    //         id: "sample_algorithm_lens",
    //         metadata_labels: METADATA_LABELS.iter().map(|&s| s.to_string()).collect(),
    //     }
    // }

    pub fn event_indexer() -> TestComponent {
        TestComponent {
            id: "sample_event_indexer",
            metadata_labels: METADATA_LABELS.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn relayer() -> TestComponent {
        let metadata_labels = METADATA_LABELS
            .iter()
            .map(|&s| s.to_string())
            .chain(vec![METADATA_LABEL_FOR_DESTINATION.to_string()])
            .collect();

        TestComponent {
            id: "sample_relayer",
            metadata_labels,
        }
    }

    pub fn snapshot_indexer_evm() -> TestComponent {
        TestComponent {
            id: "sample_snapshot_indexer_evm",
            metadata_labels: METADATA_LABELS.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn snapshot_indexer_https() -> TestComponent {
        TestComponent {
            id: "sample_snapshot_indexer_https",
            metadata_labels: METADATA_LABELS.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn snapshot_indexer_icp() -> TestComponent {
        TestComponent {
            id: "sample_snapshot_indexer_icp",
            metadata_labels: METADATA_LABELS.iter().map(|&s| s.to_string()).collect(),
        }
    }

    pub fn minimum_project_folder(root_path: &str) -> anyhow::Result<()> {
        fs::create_dir_all(format!("{}/components", root_path))?;
        fs::create_dir_all(format!("{}/interfaces", root_path))?;

        fs::write(format!("{}/{}", root_path, ".chainsight"), "")?;

        Ok(())
    }

    pub fn generate_project_manifest(ids: &[String]) -> String {
        let component_rows = ids
            .iter()
            .map(|id| format!("- component_path: components/{}.yaml", id))
            .collect::<Vec<String>>()
            .join("\n");
        format!(
            r#"version: v1
label: project
components:
{}"#,
            component_rows
        )
    }

    pub fn get_manifest_path_in_resource(id: &str) -> String {
        format!("{}/{}.yaml", TEST_PROJECT_PATH, id)
    }

    fn pre_process(root_path: &str, component: &[&TestComponent]) -> anyhow::Result<()> {
        minimum_project_folder(root_path)?;

        let mut component_ids = Vec::<String>::new();
        for c in component {
            let src_path = format!("./{}", get_manifest_path_in_resource(c.id));
            let dst_path = format!("./{}/{}", root_path, &c.component_path());
            fs::copy(&src_path, &dst_path)?;

            component_ids.push(c.id.to_string());
        }

        fs::write(
            format!("{}/{}", root_path, "project.yaml"),
            generate_project_manifest(component_ids.as_slice()),
        )?;

        Ok(())
    }

    fn execute(root_path: &str) {
        // process
        let mute_logger = create_root_logger(-4);
        let env = EnvironmentImpl::new().with_logger(mute_logger);
        assert!(generate::exec(
            &env,
            generate::GenerateOpts::new(Some(root_path.to_string())),
        )
        .is_ok());
        assert!(build::exec(
            &env,
            build::BuildOpts::new(Some(root_path.to_string()), true),
        )
        .is_ok());
    }
    fn assert_artifacts(root_path: &str, components: &[&TestComponent]) {
        let artifacts_path = format!("{}/artifacts", root_path);
        assert!(Path::new(&format!("{}/dfx.json", artifacts_path)).is_file());

        for c in components {
            assert_per_component(&artifacts_path, c);
        }
    }
    fn assert_per_component(artifacts_path: &str, c: &TestComponent) {
        assert!(Path::new(&format!("{}/{}.did", artifacts_path, c.id)).is_file());
        let wasm_path = format!("{}/{}.wasm", artifacts_path, c.id);
        assert!(Path::new(&wasm_path).is_file());

        let mut wasm_bytes = Vec::<u8>::new();
        File::open(&wasm_path)
            .unwrap()
            .read_to_end(&mut wasm_bytes)
            .unwrap();
        let module =
            ic_wasm::utils::parse_wasm(&wasm_bytes, true).expect("Failed to parse wasm to module");
        assert!(are_equal_unordered(
            list_metadata(&module),
            c.metadata_labels.iter().map(AsRef::as_ref).collect(),
        ));
    }

    fn are_equal_unordered<T: Ord>(mut a: Vec<T>, mut b: Vec<T>) -> bool {
        if a.len() != b.len() {
            return false;
        }

        a.sort();
        b.sort();

        a == b
    }

    fn post_process(root_path: &str) -> anyhow::Result<()> {
        fs::remove_dir_all(&root_path)?;
        Ok(())
    }

    #[test]
    fn test() {
        let root_path: &str = "test__component_modules";

        let components = [
            &algorthm_indexer(),
            // &algorithm_lens(),
            &event_indexer(),
            &relayer(),
            &snapshot_indexer_evm(),
            &snapshot_indexer_https(),
            &snapshot_indexer_icp(),
        ];

        assert!(pre_process(root_path, &components).is_ok());
        execute(root_path);
        assert_artifacts(root_path, &components);
        // assert!(post_process(root_path).is_ok());
    }
}
