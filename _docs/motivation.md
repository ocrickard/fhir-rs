---
docid: motivation
title: Motivation
layout: docs
permalink: /docs/motivation
---

## FHIR is a huge and confusing standard

For all of its achievements, FHIR is a huge standard, and it's just too easy to get things wrong. And when things do go wrong, most existing FHIR servers provide substandard error messages.

fhir_rs uses Rust's excellent type system to help you structure your data correctly. Your IDE can provide intelligent autocompletion to lookup the fields on every type, and you don't need to worry about messing up JSON structures when using fhir_rs.

## fhir_rs is safe

For healthcare applications safety and security is paramount. Rust provides a level of safety in its memory handling that other languages can't match. fhir_rs uses no unsafe blocks and is easy to read and understand.

## fhir_rs is fast

Many of the existing libraries for working with FHIR data are extremely slow and heavyweight. fhir_rs is just a small wrapper on top of one of the fastest JSON parsers in existence. This means your applications will be more responsive, and can handle larger loads.

## fhir_rs is efficient

fhir_rs is built on top of Serde. Internally all of the model objects just grab borrowed references to the underlying serde_json structs. This architecture allows fhir_rs to avoid copying data unnecessarily, and also allows our structs to have sparse memory layouts. In compiled languages like Rust, that is a very important property since naive FHIR structs can take up huge amounts of memory due to the deep nesting present in the data types, even with boxing.
