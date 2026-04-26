// build.rs

fn main()
{
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=source/application/backend/windows/resources.rc");
        println!("cargo:rerun-if-changed=source/application/backend/windows/icon.ico");

        match embed_resource::compile("source/application/backend/windows/resources.rc", embed_resource::NONE).manifest_optional() {
			Ok(_) => { }
			Err(error) => {
                println!("cargo:warning=Failed to compile resources: {}", error);
			}
		}
    }
}