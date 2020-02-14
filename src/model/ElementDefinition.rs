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
use crate::model::ElementDefinition_Base::ElementDefinition_Base;
use crate::model::ElementDefinition_Binding::ElementDefinition_Binding;
use crate::model::ElementDefinition_Constraint::ElementDefinition_Constraint;
use crate::model::ElementDefinition_Example::ElementDefinition_Example;
use crate::model::ElementDefinition_Mapping::ElementDefinition_Mapping;
use crate::model::ElementDefinition_Slicing::ElementDefinition_Slicing;
use crate::model::ElementDefinition_Type::ElementDefinition_Type;
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

/// Captures constraints on each element within the resource, profile, or extension.

#[derive(Debug)]
pub struct ElementDefinition<'a> {
    pub value: &'a Value,
}

impl ElementDefinition<'_> {
    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_annotation(&self) -> Option<Annotation> {
        if let Some(val) = self.value.get("patternAnnotation") {
            return Some(Annotation { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternId") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_usage_context(&self) -> Option<UsageContext> {
        if let Some(val) = self.value.get("fixedUsageContext") {
            return Some(UsageContext { value: val });
        }
        return None;
    }

    /// Extensions for patternCanonical
    pub fn _pattern_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for patternOid
    pub fn _pattern_oid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternOid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("fixedSampledData") {
            return Some(SampledData { value: val });
        }
        return None;
    }

    /// Extensions for fixedTime
    pub fn _fixed_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for patternPositiveInt
    pub fn _pattern_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternPositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("patternIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("patternQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Extensions for patternUri
    pub fn _pattern_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternUri") {
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

    /// Extensions for minValueTime
    pub fn _min_value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValueTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("fixedUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Explains how that element affects the interpretation of the resource or element
    /// that contains it.
    pub fn is_modifier_reason(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("isModifierReason") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for fixedUnsignedInt
    pub fn _fixed_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedUnsignedInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for short
    pub fn _short(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_short") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for patternDate
    pub fn _pattern_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for minValueUnsignedInt
    pub fn _min_value_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValueUnsignedInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for patternMarkdown
    pub fn _pattern_markdown(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternMarkdown") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("defaultValueTriggerDefinition") {
            return Some(TriggerDefinition { value: val });
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("minValueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_contact_detail(&self) -> Option<ContactDetail> {
        if let Some(val) = self.value.get("fixedContactDetail") {
            return Some(ContactDetail { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_human_name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("patternHumanName") {
            return Some(HumanName { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternDate") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("patternReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A reference to an invariant that may make additional statements about the
    /// cardinality or value in the instance.
    pub fn condition(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("fixedTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("minValueInstant") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_parameter_definition(&self) -> Option<ParameterDefinition> {
        if let Some(val) = self.value.get("defaultValueParameterDefinition") {
            return Some(ParameterDefinition { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("fixedAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("fixedExpression") {
            return Some(Expression { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedInstant") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for patternTime
    pub fn _pattern_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("fixedPositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("maxValueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("patternDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_contact_detail(&self) -> Option<ContactDetail> {
        if let Some(val) = self.value.get("patternContactDetail") {
            return Some(ContactDetail { value: val });
        }
        return None;
    }

    /// Extensions for alias
    pub fn _alias(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_alias") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedTime") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_count(&self) -> Option<Count> {
        if let Some(val) = self.value.get("fixedCount") {
            return Some(Count { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValuePositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("fixedMoney") {
            return Some(Money { value: val });
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

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("patternMeta") {
            return Some(Meta { value: val });
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

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedCode") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueUrl") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueCode") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("defaultValueMeta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// Extensions for meaningWhenMissing
    pub fn _meaning_when_missing(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_meaningWhenMissing") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_contributor(&self) -> Option<Contributor> {
        if let Some(val) = self.value.get("patternContributor") {
            return Some(Contributor { value: val });
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("minValueTime") {
            return Some(string);
        }
        return None;
    }

    /// A concise description of what this element means (e.g. for use in autogenerated
    /// summaries).
    pub fn short(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("short") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for maxLength
    pub fn _max_length(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxLength") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_human_name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("fixedHumanName") {
            return Some(HumanName { value: val });
        }
        return None;
    }

    /// Extensions for fixedUrl
    pub fn _fixed_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedUrl") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_related_artifact(&self) -> Option<RelatedArtifact> {
        if let Some(val) = self.value.get("fixedRelatedArtifact") {
            return Some(RelatedArtifact { value: val });
        }
        return None;
    }

    /// If true, implementations that produce or consume resources SHALL provide
    /// "support" for the element in some meaningful way.  If false, the element may be
    /// ignored and not supported. If false, whether to populate or use the data element
    /// in any way is at the discretion of the implementation.
    pub fn must_support(&self) -> Option<bool> {
        if let Some(val) = self.value.get("mustSupport") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternTime") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("fixedBoolean") {
            return Some(val.as_bool().unwrap());
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

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("maxValueTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for maxValueUnsignedInt
    pub fn _max_value_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValueUnsignedInt") {
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

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("patternPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Extensions for fixedDateTime
    pub fn _fixed_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueCanonical") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternUrl") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_related_artifact(&self) -> Option<RelatedArtifact> {
        if let Some(val) = self.value.get("defaultValueRelatedArtifact") {
            return Some(RelatedArtifact { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("defaultValueCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for isSummary
    pub fn _is_summary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isSummary") {
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

    /// Extensions for min
    pub fn _min(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_min") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Identifies an element defined elsewhere in the definition whose content rules
    /// should be applied to the current element. ContentReferences bring across all the
    /// rules that are in the ElementDefinition for the element, including definitions,
    /// cardinality constraints, bindings, invariants etc.
    pub fn content_reference(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("contentReference") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for fixedUri
    pub fn _fixed_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_contact_detail(&self) -> Option<ContactDetail> {
        if let Some(val) = self.value.get("defaultValueContactDetail") {
            return Some(ContactDetail { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_code(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternCode") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_oid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternOid") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for fixedString
    pub fn _fixed_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_oid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedOid") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("fixedAddress") {
            return Some(Address { value: val });
        }
        return None;
    }

    /// If present, indicates that the order of the repeating element has meaning and
    /// describes what that meaning is.  If absent, it means that the order of the
    /// element has no meaning.
    pub fn order_meaning(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("orderMeaning") {
            return Some(string);
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

    /// Extensions for defaultValueString
    pub fn _default_value_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueString") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_related_artifact(&self) -> Option<RelatedArtifact> {
        if let Some(val) = self.value.get("patternRelatedArtifact") {
            return Some(RelatedArtifact { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("patternAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Extensions for fixedBoolean
    pub fn _fixed_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("defaultValueReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for fixedDate
    pub fn _fixed_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for patternInstant
    pub fn _pattern_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternInstant") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for maxValuePositiveInt
    pub fn _max_value_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValuePositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_dosage(&self) -> Option<Dosage> {
        if let Some(val) = self.value.get("defaultValueDosage") {
            return Some(Dosage { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_contact_point(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("patternContactPoint") {
            return Some(ContactPoint { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fixedCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for label
    pub fn _label(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_label") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("maxValueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("defaultValueRatio") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("patternBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("fixedDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("maxValueInstant") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValueUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for patternUuid
    pub fn _pattern_uuid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternUuid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for fixedCanonical
    pub fn _fixed_canonical(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedCanonical") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for requirements
    pub fn _requirements(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_requirements") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_markdown(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternMarkdown") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("fixedCoding") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_distance(&self) -> Option<Distance> {
        if let Some(val) = self.value.get("patternDistance") {
            return Some(Distance { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_count(&self) -> Option<Count> {
        if let Some(val) = self.value.get("patternCount") {
            return Some(Count { value: val });
        }
        return None;
    }

    /// Extensions for representation
    pub fn _representation(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_representation") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Binds to a value set if this element is coded (code, Coding, CodeableConcept,
    /// Quantity), or the data types (string, uri).
    pub fn binding(&self) -> Option<ElementDefinition_Binding> {
        if let Some(val) = self.value.get("binding") {
            return Some(ElementDefinition_Binding { value: val });
        }
        return None;
    }

    /// Extensions for patternBoolean
    pub fn _pattern_boolean(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternBoolean") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for maxValueInstant
    pub fn _max_value_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValueInstant") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("minValueDateTime") {
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for maxValueDateTime
    pub fn _max_value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValueDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("defaultValueExpression") {
            return Some(Expression { value: val });
        }
        return None;
    }

    /// Extensions for orderMeaning
    pub fn _order_meaning(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_orderMeaning") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_usage_context(&self) -> Option<UsageContext> {
        if let Some(val) = self.value.get("patternUsageContext") {
            return Some(UsageContext { value: val });
        }
        return None;
    }

    /// A single preferred label which is the text to display beside the element
    /// indicating its meaning or to use to prompt for the element in a user display or
    /// form.
    pub fn label(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("label") {
            return Some(string);
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

    /// Extensions for patternString
    pub fn _pattern_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternString") {
            return Some(Element { value: val });
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

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_distance(&self) -> Option<Distance> {
        if let Some(val) = self.value.get("fixedDistance") {
            return Some(Distance { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_uuid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueUuid") {
            return Some(string);
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

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("patternAddress") {
            return Some(Address { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("patternMoney") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// Extensions for patternDecimal
    pub fn _pattern_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for minValueInstant
    pub fn _min_value_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValueInstant") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternUri") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_boolean(&self) -> Option<bool> {
        if let Some(val) = self.value.get("defaultValueBoolean") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for patternUrl
    pub fn _pattern_url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternUrl") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueDate") {
            return Some(string);
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("minValueDate") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedUri") {
            return Some(string);
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("maxValueUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternInstant") {
            return Some(string);
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("maxValueDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for sliceName
    pub fn _slice_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sliceName") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element { value: val });
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

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_parameter_definition(&self) -> Option<ParameterDefinition> {
        if let Some(val) = self.value.get("fixedParameterDefinition") {
            return Some(ParameterDefinition { value: val });
        }
        return None;
    }

    /// Extensions for fixedUuid
    pub fn _fixed_uuid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedUuid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_markdown(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueMarkdown") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("defaultValueDataRequirement") {
            return Some(DataRequirement { value: val });
        }
        return None;
    }

    /// Information about the base definition of the element, provided to make it
    /// unnecessary for tools to trace the deviation of the element through the derived
    /// and related profiles. When the element definition is not the original definition
    /// of an element - i.g. either in a constraint on another type, or for elements
    /// from a super type in a snap shot - then the information in provided in the
    /// element definition may be different to the base definition. On the original
    /// definition of the element, it will be same.
    pub fn base(&self) -> Option<ElementDefinition_Base> {
        if let Some(val) = self.value.get("base") {
            return Some(ElementDefinition_Base { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternString") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("patternTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_dosage(&self) -> Option<Dosage> {
        if let Some(val) = self.value.get("patternDosage") {
            return Some(Dosage { value: val });
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("maxValuePositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Indicates the maximum length in characters that is permitted to be present in
    /// conformant instances and which is expected to be supported by conformant
    /// consumers that support the element.
    pub fn max_length(&self) -> Option<i64> {
        if let Some(val) = self.value.get("maxLength") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// If true, indicates that this slice definition is constraining a slice definition
    /// with the same name in an inherited profile. If false, the slice is not
    /// overriding any slice in an inherited profile. If missing, the slice might or
    /// might not be overriding a slice in an inherited profile, depending on the
    /// sliceName.
    pub fn slice_is_constraining(&self) -> Option<bool> {
        if let Some(val) = self.value.get("sliceIsConstraining") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedString") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for maxValueTime
    pub fn _max_value_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValueTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The minimum number of times this element SHALL appear in the instance.
    pub fn min(&self) -> Option<u64> {
        if let Some(val) = self.value.get("min") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Extensions for sliceIsConstraining
    pub fn _slice_is_constraining(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sliceIsConstraining") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("defaultValueAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("patternDataRequirement") {
            return Some(DataRequirement { value: val });
        }
        return None;
    }

    /// A sample value for this element demonstrating the type of information that would
    /// typically be found in the element.
    pub fn example(&self) -> Option<Vec<ElementDefinition_Example>> {
        if let Some(Value::Array(val)) = self.value.get("example") {
            return Some(
                val.into_iter()
                    .map(|e| ElementDefinition_Example { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_expression(&self) -> Option<Expression> {
        if let Some(val) = self.value.get("patternExpression") {
            return Some(Expression { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("fixedSignature") {
            return Some(Signature { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternCanonical") {
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

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("patternUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Provides a complete explanation of the meaning of the data element for human
    /// readability.  For the case of elements derived from existing elements (e.g.
    /// constraints), the definition SHALL be consistent with the base definition, but
    /// convey the meaning of the element in the particular context of use of the
    /// resource. (Note: The text you are reading is specified in
    /// ElementDefinition.definition).
    pub fn definition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definition") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("fixedRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Extensions for fixedCode
    pub fn _fixed_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedCode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("fixedReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Identifies a concept from an external specification that roughly corresponds to
    /// this element.
    pub fn mapping(&self) -> Option<Vec<ElementDefinition_Mapping>> {
        if let Some(Value::Array(val)) = self.value.get("mapping") {
            return Some(
                val.into_iter()
                    .map(|e| ElementDefinition_Mapping { value: e })
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

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_contributor(&self) -> Option<Contributor> {
        if let Some(val) = self.value.get("fixedContributor") {
            return Some(Contributor { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("patternCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for minValueInteger
    pub fn _min_value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValueInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_oid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueOid") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for patternInteger
    pub fn _pattern_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The name of this element definition slice, when slicing is working. The name
    /// must be a token with no dots or spaces. This is a unique name referring to a
    /// specific set of constraints applied to this element, used to provide a name to
    /// different slices of the same element.
    pub fn slice_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sliceName") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for patternUnsignedInt
    pub fn _pattern_unsigned_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternUnsignedInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for maxValueDate
    pub fn _max_value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValueDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for condition
    pub fn _condition(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_condition") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("defaultValueAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_human_name(&self) -> Option<HumanName> {
        if let Some(val) = self.value.get("defaultValueHumanName") {
            return Some(HumanName { value: val });
        }
        return None;
    }

    /// Extensions for fixedInstant
    pub fn _fixed_instant(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedInstant") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for fixedDecimal
    pub fn _fixed_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("fixedRatio") {
            return Some(Ratio { value: val });
        }
        return None;
    }

    /// Extensions for patternBase64Binary
    pub fn _pattern_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternBase64Binary") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_contributor(&self) -> Option<Contributor> {
        if let Some(val) = self.value.get("defaultValueContributor") {
            return Some(Contributor { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("patternInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for maxValueInteger
    pub fn _max_value_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValueInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("fixedQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_annotation(&self) -> Option<Annotation> {
        if let Some(val) = self.value.get("defaultValueAnnotation") {
            return Some(Annotation { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueString") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_parameter_definition(&self) -> Option<ParameterDefinition> {
        if let Some(val) = self.value.get("patternParameterDefinition") {
            return Some(ParameterDefinition { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("fixedIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_dosage(&self) -> Option<Dosage> {
        if let Some(val) = self.value.get("fixedDosage") {
            return Some(Dosage { value: val });
        }
        return None;
    }

    /// Identifies additional names by which this element might also be known.
    pub fn alias(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("alias") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The maximum number of times this element is permitted to appear in the instance.
    pub fn max(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("max") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_canonical(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedCanonical") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("defaultValueSignature") {
            return Some(Signature { value: val });
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_unsigned_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("minValueUnsignedInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for contentReference
    pub fn _content_reference(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_contentReference") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_uuid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedUuid") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for fixedPositiveInt
    pub fn _fixed_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedPositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("patternDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("fixedInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_base_6_4_binary(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternBase64Binary") {
            return Some(string);
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("minValueDecimal") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for maxValueDecimal
    pub fn _max_value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxValueDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueUri") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("defaultValueIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Extensions for minValueDate
    pub fn _min_value_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValueDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A code that has the same meaning as the element in a particular terminology.
    pub fn code(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| Coding { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_address(&self) -> Option<Address> {
        if let Some(val) = self.value.get("defaultValueAddress") {
            return Some(Address { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("patternCoding") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("patternTriggerDefinition") {
            return Some(TriggerDefinition { value: val });
        }
        return None;
    }

    /// Extensions for minValuePositiveInt
    pub fn _min_value_positive_int(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValuePositiveInt") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("defaultValuePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_distance(&self) -> Option<Distance> {
        if let Some(val) = self.value.get("defaultValueDistance") {
            return Some(Distance { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for fixedInteger
    pub fn _fixed_integer(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedInteger") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("fixedPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Whether the element should be included if a client requests a search with the
    /// parameter _summary=true.
    pub fn is_summary(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isSummary") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for fixedMarkdown
    pub fn _fixed_markdown(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedMarkdown") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// If true, the value of this element affects the interpretation of the element or
    /// resource that contains it, and the value of the element cannot be ignored.
    /// Typically, this is used for status, negation and qualification codes. The effect
    /// of this is that the element cannot be ignored by systems: they SHALL either
    /// recognize the element and process it, and/or a pre-determination has been made
    /// that it is not relevant to their particular system.
    pub fn is_modifier(&self) -> Option<bool> {
        if let Some(val) = self.value.get("isModifier") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_count(&self) -> Option<Count> {
        if let Some(val) = self.value.get("defaultValueCount") {
            return Some(Count { value: val });
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

    /// Extensions for defaultValueBase64Binary
    pub fn _default_value_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_defaultValueBase64Binary") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternDateTime") {
            return Some(string);
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

    /// Indicates that the element is sliced into a set of alternative definitions (i.e.
    /// in a structure definition, there are multiple different constraints on a single
    /// element in the base resource). Slicing can be used in any resource that has
    /// cardinality ..* on the base resource, or any resource with a choice of types.
    /// The set of slices is any elements that come after this in the element sequence
    /// that have the same path, until a shorter path occurs (the shorter path
    /// terminates the set).
    pub fn slicing(&self) -> Option<ElementDefinition_Slicing> {
        if let Some(val) = self.value.get("slicing") {
            return Some(ElementDefinition_Slicing { value: val });
        }
        return None;
    }

    /// Explanatory notes and implementation guidance about the data element, including
    /// notes about how to use the data properly, exceptions to proper use, etc. (Note:
    /// The text you are reading is specified in ElementDefinition.comment).
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_money(&self) -> Option<Money> {
        if let Some(val) = self.value.get("defaultValueMoney") {
            return Some(Money { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("defaultValueTiming") {
            return Some(Timing { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueId") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("fixedAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("defaultValueSampledData") {
            return Some(SampledData { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_annotation(&self) -> Option<Annotation> {
        if let Some(val) = self.value.get("fixedAnnotation") {
            return Some(Annotation { value: val });
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

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_contact_point(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("defaultValueContactPoint") {
            return Some(ContactPoint { value: val });
        }
        return None;
    }

    /// Extensions for minValueDecimal
    pub fn _min_value_decimal(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValueDecimal") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("maxValueDate") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for isModifierReason
    pub fn _is_modifier_reason(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isModifierReason") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_coding(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("defaultValueCoding") {
            return Some(Coding { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedId") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_uuid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("patternUuid") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_markdown(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedMarkdown") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("patternPositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_data_requirement(&self) -> Option<DataRequirement> {
        if let Some(val) = self.value.get("fixedDataRequirement") {
            return Some(DataRequirement { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedDate") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_trigger_definition(&self) -> Option<TriggerDefinition> {
        if let Some(val) = self.value.get("fixedTriggerDefinition") {
            return Some(TriggerDefinition { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("fixedUrl") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_contact_point(&self) -> Option<ContactPoint> {
        if let Some(val) = self.value.get("fixedContactPoint") {
            return Some(ContactPoint { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("patternRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Extensions for patternId
    pub fn _pattern_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternId") {
            return Some(Element { value: val });
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

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_ratio(&self) -> Option<Ratio> {
        if let Some(val) = self.value.get("patternRatio") {
            return Some(Ratio { value: val });
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

    /// Extensions for fixedOid
    pub fn _fixed_oid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedOid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The maximum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn max_value_integer(&self) -> Option<f64> {
        if let Some(val) = self.value.get("maxValueInteger") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The data type or resource that the value of this element is permitted to be.
    pub fn fhir_type(&self) -> Option<Vec<ElementDefinition_Type>> {
        if let Some(Value::Array(val)) = self.value.get("type") {
            return Some(
                val.into_iter()
                    .map(|e| ElementDefinition_Type { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_positive_int(&self) -> Option<f64> {
        if let Some(val) = self.value.get("minValuePositiveInt") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for mustSupport
    pub fn _must_support(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_mustSupport") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// This element is for traceability of why the element was created and why the
    /// constraints exist as they do. This may be used to point to source materials or
    /// specifications that drove the structure of this element.
    pub fn requirements(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("requirements") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_decimal(&self) -> Option<f64> {
        if let Some(val) = self.value.get("defaultValueDecimal") {
            return Some(val.as_f64().unwrap());
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

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_usage_context(&self) -> Option<UsageContext> {
        if let Some(val) = self.value.get("defaultValueUsageContext") {
            return Some(UsageContext { value: val });
        }
        return None;
    }

    /// Extensions for path
    pub fn _path(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_path") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("fixedMeta") {
            return Some(Meta { value: val });
        }
        return None;
    }

    /// The path identifies the element and is expressed as a "."-separated list of
    /// ancestor elements, beginning with the name of the resource or extension.
    pub fn path(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("path") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_sampled_data(&self) -> Option<SampledData> {
        if let Some(val) = self.value.get("patternSampledData") {
            return Some(SampledData { value: val });
        }
        return None;
    }

    /// Extensions for patternDateTime
    pub fn _pattern_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The minimum allowed value for the element. The value is inclusive. This is
    /// allowed for the types date, dateTime, instant, time, decimal, integer, and
    /// Quantity.
    pub fn min_value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("minValueQuantity") {
            return Some(Quantity { value: val });
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

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueTime") {
            return Some(string);
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("defaultValueQuantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("defaultValueRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// The Implicit meaning that is to be understood when this element is missing (e.g.
    /// 'when this element is missing, the period is ongoing').
    pub fn meaning_when_missing(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("meaningWhenMissing") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for fixedId
    pub fn _fixed_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedId") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("patternAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// Extensions for isModifier
    pub fn _is_modifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_isModifier") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for fixedBase64Binary
    pub fn _fixed_base_6_4_binary(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fixedBase64Binary") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_instant(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("defaultValueInstant") {
            return Some(string);
        }
        return None;
    }

    /// Specifies a value that the value in the instance SHALL follow - that is, any
    /// value in the pattern must be found in the instance. Other additional values may
    /// be found too. This is effectively constraint by example.      When pattern[x] is
    /// used to constrain a primitive, it means that the value provided in the
    /// pattern[x] must match the instance value exactly.    When pattern[x] is used to
    /// constrain an array, it means that each element provided in the pattern[x] array
    /// must (recursively) match at least one element from the instance array.    When
    /// pattern[x] is used to constrain a complex object, it means that each property in
    /// the pattern must be present in the complex object, and its value must
    /// recursively match -- i.e.,    1. If primitive: it must match exactly the pattern
    /// value  2. If a complex object: it must match (recursively) the pattern value  3.
    /// If an array: it must match (recursively) the pattern value.
    pub fn pattern_signature(&self) -> Option<Signature> {
        if let Some(val) = self.value.get("patternSignature") {
            return Some(Signature { value: val });
        }
        return None;
    }

    /// The value that should be used if there is no value stated in the instance (e.g.
    /// 'if not otherwise specified, the abstract is false').
    pub fn default_value_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("defaultValueDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// Extensions for minValueDateTime
    pub fn _min_value_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_minValueDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Specifies a value that SHALL be exactly the value  for this element in the
    /// instance. For purposes of comparison, non-significant whitespace is ignored, and
    /// all values must be an exact match (case and accent sensitive). Missing
    /// elements/attributes must also be missing.
    pub fn fixed_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("fixedDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// Extensions for patternCode
    pub fn _pattern_code(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_patternCode") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for definition
    pub fn _definition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definition") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Formal constraints such as co-occurrence and other constraints that can be
    /// computationally evaluated within the context of the instance.
    pub fn constraint(&self) -> Option<Vec<ElementDefinition_Constraint>> {
        if let Some(Value::Array(val)) = self.value.get("constraint") {
            return Some(
                val.into_iter()
                    .map(|e| ElementDefinition_Constraint { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.pattern_annotation() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_id() {}
        if let Some(_val) = self.fixed_usage_context() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_canonical() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_oid() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_sampled_data() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_time() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_quantity() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_uri() {
            _val.validate();
        }
        if let Some(_val) = self._max() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_time() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_unsigned_int() {}
        if let Some(_val) = self.is_modifier_reason() {}
        if let Some(_val) = self._fixed_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self._short() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_date() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_markdown() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_trigger_definition() {
            _val.validate();
        }
        if let Some(_val) = self.min_value_integer() {}
        if let Some(_val) = self.fixed_contact_detail() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_human_name() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_date() {}
        if let Some(_val) = self.pattern_reference() {
            _val.validate();
        }
        if let Some(_val) = self.condition() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.fixed_timing() {
            _val.validate();
        }
        if let Some(_val) = self.min_value_instant() {}
        if let Some(_val) = self.default_value_parameter_definition() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_attachment() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_expression() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_instant() {}
        if let Some(_val) = self._pattern_time() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_positive_int() {}
        if let Some(_val) = self.max_value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_decimal() {}
        if let Some(_val) = self.pattern_contact_detail() {
            _val.validate();
        }
        if let Some(_val) = self._alias() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.default_value_base_6_4_binary() {}
        if let Some(_val) = self.fixed_time() {}
        if let Some(_val) = self.fixed_count() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_integer() {}
        if let Some(_val) = self.default_value_positive_int() {}
        if let Some(_val) = self.fixed_money() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_uri() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_meta() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_code() {}
        if let Some(_val) = self.default_value_url() {}
        if let Some(_val) = self.default_value_code() {}
        if let Some(_val) = self.default_value_meta() {
            _val.validate();
        }
        if let Some(_val) = self._meaning_when_missing() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_contributor() {
            _val.validate();
        }
        if let Some(_val) = self.min_value_time() {}
        if let Some(_val) = self.short() {}
        if let Some(_val) = self._max_length() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_human_name() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_url() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_related_artifact() {
            _val.validate();
        }
        if let Some(_val) = self.must_support() {}
        if let Some(_val) = self.pattern_time() {}
        if let Some(_val) = self.fixed_boolean() {}
        if let Some(_val) = self._default_value_oid() {
            _val.validate();
        }
        if let Some(_val) = self.max_value_time() {}
        if let Some(_val) = self._max_value_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_date() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_period() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_canonical() {}
        if let Some(_val) = self.pattern_url() {}
        if let Some(_val) = self.default_value_related_artifact() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._is_summary() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_integer() {
            _val.validate();
        }
        if let Some(_val) = self._min() {
            _val.validate();
        }
        if let Some(_val) = self.content_reference() {}
        if let Some(_val) = self._fixed_uri() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_contact_detail() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_code() {}
        if let Some(_val) = self.pattern_oid() {}
        if let Some(_val) = self._fixed_string() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_oid() {}
        if let Some(_val) = self.fixed_address() {
            _val.validate();
        }
        if let Some(_val) = self.order_meaning() {}
        if let Some(_val) = self._default_value_boolean() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_string() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_related_artifact() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_attachment() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_boolean() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_reference() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_date() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_instant() {
            _val.validate();
        }
        if let Some(_val) = self._max_value_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_dosage() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_contact_point() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._label() {
            _val.validate();
        }
        if let Some(_val) = self.max_value_date_time() {}
        if let Some(_val) = self.default_value_ratio() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_boolean() {}
        if let Some(_val) = self.fixed_decimal() {}
        if let Some(_val) = self.max_value_instant() {}
        if let Some(_val) = self.default_value_unsigned_int() {}
        if let Some(_val) = self._pattern_uuid() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_canonical() {
            _val.validate();
        }
        if let Some(_val) = self._requirements() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_markdown() {}
        if let Some(_val) = self.fixed_coding() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_distance() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_count() {
            _val.validate();
        }
        if let Some(_val) = self._representation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.binding() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_boolean() {
            _val.validate();
        }
        if let Some(_val) = self._max_value_instant() {
            _val.validate();
        }
        if let Some(_val) = self.min_value_date_time() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._max_value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_expression() {
            _val.validate();
        }
        if let Some(_val) = self._order_meaning() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_usage_context() {
            _val.validate();
        }
        if let Some(_val) = self.label() {}
        if let Some(_val) = self._default_value_instant() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_string() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_distance() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_uuid() {}
        if let Some(_val) = self._default_value_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_address() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_money() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_decimal() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_instant() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_uri() {}
        if let Some(_val) = self.default_value_boolean() {}
        if let Some(_val) = self._pattern_url() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_date() {}
        if let Some(_val) = self.min_value_date() {}
        if let Some(_val) = self.fixed_uri() {}
        if let Some(_val) = self.max_value_unsigned_int() {}
        if let Some(_val) = self.pattern_instant() {}
        if let Some(_val) = self.max_value_decimal() {}
        if let Some(_val) = self._slice_name() {
            _val.validate();
        }
        if let Some(_val) = self._comment() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_canonical() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_parameter_definition() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_uuid() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_markdown() {}
        if let Some(_val) = self.default_value_data_requirement() {
            _val.validate();
        }
        if let Some(_val) = self.base() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_string() {}
        if let Some(_val) = self.pattern_timing() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_dosage() {
            _val.validate();
        }
        if let Some(_val) = self.max_value_positive_int() {}
        if let Some(_val) = self.max_length() {}
        if let Some(_val) = self.slice_is_constraining() {}
        if let Some(_val) = self.fixed_string() {}
        if let Some(_val) = self._max_value_time() {
            _val.validate();
        }
        if let Some(_val) = self.min() {}
        if let Some(_val) = self._slice_is_constraining() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_age() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_data_requirement() {
            _val.validate();
        }
        if let Some(_val) = self.example() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.pattern_expression() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_base_6_4_binary() {}
        if let Some(_val) = self.fixed_signature() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_canonical() {}
        if let Some(_val) = self._default_value_code() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_unsigned_int() {}
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self.fixed_range() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_code() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_reference() {
            _val.validate();
        }
        if let Some(_val) = self.mapping() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.fixed_contributor() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_integer() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_oid() {}
        if let Some(_val) = self._pattern_integer() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_date_time() {}
        if let Some(_val) = self.slice_name() {}
        if let Some(_val) = self._pattern_unsigned_int() {
            _val.validate();
        }
        if let Some(_val) = self._max_value_date() {
            _val.validate();
        }
        if let Some(_val) = self._condition() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.default_value_attachment() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_human_name() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_instant() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_ratio() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_base_6_4_binary() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_contributor() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_integer() {}
        if let Some(_val) = self._max_value_integer() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_annotation() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_string() {}
        if let Some(_val) = self.pattern_parameter_definition() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_dosage() {
            _val.validate();
        }
        if let Some(_val) = self.alias() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.max() {}
        if let Some(_val) = self.fixed_canonical() {}
        if let Some(_val) = self.default_value_signature() {
            _val.validate();
        }
        if let Some(_val) = self.min_value_unsigned_int() {}
        if let Some(_val) = self._content_reference() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_uuid() {}
        if let Some(_val) = self._fixed_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_duration() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_integer() {}
        if let Some(_val) = self.pattern_base_6_4_binary() {}
        if let Some(_val) = self.min_value_decimal() {}
        if let Some(_val) = self._max_value_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_uri() {}
        if let Some(_val) = self.default_value_identifier() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_date() {
            _val.validate();
        }
        if let Some(_val) = self.code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.default_value_address() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_coding() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_trigger_definition() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_positive_int() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_period() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_distance() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_date_time() {}
        if let Some(_val) = self._fixed_integer() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_period() {
            _val.validate();
        }
        if let Some(_val) = self.is_summary() {}
        if let Some(_val) = self._fixed_markdown() {
            _val.validate();
        }
        if let Some(_val) = self.is_modifier() {}
        if let Some(_val) = self.default_value_count() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._default_value_base_6_4_binary() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_date_time() {}
        if let Some(_val) = self._default_value_url() {
            _val.validate();
        }
        if let Some(_val) = self.slicing() {
            _val.validate();
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.default_value_money() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_timing() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_id() {}
        if let Some(_val) = self.fixed_age() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_sampled_data() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_annotation() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_id() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_contact_point() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.max_value_date() {}
        if let Some(_val) = self._is_modifier_reason() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_coding() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_id() {}
        if let Some(_val) = self.pattern_uuid() {}
        if let Some(_val) = self.fixed_markdown() {}
        if let Some(_val) = self.pattern_positive_int() {}
        if let Some(_val) = self.fixed_data_requirement() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_date() {}
        if let Some(_val) = self.fixed_trigger_definition() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_url() {}
        if let Some(_val) = self.fixed_contact_point() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_range() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_id() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_time() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_ratio() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_markdown() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_oid() {
            _val.validate();
        }
        if let Some(_val) = self.max_value_integer() {}
        if let Some(_val) = self.fhir_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.min_value_positive_int() {}
        if let Some(_val) = self._must_support() {
            _val.validate();
        }
        if let Some(_val) = self.requirements() {}
        if let Some(_val) = self.default_value_decimal() {}
        if let Some(_val) = self._default_value_uuid() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_usage_context() {
            _val.validate();
        }
        if let Some(_val) = self._path() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_meta() {
            _val.validate();
        }
        if let Some(_val) = self.path() {}
        if let Some(_val) = self.pattern_sampled_data() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.min_value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self._default_value_decimal() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_time() {}
        if let Some(_val) = self.default_value_quantity() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_range() {
            _val.validate();
        }
        if let Some(_val) = self.meaning_when_missing() {}
        if let Some(_val) = self._fixed_id() {
            _val.validate();
        }
        if let Some(_val) = self.pattern_age() {
            _val.validate();
        }
        if let Some(_val) = self._is_modifier() {
            _val.validate();
        }
        if let Some(_val) = self._fixed_base_6_4_binary() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_instant() {}
        if let Some(_val) = self.pattern_signature() {
            _val.validate();
        }
        if let Some(_val) = self.default_value_duration() {
            _val.validate();
        }
        if let Some(_val) = self._min_value_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.fixed_duration() {
            _val.validate();
        }
        if let Some(_val) = self._pattern_code() {
            _val.validate();
        }
        if let Some(_val) = self._definition() {
            _val.validate();
        }
        if let Some(_val) = self.constraint() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
