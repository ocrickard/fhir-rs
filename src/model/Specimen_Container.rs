#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A sample to be used for analysis.

#[derive(Debug)]
pub struct Specimen_Container<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Specimen_Container<'_> {
    pub fn new(value: &Value) -> Specimen_Container {
        Specimen_Container {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Introduced substance to preserve, maintain or enhance the specimen. Examples:
    /// Formalin, Citrate, EDTA.
    pub fn additive_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("additiveCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Introduced substance to preserve, maintain or enhance the specimen. Examples:
    /// Formalin, Citrate, EDTA.
    pub fn additive_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("additiveReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The capacity (volume or other measure) the container may contain.
    pub fn capacity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("capacity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Textual description of the container.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
    /// there is a strict set of governance  applied to the definition and use of
    /// extensions. Though any implementer can define an extension, there is a set of
    /// requirements that SHALL be met as part of the definition of the extension.
    pub fn extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("extension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Id for container. There may be multiple; a manufacturer's bar code, lab assigned
    /// identifier, etc. The container ID may differ from the specimen id in some
    /// circumstances.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
    pub fn modifier_extension(&self) -> Option<Vec<Extension>> {
        if let Some(Value::Array(val)) = self.value.get("modifierExtension") {
            return Some(
                val.into_iter()
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The quantity of specimen in the container; may be volume, dimensions, or other
    /// appropriate measurements, depending on the specimen type.
    pub fn specimen_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("specimenQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of container associated with the specimen (e.g. slide, aliquot, etc.).
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.additive_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.additive_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.capacity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.specimen_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Specimen_ContainerBuilder {
    pub(crate) value: Value,
}

impl Specimen_ContainerBuilder {
    pub fn build(&self) -> Specimen_Container {
        Specimen_Container {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Specimen_Container) -> Specimen_ContainerBuilder {
        Specimen_ContainerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Specimen_ContainerBuilder {
        let mut __value: Value = json!({});
        return Specimen_ContainerBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut Specimen_ContainerBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn additive_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Specimen_ContainerBuilder {
        self.value["additiveCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn additive_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut Specimen_ContainerBuilder {
        self.value["additiveReference"] = json!(val.value);
        return self;
    }

    pub fn capacity<'a>(&'a mut self, val: Quantity) -> &'a mut Specimen_ContainerBuilder {
        self.value["capacity"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut Specimen_ContainerBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Specimen_ContainerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Specimen_ContainerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut Specimen_ContainerBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Specimen_ContainerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn specimen_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut Specimen_ContainerBuilder {
        self.value["specimenQuantity"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Specimen_ContainerBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
