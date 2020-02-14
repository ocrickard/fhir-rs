#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.

#[derive(Debug)]
pub struct EffectEvidenceSynthesis_ResultsByExposure<'a> {
    pub value: &'a Value,
}

impl EffectEvidenceSynthesis_ResultsByExposure<'_> {
    /// Human-readable summary of results by exposure state.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Reference to a RiskEvidenceSynthesis resource.
    pub fn risk_evidence_synthesis(&self) -> Reference {
        Reference {
            value: &self.value["riskEvidenceSynthesis"],
        }
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
                    .map(|e| Extension { value: e })
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

    /// Extensions for exposureState
    pub fn _exposure_state(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exposureState") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Used to define variant exposure states such as low-risk state.
    pub fn variant_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("variantState") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.description() {}
        let _ = self.risk_evidence_synthesis().validate();
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._exposure_state() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.exposure_state() {}
        if let Some(_val) = self.variant_state() {
            _val.validate();
        }
        return true;
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
}
