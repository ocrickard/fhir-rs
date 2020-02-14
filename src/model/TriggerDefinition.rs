#![allow(unused_imports, non_camel_case_types)]

use crate::model::DataRequirement::DataRequirement;
use crate::model::Element::Element;
use crate::model::Expression::Expression;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use crate::model::Timing::Timing;
use serde_json::value::Value;

/// A description of a triggering event. Triggering events can be named events, data
/// events, or periodic, as determined by the type element.

#[derive(Debug)]
pub struct TriggerDefinition<'a> {
    pub value: &'a Value,
}

impl TriggerDefinition<'_> {
    /// The timing of the event (if this is a periodic trigger).
    pub fn timing_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("timingReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The timing of the event (if this is a periodic trigger).
    pub fn timing_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timingDate") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for timingDate
    pub fn _timing_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timingDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The timing of the event (if this is a periodic trigger).
    pub fn timing_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timingTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// The timing of the event (if this is a periodic trigger).
    pub fn timing_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timingDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for timingDateTime
    pub fn _timing_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timingDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The triggering data of the event (if this is a data trigger). If more than one
    /// data is requirement is specified, then all the data requirements must be true.
    pub fn data(&self) -> Option<Vec<DataRequirement>> {
        if let Some(Value::Array(val)) = self.value.get("data") {
            return Some(
                val.into_iter()
                    .map(|e| DataRequirement { value: e })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A boolean-valued expression that is evaluated in the context of the container of
    /// the trigger definition and returns whether or not the trigger fires.
    pub fn condition(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("condition") {
            return Some(Expression { value: val });
        }
        return None;
    }

    /// The type of triggering event.
    pub fn fhir_type(&self) -> Option<TriggerDefinitionType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(TriggerDefinitionType::from_string(&val).unwrap());
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

    /// A formal name for the event. This may be an absolute URI that identifies the
    /// event formally (e.g. from a trigger registry), or a simple relative URI that
    /// identifies the event in a local context.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.timing_reference() {
            _val.validate();
        }
        if let Some(_val) = self.timing_date() {}
        if let Some(_val) = self._timing_date() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.timing_timing() {
            _val.validate();
        }
        if let Some(_val) = self.timing_date_time() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._timing_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.data() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.condition() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.name() {}
        return true;
    }
}

#[derive(Debug)]
pub enum TriggerDefinitionType {
    NamedEvent,
    Periodic,
    DataChanged,
    DataAdded,
    DataModified,
    DataRemoved,
    DataAccessed,
    DataAccessEnded,
}

impl TriggerDefinitionType {
    pub fn from_string(string: &str) -> Option<TriggerDefinitionType> {
        match string {
            "named-event" => Some(TriggerDefinitionType::NamedEvent),
            "periodic" => Some(TriggerDefinitionType::Periodic),
            "data-changed" => Some(TriggerDefinitionType::DataChanged),
            "data-added" => Some(TriggerDefinitionType::DataAdded),
            "data-modified" => Some(TriggerDefinitionType::DataModified),
            "data-removed" => Some(TriggerDefinitionType::DataRemoved),
            "data-accessed" => Some(TriggerDefinitionType::DataAccessed),
            "data-access-ended" => Some(TriggerDefinitionType::DataAccessEnded),
            _ => None,
        }
    }
}
