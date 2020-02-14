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
use serde_json::value::Value;

/// A task to be performed.

#[derive(Debug)]
pub struct Task_Output<'a> {
    pub value: &'a Value,
}

impl Task_Output<'_> {
    /// The value of the Output parameter as a basic type.
    pub fn value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("valueReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for valueBase64Binary
    pub fn _value_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBase64Binary") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueTime") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_contact_point(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("valueContactPoint") {
            return Some(ContactPoint { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("valueMeta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Extensions for valuePositiveInt
    pub fn _value_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valuePositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for valueUuid
    pub fn _value_uuid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUuid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("valueCodeableConcept") {
            return Some(CodeableConcept { value: val });
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

    /// Extensions for valueBoolean
    pub fn _value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueBoolean") {
            return Some(Element { value: val });
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

    /// The value of the Output parameter as a basic type.
    pub fn value_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("valueDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_related_artifact(&self) -> Option<RelatedArtifact> {
        if let Some(val) = self.value.get("valueRelatedArtifact") {
            return Some(RelatedArtifact { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("valueMoney") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_usage_context(&self) -> Option<UsageContext> {
        if let Some(val) = self.value.get("valueUsageContext") {
            return Some(UsageContext { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueCode") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueInteger
    pub fn _value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_markdown(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueMarkdown") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("valueAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("valueRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("valueExpression") {
            return Some(Expression { value: val });
        }
        return None;
    }

    /// The name of the Output parameter.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
        }
    }

    /// Extensions for valueDateTime
    pub fn _value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueInstant") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_dosage(&self) -> Option<Dosage> {
        if let Some(val) = self.value.get("valueDosage") {
            return Some(Dosage { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUrl") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("valueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for valueOid
    pub fn _value_oid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueOid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueCanonical") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueInstant
    pub fn _value_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueInstant") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_distance(&self) -> Option<Distance> {
        if let Some(val) = self.value.get("valueDistance") {
            return Some(Distance { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valuePositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_annotation(&self) -> Option<Annotation> {
        if let Some(val) = self.value.get("valueAnnotation") {
            return Some(Annotation { value: val });
        }
        return None;
    }

    /// Extensions for valueUrl
    pub fn _value_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUrl") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("valueTriggerDefinition") {
            return Some(TriggerDefinition { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("valueDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("valueDataRequirement") {
            return Some(DataRequirement { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueId") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueCode
    pub fn _value_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueCode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("valueAddress") {
            return Some(Address { value: val });
        }
        return None;
    }

    /// Extensions for valueUri
    pub fn _value_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for valueMarkdown
    pub fn _value_markdown(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueMarkdown") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("valueIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("valueTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_parameter_definition(&self) -> Option<ParameterDefinition> {
        if let Some(val) = self.value.get("valueParameterDefinition") {
            return Some(ParameterDefinition { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUri") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("valueCoding") {
            return Some(Coding { value: val });
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

    /// The value of the Output parameter as a basic type.
    pub fn value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDate") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueUnsignedInt
    pub fn _value_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueUnsignedInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for valueCanonical
    pub fn _value_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_oid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueOid") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_uuid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueUuid") {
            return Some(string);
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_count(&self) -> Option<Count> {
        if let Some(val) = self.value.get("valueCount") {
            return Some(Count { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("valueRatio") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("valuePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("valueAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_contributor(&self) -> Option<Contributor> {
        if let Some(val) = self.value.get("valueContributor") {
            return Some(Contributor { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("valueSignature") {
            return Some(Signature { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_contact_detail(&self) -> Option<ContactDetail> {
        if let Some(val) = self.value.get("valueContactDetail") {
            return Some(ContactDetail { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("valueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for valueDecimal
    pub fn _value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for valueId
    pub fn _value_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for valueString
    pub fn _value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for valueTime
    pub fn _value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_human_name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("valueHumanName") {
            return Some(HumanName { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("valueSampledData") {
            return Some(SampledData { value: val });
        }
        return None;
    }

    /// Extensions for valueDate
    pub fn _value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_valueDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value of the Output parameter as a basic type.
    pub fn value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("valueString") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.value_reference() {
            _val.validate();
        }
        if let Some(_val) = self._value_base_6_4_binary() {
            _val.validate();
        }
        if let Some(_val) = self.value_time() {}
        if let Some(_val) = self.value_contact_point() {
            _val.validate();
        }
        if let Some(_val) = self.value_meta() {
            _val.validate();
        }
        if let Some(_val) = self._value_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self._value_uuid() {
            _val.validate();
        }
        if let Some(_val) = self.value_integer() {}
        if let Some(_val) = self.value_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._value_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.value_duration() {
            _val.validate();
        }
        if let Some(_val) = self.value_base_6_4_binary() {}
        if let Some(_val) = self.value_related_artifact() {
            _val.validate();
        }
        if let Some(_val) = self.value_money() {
            _val.validate();
        }
        if let Some(_val) = self.value_usage_context() {
            _val.validate();
        }
        if let Some(_val) = self.value_code() {}
        if let Some(_val) = self._value_integer() {
            _val.validate();
        }
        if let Some(_val) = self.value_markdown() {}
        if let Some(_val) = self.value_attachment() {
            _val.validate();
        }
        if let Some(_val) = self.value_range() {
            _val.validate();
        }
        if let Some(_val) = self.value_expression() {
            _val.validate();
        }
        let _ = self.fhir_type().validate();
        if let Some(_val) = self._value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.value_instant() {}
        if let Some(_val) = self.value_dosage() {
            _val.validate();
        }
        if let Some(_val) = self.value_url() {}
        if let Some(_val) = self.value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self._value_oid() {
            _val.validate();
        }
        if let Some(_val) = self.value_unsigned_int() {}
        if let Some(_val) = self.value_canonical() {}
        if let Some(_val) = self._value_instant() {
            _val.validate();
        }
        if let Some(_val) = self.value_distance() {
            _val.validate();
        }
        if let Some(_val) = self.value_positive_int() {}
        if let Some(_val) = self.value_annotation() {
            _val.validate();
        }
        if let Some(_val) = self._value_url() {
            _val.validate();
        }
        if let Some(_val) = self.value_trigger_definition() {
            _val.validate();
        }
        if let Some(_val) = self.value_decimal() {}
        if let Some(_val) = self.value_data_requirement() {
            _val.validate();
        }
        if let Some(_val) = self.value_id() {}
        if let Some(_val) = self._value_code() {
            _val.validate();
        }
        if let Some(_val) = self.value_address() {
            _val.validate();
        }
        if let Some(_val) = self._value_uri() {
            _val.validate();
        }
        if let Some(_val) = self._value_markdown() {
            _val.validate();
        }
        if let Some(_val) = self.value_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.value_timing() {
            _val.validate();
        }
        if let Some(_val) = self.value_parameter_definition() {
            _val.validate();
        }
        if let Some(_val) = self.value_uri() {}
        if let Some(_val) = self.value_coding() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.value_date() {}
        if let Some(_val) = self._value_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self._value_canonical() {
            _val.validate();
        }
        if let Some(_val) = self.value_oid() {}
        if let Some(_val) = self.value_uuid() {}
        if let Some(_val) = self.value_count() {
            _val.validate();
        }
        if let Some(_val) = self.value_ratio() {
            _val.validate();
        }
        if let Some(_val) = self.value_period() {
            _val.validate();
        }
        if let Some(_val) = self.value_age() {
            _val.validate();
        }
        if let Some(_val) = self.value_contributor() {
            _val.validate();
        }
        if let Some(_val) = self.value_signature() {
            _val.validate();
        }
        if let Some(_val) = self.value_contact_detail() {
            _val.validate();
        }
        if let Some(_val) = self.value_boolean() {}
        if let Some(_val) = self._value_decimal() {
            _val.validate();
        }
        if let Some(_val) = self._value_id() {
            _val.validate();
        }
        if let Some(_val) = self.value_date_time() {}
        if let Some(_val) = self._value_string() {
            _val.validate();
        }
        if let Some(_val) = self._value_time() {
            _val.validate();
        }
        if let Some(_val) = self.value_human_name() {
            _val.validate();
        }
        if let Some(_val) = self.value_sampled_data() {
            _val.validate();
        }
        if let Some(_val) = self._value_date() {
            _val.validate();
        }
        if let Some(_val) = self.value_string() {}
        return true;
    }
}
