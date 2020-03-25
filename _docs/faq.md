---
docid: faq
title: FAQ
layout: docs
permalink: /docs/faq
---

## Frequently Asked Questions

### Why should I use fhir_rs instead of just using serde_json with FHIR data?

Well it's up to you, but type safety when working with such a huge and complex standard can save you lots of time debugging difficult problems. The initial setup cost of using fhir_rs will pay for itself quickly through faster iteration on the data types.

### Why do you wrap serde_json values instead of just using normal structs?

The initial version of this library used normal Rust structs... and quickly ran into stack overflows and performance problems. Rust's structs are not sparsely allocated, they consume exactly as much space as the member types require. In most mature Rust model libraries that use structs, use of enum/union types avoid the memory allocations, however the FHIR standard was built this in mind, and in practice some of the value types are enormous bags of nullable data.

As a result, a sparse structure was necessary to avoid excessive memory allocations, and we decided to rely on the underlying serde_json values to avoid data copying.
