#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::value::Value;

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object Pair
/// Instances (SOP Instances - images or other data) acquired or produced in a
/// common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.

#[derive(Debug)]
pub struct ImagingStudy_Instance<'a> {
    pub value: &'a Value,
}

impl ImagingStudy_Instance<'_> {
    /// The number of instance in the series.
    pub fn number(&self) -> Option<u64> {
        if let Some(val) = self.value.get("number") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Extensions for uid
    pub fn _uid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_uid") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// DICOM instance  type.
    pub fn sop_class(&self) -> Coding {
        Coding {
            value: &self.value["sopClass"],
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

    /// The DICOM SOP Instance UID for this image or other DICOM content.
    pub fn uid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("uid") {
            return Some(string);
        }
        return None;
    }

    /// The description of the instance.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
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

    /// Extensions for number
    pub fn _number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_number") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.number() {}
        if let Some(_val) = self._uid() {
            _val.validate();
        }
        if let Some(_val) = self._title() {
            _val.validate();
        }
        let _ = self.sop_class().validate();
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.uid() {}
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._number() {
            _val.validate();
        }
        return true;
    }
}
