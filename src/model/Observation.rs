#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Observation_Component::Observation_Component;
use crate::model::Observation_ReferenceRange::Observation_ReferenceRange;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::SampledData::SampledData;
use crate::model::Timing::Timing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Measurements and simple assertions made about a patient, device or other
/// subject.

#[derive(Debug)]
pub struct Observation<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Observation<'_> {
    pub fn new(value: &Value) -> Observation {
        Observation {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for effectiveDateTime
    pub fn _effective_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for effectiveInstant
    pub fn _effective_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_effectiveInstant") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
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

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
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

    /// Extensions for valueTime
    pub fn _value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A plan, proposal or order that is fulfilled in whole or in part by this event.
    /// For example, a MedicationRequest may require a patient to have laboratory test
    /// performed before  it is dispensed.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
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

    /// Indicates the site on the subject's body where the observation was made (i.e.
    /// the target site).
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code that classifies the general type of observation being made.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
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

    /// Describes what was observed. Sometimes this is called the observation "name".
    pub fn code(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["code"]),
        }
    }

    /// Some observations have multiple component observations.  These component
    /// observations are expressed as separate code value pairs that share the same
    /// attributes.  Examples include systolic and diastolic component observations for
    /// blood pressure measurement and multiple component observations for genetics
    /// observations.
    pub fn component(&self) -> Option<Vec<Observation_Component>> {
        if let Some(Value::Array(val)) = self.value.get("component") {
            return Some(
                val.into_iter()
                    .map(|e| Observation_Component {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// These resources do not have an independent existence apart from the resource
    /// that contains them - they cannot be identified independently, and nor can they
    /// have their own independent transaction scope.
    pub fn contained(&self) -> Option<Vec<ResourceList>> {
        if let Some(Value::Array(val)) = self.value.get("contained") {
            return Some(
                val.into_iter()
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Provides a reason why the expected value in the element Observation.value[x] is
    /// missing.
    pub fn data_absent_reason(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("dataAbsentReason") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The target resource that represents a measurement from which this observation
    /// value is derived. For example, a calculated anion gap or a fetal measurement
    /// based on an ultrasound image.
    pub fn derived_from(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("derivedFrom") {
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

    /// The device used to generate the observation data.
    pub fn device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("device") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub fn effective_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub fn effective_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("effectiveInstant") {
            return Some(string);
        }
        return None;
    }

    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The time or time-period the observed value is asserted as being true. For
    /// biological subjects - e.g. human patients - this is usually called the
    /// "physiologically relevant time". This is usually either the time of the
    /// procedure or of specimen collection, but very often the source of the date/time
    /// is not known, only the date/time itself.
    pub fn effective_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("effectiveTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The healthcare event  (e.g. a patient and healthcare provider interaction)
    /// during which this observation is made.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource. To make the use of extensions safe and manageable,
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

    /// The actual focus of an observation when it is not the patient of record
    /// representing something or someone associated with the patient such as a spouse,
    /// parent, fetus, or donor. For example, fetus observations in a mother's record.
    /// The focus of an observation could also be an existing condition,  an
    /// intervention, the subject's diet,  another observation of the subject,  or a
    /// body structure such as tumor or implanted device.   An example use case would be
    /// using the Observation resource to capture whether the mother is trained to
    /// change her child's tracheostomy tube. In this example, the child is the patient
    /// of record and the mother is the focus.
    pub fn focus(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("focus") {
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

    /// This observation is a group observation (e.g. a battery, a panel of tests, a set
    /// of vital sign measurements) that includes the target as a member of the group.
    pub fn has_member(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("hasMember") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A unique identifier assigned to this observation.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a set of rules that were followed when the resource was
    /// constructed, and which must be understood when processing the content. Often,
    /// this is a reference to an implementation guide that defines the special rules
    /// along with other profiles etc.
    pub fn implicit_rules(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("implicitRules") {
            return Some(string);
        }
        return None;
    }

    /// A categorical assessment of an observation value.  For example, high, low,
    /// normal.
    pub fn interpretation(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("interpretation") {
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

    /// The date and time this version of the observation was made available to
    /// providers, typically after the results have been reviewed and verified.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
            return Some(string);
        }
        return None;
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates the mechanism used to perform the observation.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the resource and that modifies the understanding of the element
    /// that contains it and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer is allowed to define an extension, there is a set of requirements
    /// that SHALL be met as part of the definition of the extension. Applications
    /// processing a resource are required to check for modifier extensions.    Modifier
    /// extensions SHALL NOT change the meaning of any elements on Resource or
    /// DomainResource (including cannot change the meaning of modifierExtension
    /// itself).
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

    /// Comments about the observation or the results.
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

    /// A larger event of which this particular Observation is a component or step.  For
    /// example,  an observation as part of a procedure.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
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

    /// Who was responsible for asserting the observed value as "true".
    pub fn performer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
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

    /// Guidance on how to interpret the value by comparison to a normal or recommended
    /// range.  Multiple reference ranges are interpreted as an "OR".   In other words,
    /// to represent two distinct target populations, two `referenceRange` elements
    /// would be used.
    pub fn reference_range(&self) -> Option<Vec<Observation_ReferenceRange>> {
        if let Some(Value::Array(val)) = self.value.get("referenceRange") {
            return Some(
                val.into_iter()
                    .map(|e| Observation_ReferenceRange {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The specimen that was used when this observation was made.
    pub fn specimen(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("specimen") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The status of the result value.
    pub fn status(&self) -> Option<ObservationStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ObservationStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The patient, or group of patients, location, or device this observation is about
    /// and into whose record the observation is placed. If the actual focus of the
    /// observation is different from the subject (or a sample of, part, or region of
    /// the subject), the `focus` element or the `code` itself specifies the actual
    /// focus of the observation.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A human-readable narrative that contains a summary of the resource and can be
    /// used to represent the content of the resource to a human. The narrative need not
    /// encode all the structured data, but is required to contain sufficient detail to
    /// make it "clinically safe" for a human to just read the narrative. Resource
    /// definitions may define what content should be represented in the narrative to
    /// ensure clinical safety.
    pub fn text(&self) -> Option<Narrative> {
        if let Some(val) = self.value.get("text") {
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("valuePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("valueRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("valueSampledData") {
            return Some(SampledData {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// The information determined as a result of making the observation, if the
    /// information has a simple value.
    pub fn value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueTime") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._effective_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._effective_instant() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._issued() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_integer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.category() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.code().validate() {
            return false;
        }
        if let Some(_val) = self.component() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.data_absent_reason() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.derived_from() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.device() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.effective_date_time() {}
        if let Some(_val) = self.effective_instant() {}
        if let Some(_val) = self.effective_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.effective_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.encounter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.focus() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.has_member() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.interpretation() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.issued() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.method() {
            if !_val.validate() {
                return false;
            }
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
        if let Some(_val) = self.part_of() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reference_range() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.specimen() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subject() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_sampled_data() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self.value_time() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ObservationBuilder {
    pub(crate) value: Value,
}

impl ObservationBuilder {
    pub fn build(&self) -> Observation {
        Observation {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Observation) -> ObservationBuilder {
        ObservationBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(code: CodeableConcept) -> ObservationBuilder {
        let mut __value: Value = json!({});
        __value["code"] = json!(code.value);
        return ObservationBuilder { value: __value };
    }

    pub fn _effective_date_time<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_effectiveDateTime"] = json!(val.value);
        return self;
    }

    pub fn _effective_instant<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_effectiveInstant"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _issued<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_issued"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _value_boolean<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _value_date_time<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_valueDateTime"] = json!(val.value);
        return self;
    }

    pub fn _value_integer<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_valueInteger"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn _value_time<'a>(&'a mut self, val: Element) -> &'a mut ObservationBuilder {
        self.value["_valueTime"] = json!(val.value);
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ObservationBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn category<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ObservationBuilder {
        self.value["category"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn component<'a>(
        &'a mut self,
        val: Vec<Observation_Component>,
    ) -> &'a mut ObservationBuilder {
        self.value["component"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ObservationBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn data_absent_reason<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationBuilder {
        self.value["dataAbsentReason"] = json!(val.value);
        return self;
    }

    pub fn derived_from<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationBuilder {
        self.value["derivedFrom"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn device<'a>(&'a mut self, val: Reference) -> &'a mut ObservationBuilder {
        self.value["device"] = json!(val.value);
        return self;
    }

    pub fn effective_date_time<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["effectiveDateTime"] = json!(val);
        return self;
    }

    pub fn effective_instant<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["effectiveInstant"] = json!(val);
        return self;
    }

    pub fn effective_period<'a>(&'a mut self, val: Period) -> &'a mut ObservationBuilder {
        self.value["effectivePeriod"] = json!(val.value);
        return self;
    }

    pub fn effective_timing<'a>(&'a mut self, val: Timing) -> &'a mut ObservationBuilder {
        self.value["effectiveTiming"] = json!(val.value);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut ObservationBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ObservationBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn focus<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationBuilder {
        self.value["focus"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn has_member<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationBuilder {
        self.value["hasMember"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ObservationBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn interpretation<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ObservationBuilder {
        self.value["interpretation"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn issued<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["issued"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ObservationBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn method<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ObservationBuilder {
        self.value["method"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ObservationBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut ObservationBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn performer<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ObservationBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reference_range<'a>(
        &'a mut self,
        val: Vec<Observation_ReferenceRange>,
    ) -> &'a mut ObservationBuilder {
        self.value["referenceRange"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn specimen<'a>(&'a mut self, val: Reference) -> &'a mut ObservationBuilder {
        self.value["specimen"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: ObservationStatus) -> &'a mut ObservationBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Reference) -> &'a mut ObservationBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ObservationBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn value_boolean<'a>(&'a mut self, val: bool) -> &'a mut ObservationBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ObservationBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_date_time<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["valueDateTime"] = json!(val);
        return self;
    }

    pub fn value_integer<'a>(&'a mut self, val: f64) -> &'a mut ObservationBuilder {
        self.value["valueInteger"] = json!(val);
        return self;
    }

    pub fn value_period<'a>(&'a mut self, val: Period) -> &'a mut ObservationBuilder {
        self.value["valuePeriod"] = json!(val.value);
        return self;
    }

    pub fn value_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut ObservationBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_range<'a>(&'a mut self, val: Range) -> &'a mut ObservationBuilder {
        self.value["valueRange"] = json!(val.value);
        return self;
    }

    pub fn value_ratio<'a>(&'a mut self, val: Ratio) -> &'a mut ObservationBuilder {
        self.value["valueRatio"] = json!(val.value);
        return self;
    }

    pub fn value_sampled_data<'a>(&'a mut self, val: SampledData) -> &'a mut ObservationBuilder {
        self.value["valueSampledData"] = json!(val.value);
        return self;
    }

    pub fn value_string<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }

    pub fn value_time<'a>(&'a mut self, val: &str) -> &'a mut ObservationBuilder {
        self.value["valueTime"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum ObservationStatus {
    Registered,
    Preliminary,
    Final,
    Amended,
    Corrected,
    Cancelled,
    EnteredInError,
    Unknown,
}

impl ObservationStatus {
    pub fn from_string(string: &str) -> Option<ObservationStatus> {
        match string {
            "registered" => Some(ObservationStatus::Registered),
            "preliminary" => Some(ObservationStatus::Preliminary),
            "final" => Some(ObservationStatus::Final),
            "amended" => Some(ObservationStatus::Amended),
            "corrected" => Some(ObservationStatus::Corrected),
            "cancelled" => Some(ObservationStatus::Cancelled),
            "entered-in-error" => Some(ObservationStatus::EnteredInError),
            "unknown" => Some(ObservationStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ObservationStatus::Registered => "registered".to_string(),
            ObservationStatus::Preliminary => "preliminary".to_string(),
            ObservationStatus::Final => "final".to_string(),
            ObservationStatus::Amended => "amended".to_string(),
            ObservationStatus::Corrected => "corrected".to_string(),
            ObservationStatus::Cancelled => "cancelled".to_string(),
            ObservationStatus::EnteredInError => "entered-in-error".to_string(),
            ObservationStatus::Unknown => "unknown".to_string(),
        }
    }
}
