#![allow(unused_imports, non_camel_case_types)]

use crate::model::Age::Age;
use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Significant health conditions for a person related to the patient relevant in
/// the context of care for the patient.

#[derive(Debug)]
pub struct FamilyMemberHistory_Condition<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl FamilyMemberHistory_Condition<'_> {
    pub fn new(value: &Value) -> FamilyMemberHistory_Condition {
        FamilyMemberHistory_Condition {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for contributedToDeath
    pub fn _contributed_to_death(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contributedToDeath") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for onsetString
    pub fn _onset_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onsetString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The actual condition specified. Could be a coded condition (like MI or Diabetes)
    /// or a less specific string like 'cancer' depending on how much is known about the
    /// condition and the capabilities of the creating system.
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// This condition contributed to the cause of death of the related person. If
    /// contributedToDeath is not populated, then it is unknown.
    pub fn contributed_to_death(&self) -> Option<bool> {
        if let Some(val) = self.value.get("contributedToDeath") {
            return Some(val.as_bool().unwrap());
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

    /// An area where general notes can be placed about this specific condition.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
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

    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("onsetAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("onsetPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("onsetRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Either the age of onset, range of approximate age or descriptive string can be
    /// recorded.  For conditions with multiple occurrences, this describes the first
    /// known occurrence.
    pub fn onset_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onsetString") {
            return Some(string);
        }
        return None;
    }

    /// Indicates what happened following the condition.  If the condition resulted in
    /// death, deceased date is captured on the relation.
    pub fn outcome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("outcome") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._contributed_to_death() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._onset_string() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.contributed_to_death() {}
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
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.onset_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.onset_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.onset_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.onset_string() {}
        if let Some(_val) = self.outcome() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct FamilyMemberHistory_ConditionBuilder {
    pub(crate) value: Value,
}

impl FamilyMemberHistory_ConditionBuilder {
    pub fn build(&self) -> FamilyMemberHistory_Condition {
        FamilyMemberHistory_Condition {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: FamilyMemberHistory_Condition) -> FamilyMemberHistory_ConditionBuilder {
        FamilyMemberHistory_ConditionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> FamilyMemberHistory_ConditionBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return FamilyMemberHistory_ConditionBuilder { value: __value };
    }

    pub fn _contributed_to_death<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["_contributedToDeath"] = json!(val.value);
        return self;
    }

    pub fn _onset_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["_onsetString"] = json!(val.value);
        return self;
    }

    pub fn contributed_to_death<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["contributedToDeath"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(
        &'a mut self,
        val: Vec<Annotation>,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn onset_age<'a>(&'a mut self, val: Age) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["onsetAge"] = json!(val.value);
        return self;
    }

    pub fn onset_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["onsetPeriod"] = json!(val.value);
        return self;
    }

    pub fn onset_range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["onsetRange"] = json!(val.value);
        return self;
    }

    pub fn onset_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["onsetString"] = json!(val);
        return self;
    }

    pub fn outcome<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut FamilyMemberHistory_ConditionBuilder {
        self.value["outcome"] = json!(val.value);
        return self;
    }
}
