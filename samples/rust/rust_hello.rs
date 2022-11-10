#![no_std]

use kernel::prelude::*;

module! {
    type: HelloWorld,
    name: b"hello_world",
    description: b"A simple hello world example",
    license: b"GPL v2",
}

struct HelloWorld;

impl kernel::Module for HelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");
        pr_info!("Hello world from rust!\n");
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");
        pr_info!("--------------------------------------\n");

        Ok(HelloWorld)
    }
}

impl Drop for HelloWorld {
    fn drop(&mut self) {
        pr_info!("Bye world from rust!\n");
    }
}