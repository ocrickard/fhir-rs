#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ValueSet_Contains::ValueSet_Contains;
use crate::model::ValueSet_Parameter::ValueSet_Parameter;
use serde_json::value::Value;

/// A ValueSet resource instance specifies a set of codes drawn from one or more
/// code systems, intended for use in a particular context. Value sets link between
/// [[[CodeSystem]]] definitions and their use in [coded
/// elements](terminologies.html).

#[derive(Debug)]
pub struct ValueSet_Expansion<'a> {
    pub value: &'a Value,
}

impl ValueSet_Expansion<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// The codes that are contained in the value set expansion.
    pub fn contains(&self) -> Option<Vec<ValueSet_Contains>> {
        if let Some(Value::Array(val)) = self.value.get("contains") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Contains { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for identifier
    pub fn _identifier(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identifier") {
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

    /// Extensions for timestamp
    pub fn _timestamp(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timestamp") {
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

    /// An identifier that uniquely identifies this expansion of the valueset, based on
    /// a unique combination of the provided parameters, the system default parameters,
    /// and the underlying system code system versions etc. Systems may re-use the same
    /// identifier as long as those factors remain the same, and the expansion is the
    /// same, but are not required to do so. This is a business identifier.
    pub fn identifier(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identifier") {
            return Some(string);
        }
        return None;
    }

    /// The total number of concepts in the expansion. If the number of concept nodes in
    /// this resource is less than the stated number, then the server can return more
    /// using the offset parameter.
    pub fn total(&self) -> Option<i64> {
        if let Some(val) = self.value.get("total") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for total
    pub fn _total(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_total") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// If paging is being used, the offset at which this resource starts.  I.e. this
    /// resource is a partial view into the expansion. If paging is not being used, this
    /// element SHALL NOT be present.
    pub fn offset(&self) -> Option<i64> {
        if let Some(val) = self.value.get("offset") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The time at which the expansion was produced by the expanding system.
    pub fn timestamp(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timestamp") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for offset
    pub fn _offset(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_offset") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A parameter that controlled the expansion process. These parameters may be used
    /// by users of expanded value sets to check whether the expansion is suitable for a
    /// particular purpose, or to pick the correct expansion.
    pub fn parameter(&self) -> Option<Vec<ValueSet_Parameter>> {
        if let Some(Value::Array(val)) = self.value.get("parameter") {
            return Some(
                val.into_iter()
                    .map(|e| ValueSet_Parameter { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.contains() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._identifier() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._timestamp() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {}
        if let Some(_val) = self.total() {}
        if let Some(_val) = self._total() {
            _val.validate();
        }
        if let Some(_val) = self.offset() {}
        if let Some(_val) = self.timestamp() {}
        if let Some(_val) = self._offset() {
            _val.validate();
        }
        if let Some(_val) = self.parameter() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
