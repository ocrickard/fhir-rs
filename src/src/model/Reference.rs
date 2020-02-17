#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A reference from one resource to another.

#[derive(Debug)]
pub struct Reference<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Reference<'_> {
    pub fn new(value: &Value) -> Reference {
        Reference {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for display
    pub fn _display(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_display") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for reference
    pub fn _reference(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reference") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Plain text narrative that identifies the resource in addition to the resource
    /// reference.
    pub fn display(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("display") {
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

    /// An identifier for the target resource. This is used when there is no way to
    /// reference the other resource directly, either because the entity it represents
    /// is not available through a FHIR server, or because there is no way for the
    /// author of the resource to convert a known identifier to an actual location.
    /// There is no requirement that a Reference.identifier point to something that is
    /// actually exposed as a FHIR instance, but it SHALL point to a business concept
    /// that would be expected to be exposed as a FHIR instance, and that instance would
    /// need to be of a FHIR resource type allowed by the reference.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A reference to a location at which the other resource is found. The reference
    /// may be a relative reference, in which case it is relative to the service base
    /// URL, or an absolute URL that resolves to the location where the resource is
    /// found. The reference may be version specific or not. If the reference is not to
    /// a FHIR RESTful server, then it should be assumed to be version specific.
    /// Internal fragment references (start with '#') refer to contained resources.
    pub fn reference(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("reference") {
            return Some(string);
        }
        return None;
    }

    /// The expected type of the target of the reference. If both Reference.type and
    /// Reference.reference are populated and Reference.reference is a FHIR URL, both
    /// SHALL be consistent.    The type is the Canonical URL of Resource Definition
    /// that is the type this reference refers to. References are URLs that are relative
    /// to http://hl7.org/fhir/StructureDefinition/ e.g. "Patient" is a reference to
    /// http://hl7.org/fhir/StructureDefinition/Patient. Absolute URLs are only allowed
    /// for logical models (and can only be used in references in logical models, not
    /// resources).
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._display() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.display() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.reference() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ReferenceBuilder {
    pub(crate) value: Value,
}

impl ReferenceBuilder {
    pub fn build(&self) -> Reference {
        Reference {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Reference) -> ReferenceBuilder {
        ReferenceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ReferenceBuilder {
        let mut __value: Value = json!({});
        return ReferenceBuilder { value: __value };
    }

    pub fn _display<'a>(&'a mut self, val: Element) -> &'a mut ReferenceBuilder {
        self.value["_display"] = json!(val.value);
        return self;
    }

    pub fn _reference<'a>(&'a mut self, val: Element) -> &'a mut ReferenceBuilder {
        self.value["_reference"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut ReferenceBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn display<'a>(&'a mut self, val: &str) -> &'a mut ReferenceBuilder {
        self.value["display"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ReferenceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ReferenceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut ReferenceBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn reference<'a>(&'a mut self, val: &str) -> &'a mut ReferenceBuilder {
        self.value["reference"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut ReferenceBuilder {
        self.value["type"] = json!(val);
        return self;
    }
}
