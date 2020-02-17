#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Expression::Expression;
use crate::model::Extension::Extension;
use crate::model::Period::Period;
use crate::model::Timing::Timing;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// The ResearchElementDefinition resource describes a "PICO" element that knowledge
/// (evidence, assertion, recommendation) is about.

#[derive(Debug)]
pub struct ResearchElementDefinition_Characteristic<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ResearchElementDefinition_Characteristic<'_> {
    pub fn new(value: &Value) -> ResearchElementDefinition_Characteristic {
        ResearchElementDefinition_Characteristic {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for definitionCanonical
    pub fn _definition_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definitionCanonical") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for exclude
    pub fn _exclude(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_exclude") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for participantEffectiveDateTime
    pub fn _participant_effective_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_participantEffectiveDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for participantEffectiveDescription
    pub fn _participant_effective_description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_participantEffectiveDescription") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for participantEffectiveGroupMeasure
    pub fn _participant_effective_group_measure(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_participantEffectiveGroupMeasure") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for studyEffectiveDateTime
    pub fn _study_effective_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_studyEffectiveDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for studyEffectiveDescription
    pub fn _study_effective_description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_studyEffectiveDescription") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for studyEffectiveGroupMeasure
    pub fn _study_effective_group_measure(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_studyEffectiveGroupMeasure") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Define members of the research element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definitionCanonical") {
            return Some(string);
        }
        return None;
    }

    /// Define members of the research element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("definitionCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Define members of the research element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("definitionDataRequirement") {
            return Some(DataRequirement {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Define members of the research element using Codes (such as condition,
    /// medication, or observation), Expressions ( using an expression language such as
    /// FHIRPath or CQL) or DataRequirements (such as Diabetes diagnosis onset in the
    /// last year).
    pub fn definition_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("definitionExpression") {
            return Some(Expression {
                value: Cow::Borrowed(val),
            });
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

    /// Indicates what effective period the study covers.
    pub fn participant_effective_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("participantEffectiveDateTime") {
            return Some(string);
        }
        return None;
    }

    /// A narrative description of the time period the study covers.
    pub fn participant_effective_description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("participantEffectiveDescription") {
            return Some(string);
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn participant_effective_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("participantEffectiveDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates how elements are aggregated within the study effective period.
    pub fn participant_effective_group_measure(
        &self,
    ) -> Option<ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure> {
        if let Some(Value::String(val)) = self.value.get("participantEffectiveGroupMeasure") {
            return Some(ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::from_string(&val).unwrap());
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn participant_effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("participantEffectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates duration from the participant's study entry.
    pub fn participant_effective_time_from_start(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("participantEffectiveTimeFromStart") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn participant_effective_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("participantEffectiveTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn study_effective_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("studyEffectiveDateTime") {
            return Some(string);
        }
        return None;
    }

    /// A narrative description of the time period the study covers.
    pub fn study_effective_description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("studyEffectiveDescription") {
            return Some(string);
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn study_effective_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("studyEffectiveDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates how elements are aggregated within the study effective period.
    pub fn study_effective_group_measure(
        &self,
    ) -> Option<ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure> {
        if let Some(Value::String(val)) = self.value.get("studyEffectiveGroupMeasure") {
            return Some(
                ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::from_string(
                    &val,
                )
                .unwrap(),
            );
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn study_effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("studyEffectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates duration from the study initiation.
    pub fn study_effective_time_from_start(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("studyEffectiveTimeFromStart") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates what effective period the study covers.
    pub fn study_effective_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("studyEffectiveTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Specifies the UCUM unit for the outcome.
    pub fn unit_of_measure(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("unitOfMeasure") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Use UsageContext to define the members of the population, such as Age Ranges,
    /// Genders, Settings.
    pub fn usage_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("usageContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._definition_canonical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._exclude() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._participant_effective_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._participant_effective_description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._participant_effective_group_measure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._study_effective_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._study_effective_description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._study_effective_group_measure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition_canonical() {}
        if let Some(_val) = self.definition_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition_data_requirement() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.definition_expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.exclude() {}
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
        if let Some(_val) = self.participant_effective_date_time() {}
        if let Some(_val) = self.participant_effective_description() {}
        if let Some(_val) = self.participant_effective_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.participant_effective_group_measure() {}
        if let Some(_val) = self.participant_effective_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.participant_effective_time_from_start() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.participant_effective_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.study_effective_date_time() {}
        if let Some(_val) = self.study_effective_description() {}
        if let Some(_val) = self.study_effective_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.study_effective_group_measure() {}
        if let Some(_val) = self.study_effective_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.study_effective_time_from_start() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.study_effective_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.unit_of_measure() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.usage_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ResearchElementDefinition_CharacteristicBuilder {
    pub(crate) value: Value,
}

impl ResearchElementDefinition_CharacteristicBuilder {
    pub fn build(&self) -> ResearchElementDefinition_Characteristic {
        ResearchElementDefinition_Characteristic {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(
        existing: ResearchElementDefinition_Characteristic,
    ) -> ResearchElementDefinition_CharacteristicBuilder {
        ResearchElementDefinition_CharacteristicBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ResearchElementDefinition_CharacteristicBuilder {
        let mut __value: Value = json!({});
        return ResearchElementDefinition_CharacteristicBuilder { value: __value };
    }

    pub fn _definition_canonical<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_definitionCanonical"] = json!(val.value);
        return self;
    }

    pub fn _exclude<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_exclude"] = json!(val.value);
        return self;
    }

    pub fn _participant_effective_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_participantEffectiveDateTime"] = json!(val.value);
        return self;
    }

    pub fn _participant_effective_description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_participantEffectiveDescription"] = json!(val.value);
        return self;
    }

    pub fn _participant_effective_group_measure<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_participantEffectiveGroupMeasure"] = json!(val.value);
        return self;
    }

    pub fn _study_effective_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_studyEffectiveDateTime"] = json!(val.value);
        return self;
    }

    pub fn _study_effective_description<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_studyEffectiveDescription"] = json!(val.value);
        return self;
    }

    pub fn _study_effective_group_measure<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["_studyEffectiveGroupMeasure"] = json!(val.value);
        return self;
    }

    pub fn definition_canonical<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["definitionCanonical"] = json!(val);
        return self;
    }

    pub fn definition_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["definitionCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn definition_data_requirement<'a>(
        &'a mut self,
        val: DataRequirement,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["definitionDataRequirement"] = json!(val.value);
        return self;
    }

    pub fn definition_expression<'a>(
        &'a mut self,
        val: Expression,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["definitionExpression"] = json!(val.value);
        return self;
    }

    pub fn exclude<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["exclude"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn participant_effective_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["participantEffectiveDateTime"] = json!(val);
        return self;
    }

    pub fn participant_effective_description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["participantEffectiveDescription"] = json!(val);
        return self;
    }

    pub fn participant_effective_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["participantEffectiveDuration"] = json!(val.value);
        return self;
    }

    pub fn participant_effective_group_measure<'a>(
        &'a mut self,
        val: ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["participantEffectiveGroupMeasure"] = json!(val.to_string());
        return self;
    }

    pub fn participant_effective_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["participantEffectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn participant_effective_time_from_start<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["participantEffectiveTimeFromStart"] = json!(val.value);
        return self;
    }

    pub fn participant_effective_timing<'a>(
        &'a mut self,
        val: Timing,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["participantEffectiveTiming"] = json!(val.value);
        return self;
    }

    pub fn study_effective_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["studyEffectiveDateTime"] = json!(val);
        return self;
    }

    pub fn study_effective_description<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["studyEffectiveDescription"] = json!(val);
        return self;
    }

    pub fn study_effective_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["studyEffectiveDuration"] = json!(val.value);
        return self;
    }

    pub fn study_effective_group_measure<'a>(
        &'a mut self,
        val: ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["studyEffectiveGroupMeasure"] = json!(val.to_string());
        return self;
    }

    pub fn study_effective_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["studyEffectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn study_effective_time_from_start<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["studyEffectiveTimeFromStart"] = json!(val.value);
        return self;
    }

    pub fn study_effective_timing<'a>(
        &'a mut self,
        val: Timing,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["studyEffectiveTiming"] = json!(val.value);
        return self;
    }

    pub fn unit_of_measure<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["unitOfMeasure"] = json!(val.value);
        return self;
    }

    pub fn usage_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut ResearchElementDefinition_CharacteristicBuilder {
        self.value["usageContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure {
    Mean,
    Median,
    MeanOfMean,
    MeanOfMedian,
    MedianOfMean,
    MedianOfMedian,
}

impl ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure {
    pub fn from_string(
        string: &str,
    ) -> Option<ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure> {
        match string {
        "mean" => Some(ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::Mean),
        "median" => Some(ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::Median),
        "mean-of-mean" => Some(ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MeanOfMean),
        "mean-of-median" => Some(ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MeanOfMedian),
        "median-of-mean" => Some(ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MedianOfMean),
        "median-of-median" => Some(ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MedianOfMedian),
        _ => None,
    }
    }

    pub fn to_string(&self) -> String {
        match self {
        ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::Mean => "mean".to_string(),
        ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::Median => "median".to_string(),
        ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MeanOfMean => "mean-of-mean".to_string(),
        ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MeanOfMedian => "mean-of-median".to_string(),
        ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MedianOfMean => "median-of-mean".to_string(),
        ResearchElementDefinition_CharacteristicParticipantEffectiveGroupMeasure::MedianOfMedian => "median-of-median".to_string(),
    }
    }
}

#[derive(Debug)]
pub enum ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure {
    Mean,
    Median,
    MeanOfMean,
    MeanOfMedian,
    MedianOfMean,
    MedianOfMedian,
}

impl ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure {
    pub fn from_string(
        string: &str,
    ) -> Option<ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure> {
        match string {
            "mean" => {
                Some(ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::Mean)
            }
            "median" => {
                Some(ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::Median)
            }
            "mean-of-mean" => {
                Some(ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MeanOfMean)
            }
            "mean-of-median" => Some(
                ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MeanOfMedian,
            ),
            "median-of-mean" => Some(
                ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MedianOfMean,
            ),
            "median-of-median" => Some(
                ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MedianOfMedian,
            ),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::Mean => {
                "mean".to_string()
            }
            ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::Median => {
                "median".to_string()
            }
            ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MeanOfMean => {
                "mean-of-mean".to_string()
            }
            ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MeanOfMedian => {
                "mean-of-median".to_string()
            }
            ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MedianOfMean => {
                "median-of-mean".to_string()
            }
            ResearchElementDefinition_CharacteristicStudyEffectiveGroupMeasure::MedianOfMedian => {
                "median-of-median".to_string()
            }
        }
    }
}
