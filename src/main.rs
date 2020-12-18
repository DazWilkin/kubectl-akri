use std::{convert::TryFrom, env, fs::read};
use yaml_validator::{
    yaml_rust::{Yaml, YamlLoader},
    Context, Validate,
};
mod error;
use error::Error;

const SCHEMA: &str = "./schema.yaml";
const KUBERN: &str = "./zeroconf.yaml";

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    let contents = read(SCHEMA).map_err(|e| {
        Error::FileError(format!(
            "Unable to read schema: {} [{}]",
            SCHEMA.to_string(),
            e
        ))
    })?;

    let utf8: String = String::from_utf8_lossy(&contents).parse().map_err(|e| {
        Error::FileError(format!(
            "file {} did not contain valid utf8: {}\n",
            SCHEMA.to_string(),
            e
        ))
    })?;

    let schema = YamlLoader::load_from_str(&utf8).map_err(Error::from)?;

    let contents = read(KUBERN).map_err(|e| {
        Error::FileError(format!(
            "Unable to read data: {} [{}]",
            KUBERN.to_string(),
            e
        ))
    })?;

    let utf8: String = String::from_utf8_lossy(&contents).parse().map_err(|e| {
        Error::FileError(format!(
            "file {} did not contain valid utf8: {}\n",
            KUBERN.to_string(),
            e
        ))
    })?;

    let context = Context::try_from(&schema).unwrap();
    let documents = YamlLoader::load_from_str(&utf8)
        .map_err(Error::from)
        .expect("documents");

    for doc in documents {
        context
            .get_schema("configuration")
            .unwrap()
            .validate(&context, &doc)
            .map_err(|e| Error::ValidationError(format!("{}", e)))?;
    }

    Ok(())
}
