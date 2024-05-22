use std::{
    fs::File,
    io::{BufWriter, Write},
};

use backend::graphql::magasin::MagasinSchema;
use evaluation_p14_training as backend;

fn main() {
    let schema = MagasinSchema::default();
    let mut file = BufWriter::new(File::create("app/schemas/magasin.graphqls").unwrap());
    file.write_all(schema.sdl().as_bytes()).unwrap();
    file.flush().unwrap();
}
