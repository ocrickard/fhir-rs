#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DocumentManifest_Related::DocumentManifest_Related;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A collection of documents compiled for a purpose together with metadata that
/// applies to the collection.

#[derive(Debug)]
pub struct DocumentManifest<'a> {
    pub value: &'a Value,
}

impl DocumentManifest<'_> {
    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Related identifiers or resources associated with the DocumentManifest.
    pub fn related(&self) -> Option<Vec<DocumentManifest_Related>> {
        if let Some(Value::Array(val)) = self.value.get("related") {
            return Some(
                val.into_iter()
                    .map(|e| DocumentManifest_Related { value: e })
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

    /// The status of this document manifest.
    pub fn status(&self) -> Option<DocumentManifestStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(DocumentManifestStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Who or what the set of documents is about. The documents can be about a person,
    /// (patient or healthcare practitioner), a device (i.e. machine) or even a group of
    /// subjects (such as a document about a herd of farm animals, or a set of patients
    /// that share a common exposure). If the documents cross more than one subject,
    /// then more than one subject is allowed here (unusual use case).
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Other identifiers associated with the document manifest, including version
    /// independent  identifiers.
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// Identifies who is the author of the manifest. Manifest author is not necessarly
    /// the author of the references included.
    pub fn author(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A single identifier that uniquely identifies this manifest. Principally used to
    /// refer to the manifest in non-FHIR contexts.
    pub fn master_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("masterIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The code specifying the type of clinical activity that resulted in placing the
    /// associated content into the DocumentManifest.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A patient, practitioner, or organization for which this set of documents is
    /// intended.
    pub fn recipient(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("recipient") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Identifies the source system, application, or software that produced the
    /// document manifest.
    pub fn source(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("source") {
            return Some(string);
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
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

    /// When the document manifest was created for submission to the server (not
    /// necessarily the same thing as the actual resource last modified time, since it
    /// may be modified, replicated, etc.).
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for source
    pub fn _source(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_source") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Human-readable description of the source document. This is sometimes known as
    /// the "title".
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The list of Resources that consist of the parts of this manifest.
    pub fn content(&self) -> Vec<Reference> {
        self.value
            .get("content")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Reference { value: e })
            .collect::<Vec<_>>()
    }

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.related() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.author() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.master_identifier() {
            _val.validate();
        }
        if let Some(_val) = self._created() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.recipient() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.source() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.created() {}
        if let Some(_val) = self._source() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        let _ = self.content().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.implicit_rules() {}
        return true;
    }
}

#[derive(Debug)]
pub enum DocumentManifestStatus {
    Current,
    Superseded,
    EnteredInError,
}

impl DocumentManifestStatus {
    pub fn from_string(string: &str) -> Option<DocumentManifestStatus> {
        match string {
            "current" => Some(DocumentManifestStatus::Current),
            "superseded" => Some(DocumentManifestStatus::Superseded),
            "entered-in-error" => Some(DocumentManifestStatus::EnteredInError),
            _ => None,
        }
    }
}
