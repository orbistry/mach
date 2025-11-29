mod cli;
mod cmd;
pub mod entity;
pub mod service;
pub mod tui;

pub use cli::Cli;

pub const BANNER: &str = color_print::cstr! {
r#"
      ___           ___           ___           ___     
     /\__\         /\  \         /\  \         /\__\    
    /::|  |       /::\  \       /::\  \       /:/  /    
   /:|:|  |      /:/\:\  \     /:/\:\  \     /:/__/     
  /:/|:|__|__   /::\~\:\  \   /:/  \:\  \   /::\  \ ___ 
 /:/ |::::\__\ /:/\:\ \:\__\ /:/__/ \:\__\ /:/\:\  /\__\
 \/__/~~/:/  / \/__\:\/:/  / \:\  \  \/__/ \/__\:\/:/  /
       /:/  /       \::/  /   \:\  \            \::/  / 
      /:/  /        /:/  /     \:\  \           /:/  /  
     /:/  /        /:/  /       \:\__\         /:/  /   
     \/__/         \/__/         \/__/         \/__/

 Do more with <green><bold>Mach</bold></green> CLI.

 <magenta>repo:</magenta> <blue><italic><dim>https://github.com/rvcas/mach</dim></italic></blue>
 <magenta>docs:</magenta> <blue><italic><dim>https://machich.co</dim></italic></blue>
 <magenta>sponsor:</magenta> <blue><italic><dim>https://github.com/sponsors/rvcas</dim></italic></blue>"#
};
