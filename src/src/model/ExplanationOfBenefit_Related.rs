#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_Related<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExplanationOfBenefit_Related<'_> {
    pub fn new(value: &Value) -> ExplanationOfBenefit_Related {
        ExplanationOfBenefit_Related {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Reference to a related claim.
    pub fn claim(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("claim") {
            return Some(Reference {
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

    /// An alternate organizational reference to the case or file to which this
    /// particular claim pertains.
    pub fn reference(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("reference") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code to convey how the claims are related.
    pub fn relationship(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("relationship") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.claim() {
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
        if let Some(_val) = self.reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.relationship() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ExplanationOfBenefit_RelatedBuilder {
    pub(crate) value: Value,
}

impl ExplanationOfBenefit_RelatedBuilder {
    pub fn build(&self) -> ExplanationOfBenefit_Related {
        ExplanationOfBenefit_Related {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ExplanationOfBenefit_Related) -> ExplanationOfBenefit_RelatedBuilder {
        ExplanationOfBenefit_RelatedBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ExplanationOfBenefit_RelatedBuilder {
        let mut __value: Value = json!({});
        return ExplanationOfBenefit_RelatedBuilder { value: __value };
    }

    pub fn claim<'a>(&'a mut self, val: Reference) -> &'a mut ExplanationOfBenefit_RelatedBuilder {
        self.value["claim"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_RelatedBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefit_RelatedBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_RelatedBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reference<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut ExplanationOfBenefit_RelatedBuilder {
        self.value["reference"] = json!(val.value);
        return self;
    }

    pub fn relationship<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefit_RelatedBuilder {
        self.value["relationship"] = json!(val.value);
        return self;
    }
}
