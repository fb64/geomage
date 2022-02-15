use clap::{arg, App};
use std::fs::File;
use std::io::Read;
use geojson::GeoJson;

fn main() {
    let matches = App::new("GeoMage")
        .version("0.1.0")
        .author("Florian B. <florian@flob.fr>")
        .about(" Convert geojson to image")
        .arg(
            arg!(
                -i --inputs <INPUT_FILES> "Sets the input geojson files to use"
            ).required(true))
        .arg(
            arg!(
                -o --output <OUTPUT_FILE> "Sets the input geojson files to use"
            ).required(true).takes_value(true).required(true))
        .arg(
            arg!(
                -w --width <WIDTH> "Sets the input geojson files to use"
            ).takes_value(true).default_value("1024").required(false))
        .get_matches();

    println!("TODO develop app !")
}
