#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::ExampleScenario_ContainedInstance::ExampleScenario_ContainedInstance;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Example of workflow instance.

#[derive(Debug)]
pub struct ExampleScenario_Operation<'a> {
    pub value: &'a Value,
}

impl ExampleScenario_Operation<'_> {
    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Who starts the transaction.
    pub fn initiator(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("initiator") {
            return Some(string);
        }
        return None;
    }

    /// A comment to be inserted in the diagram.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Each resource instance used by the responder.
    pub fn response(&self) -> Option<ExampleScenario_ContainedInstance> {
        if let Some(val) = self.value.get("response") {
            return Some(ExampleScenario_ContainedInstance { value: val });
        }
        return None;
    }

    /// Extensions for receiverActive
    pub fn _receiver_active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_receiverActive") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for number
    pub fn _number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_number") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Who receives the transaction.
    pub fn receiver(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("receiver") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for receiver
    pub fn _receiver(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_receiver") {
            return Some(Element { value: val });
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Whether the initiator is deactivated right after the transaction.
    pub fn initiator_active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("initiatorActive") {
            return Some(val.as_bool().unwrap());
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

    /// Each resource instance used by the initiator.
    pub fn request(&self) -> Option<ExampleScenario_ContainedInstance> {
        if let Some(val) = self.value.get("request") {
            return Some(ExampleScenario_ContainedInstance { value: val });
        }
        return None;
    }

    /// The human-friendly name of the interaction.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for initiatorActive
    pub fn _initiator_active(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_initiatorActive") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for initiator
    pub fn _initiator(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_initiator") {
            return Some(Element { value: val });
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

    /// Whether the receiver is deactivated right after the transaction.
    pub fn receiver_active(&self) -> Option<bool> {
        if let Some(val) = self.value.get("receiverActive") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The sequential number of the interaction, e.g. 1.2.5.
    pub fn number(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("number") {
            return Some(string);
        }
        return None;
    }

    /// The type of operation - CRUD.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.initiator() {}
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.response() {
            _val.validate();
        }
        if let Some(_val) = self._receiver_active() {
            _val.validate();
        }
        if let Some(_val) = self._number() {
            _val.validate();
        }
        if let Some(_val) = self.receiver() {}
        if let Some(_val) = self._receiver() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.initiator_active() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.request() {
            _val.validate();
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._initiator_active() {
            _val.validate();
        }
        if let Some(_val) = self._initiator() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.receiver_active() {}
        if let Some(_val) = self.number() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        return true;
    }
}
