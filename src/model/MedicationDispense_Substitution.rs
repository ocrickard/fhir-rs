#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Indicates that a medication product is to be or has been dispensed for a named
/// person/patient.  This includes a description of the medication product (supply)
/// provided and the instructions for administering the medication.  The medication
/// dispense is the result of a pharmacy system responding to a medication order.

#[derive(Debug)]
pub struct MedicationDispense_Substitution<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MedicationDispense_Substitution<'_> {
    pub fn new(value: &Value) -> MedicationDispense_Substitution {
        MedicationDispense_Substitution {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for wasSubstituted
    pub fn _was_substituted(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_wasSubstituted") {
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

    /// Indicates the reason for the substitution (or lack of substitution) from what
    /// was prescribed.
    pub fn reason(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reason") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The person or organization that has primary responsibility for the substitution.
    pub fn responsible_party(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("responsibleParty") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code signifying whether a different drug was dispensed from what was
    /// prescribed.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// True if the dispenser dispensed a different drug or product from what was
    /// prescribed.
    pub fn was_substituted(&self) -> Option<bool> {
        if let Some(val) = self.value.get("wasSubstituted") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._was_substituted() {
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
        if let Some(_val) = self.reason() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.responsible_party() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.was_substituted() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MedicationDispense_SubstitutionBuilder {
    pub(crate) value: Value,
}

impl MedicationDispense_SubstitutionBuilder {
    pub fn build(&self) -> MedicationDispense_Substitution {
        MedicationDispense_Substitution {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: MedicationDispense_Substitution,
    ) -> MedicationDispense_SubstitutionBuilder {
        MedicationDispense_SubstitutionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MedicationDispense_SubstitutionBuilder {
        let mut __value: Value = json!({});
        return MedicationDispense_SubstitutionBuilder { value: __value };
    }

    pub fn _was_substituted<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["_wasSubstituted"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["reason"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn responsible_party<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["responsibleParty"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn was_substituted<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut MedicationDispense_SubstitutionBuilder {
        self.value["wasSubstituted"] = json!(val);
        return self;
    }
}
