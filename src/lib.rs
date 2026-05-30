//! Read and write PADMet files

/* std use */

/* crate use */

/* module declaration */

pub mod cli;
pub mod io;
pub mod error;

pub mod spec;
pub mod node;
pub mod relation;
pub mod policy;

/* project use */

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;
}
