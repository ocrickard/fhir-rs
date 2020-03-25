---
docid: intro
title: FHIR for Rust
layout: docs
permalink: /docs/intro
---

A simple, fast, and safe interface for FHIR (Fast Healthcare Interoperability Resources) JSON data in Rust.

Here's how parsing works:

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

You can also create FHIR resources using the type-safe model builders for every FHIR type:

```rust
let patient = model::Patient::PatientBuilder::new()
  .name(vec![model::HumanName::HumanNameBuilder::new()
    .given(vec!["Bob"])
    .family("Builder")
    .build()])
  .gender(model::Patient::PatientGender::Male)
  .build();
let json = patient.to_json().to_string();
```

## Built on top of Serde

fhir_rs provides a type-safe and efficient collection of wrappers over serde_json. This allows fhir_rs to be extremely fast, efficient, and safe. 
