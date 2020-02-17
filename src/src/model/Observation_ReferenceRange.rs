#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Measurements and simple assertions made about a patient, device or other
/// subject.

#[derive(Debug)]
pub struct Observation_ReferenceRange<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Observation_ReferenceRange<'_> {
    pub fn new(value: &Value) -> Observation_ReferenceRange {
        Observation_ReferenceRange {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The age at which this reference range is applicable. This is a neonatal age
    /// (e.g. number of weeks at term) if the meaning says so.
    pub fn age(&self) -> Option<Range> {
        if let Some(val) = self.value.get("age") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Codes to indicate the target population this reference range applies to.  For
    /// example, a reference range may be based on the normal population or a particular
    /// sex or race.  Multiple `appliesTo`  are interpreted as an "AND" of the target
    /// populations.  For example, to represent a target population of African American
    /// females, both a code of female and a code for African American would be used.
    pub fn applies_to(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("appliesTo") {
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

    /// The value of the high bound of the reference range.  The high bound of the
    /// reference range endpoint is inclusive of the value (e.g.  reference range is >=5
    /// - <=9). If the high bound is omitted,  it is assumed to be meaningless (e.g.
    /// reference range is >= 2.3).
    pub fn high(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("high") {
            return Some(Quantity {
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

    /// The value of the low bound of the reference range.  The low bound of the
    /// reference range endpoint is inclusive of the value (e.g.  reference range is >=5
    /// - <=9). If the low bound is omitted,  it is assumed to be meaningless (e.g.
    /// reference range is <=2.3).
    pub fn low(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("low") {
            return Some(Quantity {
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

    /// Text based reference range in an observation which may be used when a
    /// quantitative range is not appropriate for an observation.  An example would be a
    /// reference value of "Negative" or a list or table of "normals".
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// Codes to indicate the what part of the targeted reference population it applies
    /// to. For example, the normal or therapeutic range.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.applies_to() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.high() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.low() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Observation_ReferenceRangeBuilder {
    pub(crate) value: Value,
}

impl Observation_ReferenceRangeBuilder {
    pub fn build(&self) -> Observation_ReferenceRange {
        Observation_ReferenceRange {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Observation_ReferenceRange) -> Observation_ReferenceRangeBuilder {
        Observation_ReferenceRangeBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Observation_ReferenceRangeBuilder {
        let mut __value: Value = json!({});
        return Observation_ReferenceRangeBuilder { value: __value };
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn age<'a>(&'a mut self, val: Range) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["age"] = json!(val.value);
        return self;
    }

    pub fn applies_to<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["appliesTo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn high<'a>(&'a mut self, val: Quantity) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["high"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn low<'a>(&'a mut self, val: Quantity) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["low"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Observation_ReferenceRangeBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}
