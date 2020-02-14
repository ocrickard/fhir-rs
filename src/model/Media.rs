#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A photo, video, or audio recording acquired or used in healthcare. The actual
/// content may be inline or provided by direct reference.

#[derive(Debug)]
pub struct Media<'a> {
    pub value: &'a Value,
}

impl Media<'_> {
    /// The name of the device / manufacturer of the device  that was used to make the
    /// recording.
    pub fn device_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deviceName") {
            return Some(string);
        }
        return None;
    }

    /// A larger event of which this particular event is a component or step.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for frames
    pub fn _frames(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frames") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Details of the type of the media - usually, how it was acquired (what type of
    /// device). If images sourced from a DICOM system, are wrapped in a Media resource,
    /// then this is the modality.
    pub fn modality(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("modality") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Who/What this Media is a record of.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// A procedure that is fulfilled in whole or in part by the creation of this media.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Describes why the event occurred in coded or textual form.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element { value: val });
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
                    .map(|e| ResourceList { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The name of the imaging view e.g. Lateral or Antero-posterior (AP).
    pub fn view(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("view") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Width of the image in pixels (photo/video).
    pub fn width(&self) -> Option<i64> {
        if let Some(val) = self.value.get("width") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Comments made about the media by the performer, subject or other participants.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
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

    /// Extensions for duration
    pub fn _duration(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_duration") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A code that classifies whether the media is an image, video or audio recording
    /// or some other media category.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for deviceName
    pub fn _device_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deviceName") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates the site on the subject's body where the observation was made (i.e.
    /// the target site).
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The device used to collect the media.
    pub fn device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("device") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Height of the image in pixels (photo/video).
    pub fn height(&self) -> Option<i64> {
        if let Some(val) = self.value.get("height") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for width
    pub fn _width(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_width") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date and time(s) at which the media was collected.
    pub fn created_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("createdDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for height
    pub fn _height(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_height") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The number of frames in a photo. This is used with a multi-page fax, or an
    /// imaging acquisition context that takes multiple slices in a single image, or an
    /// animated gif. If there is more than one frame, this SHALL have a value in order
    /// to alert interface software that a multi-frame capable rendering widget is
    /// required.
    pub fn frames(&self) -> Option<i64> {
        if let Some(val) = self.value.get("frames") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The duration of the recording in seconds - for audio and video.
    pub fn duration(&self) -> Option<f64> {
        if let Some(val) = self.value.get("duration") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The actual content of the media - inline or by direct reference to the media
    /// source file.
    pub fn content(&self) -> Attachment {
        Attachment {
            value: &self.value["content"],
        }
    }

    /// Identifiers associated with the image - these may include identifiers for the
    /// image itself, identifiers for the context of its collection (e.g. series ids)
    /// and context ids such as accession numbers or other workflow identifiers.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
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

    /// Extensions for createdDateTime
    pub fn _created_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_createdDateTime") {
            return Some(Element { value: val });
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
            return Some(Narrative { value: val });
        }
        return None;
    }

    /// The encounter that establishes the context for this media.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// The person who administered the collection of the image.
    pub fn operator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("operator") {
            return Some(Reference { value: val });
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

    /// The current state of the {{title}}.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// The date and time(s) at which the media was collected.
    pub fn created_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("createdPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// The date and time this version of the media was made available to providers,
    /// typically after having been reviewed.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.device_name() {}
        if let Some(_val) = self.part_of() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._frames() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modality() {
            _val.validate();
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.based_on() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reason_code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._issued() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.view() {
            _val.validate();
        }
        if let Some(_val) = self.width() {}
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._duration() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self._device_name() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.body_site() {
            _val.validate();
        }
        if let Some(_val) = self.device() {
            _val.validate();
        }
        if let Some(_val) = self.height() {}
        if let Some(_val) = self._width() {
            _val.validate();
        }
        if let Some(_val) = self.created_date_time() {}
        if let Some(_val) = self._height() {
            _val.validate();
        }
        if let Some(_val) = self.frames() {}
        if let Some(_val) = self.duration() {}
        let _ = self.content().validate();
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._created_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self.operator() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.created_period() {
            _val.validate();
        }
        if let Some(_val) = self.issued() {}
        return true;
    }
}
