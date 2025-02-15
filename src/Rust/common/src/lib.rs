/*
    This is the basic (core) "lib" module for a non-executable crate. For this
    project, the code is in the modules run.rs and setup.rs, so this module
    only needs to refer to them so that the linker for the executables can find
    them.
*/

pub mod run;
pub mod setup;
