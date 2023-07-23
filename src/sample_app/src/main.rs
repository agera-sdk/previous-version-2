#![feature(async_closure)]

mod ftl;

rialight::initialize!(async move |app| {
    let ftl = crate::ftl::create();
    if !ftl.load(None).await {
        return;
    }
    println!("{}", ftl.get_message("hello-world", None, &mut vec![]).unwrap());
});