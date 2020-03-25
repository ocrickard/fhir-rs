# fhir_rs: FHIR models in Rust

fhir_rs allows you to work with FHIR data in Rust:

```rust
use fhir_rs::{fhir_parse, model};

if let Some(resource_list) = fhir_parse(&json_string) {
  match resource_list.resource() {
    model::ResourceList::ResourceListEnum::ResourcePatient(patient) => {
      println!("Patient: {:?}", patient);
    }
    _ => {}
  }
}
```

