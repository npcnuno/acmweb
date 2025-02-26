use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // userposts
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .file_descriptor_set_path(out_dir.join("userposts_descriptor.bin"))
        .compile(&["protos/userposts.proto"], &["proto"])
        .unwrap();

    // auth
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .file_descriptor_set_path(out_dir.join("auth_descriptor.bin"))
        .compile(&["protos/auth.proto"], &["proto"])
        .unwrap();

    // projects
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .file_descriptor_set_path(out_dir.join("projects_descriptor.bin"))
        .compile(&["protos/projects.proto"], &["proto"])
        .unwrap();

    // student
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize)]")
        .file_descriptor_set_path(out_dir.join("student_descriptor.bin"))
        .compile(&["protos/student.proto"], &["proto"])
        .unwrap();

    Ok(())
}
