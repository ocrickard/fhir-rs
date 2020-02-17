#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::Specimen_Collection::Specimen_Collection;
use crate::model::Specimen_Container::Specimen_Container;
use crate::model::Specimen_Processing::Specimen_Processing;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A sample to be used for analysis.

#[derive(Debug)]
pub struct Specimen<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Specimen<'_> {
    pub fn new(value: &Value) -> Specimen {
        Specimen {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for receivedTime
    pub fn _received_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_receivedTime") {
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

    /// The identifier assigned by the lab when accessioning specimen(s). This is not
    /// necessarily the same as the specimen identifier, depending on local lab
    /// procedures.
    pub fn accession_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("accessionIdentifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Details concerning the specimen collection.
    pub fn collection(&self) -> Option<Specimen_Collection> {
        if let Some(val) = self.value.get("collection") {
            return Some(Specimen_Collection {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A mode or state of being that describes the nature of the specimen.
    pub fn condition(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
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

    /// The container holding the specimen.  The recursive nature of containers; i.e.
    /// blood in tube in tray in rack is not addressed here.
    pub fn container(&self) -> Option<Vec<Specimen_Container>> {
        if let Some(Value::Array(val)) = self.value.get("container") {
            return Some(
                val.into_iter()
                    .map(|e| Specimen_Container {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Id for specimen.
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

    /// To communicate any details or issues about the specimen or during the specimen
    /// collection. (for example: broken vial, sent with patient, frozen).
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

    /// Reference to the parent (source) specimen which is used when the specimen was
    /// either derived from or a component of another specimen.
    pub fn parent(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("parent") {
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

    /// Details concerning processing and processing steps for the specimen.
    pub fn processing(&self) -> Option<Vec<Specimen_Processing>> {
        if let Some(Value::Array(val)) = self.value.get("processing") {
            return Some(
                val.into_iter()
                    .map(|e| Specimen_Processing {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Time when specimen was received for processing or testing.
    pub fn received_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("receivedTime") {
            return Some(string);
        }
        return None;
    }

    /// Details concerning a service request that required a specimen to be collected.
    pub fn request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("request") {
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

    /// The availability of the specimen.
    pub fn status(&self) -> Option<SpecimenStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(SpecimenStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Where the specimen came from. This may be from patient(s), from a location
    /// (e.g., the source of an environmental sample), or a sampling of a substance or a
    /// device.
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

    /// The kind of material that forms the specimen.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._received_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.accession_identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.collection() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.condition() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.container() {
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
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.meta() {
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
        if let Some(_val) = self.parent() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.processing() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.received_time() {}
        if let Some(_val) = self.request() {
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
        return true;
    }
}

#[derive(Debug)]
pub struct SpecimenBuilder {
    pub(crate) value: Value,
}

impl SpecimenBuilder {
    pub fn build(&self) -> Specimen {
        Specimen {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Specimen) -> SpecimenBuilder {
        SpecimenBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SpecimenBuilder {
        let mut __value: Value = json!({});
        return SpecimenBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut SpecimenBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SpecimenBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _received_time<'a>(&'a mut self, val: Element) -> &'a mut SpecimenBuilder {
        self.value["_receivedTime"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut SpecimenBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn accession_identifier<'a>(&'a mut self, val: Identifier) -> &'a mut SpecimenBuilder {
        self.value["accessionIdentifier"] = json!(val.value);
        return self;
    }

    pub fn collection<'a>(&'a mut self, val: Specimen_Collection) -> &'a mut SpecimenBuilder {
        self.value["collection"] = json!(val.value);
        return self;
    }

    pub fn condition<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut SpecimenBuilder {
        self.value["condition"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut SpecimenBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn container<'a>(&'a mut self, val: Vec<Specimen_Container>) -> &'a mut SpecimenBuilder {
        self.value["container"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SpecimenBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SpecimenBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut SpecimenBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SpecimenBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SpecimenBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SpecimenBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SpecimenBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn note<'a>(&'a mut self, val: Vec<Annotation>) -> &'a mut SpecimenBuilder {
        self.value["note"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn parent<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut SpecimenBuilder {
        self.value["parent"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn processing<'a>(&'a mut self, val: Vec<Specimen_Processing>) -> &'a mut SpecimenBuilder {
        self.value["processing"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn received_time<'a>(&'a mut self, val: &str) -> &'a mut SpecimenBuilder {
        self.value["receivedTime"] = json!(val);
        return self;
    }

    pub fn request<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut SpecimenBuilder {
        self.value["request"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: SpecimenStatus) -> &'a mut SpecimenBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Reference) -> &'a mut SpecimenBuilder {
        self.value["subject"] = json!(val.value);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SpecimenBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut SpecimenBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum SpecimenStatus {
    Available,
    Unavailable,
    Unsatisfactory,
    EnteredInError,
}

impl SpecimenStatus {
    pub fn from_string(string: &str) -> Option<SpecimenStatus> {
        match string {
            "available" => Some(SpecimenStatus::Available),
            "unavailable" => Some(SpecimenStatus::Unavailable),
            "unsatisfactory" => Some(SpecimenStatus::Unsatisfactory),
            "entered-in-error" => Some(SpecimenStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            SpecimenStatus::Available => "available".to_string(),
            SpecimenStatus::Unavailable => "unavailable".to_string(),
            SpecimenStatus::Unsatisfactory => "unsatisfactory".to_string(),
            SpecimenStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}
