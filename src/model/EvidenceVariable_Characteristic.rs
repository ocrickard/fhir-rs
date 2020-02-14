#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Expression::Expression;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::Timing::Timing;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// The EvidenceVariable resource describes a "PICO" element that knowledge
/// (evidence, assertion, recommendation) is about.

#[derive(Debug)]
pub struct EvidenceVariable_Characteristic<'a> {
    pub value: &'a Value,
}

impl EvidenceVariable_Characteristic<'_> {
    /// Indicates what effective period the study covers.
    pub fn participant_effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("participantEffectivePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn participant_effective_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("participantEffectiveDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("definitionCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("definitionReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn participant_effective_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("participantEffectiveTiming") {
            return Some(Timing { value: val });
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

    /// Use UsageContext to define the members of the population, such as Age Ranges,
    /// Genders, Settings.
    pub fn usage_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("usageContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A short, natural language description of the characteristic that could be used
    /// to communicate the criteria to an end-user.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definitionCanonical") {
            return Some(string);
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn participant_effective_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("participantEffectiveDateTime") {
            return Some(string);
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

    /// When true, members with this characteristic are excluded from the element.
    pub fn exclude(&self) -> Option<bool> {
        if let Some(val) = self.value.get("exclude") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Indicates how elements are aggregated within the study effective period.
    pub fn group_measure(&self) -> Option<EvidenceVariable_CharacteristicGroupMeasure> {
        if let Some(Value::String(val)) = self.value.get("groupMeasure") {
            return Some(EvidenceVariable_CharacteristicGroupMeasure::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for participantEffectiveDateTime
    pub fn _participant_effective_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_participantEffectiveDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates duration from the participant's study entry.
    pub fn time_from_start(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("timeFromStart") {
            return Some(Duration { value: val });
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

    /// Extensions for groupMeasure
    pub fn _group_measure(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_groupMeasure") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("definitionDataRequirement") {
            return Some(DataRequirement { value: val });
        }
        return None;
    }

    /// Extensions for exclude
    pub fn _exclude(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exclude") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("definitionExpression") {
            return Some(Expression { value: val });
        }
        return None;
    }

    /// Define members of the evidence element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("definitionTriggerDefinition") {
            return Some(TriggerDefinition { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for definitionCanonical
    pub fn _definition_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definitionCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.participant_effective_period() {
            _val.validate();
        }
        if let Some(_val) = self.participant_effective_duration() {
            _val.validate();
        }
        if let Some(_val) = self.definition_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.definition_reference() {
            _val.validate();
        }
        if let Some(_val) = self.participant_effective_timing() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.usage_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.definition_canonical() {}
        if let Some(_val) = self.participant_effective_date_time() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.exclude() {}
        if let Some(_val) = self.group_measure() {}
        if let Some(_val) = self._participant_effective_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.time_from_start() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._group_measure() {
            _val.validate();
        }
        if let Some(_val) = self.definition_data_requirement() {
            _val.validate();
        }
        if let Some(_val) = self._exclude() {
            _val.validate();
        }
        if let Some(_val) = self.definition_expression() {
            _val.validate();
        }
        if let Some(_val) = self.definition_trigger_definition() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._definition_canonical() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum EvidenceVariable_CharacteristicGroupMeasure {
    Mean,
    Median,
    MeanOfMean,
    MeanOfMedian,
    MedianOfMean,
    MedianOfMedian,
}

impl EvidenceVariable_CharacteristicGroupMeasure {
    pub fn from_string(string: &str) -> Option<EvidenceVariable_CharacteristicGroupMeasure> {
        match string {
            "mean" => Some(EvidenceVariable_CharacteristicGroupMeasure::Mean),
            "median" => Some(EvidenceVariable_CharacteristicGroupMeasure::Median),
            "mean-of-mean" => Some(EvidenceVariable_CharacteristicGroupMeasure::MeanOfMean),
            "mean-of-median" => Some(EvidenceVariable_CharacteristicGroupMeasure::MeanOfMedian),
            "median-of-mean" => Some(EvidenceVariable_CharacteristicGroupMeasure::MedianOfMean),
            "median-of-median" => Some(EvidenceVariable_CharacteristicGroupMeasure::MedianOfMedian),
            _ => None,
        }
    }
}
