#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_ResultsByExposure<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl EffectEvidenceSynthesis_ResultsByExposure<'_> {
    pub fn new(value: &Value) -> EffectEvidenceSynthesis_ResultsByExposure {
        EffectEvidenceSynthesis_ResultsByExposure {
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

    /// Extensions for exposureState
    pub fn _exposure_state(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exposureState") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Human-readable summary of results by exposure state.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Whether these results are for the exposure state or alternative exposure state.
    pub fn exposure_state(&self) -> Option<EffectEvidenceSynthesis_ResultsByExposureExposureState> {
        if let Some(Value::String(val)) = self.value.get("exposureState") {
            return Some(
                EffectEvidenceSynthesis_ResultsByExposureExposureState::from_string(&val).unwrap(),
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

    /// Reference to a RiskEvidenceSynthesis resource.
    pub fn risk_evidence_synthesis(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["riskEvidenceSynthesis"]),
        }
    }

    /// Used to define variant exposure states such as low-risk state.
    pub fn variant_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("variantState") {
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
        if let Some(_val) = self._exposure_state() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.exposure_state() {}
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
        if !self.risk_evidence_synthesis().validate() {
            return false;
        }
        if let Some(_val) = self.variant_state() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_ResultsByExposureBuilder {
    pub(crate) value: Value,
}

impl EffectEvidenceSynthesis_ResultsByExposureBuilder {
    pub fn build(&self) -> EffectEvidenceSynthesis_ResultsByExposure {
        EffectEvidenceSynthesis_ResultsByExposure {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: EffectEvidenceSynthesis_ResultsByExposure,
    ) -> EffectEvidenceSynthesis_ResultsByExposureBuilder {
        EffectEvidenceSynthesis_ResultsByExposureBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        risk_evidence_synthesis: Reference,
    ) -> EffectEvidenceSynthesis_ResultsByExposureBuilder {
        let mut __value: Value = json!({});
        __value["riskEvidenceSynthesis"] = json!(risk_evidence_synthesis.value);
        return EffectEvidenceSynthesis_ResultsByExposureBuilder { value: __value };
    }

    pub fn _description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _exposure_state<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["_exposureState"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn exposure_state<'a>(
        &'a mut self,
        val: EffectEvidenceSynthesis_ResultsByExposureExposureState,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["exposureState"] = json!(val.to_string());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn variant_state<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut EffectEvidenceSynthesis_ResultsByExposureBuilder {
        self.value["variantState"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum EffectEvidenceSynthesis_ResultsByExposureExposureState {
    Exposure,
    ExposureAlternative,
}

impl EffectEvidenceSynthesis_ResultsByExposureExposureState {
    pub fn from_string(
        string: &str,
    ) -> Option<EffectEvidenceSynthesis_ResultsByExposureExposureState> {
        match string {
            "exposure" => Some(EffectEvidenceSynthesis_ResultsByExposureExposureState::Exposure),
            "exposure-alternative" => {
                Some(EffectEvidenceSynthesis_ResultsByExposureExposureState::ExposureAlternative)
            }
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            EffectEvidenceSynthesis_ResultsByExposureExposureState::Exposure => {
                "exposure".to_string()
            }
            EffectEvidenceSynthesis_ResultsByExposureExposureState::ExposureAlternative => {
                "exposure-alternative".to_string()
            }
        }
    }
}
