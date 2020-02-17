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

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Source<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl StructureMap_Source<'_> {
    pub fn new(value: &Value) -> StructureMap_Source {
        StructureMap_Source {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for check
    pub fn _check(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_check") {
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

    /// Extensions for context
    pub fn _context(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_context") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueBase64Binary
    pub fn _default_value_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueBase64Binary") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueBoolean
    pub fn _default_value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueBoolean") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueCanonical
    pub fn _default_value_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueCanonical") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueCode
    pub fn _default_value_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueCode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueDate
    pub fn _default_value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueDate") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueDateTime
    pub fn _default_value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueDecimal
    pub fn _default_value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueDecimal") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueId
    pub fn _default_value_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueInstant
    pub fn _default_value_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueInstant") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueInteger
    pub fn _default_value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueInteger") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueMarkdown
    pub fn _default_value_markdown(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueMarkdown") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueOid
    pub fn _default_value_oid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueOid") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValuePositiveInt
    pub fn _default_value_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValuePositiveInt") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueString
    pub fn _default_value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueString") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueTime
    pub fn _default_value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueUnsignedInt
    pub fn _default_value_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUnsignedInt") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueUri
    pub fn _default_value_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueUrl
    pub fn _default_value_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUrl") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for defaultValueUuid
    pub fn _default_value_uuid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUuid") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for element
    pub fn _element(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_element") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for listMode
    pub fn _list_mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_listMode") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for logMessage
    pub fn _log_message(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_logMessage") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for max
    pub fn _max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_max") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for variable
    pub fn _variable(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_variable") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// FHIRPath expression  - must be true or the mapping engine throws an error
    /// instead of completing.
    pub fn check(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("check") {
            return Some(string);
        }
        return None;
    }

    /// FHIRPath expression  - must be true or the rule does not apply.
    pub fn condition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("condition") {
            return Some(string);
        }
        return None;
    }

    /// Type or variable this rule applies to.
    pub fn context(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("context") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("defaultValueAddress") {
            return Some(Address {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("defaultValueAge") {
            return Some(Age {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_annotation(&self) -> Option<Annotation> {
        if let Some(val) = self.value.get("defaultValueAnnotation") {
            return Some(Annotation {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("defaultValueAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("defaultValueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueCanonical") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueCode") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("defaultValueCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("defaultValueCoding") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_contact_detail(&self) -> Option<ContactDetail> {
        if let Some(val) = self.value.get("defaultValueContactDetail") {
            return Some(ContactDetail {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_contact_point(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("defaultValueContactPoint") {
            return Some(ContactPoint {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_contributor(&self) -> Option<Contributor> {
        if let Some(val) = self.value.get("defaultValueContributor") {
            return Some(Contributor {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_count(&self) -> Option<Count> {
        if let Some(val) = self.value.get("defaultValueCount") {
            return Some(Count {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("defaultValueDataRequirement") {
            return Some(DataRequirement {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueDate") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValueDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_distance(&self) -> Option<Distance> {
        if let Some(val) = self.value.get("defaultValueDistance") {
            return Some(Distance {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_dosage(&self) -> Option<Dosage> {
        if let Some(val) = self.value.get("defaultValueDosage") {
            return Some(Dosage {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("defaultValueDuration") {
            return Some(Duration {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("defaultValueExpression") {
            return Some(Expression {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_human_name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("defaultValueHumanName") {
            return Some(HumanName {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueId") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("defaultValueIdentifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueInstant") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_markdown(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueMarkdown") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("defaultValueMeta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("defaultValueMoney") {
            return Some(Money {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_oid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueOid") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_parameter_definition(&self) -> Option<ParameterDefinition> {
        if let Some(val) = self.value.get("defaultValueParameterDefinition") {
            return Some(ParameterDefinition {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("defaultValuePeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValuePositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("defaultValueQuantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("defaultValueRange") {
            return Some(Range {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("defaultValueRatio") {
            return Some(Ratio {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("defaultValueReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_related_artifact(&self) -> Option<RelatedArtifact> {
        if let Some(val) = self.value.get("defaultValueRelatedArtifact") {
            return Some(RelatedArtifact {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("defaultValueSampledData") {
            return Some(SampledData {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("defaultValueSignature") {
            return Some(Signature {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueString") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueTime") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("defaultValueTiming") {
            return Some(Timing {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("defaultValueTriggerDefinition") {
            return Some(TriggerDefinition {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValueUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueUri") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueUrl") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_usage_context(&self) -> Option<UsageContext> {
        if let Some(val) = self.value.get("defaultValueUsageContext") {
            return Some(UsageContext {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_uuid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueUuid") {
            return Some(string);
        }
        return None;
    }

    /// Optional field for this source.
    pub fn element(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("element") {
            return Some(string);
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

    /// How to handle the list mode for this element.
    pub fn list_mode(&self) -> Option<StructureMap_SourceListMode> {
        if let Some(Value::String(val)) = self.value.get("listMode") {
            return Some(StructureMap_SourceListMode::from_string(&val).unwrap());
        }
        return None;
    }

    /// A FHIRPath expression which specifies a message to put in the transform log when
    /// content matching the source rule is found.
    pub fn log_message(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("logMessage") {
            return Some(string);
        }
        return None;
    }

    /// Specified maximum cardinality for the element - a number or a "*". This is
    /// optional; if present, it acts an implicit check on the input content (* just
    /// serves as documentation; it's the default value).
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// Specified minimum cardinality for the element. This is optional; if present, it
    /// acts an implicit check on the input content.
    pub fn min(&self) -> Option<i64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_i64().unwrap());
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

    /// Specified type for the element. This works as a condition on the mapping - use
    /// for polymorphic elements.
    pub fn fhir_type(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("type") {
            return Some(string);
        }
        return None;
    }

    /// Named context for field, if a field is specified.
    pub fn variable(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("variable") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._check() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._condition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_base_6_4_binary() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_boolean() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_canonical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_code() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_decimal() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_instant() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_integer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_markdown() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_oid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_positive_int() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_string() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_unsigned_int() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._default_value_uuid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._element() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._list_mode() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._log_message() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._max() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._min() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._variable() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.check() {}
        if let Some(_val) = self.condition() {}
        if let Some(_val) = self.context() {}
        if let Some(_val) = self.default_value_address() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_age() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_annotation() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_base_6_4_binary() {}
        if let Some(_val) = self.default_value_boolean() {}
        if let Some(_val) = self.default_value_canonical() {}
        if let Some(_val) = self.default_value_code() {}
        if let Some(_val) = self.default_value_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_coding() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_contact_detail() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_contact_point() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_contributor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_count() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_data_requirement() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_date() {}
        if let Some(_val) = self.default_value_date_time() {}
        if let Some(_val) = self.default_value_decimal() {}
        if let Some(_val) = self.default_value_distance() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_dosage() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_expression() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_human_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_id() {}
        if let Some(_val) = self.default_value_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_instant() {}
        if let Some(_val) = self.default_value_integer() {}
        if let Some(_val) = self.default_value_markdown() {}
        if let Some(_val) = self.default_value_meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_money() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_oid() {}
        if let Some(_val) = self.default_value_parameter_definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_positive_int() {}
        if let Some(_val) = self.default_value_quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_range() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_ratio() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_related_artifact() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_sampled_data() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_signature() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_string() {}
        if let Some(_val) = self.default_value_time() {}
        if let Some(_val) = self.default_value_timing() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_trigger_definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_unsigned_int() {}
        if let Some(_val) = self.default_value_uri() {}
        if let Some(_val) = self.default_value_url() {}
        if let Some(_val) = self.default_value_usage_context() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.default_value_uuid() {}
        if let Some(_val) = self.element() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.list_mode() {}
        if let Some(_val) = self.log_message() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.variable() {}
        return true;
    }
}

#[derive(Debug)]
pub struct StructureMap_SourceBuilder {
    pub(crate) value: Value,
}

impl StructureMap_SourceBuilder {
    pub fn build(&self) -> StructureMap_Source {
        StructureMap_Source {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: StructureMap_Source) -> StructureMap_SourceBuilder {
        StructureMap_SourceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> StructureMap_SourceBuilder {
        let mut __value: Value = json!({});
        return StructureMap_SourceBuilder { value: __value };
    }

    pub fn _check<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_check"] = json!(val.value);
        return self;
    }

    pub fn _condition<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_condition"] = json!(val.value);
        return self;
    }

    pub fn _context<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_context"] = json!(val.value);
        return self;
    }

    pub fn _default_value_base_6_4_binary<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueBase64Binary"] = json!(val.value);
        return self;
    }

    pub fn _default_value_boolean<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueBoolean"] = json!(val.value);
        return self;
    }

    pub fn _default_value_canonical<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueCanonical"] = json!(val.value);
        return self;
    }

    pub fn _default_value_code<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueCode"] = json!(val.value);
        return self;
    }

    pub fn _default_value_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueDate"] = json!(val.value);
        return self;
    }

    pub fn _default_value_date_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueDateTime"] = json!(val.value);
        return self;
    }

    pub fn _default_value_decimal<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueDecimal"] = json!(val.value);
        return self;
    }

    pub fn _default_value_id<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueId"] = json!(val.value);
        return self;
    }

    pub fn _default_value_instant<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueInstant"] = json!(val.value);
        return self;
    }

    pub fn _default_value_integer<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueInteger"] = json!(val.value);
        return self;
    }

    pub fn _default_value_markdown<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueMarkdown"] = json!(val.value);
        return self;
    }

    pub fn _default_value_oid<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueOid"] = json!(val.value);
        return self;
    }

    pub fn _default_value_positive_int<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValuePositiveInt"] = json!(val.value);
        return self;
    }

    pub fn _default_value_string<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueString"] = json!(val.value);
        return self;
    }

    pub fn _default_value_time<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueTime"] = json!(val.value);
        return self;
    }

    pub fn _default_value_unsigned_int<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueUnsignedInt"] = json!(val.value);
        return self;
    }

    pub fn _default_value_uri<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueUri"] = json!(val.value);
        return self;
    }

    pub fn _default_value_url<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueUrl"] = json!(val.value);
        return self;
    }

    pub fn _default_value_uuid<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["_defaultValueUuid"] = json!(val.value);
        return self;
    }

    pub fn _element<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_element"] = json!(val.value);
        return self;
    }

    pub fn _list_mode<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_listMode"] = json!(val.value);
        return self;
    }

    pub fn _log_message<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_logMessage"] = json!(val.value);
        return self;
    }

    pub fn _max<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_max"] = json!(val.value);
        return self;
    }

    pub fn _min<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_min"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn _variable<'a>(&'a mut self, val: Element) -> &'a mut StructureMap_SourceBuilder {
        self.value["_variable"] = json!(val.value);
        return self;
    }

    pub fn check<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["check"] = json!(val);
        return self;
    }

    pub fn condition<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["condition"] = json!(val);
        return self;
    }

    pub fn context<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["context"] = json!(val);
        return self;
    }

    pub fn default_value_address<'a>(
        &'a mut self,
        val: Address,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueAddress"] = json!(val.value);
        return self;
    }

    pub fn default_value_age<'a>(&'a mut self, val: Age) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueAge"] = json!(val.value);
        return self;
    }

    pub fn default_value_annotation<'a>(
        &'a mut self,
        val: Annotation,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueAnnotation"] = json!(val.value);
        return self;
    }

    pub fn default_value_attachment<'a>(
        &'a mut self,
        val: Attachment,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueAttachment"] = json!(val.value);
        return self;
    }

    pub fn default_value_base_6_4_binary<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueBase64Binary"] = json!(val);
        return self;
    }

    pub fn default_value_boolean<'a>(
        &'a mut self,
        val: bool,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueBoolean"] = json!(val);
        return self;
    }

    pub fn default_value_canonical<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueCanonical"] = json!(val);
        return self;
    }

    pub fn default_value_code<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueCode"] = json!(val);
        return self;
    }

    pub fn default_value_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn default_value_coding<'a>(
        &'a mut self,
        val: Coding,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueCoding"] = json!(val.value);
        return self;
    }

    pub fn default_value_contact_detail<'a>(
        &'a mut self,
        val: ContactDetail,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueContactDetail"] = json!(val.value);
        return self;
    }

    pub fn default_value_contact_point<'a>(
        &'a mut self,
        val: ContactPoint,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueContactPoint"] = json!(val.value);
        return self;
    }

    pub fn default_value_contributor<'a>(
        &'a mut self,
        val: Contributor,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueContributor"] = json!(val.value);
        return self;
    }

    pub fn default_value_count<'a>(&'a mut self, val: Count) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueCount"] = json!(val.value);
        return self;
    }

    pub fn default_value_data_requirement<'a>(
        &'a mut self,
        val: DataRequirement,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueDataRequirement"] = json!(val.value);
        return self;
    }

    pub fn default_value_date<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueDate"] = json!(val);
        return self;
    }

    pub fn default_value_date_time<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueDateTime"] = json!(val);
        return self;
    }

    pub fn default_value_decimal<'a>(&'a mut self, val: f64) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueDecimal"] = json!(val);
        return self;
    }

    pub fn default_value_distance<'a>(
        &'a mut self,
        val: Distance,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueDistance"] = json!(val.value);
        return self;
    }

    pub fn default_value_dosage<'a>(
        &'a mut self,
        val: Dosage,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueDosage"] = json!(val.value);
        return self;
    }

    pub fn default_value_duration<'a>(
        &'a mut self,
        val: Duration,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueDuration"] = json!(val.value);
        return self;
    }

    pub fn default_value_expression<'a>(
        &'a mut self,
        val: Expression,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueExpression"] = json!(val.value);
        return self;
    }

    pub fn default_value_human_name<'a>(
        &'a mut self,
        val: HumanName,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueHumanName"] = json!(val.value);
        return self;
    }

    pub fn default_value_id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueId"] = json!(val);
        return self;
    }

    pub fn default_value_identifier<'a>(
        &'a mut self,
        val: Identifier,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueIdentifier"] = json!(val.value);
        return self;
    }

    pub fn default_value_instant<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueInstant"] = json!(val);
        return self;
    }

    pub fn default_value_integer<'a>(&'a mut self, val: f64) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueInteger"] = json!(val);
        return self;
    }

    pub fn default_value_markdown<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueMarkdown"] = json!(val);
        return self;
    }

    pub fn default_value_meta<'a>(&'a mut self, val: Meta) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueMeta"] = json!(val.value);
        return self;
    }

    pub fn default_value_money<'a>(&'a mut self, val: Money) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueMoney"] = json!(val.value);
        return self;
    }

    pub fn default_value_oid<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueOid"] = json!(val);
        return self;
    }

    pub fn default_value_parameter_definition<'a>(
        &'a mut self,
        val: ParameterDefinition,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueParameterDefinition"] = json!(val.value);
        return self;
    }

    pub fn default_value_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValuePeriod"] = json!(val.value);
        return self;
    }

    pub fn default_value_positive_int<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValuePositiveInt"] = json!(val);
        return self;
    }

    pub fn default_value_quantity<'a>(
        &'a mut self,
        val: Quantity,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueQuantity"] = json!(val.value);
        return self;
    }

    pub fn default_value_range<'a>(&'a mut self, val: Range) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueRange"] = json!(val.value);
        return self;
    }

    pub fn default_value_ratio<'a>(&'a mut self, val: Ratio) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueRatio"] = json!(val.value);
        return self;
    }

    pub fn default_value_reference<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueReference"] = json!(val.value);
        return self;
    }

    pub fn default_value_related_artifact<'a>(
        &'a mut self,
        val: RelatedArtifact,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueRelatedArtifact"] = json!(val.value);
        return self;
    }

    pub fn default_value_sampled_data<'a>(
        &'a mut self,
        val: SampledData,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueSampledData"] = json!(val.value);
        return self;
    }

    pub fn default_value_signature<'a>(
        &'a mut self,
        val: Signature,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueSignature"] = json!(val.value);
        return self;
    }

    pub fn default_value_string<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueString"] = json!(val);
        return self;
    }

    pub fn default_value_time<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueTime"] = json!(val);
        return self;
    }

    pub fn default_value_timing<'a>(
        &'a mut self,
        val: Timing,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueTiming"] = json!(val.value);
        return self;
    }

    pub fn default_value_trigger_definition<'a>(
        &'a mut self,
        val: TriggerDefinition,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueTriggerDefinition"] = json!(val.value);
        return self;
    }

    pub fn default_value_unsigned_int<'a>(
        &'a mut self,
        val: f64,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueUnsignedInt"] = json!(val);
        return self;
    }

    pub fn default_value_uri<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueUri"] = json!(val);
        return self;
    }

    pub fn default_value_url<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueUrl"] = json!(val);
        return self;
    }

    pub fn default_value_usage_context<'a>(
        &'a mut self,
        val: UsageContext,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueUsageContext"] = json!(val.value);
        return self;
    }

    pub fn default_value_uuid<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["defaultValueUuid"] = json!(val);
        return self;
    }

    pub fn element<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["element"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut StructureMap_SourceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn list_mode<'a>(
        &'a mut self,
        val: StructureMap_SourceListMode,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["listMode"] = json!(val.to_string());
        return self;
    }

    pub fn log_message<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["logMessage"] = json!(val);
        return self;
    }

    pub fn max<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["max"] = json!(val);
        return self;
    }

    pub fn min<'a>(&'a mut self, val: i64) -> &'a mut StructureMap_SourceBuilder {
        self.value["min"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut StructureMap_SourceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["type"] = json!(val);
        return self;
    }

    pub fn variable<'a>(&'a mut self, val: &str) -> &'a mut StructureMap_SourceBuilder {
        self.value["variable"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum StructureMap_SourceListMode {
    First,
    NotFirst,
    Last,
    NotLast,
    OnlyOne,
}

impl StructureMap_SourceListMode {
    pub fn from_string(string: &str) -> Option<StructureMap_SourceListMode> {
        match string {
            "first" => Some(StructureMap_SourceListMode::First),
            "not_first" => Some(StructureMap_SourceListMode::NotFirst),
            "last" => Some(StructureMap_SourceListMode::Last),
            "not_last" => Some(StructureMap_SourceListMode::NotLast),
            "only_one" => Some(StructureMap_SourceListMode::OnlyOne),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            StructureMap_SourceListMode::First => "first".to_string(),
            StructureMap_SourceListMode::NotFirst => "not_first".to_string(),
            StructureMap_SourceListMode::Last => "last".to_string(),
            StructureMap_SourceListMode::NotLast => "not_last".to_string(),
            StructureMap_SourceListMode::OnlyOne => "only_one".to_string(),
        }
    }
}
