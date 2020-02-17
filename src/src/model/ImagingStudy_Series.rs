#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ImagingStudy_Instance::ImagingStudy_Instance;
use crate::model::ImagingStudy_Performer::ImagingStudy_Performer;
use crate::model::Reference::Reference;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Representation of the content produced in a DICOM imaging study. A study
/// comprises a set of series, each of which includes a set of Service-Object Pair
/// Instances (SOP Instances - images or other data) acquired or produced in a
/// common context.  A series is of only one modality (e.g. X-ray, CT, MR,
/// ultrasound), but a study may have multiple series of different modalities.

#[derive(Debug)]
pub struct ImagingStudy_Series<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImagingStudy_Series<'_> {
    pub fn new(value: &Value) -> ImagingStudy_Series {
        ImagingStudy_Series {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
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

    /// Extensions for numberOfInstances
    pub fn _number_of_instances(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfInstances") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for started
    pub fn _started(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_started") {
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

    /// The anatomic structures examined. See DICOM Part 16 Annex L
    /// (http://dicom.nema.org/medical/dicom/current/output/chtml/part16/chapter_L.html)
    /// for DICOM to SNOMED-CT mappings. The bodySite may indicate the laterality of
    /// body part imaged; if so, it shall be consistent with any content of
    /// ImagingStudy.series.laterality.
    pub fn body_site(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A description of the series.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The network service providing access (e.g., query, view, or retrieval) for this
    /// series. See implementation notes for information about using DICOM endpoints. A
    /// series-level endpoint, if present, has precedence over a study-level endpoint
    /// with the same Endpoint.connectionType.
    pub fn endpoint(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("endpoint") {
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

    /// A single SOP instance within the series, e.g. an image, or presentation state.
    pub fn instance(&self) -> Option<Vec<ImagingStudy_Instance>> {
        if let Some(Value::Array(val)) = self.value.get("instance") {
            return Some(
                val.into_iter()
                    .map(|e| ImagingStudy_Instance {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The laterality of the (possibly paired) anatomic structures examined. E.g., the
    /// left knee, both lungs, or unpaired abdomen. If present, shall be consistent with
    /// any laterality information indicated in ImagingStudy.series.bodySite.
    pub fn laterality(&self) -> Option<Coding> {
        if let Some(val) = self.value.get("laterality") {
            return Some(Coding {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The modality of this series sequence.
    pub fn modality(&self) -> Coding {
        Coding {
            value: Cow::Borrowed(&self.value["modality"]),
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The numeric identifier of this series in the study.
    pub fn number(&self) -> Option<u64> {
        if let Some(val) = self.value.get("number") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Number of SOP Instances in the Study. The value given may be larger than the
    /// number of instance elements this resource contains due to resource availability,
    /// security, or other factors. This element should be present if any instance
    /// elements are present.
    pub fn number_of_instances(&self) -> Option<u64> {
        if let Some(val) = self.value.get("numberOfInstances") {
            return Some(val.as_u64().unwrap());
        }
        return None;
    }

    /// Indicates who or what performed the series and how they were involved.
    pub fn performer(&self) -> Option<Vec<ImagingStudy_Performer>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
            return Some(
                val.into_iter()
                    .map(|e| ImagingStudy_Performer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The specimen imaged, e.g., for whole slide imaging of a biopsy.
    pub fn specimen(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("specimen") {
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

    /// The date and time the series was started.
    pub fn started(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("started") {
            return Some(string);
        }
        return None;
    }

    /// The DICOM Series Instance UID for the series.
    pub fn uid(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("uid") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._number_of_instances() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._started() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._uid() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.endpoint() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.instance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.laterality() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.modality().validate() {
            return false;
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.number() {}
        if let Some(_val) = self.number_of_instances() {}
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.specimen() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.started() {}
        if let Some(_val) = self.uid() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImagingStudy_SeriesBuilder {
    pub(crate) value: Value,
}

impl ImagingStudy_SeriesBuilder {
    pub fn build(&self) -> ImagingStudy_Series {
        ImagingStudy_Series {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImagingStudy_Series) -> ImagingStudy_SeriesBuilder {
        ImagingStudy_SeriesBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(modality: Coding) -> ImagingStudy_SeriesBuilder {
        let mut __value: Value = json!({});
        __value["modality"] = json!(modality.value);
        return ImagingStudy_SeriesBuilder { value: __value };
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _number<'a>(&'a mut self, val: Element) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["_number"] = json!(val.value);
        return self;
    }

    pub fn _number_of_instances<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["_numberOfInstances"] = json!(val.value);
        return self;
    }

    pub fn _started<'a>(&'a mut self, val: Element) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["_started"] = json!(val.value);
        return self;
    }

    pub fn _uid<'a>(&'a mut self, val: Element) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["_uid"] = json!(val.value);
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: Coding) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn endpoint<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["endpoint"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn instance<'a>(
        &'a mut self,
        val: Vec<ImagingStudy_Instance>,
    ) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["instance"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn laterality<'a>(&'a mut self, val: Coding) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["laterality"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number<'a>(&'a mut self, val: u64) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["number"] = json!(val);
        return self;
    }

    pub fn number_of_instances<'a>(&'a mut self, val: u64) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["numberOfInstances"] = json!(val);
        return self;
    }

    pub fn performer<'a>(
        &'a mut self,
        val: Vec<ImagingStudy_Performer>,
    ) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn specimen<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["specimen"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn started<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["started"] = json!(val);
        return self;
    }

    pub fn uid<'a>(&'a mut self, val: &str) -> &'a mut ImagingStudy_SeriesBuilder {
        self.value["uid"] = json!(val);
        return self;
    }
}
