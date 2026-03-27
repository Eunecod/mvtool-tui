// build.rs

fn main()
{
    let _ = embed_resource::compile("res/resources.rc", embed_resource::NONE);
}