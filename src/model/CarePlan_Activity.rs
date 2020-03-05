#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CarePlan_Detail::CarePlan_Detail;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Describes the intention of how one or more practitioners intend to deliver care
/// for a particular patient, group or community for a period of time, possibly
/// limited to care for a specific condition or set of conditions.

#[derive(Debug)]
pub struct CarePlan_Activity<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CarePlan_Activity<'_> {
    pub fn new(value: &Value) -> CarePlan_Activity {
        CarePlan_Activity {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// A simple summary of a planned activity suitable for a general care plan system
    /// (e.g. form driven) that doesn't know about specific resources such as procedure
    /// etc.
    pub fn detail(&self) -> Option<CarePlan_Detail> {
        if let Some(val) = self.value.get("detail") {
            return Some(CarePlan_Detail {
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

    /// Identifies the outcome at the point when the status of the activity is assessed.
    /// For example, the outcome of an education activity could be patient understands
    /// (or not).
    pub fn outcome_codeable_concept(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("outcomeCodeableConcept") {
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

    /// Details of the outcome or action resulting from the activity.  The reference to
    /// an "event" resource, such as Procedure or Encounter or Observation, is the
    /// result/outcome of the activity itself.  The activity can be conveyed using
    /// CarePlan.activity.detail OR using the CarePlan.activity.reference (a reference
    /// to a “request” resource).
    pub fn outcome_reference(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("outcomeReference") {
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

    /// Notes about the adherence/status/progress of the activity.
    pub fn progress(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("progress") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The details of the proposed activity represented in a specific resource.
    pub fn reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.detail() {
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
        if let Some(_val) = self.outcome_codeable_concept() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.outcome_reference() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.progress() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reference() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct CarePlan_ActivityBuilder {
    pub(crate) value: Value,
}

impl CarePlan_ActivityBuilder {
    pub fn build(&self) -> CarePlan_Activity {
        CarePlan_Activity {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CarePlan_Activity) -> CarePlan_ActivityBuilder {
        CarePlan_ActivityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> CarePlan_ActivityBuilder {
        let mut __value: Value = json!({});
        return CarePlan_ActivityBuilder { value: __value };
    }

    pub fn detail<'a>(&'a mut self, val: CarePlan_Detail) -> &'a mut CarePlan_ActivityBuilder {
        self.value["detail"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut CarePlan_ActivityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CarePlan_ActivityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CarePlan_ActivityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome_codeable_concept<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut CarePlan_ActivityBuilder {
        self.value["outcomeCodeableConcept"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome_reference<'a>(
        &'a mut self,
        val: Vec<Reference>,
    ) -> &'a mut CarePlan_ActivityBuilder {
        self.value["outcomeReference"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn progress<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut CarePlan_ActivityBuilder {
        self.value["progress"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reference<'a>(&'a mut self, val: Reference) -> &'a mut CarePlan_ActivityBuilder {
        self.value["reference"] = json!(val.value);
        return self;
    }
}
