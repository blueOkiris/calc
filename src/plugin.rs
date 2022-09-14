/*
 * Author: Dylan Turner
 * Description:
 * - You can call external functions from .so files via the built-in 'call' function
 * - Under the hood, it uses this code to do so
 */

use std::path::Path;
use dirs::config_dir;
use dlopen_derive::WrapperApi;
use dlopen::{
    Error,
    wrapper::{
        Container, WrapperApi
    }
};
use crate::var::Var;

#[derive(WrapperApi)]
pub struct Plugin {
    execute: extern fn(vars: &Vec<Var>) -> Result<Var, String>
}

pub fn call_ext_fn(lib_name: &str, vars: &Vec<Var>) -> Result<Var, String> {
    let conf = config_dir();
    if conf.is_none() {
        Err(String::from("Config directory does not exist."))
    } else {
        let mut plugin_dir = conf.unwrap();
        plugin_dir.push("calc");
        plugin_dir.push("plugins");
        
        if Path::new(&plugin_dir).exists() {
            plugin_dir.push(format!("lib{}.so", lib_name));

            if Path::new(&plugin_dir).exists() {
                let plugin_res: Result<Container<Plugin>, Error> = unsafe {
                    Container::load(plugin_dir.to_str().unwrap())
                };
                match plugin_res {
                    Ok(plugin) => {
                        plugin.execute(vars)
                    }, Err(err) => Err(
                        format!("Failed to run library execute function - {}.", err)
                    )
                }
            } else {
                Err(String::from("Specified library does not exist."))
            }
        } else {
            Err(String::from("Plugin directory does not exist."))
        }
    }
} 

