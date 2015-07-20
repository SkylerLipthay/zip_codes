extern crate csv;
extern crate phf_codegen;
extern crate rustc_serialize;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(RustcDecodable)]
struct TempRecord {
    zip_code: String,
    zip_code_type: String,
    city: String,
    state: String,
    _location_type: String,
    latitude: Option<f64>,
    longitude: Option<f64>,
    _location: String,
    is_decommissioned: String,
    tax_returns_filed: Option<u64>,
    estimated_population: Option<u64>,
    total_wages: Option<u64>,
}

impl TempRecord {
    fn to_syntax(&self) -> String {
        let mut output = String::from("Record {");
        output.push_str(&format!("zip_code: \"{}\",", self.zip_code));
        output.push_str(&format!("zip_code_type: {},", self.encoded_type()));
        output.push_str(&format!("city: \"{}\",", self.city));
        output.push_str(&format!("state: \"{}\",", self.state));
        output.push_str(&format!("coordinates: {},", self.coordinates()));
        output.push_str(&format!("is_decommissioned: {},", self.is_decommissioned));
        output.push_str(&format!("tax_returns_filed: {:?},", self.tax_returns_filed));
        output.push_str(&format!("estimated_population: {:?},", self.estimated_population));
        output.push_str(&format!("total_wages: {:?},", self.total_wages));
        output.push_str("}");
        output
    }

    fn encoded_type(&self) -> &'static str {
        match &self.zip_code_type[..] {
            "STANDARD" => "Type::Standard",
            "PO BOX" => "Type::PoBox",
            "UNIQUE" => "Type::Unique",
            "MILITARY" => "Type::Military",
            t => panic!("invalid ZIP code type \"{}\"", t),
        }
    }

    fn coordinates(&self) -> String {
        if let Some(ref latitude) = self.latitude {
            if let Some(ref longitude) = self.longitude {
                return format!("Some(({} as f64, {} as f64))", latitude, longitude);
            }
        }

        "None".into()
    }
}

fn main() {
    let path = Path::new(env!("OUT_DIR")).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(&mut file, "static ZIP_CODES: phf::Map<&'static str, Record> = ").unwrap();
    let mut reader = csv::Reader::from_file("./data/zip_codes.csv").unwrap();
    let records = reader.decode().map(|r| r.unwrap()).collect::<Vec<TempRecord>>();

    {
        let mut map = phf_codegen::Map::new();
        for ref record in &records {
            map.entry(&record.zip_code[..], &record.to_syntax());
        }
        map.build(&mut file).unwrap();
    }

    write!(&mut file, ";\n").unwrap();
}
