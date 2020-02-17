#![allow(unused_imports, non_camel_case_types)]

use crate::model::Address::Address;
use crate::model::Age::Age;
use crate::model::Annotation::Annotation;
use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Coding::Coding;
use crate::model::ContactDetail::ContactDetail;
use crate::model::ContactPoint::ContactPoint;
use crate::model::Contributor::Contributor;
use crate::model::Count::Count;
use crate::model::DataRequirement::DataRequirement;
use crate::model::Distance::Distance;
use crate::model::Dosage::Dosage;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Expression::Expression;
use crate::model::Extension::Extension;
use crate::model::HumanName::HumanName;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Money::Money;
use crate::model::ParameterDefinition::ParameterDefinition;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Ratio::Ratio;
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::SampledData::SampledData;
use crate::model::Signature::Signature;
use crate::model::Timing::Timing;
use crate::model::TriggerDefinition::TriggerDefinition;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A task to be performed.

#[derive(Debug)]
pub struct Task_Input<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Task_Input<'_> {
    pub fn new(value: &Value) -> Task_Input {
        Task_Input {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for valueBase64Binary
    pub fn _value_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBase64Binary") {
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

    /// Extensions for valueCanonical
    pub fn _value_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueCanonical") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueCode
    pub fn _value_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueCode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueDate
    pub fn _value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDate") {
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

    /// Extensions for valueDecimal
    pub fn _value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDecimal") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueId
    pub fn _value_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueInstant
    pub fn _value_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInstant") {
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

    /// Extensions for valueMarkdown
    pub fn _value_markdown(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueMarkdown") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueOid
    pub fn _value_oid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueOid") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valuePositiveInt
    pub fn _value_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valuePositiveInt") {
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

    /// Extensions for valueUnsignedInt
    pub fn _value_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUnsignedInt") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueUri
    pub fn _value_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueUrl
    pub fn _value_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUrl") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for valueUuid
    pub fn _value_uuid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUuid") {
            return Some(Element {
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

    /// A code or description indicating how the input is intended to be used as part of
    /// the task execution.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["type"]),
        }
    }

    /// The value of the input parameter as a basic type.
    pub fn value_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("valueAddress") {
            return Some(Address {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("valueAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_annotation(&self) -> Option<Annotation> {
        if let Some(val) = self.value.get("valueAnnotation") {
            return Some(Annotation {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("valueAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueCanonical") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueCode") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("valueCoding") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_contact_detail(&self) -> Option<ContactDetail> {
        if let Some(val) = self.value.get("valueContactDetail") {
            return Some(ContactDetail {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_contact_point(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("valueContactPoint") {
            return Some(ContactPoint {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_contributor(&self) -> Option<Contributor> {
        if let Some(val) = self.value.get("valueContributor") {
            return Some(Contributor {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_count(&self) -> Option<Count> {
        if let Some(val) = self.value.get("valueCount") {
            return Some(Count {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("valueDataRequirement") {
            return Some(DataRequirement {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDate") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_distance(&self) -> Option<Distance> {
        if let Some(val) = self.value.get("valueDistance") {
            return Some(Distance {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_dosage(&self) -> Option<Dosage> {
        if let Some(val) = self.value.get("valueDosage") {
            return Some(Dosage {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("valueDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("valueExpression") {
            return Some(Expression {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_human_name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("valueHumanName") {
            return Some(HumanName {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueId") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("valueIdentifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueInstant") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_markdown(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueMarkdown") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("valueMeta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("valueMoney") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_oid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueOid") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_parameter_definition(&self) -> Option<ParameterDefinition> {
        if let Some(val) = self.value.get("valueParameterDefinition") {
            return Some(ParameterDefinition {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("valuePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valuePositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("valueRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_related_artifact(&self) -> Option<RelatedArtifact> {
        if let Some(val) = self.value.get("valueRelatedArtifact") {
            return Some(RelatedArtifact {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("valueSampledData") {
            return Some(SampledData {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("valueSignature") {
            return Some(Signature {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueTime") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("valueTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("valueTriggerDefinition") {
            return Some(TriggerDefinition {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUri") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUrl") {
            return Some(string);
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_usage_context(&self) -> Option<UsageContext> {
        if let Some(val) = self.value.get("valueUsageContext") {
            return Some(UsageContext {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The value of the input parameter as a basic type.
    pub fn value_uuid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUuid") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._value_base_6_4_binary() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_canonical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_decimal() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_instant() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_integer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_markdown() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_oid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_positive_int() {
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
        if let Some(_val) = self._value_unsigned_int() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._value_uuid() {
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
        if !self.fhir_type().validate() {
            return false;
        }
        if let Some(_val) = self.value_address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_annotation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_base_6_4_binary() {}
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self.value_canonical() {}
        if let Some(_val) = self.value_code() {}
        if let Some(_val) = self.value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_coding() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_contact_detail() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_contact_point() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_contributor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_data_requirement() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_date() {}
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self.value_decimal() {}
        if let Some(_val) = self.value_distance() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_dosage() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_human_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_id() {}
        if let Some(_val) = self.value_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_instant() {}
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_markdown() {}
        if let Some(_val) = self.value_meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_money() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_oid() {}
        if let Some(_val) = self.value_parameter_definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_positive_int() {}
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
        if let Some(_val) = self.value_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_related_artifact() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_sampled_data() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_signature() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_string() {}
        if let Some(_val) = self.value_time() {}
        if let Some(_val) = self.value_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_trigger_definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_unsigned_int() {}
        if let Some(_val) = self.value_uri() {}
        if let Some(_val) = self.value_url() {}
        if let Some(_val) = self.value_usage_context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.value_uuid() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Task_InputBuilder {
    pub(crate) value: Value,
}

impl Task_InputBuilder {
    pub fn build(&self) -> Task_Input {
        Task_Input {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Task_Input) -> Task_InputBuilder {
        Task_InputBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(fhir_type: CodeableConcept) -> Task_InputBuilder {
        let mut __value: Value = json!({});
        __value["type"] = json!(fhir_type.value);
        return Task_InputBuilder { value: __value };
    }

    pub fn _value_base_6_4_binary<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueBase64Binary"] = json!(val.value);
        return self;
    }

    pub fn _value_boolean<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _value_canonical<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueCanonical"] = json!(val.value);
        return self;
    }

    pub fn _value_code<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueCode"] = json!(val.value);
        return self;
    }

    pub fn _value_date<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueDate"] = json!(val.value);
        return self;
    }

    pub fn _value_date_time<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueDateTime"] = json!(val.value);
        return self;
    }

    pub fn _value_decimal<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueDecimal"] = json!(val.value);
        return self;
    }

    pub fn _value_id<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueId"] = json!(val.value);
        return self;
    }

    pub fn _value_instant<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueInstant"] = json!(val.value);
        return self;
    }

    pub fn _value_integer<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueInteger"] = json!(val.value);
        return self;
    }

    pub fn _value_markdown<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueMarkdown"] = json!(val.value);
        return self;
    }

    pub fn _value_oid<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueOid"] = json!(val.value);
        return self;
    }

    pub fn _value_positive_int<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valuePositiveInt"] = json!(val.value);
        return self;
    }

    pub fn _value_string<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueString"] = json!(val.value);
        return self;
    }

    pub fn _value_time<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueTime"] = json!(val.value);
        return self;
    }

    pub fn _value_unsigned_int<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueUnsignedInt"] = json!(val.value);
        return self;
    }

    pub fn _value_uri<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueUri"] = json!(val.value);
        return self;
    }

    pub fn _value_url<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueUrl"] = json!(val.value);
        return self;
    }

    pub fn _value_uuid<'a>(&'a mut self, val: Element) -> &'a mut Task_InputBuilder {
        self.value["_valueUuid"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Task_InputBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Task_InputBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn value_address<'a>(&'a mut self, val: Address) -> &'a mut Task_InputBuilder {
        self.value["valueAddress"] = json!(val.value);
        return self;
    }

    pub fn value_age<'a>(&'a mut self, val: Age) -> &'a mut Task_InputBuilder {
        self.value["valueAge"] = json!(val.value);
        return self;
    }

    pub fn value_annotation<'a>(&'a mut self, val: Annotation) -> &'a mut Task_InputBuilder {
        self.value["valueAnnotation"] = json!(val.value);
        return self;
    }

    pub fn value_attachment<'a>(&'a mut self, val: Attachment) -> &'a mut Task_InputBuilder {
        self.value["valueAttachment"] = json!(val.value);
        return self;
    }

    pub fn value_base_6_4_binary<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueBase64Binary"] = json!(val);
        return self;
    }

    pub fn value_boolean<'a>(&'a mut self, val: bool) -> &'a mut Task_InputBuilder {
        self.value["valueBoolean"] = json!(val);
        return self;
    }

    pub fn value_canonical<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueCanonical"] = json!(val);
        return self;
    }

    pub fn value_code<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueCode"] = json!(val);
        return self;
    }

    pub fn value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut Task_InputBuilder {
        self.value["valueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn value_coding<'a>(&'a mut self, val: Coding) -> &'a mut Task_InputBuilder {
        self.value["valueCoding"] = json!(val.value);
        return self;
    }

    pub fn value_contact_detail<'a>(&'a mut self, val: ContactDetail) -> &'a mut Task_InputBuilder {
        self.value["valueContactDetail"] = json!(val.value);
        return self;
    }

    pub fn value_contact_point<'a>(&'a mut self, val: ContactPoint) -> &'a mut Task_InputBuilder {
        self.value["valueContactPoint"] = json!(val.value);
        return self;
    }

    pub fn value_contributor<'a>(&'a mut self, val: Contributor) -> &'a mut Task_InputBuilder {
        self.value["valueContributor"] = json!(val.value);
        return self;
    }

    pub fn value_count<'a>(&'a mut self, val: Count) -> &'a mut Task_InputBuilder {
        self.value["valueCount"] = json!(val.value);
        return self;
    }

    pub fn value_data_requirement<'a>(
        &'a mut self,
        val: DataRequirement,
    ) -> &'a mut Task_InputBuilder {
        self.value["valueDataRequirement"] = json!(val.value);
        return self;
    }

    pub fn value_date<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueDate"] = json!(val);
        return self;
    }

    pub fn value_date_time<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueDateTime"] = json!(val);
        return self;
    }

    pub fn value_decimal<'a>(&'a mut self, val: f64) -> &'a mut Task_InputBuilder {
        self.value["valueDecimal"] = json!(val);
        return self;
    }

    pub fn value_distance<'a>(&'a mut self, val: Distance) -> &'a mut Task_InputBuilder {
        self.value["valueDistance"] = json!(val.value);
        return self;
    }

    pub fn value_dosage<'a>(&'a mut self, val: Dosage) -> &'a mut Task_InputBuilder {
        self.value["valueDosage"] = json!(val.value);
        return self;
    }

    pub fn value_duration<'a>(&'a mut self, val: Duration) -> &'a mut Task_InputBuilder {
        self.value["valueDuration"] = json!(val.value);
        return self;
    }

    pub fn value_expression<'a>(&'a mut self, val: Expression) -> &'a mut Task_InputBuilder {
        self.value["valueExpression"] = json!(val.value);
        return self;
    }

    pub fn value_human_name<'a>(&'a mut self, val: HumanName) -> &'a mut Task_InputBuilder {
        self.value["valueHumanName"] = json!(val.value);
        return self;
    }

    pub fn value_id<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueId"] = json!(val);
        return self;
    }

    pub fn value_identifier<'a>(&'a mut self, val: Identifier) -> &'a mut Task_InputBuilder {
        self.value["valueIdentifier"] = json!(val.value);
        return self;
    }

    pub fn value_instant<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueInstant"] = json!(val);
        return self;
    }

    pub fn value_integer<'a>(&'a mut self, val: f64) -> &'a mut Task_InputBuilder {
        self.value["valueInteger"] = json!(val);
        return self;
    }

    pub fn value_markdown<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueMarkdown"] = json!(val);
        return self;
    }

    pub fn value_meta<'a>(&'a mut self, val: Meta) -> &'a mut Task_InputBuilder {
        self.value["valueMeta"] = json!(val.value);
        return self;
    }

    pub fn value_money<'a>(&'a mut self, val: Money) -> &'a mut Task_InputBuilder {
        self.value["valueMoney"] = json!(val.value);
        return self;
    }

    pub fn value_oid<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueOid"] = json!(val);
        return self;
    }

    pub fn value_parameter_definition<'a>(
        &'a mut self,
        val: ParameterDefinition,
    ) -> &'a mut Task_InputBuilder {
        self.value["valueParameterDefinition"] = json!(val.value);
        return self;
    }

    pub fn value_period<'a>(&'a mut self, val: Period) -> &'a mut Task_InputBuilder {
        self.value["valuePeriod"] = json!(val.value);
        return self;
    }

    pub fn value_positive_int<'a>(&'a mut self, val: f64) -> &'a mut Task_InputBuilder {
        self.value["valuePositiveInt"] = json!(val);
        return self;
    }

    pub fn value_quantity<'a>(&'a mut self, val: Quantity) -> &'a mut Task_InputBuilder {
        self.value["valueQuantity"] = json!(val.value);
        return self;
    }

    pub fn value_range<'a>(&'a mut self, val: Range) -> &'a mut Task_InputBuilder {
        self.value["valueRange"] = json!(val.value);
        return self;
    }

    pub fn value_ratio<'a>(&'a mut self, val: Ratio) -> &'a mut Task_InputBuilder {
        self.value["valueRatio"] = json!(val.value);
        return self;
    }

    pub fn value_reference<'a>(&'a mut self, val: Reference) -> &'a mut Task_InputBuilder {
        self.value["valueReference"] = json!(val.value);
        return self;
    }

    pub fn value_related_artifact<'a>(
        &'a mut self,
        val: RelatedArtifact,
    ) -> &'a mut Task_InputBuilder {
        self.value["valueRelatedArtifact"] = json!(val.value);
        return self;
    }

    pub fn value_sampled_data<'a>(&'a mut self, val: SampledData) -> &'a mut Task_InputBuilder {
        self.value["valueSampledData"] = json!(val.value);
        return self;
    }

    pub fn value_signature<'a>(&'a mut self, val: Signature) -> &'a mut Task_InputBuilder {
        self.value["valueSignature"] = json!(val.value);
        return self;
    }

    pub fn value_string<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueString"] = json!(val);
        return self;
    }

    pub fn value_time<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueTime"] = json!(val);
        return self;
    }

    pub fn value_timing<'a>(&'a mut self, val: Timing) -> &'a mut Task_InputBuilder {
        self.value["valueTiming"] = json!(val.value);
        return self;
    }

    pub fn value_trigger_definition<'a>(
        &'a mut self,
        val: TriggerDefinition,
    ) -> &'a mut Task_InputBuilder {
        self.value["valueTriggerDefinition"] = json!(val.value);
        return self;
    }

    pub fn value_unsigned_int<'a>(&'a mut self, val: f64) -> &'a mut Task_InputBuilder {
        self.value["valueUnsignedInt"] = json!(val);
        return self;
    }

    pub fn value_uri<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueUri"] = json!(val);
        return self;
    }

    pub fn value_url<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueUrl"] = json!(val);
        return self;
    }

    pub fn value_usage_context<'a>(&'a mut self, val: UsageContext) -> &'a mut Task_InputBuilder {
        self.value["valueUsageContext"] = json!(val.value);
        return self;
    }

    pub fn value_uuid<'a>(&'a mut self, val: &str) -> &'a mut Task_InputBuilder {
        self.value["valueUuid"] = json!(val);
        return self;
    }
}
