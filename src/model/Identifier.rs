#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An identifier - identifies some entity uniquely and unambiguously. Typically
/// this is used for business identifiers.

#[derive(Debug)]
pub struct Identifier<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Identifier<'_> {
    pub fn new(value: &Value) -> Identifier {
        Identifier {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for system
    pub fn _system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_system") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for use
    pub fn _use(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_use") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Organization that issued/manages the identifier.
    pub fn assigner(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("assigner") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// Time period during which identifier is/was valid for use.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Establishes the namespace for the value - that is, a URL that describes a set
    /// values that are unique.
    pub fn system(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("system") {
            return Some(string);
        }
        return None;
    }

    /// A coded type for the identifier that can be used to determine which identifier
    /// to use for a specific purpose.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The purpose of this identifier.
    pub fn fhir_use(&self) -> Option<IdentifierUse> {
        if let Some(Value::String(val)) = self.value.get("use") {
            return Some(IdentifierUse::from_string(&val).unwrap());
        }
        return None;
    }

    /// The portion of the identifier typically relevant to the user and which is unique
    /// within the context of the system.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._system() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._use() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.assigner() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.system() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_use() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct IdentifierBuilder {
    pub(crate) value: Value,
}

impl IdentifierBuilder {
    pub fn build(&self) -> Identifier {
        Identifier {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Identifier) -> IdentifierBuilder {
        IdentifierBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> IdentifierBuilder {
        let mut __value: Value = json!({});
        return IdentifierBuilder { value: __value };
    }

    pub fn _system<'a>(&'a mut self, val: Element) -> &'a mut IdentifierBuilder {
        self.value["_system"] = json!(val.value);
        return self;
    }

    pub fn _use<'a>(&'a mut self, val: Element) -> &'a mut IdentifierBuilder {
        self.value["_use"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut IdentifierBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn assigner<'a>(&'a mut self, val: Reference) -> &'a mut IdentifierBuilder {
        self.value["assigner"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut IdentifierBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut IdentifierBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut IdentifierBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn system<'a>(&'a mut self, val: &str) -> &'a mut IdentifierBuilder {
        self.value["system"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut IdentifierBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn fhir_use<'a>(&'a mut self, val: IdentifierUse) -> &'a mut IdentifierBuilder {
        self.value["use"] = json!(val.to_string());
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut IdentifierBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum IdentifierUse {
    Usual,
    Official,
    Temp,
    Secondary,
    Old,
}

impl IdentifierUse {
    pub fn from_string(string: &str) -> Option<IdentifierUse> {
        match string {
            "usual" => Some(IdentifierUse::Usual),
            "official" => Some(IdentifierUse::Official),
            "temp" => Some(IdentifierUse::Temp),
            "secondary" => Some(IdentifierUse::Secondary),
            "old" => Some(IdentifierUse::Old),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            IdentifierUse::Usual => "usual".to_string(),
            IdentifierUse::Official => "official".to_string(),
            IdentifierUse::Temp => "temp".to_string(),
            IdentifierUse::Secondary => "secondary".to_string(),
            IdentifierUse::Old => "old".to_string(),
        }
    }
}
