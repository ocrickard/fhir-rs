#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Risk of harmful or undesirable, physiological response which is unique to an
/// individual and associated with exposure to a substance.

#[derive(Debug)]
pub struct AllergyIntolerance_Reaction<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl AllergyIntolerance_Reaction<'_> {
    pub fn new(value: &Value) -> AllergyIntolerance_Reaction {
        AllergyIntolerance_Reaction {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for onset
    pub fn _onset(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_onset") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for severity
    pub fn _severity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_severity") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Text description about the reaction as a whole, including details of the
    /// manifestation if required.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Identification of the route by which the subject was exposed to the substance.
    pub fn exposure_route(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("exposureRoute") {
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

    /// Clinical symptoms and/or signs that are observed or associated with the adverse
    /// reaction event.
    pub fn manifestation(&self) -> Vec<CodeableConcept> {
        self.value
            .get("manifestation")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| CodeableConcept {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
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

    /// Additional text about the adverse reaction event not captured in other fields.
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

    /// Record of the date and/or time of the onset of the Reaction.
    pub fn onset(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("onset") {
            return Some(string);
        }
        return None;
    }

    /// Clinical assessment of the severity of the reaction event as a whole,
    /// potentially considering multiple different manifestations.
    pub fn severity(&self) -> Option<AllergyIntolerance_ReactionSeverity> {
        if let Some(Value::String(val)) = self.value.get("severity") {
            return Some(AllergyIntolerance_ReactionSeverity::from_string(&val).unwrap());
        }
        return None;
    }

    /// Identification of the specific substance (or pharmaceutical product) considered
    /// to be responsible for the Adverse Reaction event. Note: the substance for a
    /// specific reaction may be different from the substance identified as the cause of
    /// the risk, but it must be consistent with it. For instance, it may be a more
    /// specific substance (e.g. a brand medication) or a composite product that
    /// includes the identified substance. It must be clinically safe to only process
    /// the 'code' and ignore the 'reaction.substance'.  If a receiving system is unable
    /// to confirm that AllergyIntolerance.reaction.substance falls within the semantic
    /// scope of AllergyIntolerance.code, then the receiving system should ignore
    /// AllergyIntolerance.reaction.substance.
    pub fn substance(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("substance") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._onset() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._severity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.exposure_route() {
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
        if !self
            .manifestation()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
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
        if let Some(_val) = self.onset() {}
        if let Some(_val) = self.severity() {}
        if let Some(_val) = self.substance() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct AllergyIntolerance_ReactionBuilder {
    pub(crate) value: Value,
}

impl AllergyIntolerance_ReactionBuilder {
    pub fn build(&self) -> AllergyIntolerance_Reaction {
        AllergyIntolerance_Reaction {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: AllergyIntolerance_Reaction) -> AllergyIntolerance_ReactionBuilder {
        AllergyIntolerance_ReactionBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(manifestation: Vec<CodeableConcept>) -> AllergyIntolerance_ReactionBuilder {
        let mut __value: Value = json!({});
        __value["manifestation"] = json!(manifestation
            .into_iter()
            .map(|e| e.value)
            .collect::<Vec<_>>());
        return AllergyIntolerance_ReactionBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _onset<'a>(&'a mut self, val: Element) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["_onset"] = json!(val.value);
        return self;
    }

    pub fn _severity<'a>(&'a mut self, val: Element) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["_severity"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn exposure_route<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["exposureRoute"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(
        &'a mut self,
        val: Vec<Annotation>,
    ) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn onset<'a>(&'a mut self, val: &str) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["onset"] = json!(val);
        return self;
    }

    pub fn severity<'a>(
        &'a mut self,
        val: AllergyIntolerance_ReactionSeverity,
    ) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["severity"] = json!(val.to_string());
        return self;
    }

    pub fn substance<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut AllergyIntolerance_ReactionBuilder {
        self.value["substance"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum AllergyIntolerance_ReactionSeverity {
    Mild,
    Moderate,
    Severe,
}

impl AllergyIntolerance_ReactionSeverity {
    pub fn from_string(string: &str) -> Option<AllergyIntolerance_ReactionSeverity> {
        match string {
            "mild" => Some(AllergyIntolerance_ReactionSeverity::Mild),
            "moderate" => Some(AllergyIntolerance_ReactionSeverity::Moderate),
            "severe" => Some(AllergyIntolerance_ReactionSeverity::Severe),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AllergyIntolerance_ReactionSeverity::Mild => "mild".to_string(),
            AllergyIntolerance_ReactionSeverity::Moderate => "moderate".to_string(),
            AllergyIntolerance_ReactionSeverity::Severe => "severe".to_string(),
        }
    }
}
