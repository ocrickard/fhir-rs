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
use serde_json::value::Value;

/// A sample to be used for analysis.

#[derive(Debug)]
pub struct Specimen<'a> {
    pub value: &'a Value,
}

impl Specimen<'_> {
    /// To communicate any details or issues about the specimen or during the specimen
    /// collection. (for example: broken vial, sent with patient, frozen).
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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
                    .map(|e| Identifier { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Where the specimen came from. This may be from patient(s), from a location
    /// (e.g., the source of an environmental sample), or a sampling of a substance or a
    /// device.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Details concerning a service request that required a specimen to be collected.
    pub fn request(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("request") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
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

    /// The availability of the specimen.
    pub fn status(&self) -> Option<SpecimenStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(SpecimenStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The identifier assigned by the lab when accessioning specimen(s). This is not
    /// necessarily the same as the specimen identifier, depending on local lab
    /// procedures.
    pub fn accession_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("accessionIdentifier") {
            return Some(Identifier { value: val });
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

    /// The kind of material that forms the specimen.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
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

    /// Reference to the parent (source) specimen which is used when the specimen was
    /// either derived from or a component of another specimen.
    pub fn parent(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("parent") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for receivedTime
    pub fn _received_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_receivedTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Details concerning the specimen collection.
    pub fn collection(&self) -> Option<Specimen_Collection> {
        if let Some(val) = self.value.get("collection") {
            return Some(Specimen_Collection { value: val });
        }
        return None;
    }

    /// Details concerning processing and processing steps for the specimen.
    pub fn processing(&self) -> Option<Vec<Specimen_Processing>> {
        if let Some(Value::Array(val)) = self.value.get("processing") {
            return Some(
                val.into_iter()
                    .map(|e| Specimen_Processing { value: e })
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
                    .map(|e| Specimen_Container { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A mode or state of being that describes the nature of the specimen.
    pub fn condition(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("condition") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.request() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.accession_identifier() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.received_time() {}
        if let Some(_val) = self.parent() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._received_time() {
            _val.validate();
        }
        if let Some(_val) = self.collection() {
            _val.validate();
        }
        if let Some(_val) = self.processing() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.container() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.condition() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
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
}
