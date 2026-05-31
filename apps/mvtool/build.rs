// apps/mvtool/build.rs

fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=src/platforms/windows/resources.rc");
        println!("cargo:rerun-if-changed=src/platforms/windows/icon.ico");

        match embed_resource::compile("src/platforms/windows/resources.rc", embed_resource::NONE)
            .manifest_optional()
        {
            Ok(_) => (),
            Err(error) => {
                println!("cargo:warning=Failed to compile resources: {}", error);
            }
        }
    }
}
