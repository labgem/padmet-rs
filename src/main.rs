

mod node;
mod policy;
mod relation;
mod spec;

fn main() {
    let filename = "/mnt/shared/bank/metacyc.padmet";
    let padmet_object = spec::PadmetSpec::from_file(filename);
}
