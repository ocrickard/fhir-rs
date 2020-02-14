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

/// A Map of relationships between 2 structures that can be used to transform data.

#[derive(Debug)]
pub struct StructureMap_Source<'a> {
    pub value: &'a Value,
}

impl StructureMap_Source<'_> {
    /// A value to use if there is no existing value in the source object.
    pub fn default_value_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("defaultValueAddress") {
            return Some(Address { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("defaultValueRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_related_artifact(&self) -> Option<RelatedArtifact> {
        if let Some(val) = self.value.get("defaultValueRelatedArtifact") {
            return Some(RelatedArtifact { value: val });
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

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueUrl") {
            return Some(string);
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

    /// Extensions for defaultValueCanonical
    pub fn _default_value_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for variable
    pub fn _variable(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_variable") {
            return Some(Element { value: val });
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

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("defaultValueMoney") {
            return Some(Money { value: val });
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

    /// Extensions for condition
    pub fn _condition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_condition") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueOid
    pub fn _default_value_oid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueOid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueDecimal
    pub fn _default_value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("defaultValueAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueMarkdown
    pub fn _default_value_markdown(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueMarkdown") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("defaultValueAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueUnsignedInt
    pub fn _default_value_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUnsignedInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueInteger
    pub fn _default_value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueDate
    pub fn _default_value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueDate") {
            return Some(Element { value: val });
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

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_count(&self) -> Option<Count> {
        if let Some(val) = self.value.get("defaultValueCount") {
            return Some(Count { value: val });
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
    pub fn default_value_distance(&self) -> Option<Distance> {
        if let Some(val) = self.value.get("defaultValueDistance") {
            return Some(Distance { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueUri
    pub fn _default_value_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_human_name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("defaultValueHumanName") {
            return Some(HumanName { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("defaultValueTriggerDefinition") {
            return Some(TriggerDefinition { value: val });
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
    pub fn default_value_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValueUnsignedInt") {
            return Some(val.as_f64().unwrap());
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

    /// FHIRPath expression  - must be true or the mapping engine throws an error
    /// instead of completing.
    pub fn check(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("check") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_contributor(&self) -> Option<Contributor> {
        if let Some(val) = self.value.get("defaultValueContributor") {
            return Some(Contributor { value: val });
        }
        return None;
    }

    /// Extensions for logMessage
    pub fn _log_message(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_logMessage") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for max
    pub fn _max(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_max") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_contact_detail(&self) -> Option<ContactDetail> {
        if let Some(val) = self.value.get("defaultValueContactDetail") {
            return Some(ContactDetail { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("defaultValueIdentifier") {
            return Some(Identifier { value: val });
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
    pub fn default_value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("defaultValueReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for defaultValuePositiveInt
    pub fn _default_value_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValuePositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueString
    pub fn _default_value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueUuid
    pub fn _default_value_uuid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUuid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueBoolean
    pub fn _default_value_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueBase64Binary
    pub fn _default_value_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueBase64Binary") {
            return Some(Element { value: val });
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

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_oid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueOid") {
            return Some(string);
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

    /// Extensions for context
    pub fn _context(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_context") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_contact_point(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("defaultValueContactPoint") {
            return Some(ContactPoint { value: val });
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

    /// Extensions for defaultValueUrl
    pub fn _default_value_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueUrl") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("defaultValueDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("defaultValuePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("defaultValueDataRequirement") {
            return Some(DataRequirement { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_parameter_definition(&self) -> Option<ParameterDefinition> {
        if let Some(val) = self.value.get("defaultValueParameterDefinition") {
            return Some(ParameterDefinition { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("defaultValueSignature") {
            return Some(Signature { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_dosage(&self) -> Option<Dosage> {
        if let Some(val) = self.value.get("defaultValueDosage") {
            return Some(Dosage { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("defaultValueMeta") {
            return Some(Meta { value: val });
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

    /// FHIRPath expression  - must be true or the rule does not apply.
    pub fn condition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("condition") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for check
    pub fn _check(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_check") {
            return Some(Element { value: val });
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

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("defaultValueQuantity") {
            return Some(Quantity { value: val });
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

    /// Extensions for defaultValueCode
    pub fn _default_value_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueCode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueInstant
    pub fn _default_value_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueInstant") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueDateTime
    pub fn _default_value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueDateTime") {
            return Some(Element { value: val });
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
    pub fn default_value_annotation(&self) -> Option<Annotation> {
        if let Some(val) = self.value.get("defaultValueAnnotation") {
            return Some(Annotation { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("defaultValueCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("defaultValueRatio") {
            return Some(Ratio { value: val });
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

    /// How to handle the list mode for this element.
    pub fn list_mode(&self) -> Option<StructureMap_SourceListMode> {
        if let Some(Value::String(val)) = self.value.get("listMode") {
            return Some(StructureMap_SourceListMode::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for defaultValueId
    pub fn _default_value_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("defaultValueExpression") {
            return Some(Expression { value: val });
        }
        return None;
    }

    /// Extensions for defaultValueTime
    pub fn _default_value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueTime") {
            return Some(Element { value: val });
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
    pub fn default_value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueString") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_usage_context(&self) -> Option<UsageContext> {
        if let Some(val) = self.value.get("defaultValueUsageContext") {
            return Some(UsageContext { value: val });
        }
        return None;
    }

    /// Extensions for listMode
    pub fn _list_mode(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_listMode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("defaultValueCoding") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("defaultValueTiming") {
            return Some(Timing { value: val });
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
    pub fn default_value_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueInstant") {
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

    /// Specified maximum cardinality for the element - a number or a "*". This is
    /// optional; if present, it acts an implicit check on the input content (* just
    /// serves as documentation; it's the default value).
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// A value to use if there is no existing value in the source object.
    pub fn default_value_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("defaultValueSampledData") {
            return Some(SampledData { value: val });
        }
        return None;
    }

    /// Extensions for element
    pub fn _element(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_element") {
            return Some(Element { value: val });
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

    /// Type or variable this rule applies to.
    pub fn context(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("context") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.default_value_address() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_range() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_related_artifact() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_uuid() {}
        if let Some(_val) = self.default_value_url() {}
        if let Some(_val) = self.default_value_positive_int() {}
        if let Some(_val) = self._default_value_canonical() {
            _val.validate();
        }
        if let Some(_val) = self._variable() {
            _val.validate();
        }
        if let Some(_val) = self.log_message() {}
        if let Some(_val) = self.default_value_money() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_canonical() {}
        if let Some(_val) = self._condition() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_oid() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_age() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_markdown() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_attachment() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_integer() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_date() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.default_value_count() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_code() {}
        if let Some(_val) = self.default_value_distance() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_uri() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_human_name() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_trigger_definition() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_time() {}
        if let Some(_val) = self.default_value_unsigned_int() {}
        if let Some(_val) = self.default_value_decimal() {}
        if let Some(_val) = self.check() {}
        if let Some(_val) = self.default_value_contributor() {
            _val.validate();
        }
        if let Some(_val) = self._log_message() {
            _val.validate();
        }
        if let Some(_val) = self._max() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_contact_detail() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_date() {}
        if let Some(_val) = self.default_value_reference() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_string() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_uuid() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_boolean() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_base_6_4_binary() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.default_value_oid() {}
        if let Some(_val) = self.default_value_markdown() {}
        if let Some(_val) = self._context() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_contact_point() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_integer() {}
        if let Some(_val) = self._default_value_url() {
            _val.validate();
        }
        if let Some(_val) = self._min() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_duration() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_period() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_data_requirement() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_parameter_definition() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_signature() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_dosage() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_meta() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.condition() {}
        if let Some(_val) = self._check() {
            _val.validate();
        }
        if let Some(_val) = self.variable() {}
        if let Some(_val) = self.default_value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.element() {}
        if let Some(_val) = self._default_value_code() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_instant() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_id() {}
        if let Some(_val) = self.default_value_annotation() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_ratio() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_uri() {}
        if let Some(_val) = self.list_mode() {}
        if let Some(_val) = self._default_value_id() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_expression() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_time() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_date_time() {}
        if let Some(_val) = self.default_value_string() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.default_value_usage_context() {
            _val.validate();
        }
        if let Some(_val) = self._list_mode() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_coding() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_timing() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_base_6_4_binary() {}
        if let Some(_val) = self.default_value_instant() {}
        if let Some(_val) = self.min() {}
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.default_value_sampled_data() {
            _val.validate();
        }
        if let Some(_val) = self._element() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_boolean() {}
        if let Some(_val) = self.context() {}
        return true;
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
}
