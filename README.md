# FHIR for Rust!

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

## Frequently Asked Questions

### Why should I use fhir_rs instead of just using serde_json with FHIR data?

Well it's up to you, but type safety when working with such a huge and complex standard can save you lots of time debugging difficult problems. The initial setup cost of using fhir_rs will pay for itself quickly through faster iteration on the data types.

### Why do you wrap serde_json values instead of just using normal structs?

The initial version of this library used normal Rust structs... and quickly ran into stack overflows and performance problems. Rust's structs are not sparsely allocated, they consume exactly as much space as the member types require. In most mature Rust model libraries that use structs, use of enum/union types avoid the memory allocations, however the FHIR standard was built this in mind, and in practice some of the value types are enormous bags of nullable data.

As a result, a sparse structure was necessary to avoid excessive memory allocations, and we decided to rely on the underlying serde_json values to avoid data copying.

### Why was fhir_rs built?

I built fhir_rs so that I could prototype a healthcare server. The challenges I had in building out the FHIR spec as Rust data types made me want to share my work with others. Maybe it'll be useful, maybe not...

### What features still need to be implemented?

If you'd like to help out, please take one of these and turn it into a Pull Request! All help is welcome.

1. A better numeric interface for floats. At the moment floating point values are represented as f64, and although we internally use an arbitrary precision feature inside Serde, the values we expose to the user are still f64. As a result, you may experience a reduction in precision when actually retrieving values, and the number of places after the decimal point are not preserved. This is true of many FHIR implementations, but we will be resolving this ASAP.

2. Full (optional) OAuth support for authentication for full SMART on FHIR (SoF) interop. We don't want these additional features to bloat the core, it'll always stay a small collection of model wrappers, but over time we'd like to see this library expand optional support for SoF communication.

3. (Optional) API Call interface. After we add authentication support, I'd like to provide an async interface for interacting with SoF APIs. This way you get to interact with a code completion engine for your remote calls in addition to your local data processing.