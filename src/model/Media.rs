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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A photo, video, or audio recording acquired or used in healthcare. The actual
/// content may be inline or provided by direct reference.

#[derive(Debug)]
pub struct Media<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Media<'_> {
    pub fn new(value: &Value) -> Media {
        Media {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for createdDateTime
    pub fn _created_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_createdDateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for deviceName
    pub fn _device_name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_deviceName") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for duration
    pub fn _duration(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_duration") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for frames
    pub fn _frames(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_frames") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for height
    pub fn _height(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_height") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for width
    pub fn _width(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_width") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A procedure that is fulfilled in whole or in part by the creation of this media.
    pub fn based_on(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("basedOn") {
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

    /// Indicates the site on the subject's body where the observation was made (i.e.
    /// the target site).
    pub fn body_site(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("bodySite") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The actual content of the media - inline or by direct reference to the media
    /// source file.
    pub fn content(&self) -> Attachment {
        Attachment {
            value: Cow::Borrowed(&self.value["content"]),
        }
    }

    /// The date and time(s) at which the media was collected.
    pub fn created_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("createdDateTime") {
            return Some(string);
        }
        return None;
    }

    /// The date and time(s) at which the media was collected.
    pub fn created_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("createdPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The device used to collect the media.
    pub fn device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("device") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The name of the device / manufacturer of the device  that was used to make the
    /// recording.
    pub fn device_name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("deviceName") {
            return Some(string);
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

    /// The encounter that establishes the context for this media.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Height of the image in pixels (photo/video).
    pub fn height(&self) -> Option<i64> {
        if let Some(val) = self.value.get("height") {
            return Some(val.as_i64().unwrap());
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

    /// Identifiers associated with the image - these may include identifiers for the
    /// image itself, identifiers for the context of its collection (e.g. series ids)
    /// and context ids such as accession numbers or other workflow identifiers.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
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

    /// The date and time this version of the media was made available to providers,
    /// typically after having been reviewed.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
            return Some(string);
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

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Details of the type of the media - usually, how it was acquired (what type of
    /// device). If images sourced from a DICOM system, are wrapped in a Media resource,
    /// then this is the modality.
    pub fn modality(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("modality") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Comments made about the media by the performer, subject or other participants.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The person who administered the collection of the image.
    pub fn operator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("operator") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A larger event of which this particular event is a component or step.
    pub fn part_of(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
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

    /// Describes why the event occurred in coded or textual form.
    pub fn reason_code(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("reasonCode") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Who/What this Media is a record of.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A code that classifies whether the media is an image, video or audio recording
    /// or some other media category.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The name of the imaging view e.g. Lateral or Antero-posterior (AP).
    pub fn view(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("view") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._created_date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._device_name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._duration() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._frames() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._height() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._issued() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._width() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.based_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.body_site() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.content().validate() {
            return false;
        }
        if let Some(_val) = self.created_date_time() {}
        if let Some(_val) = self.created_period() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.device() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.device_name() {}
        if let Some(_val) = self.duration() {}
        if let Some(_val) = self.encounter() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.frames() {}
        if let Some(_val) = self.height() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.issued() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modality() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.note() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.operator() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.part_of() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.reason_code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subject() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.view() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.width() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MediaBuilder {
    pub(crate) value: Value,
}

impl MediaBuilder {
    pub fn build(&self) -> Media {
        Media {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Media) -> MediaBuilder {
        MediaBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(content: Attachment) -> MediaBuilder {
        let mut __value: Value = json!({});
        __value["content"] = json!(content.value);
        return MediaBuilder { value: __value };
    }

    pub fn _created_date_time<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_createdDateTime"] = json!(val.value);
        return self;
    }

    pub fn _device_name<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_deviceName"] = json!(val.value);
        return self;
    }

    pub fn _duration<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_duration"] = json!(val.value);
        return self;
    }

    pub fn _frames<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_frames"] = json!(val.value);
        return self;
    }

    pub fn _height<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_height"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _issued<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_issued"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _width<'a>(&'a mut self, val: Element) -> &'a mut MediaBuilder {
        self.value["_width"] = json!(val.value);
        return self;
    }

    pub fn based_on<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut MediaBuilder {
        self.value["basedOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn body_site<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MediaBuilder {
        self.value["bodySite"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut MediaBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn created_date_time<'a>(&'a mut self, val: &str) -> &'a mut MediaBuilder {
        self.value["createdDateTime"] = json!(val);
        return self;
    }

    pub fn created_period<'a>(&'a mut self, val: Period) -> &'a mut MediaBuilder {
        self.value["createdPeriod"] = json!(val.value);
        return self;
    }

    pub fn device<'a>(&'a mut self, val: Reference) -> &'a mut MediaBuilder {
        self.value["device"] = json!(val.value);
        return self;
    }

    pub fn device_name<'a>(&'a mut self, val: &str) -> &'a mut MediaBuilder {
        self.value["deviceName"] = json!(val);
        return self;
    }

    pub fn duration<'a>(&'a mut self, val: f64) -> &'a mut MediaBuilder {
        self.value["duration"] = json!(val);
        return self;
    }

    pub fn encounter<'a>(&'a mut self, val: Reference) -> &'a mut MediaBuilder {
        self.value["encounter"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MediaBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn frames<'a>(&'a mut self, val: i64) -> &'a mut MediaBuilder {
        self.value["frames"] = json!(val);
        return self;
    }

    pub fn height<'a>(&'a mut self, val: i64) -> &'a mut MediaBuilder {
        self.value["height"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MediaBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut MediaBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut MediaBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn issued<'a>(&'a mut self, val: &str) -> &'a mut MediaBuilder {
        self.value["issued"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MediaBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MediaBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modality<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MediaBuilder {
        self.value["modality"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MediaBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut MediaBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn operator<'a>(&'a mut self, val: Reference) -> &'a mut MediaBuilder {
        self.value["operator"] = json!(val.value);
        return self;
    }

    pub fn part_of<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut MediaBuilder {
        self.value["partOf"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn reason_code<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut MediaBuilder {
        self.value["reasonCode"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut MediaBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Reference) -> &'a mut MediaBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MediaBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MediaBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn view<'a>(&'a mut self, val: CodeableConcept) -> &'a mut MediaBuilder {
        self.value["view"] = json!(val.value);
        return self;
    }

    pub fn width<'a>(&'a mut self, val: i64) -> &'a mut MediaBuilder {
        self.value["width"] = json!(val);
        return self;
    }
}
