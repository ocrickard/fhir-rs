#![allow(unused_imports, non_camel_case_types)]

use crate::model::AdverseEvent_Causality::AdverseEvent_Causality;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Actual or  potential/avoided event causing unintended physical injury resulting
/// from or contributed to by medical care, a research study or other healthcare
/// setting factors that requires additional monitoring, treatment, or
/// hospitalization, or that results in death.

#[derive(Debug)]
pub struct AdverseEvent_SuspectEntity<'a> {
    pub value: &'a Value,
}

impl AdverseEvent_SuspectEntity<'_> {
    /// Identifies the actual instance of what caused the adverse event.  May be a
    /// substance, medication, medication administration, medication statement or a
    /// device.
    pub fn instance(&self) -> Reference {
        Reference {
            value: &self.value["instance"],
        }
    }

    /// Information on the possible cause of the event.
    pub fn causality(&self) -> Option<Vec<AdverseEvent_Causality>> {
        if let Some(Value::Array(val)) = self.value.get("causality") {
            return Some(
                val.into_iter()
                    .map(|e| AdverseEvent_Causality { value: e })
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        let _ = self.instance().validate();
        if let Some(_val) = self.causality() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        return true;
    }
}
