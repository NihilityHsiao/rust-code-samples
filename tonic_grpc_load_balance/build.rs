/// protobuf文件变动时自动执行build
use std::process::Command;

pub trait BuilderExt {
    fn with_serde(self, path: &[&str]) -> Self;
}

impl BuilderExt for tonic_build::Builder {
    fn with_serde(self, path: &[&str]) -> Self {
        path.iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(serde::Serialize, serde::Deserialize)]")
        })
    }
}

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_serde(&["PlatformType", "Msg"])
        .compile_protos(&["protos/messages.proto"], &["protos"])
        .expect("compile proto files success");

    Command::new("cargo")
        .arg("fmt")
        .output()
        .expect("cargo fmt success");

    println!("cargo: rerun-if-changed=tonic_grpc_load_balance/protos/messages.proto");
}
