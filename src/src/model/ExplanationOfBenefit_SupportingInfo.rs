#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// This resource provides: the claim details; adjudication details from the
/// processing of a Claim; and optionally account balance information, for informing
/// the subscriber of the benefits provided.

#[derive(Debug)]
pub struct ExplanationOfBenefit_SupportingInfo<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ExplanationOfBenefit_SupportingInfo<'_> {
    pub fn new(value: &Value) -> ExplanationOfBenefit_SupportingInfo {
        ExplanationOfBenefit_SupportingInfo {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for timingDate
    pub fn _timing_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timingDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The general class of the information supplied: information; exception; accident,
    /// employment; onset, etc.
    pub fn category(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["category"]),
        }
    }

    /// System and code pertaining to the specific information regarding special
    /// conditions relating to the setting, treatment or patient  for which care is
    /// sought.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept {
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

    /// Provides the reason in the situation where a reason code is required in addition
    /// to the content.
    pub fn reason(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("reason") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A number to uniquely identify supporting information entries.
    pub fn sequence(&self) -> Option<i64> {
        if let Some(val) = self.value.get("sequence") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The date when or period to which this information refers.
    pub fn timing_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timingDate") {
            return Some(string);
        }
        return None;
    }

    /// The date when or period to which this information refers.
    pub fn timing_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("timingPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub fn value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("valueAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Additional data or information such as resources, documents, images etc.
    /// including references to the data or the actual inclusion of the data.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._timing_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_string() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.category().validate() {
            return false;
        }
        if let Some(_val) = self.code() {
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
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sequence() {}
        if let Some(_val) = self.timing_date() {}
        if let Some(_val) = self.timing_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_string() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ExplanationOfBenefit_SupportingInfoBuilder {
    pub(crate) value: Value,
}

impl ExplanationOfBenefit_SupportingInfoBuilder {
    pub fn build(&self) -> ExplanationOfBenefit_SupportingInfo {
        ExplanationOfBenefit_SupportingInfo {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ExplanationOfBenefit_SupportingInfo,
    ) -> ExplanationOfBenefit_SupportingInfoBuilder {
        ExplanationOfBenefit_SupportingInfoBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(category: CodeableConcept) -> ExplanationOfBenefit_SupportingInfoBuilder {
        let mut __value: Value = json!({});
        __value["category"] = json!(category.value);
        return ExplanationOfBenefit_SupportingInfoBuilder { value: __value };
    }

    pub fn _sequence<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["_sequence"] = json!(val.value);
        return self;
    }

    pub fn _timing_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["_timingDate"] = json!(val.value);
        return self;
    }

    pub fn _value_boolean<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn code<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["code"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason<'a>(
        &'a mut self,
        val: Coding,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["reason"] = json!(val.value);
        return self;
    }

    pub fn sequence<'a>(
        &'a mut self,
        val: i64,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["sequence"] = json!(val);
        return self;
    }

    pub fn timing_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["timingDate"] = json!(val);
        return self;
    }

    pub fn timing_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["timingPeriod"] = json!(val.value);
        return self;
    }

    pub fn value_attachment<'a>(
        &'a mut self,
        val: Attachment,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["valueAttachment"] = json!(val.value);
        return self;
    }

    pub fn value_boolean<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["valueReference"] = json!(val.value);
        return self;
    }

    pub fn value_string<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ExplanationOfBenefit_SupportingInfoBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }
}
