#![allow(dead_code)]

use std::{
    collections::HashMap,
    env,
    fs::{self, read_dir, File},
    io::{self, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use strfmt::strfmt;

#[derive(Debug)]
struct Error {
    message: String,
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

impl From<env::VarError> for Error {
    fn from(error: env::VarError) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

type Matcher = fn(&PathBuf) -> bool;

fn generate_tests(destination: &str, module: &str, path: &str, template: &str, matcher: Matcher) -> Result<(), Error> {
    let out_dir = env::var("OUT_DIR")?;
    let destination = Path::new(&out_dir).join(destination);
    let mut test_file = File::create(&destination)?;
    start_module(&mut test_file, module)?;
    generate_tests_module(&mut test_file, &PathBuf::from_str(path).unwrap(), template, matcher)?;
    end_module(&mut test_file)?;
    Ok(())
}

fn generate_tests_module(
    mut test_file: &mut File,
    dir: &PathBuf,
    template: &str,
    matcher: Matcher,
) -> Result<(), Error> {
    let module_name = normalize_file_stem(dir)?;
    start_module(&mut test_file, &module_name)?;

    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path().canonicalize()?;

        if path.is_dir() {
            generate_tests_module(test_file, &path, template, matcher)?;
        } else {
            if matcher(&path) {
                generate_test(test_file, &path, template)?;
            }
        }
    }

    end_module(&mut test_file)?;
    Ok(())
}

fn generate_test(test_file: &mut File, path: &PathBuf, template: &str) -> Result<(), Error> {
    let mut vars = HashMap::new();
    vars.insert("name".to_string(), format!("{}", normalize_file_stem(&path)?));
    vars.insert(
        "path".to_string(),
        path.to_str().expect("unable to format path as a string").to_string(),
    );

    let template = fs::read_to_string(template).expect("unable to read template");
    let content = strfmt(&template, &vars).expect("unable to format template");
    test_file.write_all(content.as_bytes())?;
    Ok(())
}

fn start_module(test_file: &mut File, name: &str) -> Result<(), Error> {
    write!(
        test_file,
        r#"
#[allow(unused_imports)]
mod {name} {{
    use jellyschema::keywords::{{ErrorKind, Error}};
    use jellyschema::{{Scope, Schema}};
    use serde_json;
    use serde_yaml;
    use std::str::FromStr;
    use std::path::PathBuf;
    use std::fs;

        "#,
        name = name
    )?;
    Ok(())
}

fn end_module(test_file: &mut File) -> Result<(), Error> {
    write!(
        test_file,
        r#"
}}

        "#
    )?;
    Ok(())
}

fn normalize_file_stem(path: &PathBuf) -> Result<String, Error> {
    let result = path
        .file_stem()
        .ok_or(Error {
            message: "cannot convert os string into string".to_string(),
        })?
        .to_string_lossy()
        .replace("-", "_");
    Ok(result.to_string())
}

fn validator_tests_matcher(path: &PathBuf) -> bool {
    match path.extension() {
        Some(ext) => ext == "yaml",
        _ => false,
    }
}

fn data_type_validator_tests_matcher(path: &PathBuf) -> bool {
    match path.file_name() {
        Some(name) => name == "validation.yaml",
        _ => false,
    }
}

fn main() -> Result<(), Error> {
    generate_tests(
        "validators_data_tests.rs",
        "vd",
        "./tests/validators/data",
        "./tests/validators/data-test-template",
        validator_tests_matcher,
    )?;
    generate_tests(
        "validators_errors_tests.rs",
        "ve",
        "./tests/validators/errors",
        "./tests/validators/errors-test-template",
        validator_tests_matcher,
    )?;
    generate_tests(
        "data_types_validator_tests.rs",
        "dt",
        "./src",
        "./tests/data-types/data-type-validator-test-template",
        data_type_validator_tests_matcher,
    )?;

    //    generate_tests(
    //        "generator_invalid_tests.rs",
    //        "gi",
    //        "./tests/generator/invalid",
    //        "./tests/generator/invalid-test-template",
    //        generator_tests_matcher,
    //    )?;
    //    generate_tests(
    //        "generator_valid_tests.rs",
    //        "gb",
    //        "./tests/generator/valid",
    //        "./tests/generator/valid-test-template",
    //        generator_tests_matcher,
    //    )?;
    Ok(())
}
