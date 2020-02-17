#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Demographics and administrative information about a person independent of a
/// specific health-related context.

#[derive(Debug)]
pub struct Person_Link<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Person_Link<'_> {
    pub fn new(value: &Value) -> Person_Link {
        Person_Link {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for assurance
    pub fn _assurance(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_assurance") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Level of assurance that this link is associated with the target resource.
    pub fn assurance(&self) -> Option<Person_LinkAssurance> {
        if let Some(Value::String(val)) = self.value.get("assurance") {
            return Some(Person_LinkAssurance::from_string(&val).unwrap());
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

    /// The resource to which this actual person is associated.
    pub fn target(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["target"]),
        }
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._assurance() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.assurance() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.target().validate() {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Person_LinkBuilder {
    pub(crate) value: Value,
}

impl Person_LinkBuilder {
    pub fn build(&self) -> Person_Link {
        Person_Link {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Person_Link) -> Person_LinkBuilder {
        Person_LinkBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(target: Reference) -> Person_LinkBuilder {
        let mut __value: Value = json!({});
        __value["target"] = json!(target.value);
        return Person_LinkBuilder { value: __value };
    }

    pub fn _assurance<'a>(&'a mut self, val: Element) -> &'a mut Person_LinkBuilder {
        self.value["_assurance"] = json!(val.value);
        return self;
    }

    pub fn assurance<'a>(&'a mut self, val: Person_LinkAssurance) -> &'a mut Person_LinkBuilder {
        self.value["assurance"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Person_LinkBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Person_LinkBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Person_LinkBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum Person_LinkAssurance {
    Level1,
    Level2,
    Level3,
    Level4,
}

impl Person_LinkAssurance {
    pub fn from_string(string: &str) -> Option<Person_LinkAssurance> {
        match string {
            "level1" => Some(Person_LinkAssurance::Level1),
            "level2" => Some(Person_LinkAssurance::Level2),
            "level3" => Some(Person_LinkAssurance::Level3),
            "level4" => Some(Person_LinkAssurance::Level4),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Person_LinkAssurance::Level1 => "level1".to_string(),
            Person_LinkAssurance::Level2 => "level2".to_string(),
            Person_LinkAssurance::Level3 => "level3".to_string(),
            Person_LinkAssurance::Level4 => "level4".to_string(),
        }
    }
}
