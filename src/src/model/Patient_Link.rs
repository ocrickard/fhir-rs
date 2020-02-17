#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Demographics and other administrative information about an individual or animal
/// receiving care or other health-related services.

#[derive(Debug)]
pub struct Patient_Link<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Patient_Link<'_> {
    pub fn new(value: &Value) -> Patient_Link {
        Patient_Link {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// The other patient resource that the link refers to.
    pub fn other(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["other"]),
        }
    }

    /// The type of link between this patient resource and another patient resource.
    pub fn fhir_type(&self) -> Option<Patient_LinkType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Patient_LinkType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._type() {
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
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.other().validate() {
            return false;
        }
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Patient_LinkBuilder {
    pub(crate) value: Value,
}

impl Patient_LinkBuilder {
    pub fn build(&self) -> Patient_Link {
        Patient_Link {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Patient_Link) -> Patient_LinkBuilder {
        Patient_LinkBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(other: Reference) -> Patient_LinkBuilder {
        let mut __value: Value = json!({});
        __value["other"] = json!(other.value);
        return Patient_LinkBuilder { value: __value };
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut Patient_LinkBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Patient_LinkBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Patient_LinkBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Patient_LinkBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Patient_LinkType) -> &'a mut Patient_LinkBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum Patient_LinkType {
    ReplacedBy,
    Replaces,
    Refer,
    Seealso,
}

impl Patient_LinkType {
    pub fn from_string(string: &str) -> Option<Patient_LinkType> {
        match string {
            "replaced-by" => Some(Patient_LinkType::ReplacedBy),
            "replaces" => Some(Patient_LinkType::Replaces),
            "refer" => Some(Patient_LinkType::Refer),
            "seealso" => Some(Patient_LinkType::Seealso),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Patient_LinkType::ReplacedBy => "replaced-by".to_string(),
            Patient_LinkType::Replaces => "replaces".to_string(),
            Patient_LinkType::Refer => "refer".to_string(),
            Patient_LinkType::Seealso => "seealso".to_string(),
        }
    }
}
