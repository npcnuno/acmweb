use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for (proto, descriptor) in [
        ("userposts", "userposts_descriptor.bin"),
        ("auth", "auth_descriptor.bin"),
        ("projects", "projects_descriptor.bin"),
        ("student", "student_descriptor.bin"),
    ] {
        tonic_build::configure()
            .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
            .file_descriptor_set_path(out_dir.join(descriptor))
            .compile_protos(&[format!("protos/{}.proto", proto)], &["proto"])
            .unwrap();
    }

    Ok(())
}
