#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// An action that is or was performed on or for a patient. This can be a physical
/// intervention like an operation, or less invasive like long term services,
/// counseling, or hypnotherapy.

#[derive(Debug)]
pub struct Procedure_Performer<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Procedure_Performer<'_> {
    pub fn new(value: &Value) -> Procedure_Performer {
        Procedure_Performer {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// The practitioner who was involved in the procedure.
    pub fn actor(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["actor"]),
        }
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

    /// Distinguishes the type of involvement of the performer in the procedure. For
    /// example, surgeon, anaesthetist, endoscopist.
    pub fn function(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("function") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// The organization the device or practitioner was acting on behalf of.
    pub fn on_behalf_of(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("onBehalfOf") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if !self.actor().validate() {
            return false;
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.function() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.on_behalf_of() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Procedure_PerformerBuilder {
    pub(crate) value: Value,
}

impl Procedure_PerformerBuilder {
    pub fn build(&self) -> Procedure_Performer {
        Procedure_Performer {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Procedure_Performer) -> Procedure_PerformerBuilder {
        Procedure_PerformerBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(actor: Reference) -> Procedure_PerformerBuilder {
        let mut __value: Value = json!({});
        __value["actor"] = json!(actor.value);
        return Procedure_PerformerBuilder { value: __value };
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Procedure_PerformerBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn function<'a>(&'a mut self, val: CodeableConcept) -> &'a mut Procedure_PerformerBuilder {
        self.value["function"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Procedure_PerformerBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Procedure_PerformerBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn on_behalf_of<'a>(&'a mut self, val: Reference) -> &'a mut Procedure_PerformerBuilder {
        self.value["onBehalfOf"] = json!(val.value);
        return self;
    }
}
