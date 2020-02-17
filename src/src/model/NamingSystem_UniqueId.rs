#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A curated namespace that issues unique symbols within that namespace for the
/// identification of concepts, people, devices, etc.  Represents a "System" used
/// within the Identifier and Coding data types.

#[derive(Debug)]
pub struct NamingSystem_UniqueId<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl NamingSystem_UniqueId<'_> {
    pub fn new(value: &Value) -> NamingSystem_UniqueId {
        NamingSystem_UniqueId {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preferred
    pub fn _preferred(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preferred") {
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

    /// Extensions for value
    pub fn _value(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_value") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Notes about the past or intended usage of this identifier.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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

    /// Identifies the period of time over which this identifier is considered
    /// appropriate to refer to the naming system.  Outside of this window, the
    /// identifier might be non-deterministic.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates whether this identifier is the "preferred" identifier of this type.
    pub fn preferred(&self) -> Option<bool> {
        if let Some(val) = self.value.get("preferred") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Identifies the unique identifier scheme used for this particular identifier.
    pub fn fhir_type(&self) -> Option<NamingSystem_UniqueIdType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(NamingSystem_UniqueIdType::from_string(&val).unwrap());
        }
        return None;
    }

    /// The string that should be sent over the wire to identify the code system or
    /// identifier system.
    pub fn value(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("value") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._comment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._preferred() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.comment() {}
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
        if let Some(_val) = self.period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.preferred() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.value() {}
        return true;
    }
}

#[derive(Debug)]
pub struct NamingSystem_UniqueIdBuilder {
    pub(crate) value: Value,
}

impl NamingSystem_UniqueIdBuilder {
    pub fn build(&self) -> NamingSystem_UniqueId {
        NamingSystem_UniqueId {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: NamingSystem_UniqueId) -> NamingSystem_UniqueIdBuilder {
        NamingSystem_UniqueIdBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> NamingSystem_UniqueIdBuilder {
        let mut __value: Value = json!({});
        return NamingSystem_UniqueIdBuilder { value: __value };
    }

    pub fn _comment<'a>(&'a mut self, val: Element) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["_comment"] = json!(val.value);
        return self;
    }

    pub fn _preferred<'a>(&'a mut self, val: Element) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["_preferred"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _value<'a>(&'a mut self, val: Element) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["_value"] = json!(val.value);
        return self;
    }

    pub fn comment<'a>(&'a mut self, val: &str) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["comment"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn preferred<'a>(&'a mut self, val: bool) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["preferred"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: NamingSystem_UniqueIdType,
    ) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }

    pub fn value<'a>(&'a mut self, val: &str) -> &'a mut NamingSystem_UniqueIdBuilder {
        self.value["value"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum NamingSystem_UniqueIdType {
    Oid,
    Uuid,
    Uri,
    Other,
}

impl NamingSystem_UniqueIdType {
    pub fn from_string(string: &str) -> Option<NamingSystem_UniqueIdType> {
        match string {
            "oid" => Some(NamingSystem_UniqueIdType::Oid),
            "uuid" => Some(NamingSystem_UniqueIdType::Uuid),
            "uri" => Some(NamingSystem_UniqueIdType::Uri),
            "other" => Some(NamingSystem_UniqueIdType::Other),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            NamingSystem_UniqueIdType::Oid => "oid".to_string(),
            NamingSystem_UniqueIdType::Uuid => "uuid".to_string(),
            NamingSystem_UniqueIdType::Uri => "uri".to_string(),
            NamingSystem_UniqueIdType::Other => "other".to_string(),
        }
    }
}
