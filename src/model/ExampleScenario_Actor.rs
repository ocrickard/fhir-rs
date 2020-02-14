#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Actor<'a> {
    pub value: &'a Value,
}

impl ExampleScenario_Actor<'_> {
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

    /// The name of the actor as shown in the page.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// ID or acronym of actor.
    pub fn actor_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("actorId") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for actorId
    pub fn _actor_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actorId") {
            return Some(Element { value: val });
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

    /// The description of the actor.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The type of actor - person or system.
    pub fn fhir_type(&self) -> Option<ExampleScenario_ActorType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(ExampleScenario_ActorType::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.actor_id() {}
        if let Some(_val) = self._actor_id() {
            _val.validate();
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum ExampleScenario_ActorType {
    Person,
    Entity,
}

impl ExampleScenario_ActorType {
    pub fn from_string(string: &str) -> Option<ExampleScenario_ActorType> {
        match string {
            "person" => Some(ExampleScenario_ActorType::Person),
            "entity" => Some(ExampleScenario_ActorType::Entity),
            _ => None,
        }
    }
}
