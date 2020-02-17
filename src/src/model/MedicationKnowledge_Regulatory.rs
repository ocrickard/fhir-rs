#![allow(unused_imports, non_camel_case_types)]

use crate::model::Extension::Extension;
use crate::model::MedicationKnowledge_MaxDispense::MedicationKnowledge_MaxDispense;
use crate::model::MedicationKnowledge_Schedule::MedicationKnowledge_Schedule;
use crate::model::MedicationKnowledge_Substitution::MedicationKnowledge_Substitution;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Information about a medication that is used to support knowledge.

#[derive(Debug)]
pub struct MedicationKnowledge_Regulatory<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationKnowledge_Regulatory<'_> {
    pub fn new(value: &Value) -> MedicationKnowledge_Regulatory {
        MedicationKnowledge_Regulatory {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// The maximum number of units of the medication that can be dispensed in a period.
    pub fn max_dispense(&self) -> Option<MedicationKnowledge_MaxDispense> {
        if let Some(val) = self.value.get("maxDispense") {
            return Some(MedicationKnowledge_MaxDispense {
                value: Cow::Borrowed(val),
            });
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

    /// The authority that is specifying the regulations.
    pub fn regulatory_authority(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["regulatoryAuthority"]),
        }
    }

    /// Specifies the schedule of a medication in jurisdiction.
    pub fn schedule(&self) -> Option<Vec<MedicationKnowledge_Schedule>> {
        if let Some(Value::Array(val)) = self.value.get("schedule") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Schedule {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies if changes are allowed when dispensing a medication from a regulatory
    /// perspective.
    pub fn substitution(&self) -> Option<Vec<MedicationKnowledge_Substitution>> {
        if let Some(Value::Array(val)) = self.value.get("substitution") {
            return Some(
                val.into_iter()
                    .map(|e| MedicationKnowledge_Substitution {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.max_dispense() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.regulatory_authority().validate() {
            return false;
        }
        if let Some(_val) = self.schedule() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.substitution() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationKnowledge_RegulatoryBuilder {
    pub(crate) value: Value,
}

impl MedicationKnowledge_RegulatoryBuilder {
    pub fn build(&self) -> MedicationKnowledge_Regulatory {
        MedicationKnowledge_Regulatory {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MedicationKnowledge_Regulatory) -> MedicationKnowledge_RegulatoryBuilder {
        MedicationKnowledge_RegulatoryBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(regulatory_authority: Reference) -> MedicationKnowledge_RegulatoryBuilder {
        let mut __value: Value = json!({});
        __value["regulatoryAuthority"] = json!(regulatory_authority.value);
        return MedicationKnowledge_RegulatoryBuilder { value: __value };
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_RegulatoryBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationKnowledge_RegulatoryBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn max_dispense<'a>(
        &'a mut self,
        val: MedicationKnowledge_MaxDispense,
    ) -> &'a mut MedicationKnowledge_RegulatoryBuilder {
        self.value["maxDispense"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationKnowledge_RegulatoryBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn schedule<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Schedule>,
    ) -> &'a mut MedicationKnowledge_RegulatoryBuilder {
        self.value["schedule"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn substitution<'a>(
        &'a mut self,
        val: Vec<MedicationKnowledge_Substitution>,
    ) -> &'a mut MedicationKnowledge_RegulatoryBuilder {
        self.value["substitution"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}
