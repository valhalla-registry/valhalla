pub mod builder;
mod cargo_metadata;
mod db;
mod details;
pub mod error;
pub mod metadata;
mod util;

// use std::{
//     fs::File,
//     path::{Path, PathBuf},
//     process::Command,
//     thread::JoinHandle,
// };

// use crossbeam::channel::{unbounded, Receiver, Sender};
// use flate2::read::GzDecoder;
// use semver::Version;
// use tar::Archive;

// pub struct RustdocBuilder {
//     pub source_path: PathBuf,
//     pub output_path: PathBuf,
//     pub queue: Sender<DocBuildJob>,
//     pub builder_thread_handle: JoinHandle<()>,
// }

// impl RustdocBuilder {
//     pub fn init(source_path: PathBuf, output_path: PathBuf) -> Self {
//         // this is the folder where the documentation will be built
//         let build_directory = Path::new("/opt/valhall/doc_build_dir");
//         // create the folder if it does not exist yet
//         std::fs::create_dir_all(build_directory).unwrap();

//         let (s, r): (Sender<DocBuildJob>, Receiver<DocBuildJob>) = unbounded();
//         let handle = std::thread::spawn(move || loop {
//             let job: DocBuildJob = r.recv().unwrap();

//             // steps:
//             // - extract .crate file in docs build directory
//             // - run cargo doc
//             // - copy output into separate directory

//             // extract tarball
//             let file = File::open(&job.crate_file_path).unwrap();
//             let tar = GzDecoder::new(file);
//             Archive::new(tar).unpack(&build_directory).unwrap();

//             let wd = build_directory.join(format!("{}-{}", &job.crate_name, &job.crate_version));

//             // cargo update
//             run_cargo_update(&wd);
//             // cargo doc
//             run_cargo_doc(&wd);
//         });

//         Self {
//             source_path,
//             output_path,
//             queue: s,
//             builder_thread_handle: handle,
//         }
//     }

//     pub fn add_to_queue(&self, crate_name: String, version: Version) {
//         let crate_file_path = self
//             .source_path
//             .join(&crate_name)
//             .join(format!("{}-{}.crate", &crate_name, &version));

//         self.queue
//             .send(DocBuildJob {
//                 crate_name,
//                 crate_version: version,
//                 crate_file_path,
//             })
//             .unwrap();
//     }
// }

// pub struct DocBuildJob {
//     pub crate_name: String,
//     pub crate_version: Version,
//     pub crate_file_path: PathBuf,
// }

// fn run_cargo_update(wd: &Path) {
//     Command::new("cargo")
//         .arg("update")
//         // .arg("--no-deps")
//         // .arg("--manifest-path")
//         // .arg("Cargo.toml")
//         .current_dir(wd)
//         .spawn()
//         .unwrap()
//         .wait()
//         .unwrap();
// }

// fn run_cargo_doc(wd: &Path) {
//     Command::new("cargo")
//         .arg("doc")
//         .arg("--no-deps")
//         .arg("--manifest-path")
//         .arg("Cargo.toml")
//         .current_dir(wd)
//         .spawn()
//         .unwrap()
//         .wait()
//         .unwrap();
// }
