#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Range::Range;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Set of definitional characteristics for a kind of observation or measurement
/// produced or consumed by an orderable health care service.

#[derive(Debug)]
pub struct ObservationDefinition_QualifiedInterval<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ObservationDefinition_QualifiedInterval<'_> {
    pub fn new(value: &Value) -> ObservationDefinition_QualifiedInterval {
        ObservationDefinition_QualifiedInterval {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for category
    pub fn _category(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_category") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for condition
    pub fn _condition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_condition") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for gender
    pub fn _gender(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gender") {
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

    /// Codes to indicate the target population this reference range applies to.
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

    /// The category of interval of values for continuous or ordinal observations
    /// conforming to this ObservationDefinition.
    pub fn category(&self) -> Option<ObservationDefinition_QualifiedIntervalCategory> {
        if let Some(Value::String(val)) = self.value.get("category") {
            return Some(
                ObservationDefinition_QualifiedIntervalCategory::from_string(&val).unwrap(),
            );
        }
        return None;
    }

    /// Text based condition for which the reference range is valid.
    pub fn condition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("condition") {
            return Some(string);
        }
        return None;
    }

    /// Codes to indicate the health context the range applies to. For example, the
    /// normal or therapeutic range.
    pub fn context(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("context") {
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

    /// Sex of the population the range applies to.
    pub fn gender(&self) -> Option<ObservationDefinition_QualifiedIntervalGender> {
        if let Some(Value::String(val)) = self.value.get("gender") {
            return Some(ObservationDefinition_QualifiedIntervalGender::from_string(&val).unwrap());
        }
        return None;
    }

    /// The gestational age to which this reference range is applicable, in the context
    /// of pregnancy.
    pub fn gestational_age(&self) -> Option<Range> {
        if let Some(val) = self.value.get("gestationalAge") {
            return Some(Range {
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

    /// The low and high values determining the interval. There may be only one of the
    /// two.
    pub fn range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("range") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._category() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._condition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._gender() {
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
        if let Some(_val) = self.category() {}
        if let Some(_val) = self.condition() {}
        if let Some(_val) = self.context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.gender() {}
        if let Some(_val) = self.gestational_age() {
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
        if let Some(_val) = self.range() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ObservationDefinition_QualifiedIntervalBuilder {
    pub(crate) value: Value,
}

impl ObservationDefinition_QualifiedIntervalBuilder {
    pub fn build(&self) -> ObservationDefinition_QualifiedInterval {
        ObservationDefinition_QualifiedInterval {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ObservationDefinition_QualifiedInterval,
    ) -> ObservationDefinition_QualifiedIntervalBuilder {
        ObservationDefinition_QualifiedIntervalBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ObservationDefinition_QualifiedIntervalBuilder {
        let mut __value: Value = json!({});
        return ObservationDefinition_QualifiedIntervalBuilder { value: __value };
    }

    pub fn _category<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["_category"] = json!(val.value);
        return self;
    }

    pub fn _condition<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["_condition"] = json!(val.value);
        return self;
    }

    pub fn _gender<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["_gender"] = json!(val.value);
        return self;
    }

    pub fn age<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["age"] = json!(val.value);
        return self;
    }

    pub fn applies_to<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["appliesTo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn category<'a>(
        &'a mut self,
        val: ObservationDefinition_QualifiedIntervalCategory,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["category"] = json!(val.to_string());
        return self;
    }

    pub fn condition<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["condition"] = json!(val);
        return self;
    }

    pub fn context<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["context"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn gender<'a>(
        &'a mut self,
        val: ObservationDefinition_QualifiedIntervalGender,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["gender"] = json!(val.to_string());
        return self;
    }

    pub fn gestational_age<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["gestationalAge"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn range<'a>(
        &'a mut self,
        val: Range,
    ) -> &'a mut ObservationDefinition_QualifiedIntervalBuilder {
        self.value["range"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum ObservationDefinition_QualifiedIntervalCategory {
    Reference,
    Critical,
    Absolute,
}

impl ObservationDefinition_QualifiedIntervalCategory {
    pub fn from_string(string: &str) -> Option<ObservationDefinition_QualifiedIntervalCategory> {
        match string {
            "reference" => Some(ObservationDefinition_QualifiedIntervalCategory::Reference),
            "critical" => Some(ObservationDefinition_QualifiedIntervalCategory::Critical),
            "absolute" => Some(ObservationDefinition_QualifiedIntervalCategory::Absolute),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ObservationDefinition_QualifiedIntervalCategory::Reference => "reference".to_string(),
            ObservationDefinition_QualifiedIntervalCategory::Critical => "critical".to_string(),
            ObservationDefinition_QualifiedIntervalCategory::Absolute => "absolute".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ObservationDefinition_QualifiedIntervalGender {
    Male,
    Female,
    Other,
    Unknown,
}

impl ObservationDefinition_QualifiedIntervalGender {
    pub fn from_string(string: &str) -> Option<ObservationDefinition_QualifiedIntervalGender> {
        match string {
            "male" => Some(ObservationDefinition_QualifiedIntervalGender::Male),
            "female" => Some(ObservationDefinition_QualifiedIntervalGender::Female),
            "other" => Some(ObservationDefinition_QualifiedIntervalGender::Other),
            "unknown" => Some(ObservationDefinition_QualifiedIntervalGender::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ObservationDefinition_QualifiedIntervalGender::Male => "male".to_string(),
            ObservationDefinition_QualifiedIntervalGender::Female => "female".to_string(),
            ObservationDefinition_QualifiedIntervalGender::Other => "other".to_string(),
            ObservationDefinition_QualifiedIntervalGender::Unknown => "unknown".to_string(),
        }
    }
}
