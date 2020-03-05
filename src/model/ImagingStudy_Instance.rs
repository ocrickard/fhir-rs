#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object Pair
/// Instances (SOP Instances - images or other data) acquired or produced in a
/// common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.

#[derive(Debug)]
pub struct ImagingStudy_Instance<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImagingStudy_Instance<'_> {
    pub fn new(value: &Value) -> ImagingStudy_Instance {
        ImagingStudy_Instance {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for number
    pub fn _number(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_number") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for uid
    pub fn _uid(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_uid") {
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

    /// The number of instance in the series.
    pub fn number(&self) -> Option<u64> {
        if let Some(val) = self.value.get("number") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// DICOM instance  type.
    pub fn sop_class(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["sopClass"]),
        }
    }

    /// The description of the instance.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._uid() {
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
        if let Some(_val) = self.number() {}
        if !self.sop_class().validate() {
            return false;
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.uid() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImagingStudy_InstanceBuilder {
    pub(crate) value: Value,
}

impl ImagingStudy_InstanceBuilder {
    pub fn build(&self) -> ImagingStudy_Instance {
        ImagingStudy_Instance {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImagingStudy_Instance) -> ImagingStudy_InstanceBuilder {
        ImagingStudy_InstanceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(sop_class: Coding) -> ImagingStudy_InstanceBuilder {
        let mut __value: Value = json!({});
        __value["sopClass"] = json!(sop_class.value);
        return ImagingStudy_InstanceBuilder { value: __value };
    }

    pub fn _number<'a>(&'a mut self, val: Element) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["_number"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _uid<'a>(&'a mut self, val: Element) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["_uid"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number<'a>(&'a mut self, val: u64) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["number"] = json!(val);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn uid<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_InstanceBuilder {
        self.value["uid"] = json!(val);
        return self;
    }
}
