extern crate serde;
extern crate serde_json;

pub mod model;

#[cfg(test)]
mod tests {
  use serde_json::Result;
  use std::borrow::Cow;
  use std::fs;
  use std::time::Instant;
  #[test]
  fn it_works() {
    let paths = fs::read_dir("examples-json/").unwrap();

    println!(
      "sizeof element definition:{:?}",
      std::mem::size_of::<crate::model::ElementDefinition::ElementDefinition>()
    );

    for path in paths {
      let unwrapped_path = path.unwrap().path();
      println!("Beginning {}", &unwrapped_path.to_str().unwrap());
      let schema_contents =
        fs::read_to_string(&unwrapped_path).expect("Something went wrong reading the file");
      let string_parse_begin = Instant::now();
      let parsed: Result<serde_json::value::Value> = serde_json::from_str(&schema_contents);
      match parsed {
        Ok(value) => {
          println!(
            "Successfully parsed json {}: {}us",
            &unwrapped_path.to_str().unwrap(),
            string_parse_begin.elapsed().as_micros()
          );
          let resource_parse_begin = Instant::now();
          let resource = crate::model::ResourceList::ResourceList {
            value: Cow::Borrowed(&value),
          };
          println!(
            "Parsed resource {}: {}us",
            &unwrapped_path.to_str().unwrap(),
            resource_parse_begin.elapsed().as_micros()
          );
          let resource_validate_begin = Instant::now();
          resource.validate();
          println!(
            "Successfully validated resource {}: {}us",
            &unwrapped_path.to_str().unwrap(),
            resource_validate_begin.elapsed().as_micros()
          );
          // println!("Resource: {:?}", resource.resource());
        }
        Err(m) => assert!(
          false,
          "Error parsing {}: {}",
          &unwrapped_path.to_str().unwrap(),
          m
        ),
      }
    }
  }
}
