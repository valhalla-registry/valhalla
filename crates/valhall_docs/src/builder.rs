use std::{path::Path, time::Duration};

use rustwide::{
    cmd::{Command, SandboxBuilder, SandboxImage},
    AlternativeRegistry, Build, Crate, Toolchain, Workspace, WorkspaceBuilder,
};

use crate::{
    cargo_metadata::CargoMetadata,
    error::Error,
    metadata::{BuildTargets, Metadata},
    util::copy_dir_all,
};

const USER_AGENT: &str = "valhall docs builder";
const DUMMY_CRATE_NAME: &str = "empty-library";
const DUMMY_CRATE_VERSION: &str = "1.0.0";

fn build_workspace() -> Result<Workspace, Error> {
    let builder = WorkspaceBuilder::new(Path::new("/opt/valhall/workspace"), USER_AGENT)
        .running_inside_docker(true)
        .sandbox_image(SandboxImage::remote(
            "ghcr.io/rust-lang/crates-build-env/linux-micro",
        )?);

    let workspace = builder.init()?;
    workspace.purge_all_build_dirs()?;

    Ok(workspace)
}

pub struct DocBuilder {
    workspace: Workspace,
    toolchain: Toolchain,
}

impl DocBuilder {
    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let workspace = build_workspace()?;
        let toolchain = Toolchain::dist("nightly");
        toolchain.install(&workspace)?;

        Ok(Self {
            workspace,
            toolchain,
        })
    }

    fn prepare_sandbox(&self) -> SandboxBuilder {
        SandboxBuilder::new()
        // .cpu_limit(self.config.build_cpu_limit.map(|limit| limit as f32))
        // .memory_limit(Some(limits.memory()))
        // .enable_networking(limits.networking())
    }

    pub fn build_package(&mut self, name: &str, version: &str) {
        tracing::trace!("building docs for crate {} ({})", name, version);

        let mut build_dir = self.workspace.build_dir(&format!("{name}-{version}"));
        let krate = {
            // TODO: somehow add api token to the sandboxed cargo
            let registry = AlternativeRegistry::new("https://github.com/lucalewin/valhall-index");
            let krate = Crate::registry(registry, name, version);
            krate.fetch(&self.workspace).unwrap();
            krate
        };

        let successful = build_dir
            .build(&self.toolchain, &krate, self.prepare_sandbox())
            .run(|build| {
                let metadata = Metadata::from_crate_root(build.host_source_dir())?;
                let BuildTargets {
                    default_target,
                    ..
                    //other_targets,
                } = metadata.targets(true);

                let cargo_metadata = CargoMetadata::load_from_rustwide(
                    &self.workspace,
                    &self.toolchain,
                    &build.host_source_dir(),
                )?;

                std::fs::create_dir_all("./temp")?;
                let local_storage = tempfile::tempdir_in("./temp")?;

                let res = self.execute_build(&build, default_target, &metadata);

                // check if the build generated docs
                let mut has_docs = false;
                if res {
                    if let Some(name) = cargo_metadata.root().library_name() {
                        let host_target = build.host_target_dir();
                        has_docs = host_target
                            .join(default_target)
                            .join("doc")
                            .join(name)
                            .is_dir();
                    }
                }

                tracing::info!(docs =? has_docs);

                // if the build generated docs, copy them to a temporary folder
                if has_docs {
                    tracing::debug!("adding documentation for the default target to the database");
                    self.copy_docs(
                        &build.host_target_dir(),
                        local_storage.path(),
                        default_target,
                        true,
                    )?;
                }

                copy_dir_all(
                    local_storage.path(),
                    format!("/opt/valhall/docs/{name}/{version}"),
                )?;

                return Ok(false);
            })
            .unwrap();
    }

    fn execute_build<'ws>(&self, build: &'ws Build, target: &str, metadata: &Metadata) -> bool {
        let cmd = self.prepare_command(build, target, metadata).unwrap(); // FIXME
        cmd.run().is_ok()
    }

    fn prepare_command<'ws, 'pl>(
        &self,
        build: &'ws Build,
        target: &str,
        metadata: &Metadata,
    ) -> Result<Command<'ws, 'pl>, Error> {
        let cargo_args: Vec<String> = vec![
            "--offline".into(),
            format!(
                // r#"--config=doc.extern-map.registries.crates-io="https://docs.rs/{{pkg_name}}/{{version}}/{target}""#
                r#"--config=doc.extern-map.registries.crates-io="https://docs.rs/{{pkg_name}}/{{version}}/{target}""#
            ),
            "--target".into(),
            target.into(),
        ];

        #[rustfmt::skip]
        let rustdoc_flags: Vec<String> = vec![
            "--static-root-path".into(), "/static/rustdoc/".into(),
            "--cap-lints".into(), "warn".into(),
            "--extern-html-root-takes-precedence".into(),
        ];

        let cargo_args = metadata.cargo_args(&cargo_args, &rustdoc_flags);

        let mut command = build.cargo().timeout(Some(Duration::from_secs(30))); // FIXME: timeout value

        for (key, val) in metadata.environment_variables() {
            command = command.env(key, val);
        }

        Ok(command.args(&cargo_args))
    }

    fn copy_docs(
        &self,
        target_dir: &Path,
        local_storage: &Path,
        target: &str,
        is_default_target: bool,
    ) -> Result<(), Error> {
        let source = target_dir.join(target).join("doc");

        let mut dest = local_storage.to_path_buf();
        // only add target name to destination directory when we are copying a non-default target.
        // this is allowing us to host documents in the root of the crate documentation directory.
        // for example winapi will be available in docs.rs/winapi/$version/winapi/ for it's
        // default target: x86_64-pc-windows-msvc. But since it will be built under
        // target/x86_64-pc-windows-msvc we still need target in this function.
        if !is_default_target {
            dest = dest.join(target);
        }

        tracing::info!("copy {} to {}", source.display(), dest.display());
        copy_dir_all(source, dest).map_err(Into::into)
    }
}
