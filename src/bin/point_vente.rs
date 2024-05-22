use std::{
    fs::File,
    io::{BufWriter, Write},
};

use backend::graphql::point_vente::PointVenteSchema;
use evaluation_p14_training as backend;

fn main() {
    let schema = PointVenteSchema::default();
    let mut file = BufWriter::new(File::create("app/schemas/point-vente.graphqls").unwrap());
    file.write_all(schema.sdl().as_bytes()).unwrap();
    file.flush().unwrap();
}
