use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/hello.proto")?;
    tonic_build::compile_protos("proto/auth.proto")?;
    tonic_build::compile_protos("proto/userposts.proto")?;
    tonic_build::compile_protos("proto/projects.proto")?;
    tonic_build::compile_protos("proto/languages.proto")?;
    tonic_build::compile_protos("proto/student.proto")?;
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("auth_descriptor.bin"))
        .compile_protos(&["proto/auth.proto"], &["proto"])
        .unwrap();
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("userposts_descriptor.bin"))
        .compile_protos(&["proto/userposts.proto"], &["proto"])
        .unwrap();
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("projects_descriptor.bin"))
        .compile_protos(&["proto/projects.proto"], &["proto"])
        .unwrap();
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("languages_descriptor.bin"))
        .compile_protos(&["proto/languages.proto"], &["proto"])
        .unwrap();
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("student_descriptor.bin"))
        .compile_protos(&["proto/student.proto"], &["proto"])
        .unwrap();
    Ok(())
}
