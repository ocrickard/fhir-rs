#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Actor<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExampleScenario_Actor<'_> {
    pub fn new(value: &Value) -> ExampleScenario_Actor {
        ExampleScenario_Actor {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for actorId
    pub fn _actor_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_actorId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
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

    /// ID or acronym of actor.
    pub fn actor_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("actorId") {
            return Some(string);
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

    /// The name of the actor as shown in the page.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._actor_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.actor_id() {}
        if let Some(_val) = self.description() {}
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ExampleScenario_ActorBuilder {
    pub(crate) value: Value,
}

impl ExampleScenario_ActorBuilder {
    pub fn build(&self) -> ExampleScenario_Actor {
        ExampleScenario_Actor {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExampleScenario_Actor) -> ExampleScenario_ActorBuilder {
        ExampleScenario_ActorBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ExampleScenario_ActorBuilder {
        let mut __value: Value = json!({});
        return ExampleScenario_ActorBuilder { value: __value };
    }

    pub fn _actor_id<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["_actorId"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn actor_id<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["actorId"] = json!(val);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: ExampleScenario_ActorType,
    ) -> &'a mut ExampleScenario_ActorBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            ExampleScenario_ActorType::Person => "person".to_string(),
            ExampleScenario_ActorType::Entity => "entity".to_string(),
        }
    }
}
