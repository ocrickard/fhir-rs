#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An interaction between a patient and healthcare provider(s) for the purpose of
/// providing healthcare service(s) or assessing the health status of a patient.

#[derive(Debug)]
pub struct Encounter_Location<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Encounter_Location<'_> {
    pub fn new(value: &Value) -> Encounter_Location {
        Encounter_Location {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
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

    /// The location where the encounter takes place.
    pub fn location(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["location"]),
        }
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

    /// Time period during which the patient was present at the location.
    pub fn period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("period") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// This will be used to specify the required levels (bed/ward/room/etc.) desired to
    /// be recorded to simplify either messaging or query.
    pub fn physical_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("physicalType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of the participants' presence at the specified location during the
    /// period specified. If the participant is no longer at the location, then the
    /// period will have an end date/time.
    pub fn status(&self) -> Option<Encounter_LocationStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(Encounter_LocationStatus::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._status() {
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
        if !self.location().validate() {
            return false;
        }
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
        if let Some(_val) = self.physical_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Encounter_LocationBuilder {
    pub(crate) value: Value,
}

impl Encounter_LocationBuilder {
    pub fn build(&self) -> Encounter_Location {
        Encounter_Location {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Encounter_Location) -> Encounter_LocationBuilder {
        Encounter_LocationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(location: Reference) -> Encounter_LocationBuilder {
        let mut __value: Value = json!({});
        __value["location"] = json!(location.value);
        return Encounter_LocationBuilder { value: __value };
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut Encounter_LocationBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Encounter_LocationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Encounter_LocationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Encounter_LocationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn period<'a>(&'a mut self, val: Period) -> &'a mut Encounter_LocationBuilder {
        self.value["period"] = json!(val.value);
        return self;
    }

    pub fn physical_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Encounter_LocationBuilder {
        self.value["physicalType"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: Encounter_LocationStatus,
    ) -> &'a mut Encounter_LocationBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum Encounter_LocationStatus {
    Planned,
    Active,
    Reserved,
    Completed,
}

impl Encounter_LocationStatus {
    pub fn from_string(string: &str) -> Option<Encounter_LocationStatus> {
        match string {
            "planned" => Some(Encounter_LocationStatus::Planned),
            "active" => Some(Encounter_LocationStatus::Active),
            "reserved" => Some(Encounter_LocationStatus::Reserved),
            "completed" => Some(Encounter_LocationStatus::Completed),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Encounter_LocationStatus::Planned => "planned".to_string(),
            Encounter_LocationStatus::Active => "active".to_string(),
            Encounter_LocationStatus::Reserved => "reserved".to_string(),
            Encounter_LocationStatus::Completed => "completed".to_string(),
        }
    }
}
