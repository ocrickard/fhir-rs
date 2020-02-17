#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Identifies two or more records (resource instances) that refer to the same real-
/// world "occurrence".

#[derive(Debug)]
pub struct Linkage_Item<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Linkage_Item<'_> {
    pub fn new(value: &Value) -> Linkage_Item {
        Linkage_Item {
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

    /// The resource instance being linked as part of the group.
    pub fn resource(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["resource"]),
        }
    }

    /// Distinguishes which item is "source of truth" (if any) and which items are no
    /// longer considered to be current representations.
    pub fn fhir_type(&self) -> Option<Linkage_ItemType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Linkage_ItemType::from_string(&val).unwrap());
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
        if !self.resource().validate() {
            return false;
        }
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Linkage_ItemBuilder {
    pub(crate) value: Value,
}

impl Linkage_ItemBuilder {
    pub fn build(&self) -> Linkage_Item {
        Linkage_Item {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Linkage_Item) -> Linkage_ItemBuilder {
        Linkage_ItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(resource: Reference) -> Linkage_ItemBuilder {
        let mut __value: Value = json!({});
        __value["resource"] = json!(resource.value);
        return Linkage_ItemBuilder { value: __value };
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut Linkage_ItemBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Linkage_ItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Linkage_ItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Linkage_ItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: Linkage_ItemType) -> &'a mut Linkage_ItemBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum Linkage_ItemType {
    Source,
    Alternate,
    Historical,
}

impl Linkage_ItemType {
    pub fn from_string(string: &str) -> Option<Linkage_ItemType> {
        match string {
            "source" => Some(Linkage_ItemType::Source),
            "alternate" => Some(Linkage_ItemType::Alternate),
            "historical" => Some(Linkage_ItemType::Historical),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Linkage_ItemType::Source => "source".to_string(),
            Linkage_ItemType::Alternate => "alternate".to_string(),
            Linkage_ItemType::Historical => "historical".to_string(),
        }
    }
}
