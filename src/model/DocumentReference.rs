#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DocumentReference_Content::DocumentReference_Content;
use crate::model::DocumentReference_Context::DocumentReference_Context;
use crate::model::DocumentReference_RelatesTo::DocumentReference_RelatesTo;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A reference to a document of any kind for any purpose. Provides metadata about
/// the document so that the document can be discovered and managed. The scope of a
/// document is any seralized object with a mime-type, so includes formal patient
/// centric documents (CDA), cliical notes, scanned paper, and non-patient specific
/// documents like policy text.

#[derive(Debug)]
pub struct DocumentReference<'a> {
    pub value: &'a Value,
}

impl DocumentReference<'_> {
    /// Document identifier as assigned by the source of the document. This identifier
    /// is specific to this version of the document. This unique identifier may be used
    /// elsewhere to identify this version of the document.
    pub fn master_identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("masterIdentifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Relationships that this document has with other document references that already
    /// exist.
    pub fn relates_to(&self) -> Option<Vec<DocumentReference_RelatesTo>> {
        if let Some(Value::Array(val)) = self.value.get("relatesTo") {
            return Some(
                val.into_iter()
                    .map(|e| DocumentReference_RelatesTo { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Human-readable description of the source document.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The status of this document reference.
    pub fn status(&self) -> Option<DocumentReferenceStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(DocumentReferenceStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// Which person or organization authenticates that this document is valid.
    pub fn authenticator(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("authenticator") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Identifies the organization or group who is responsible for ongoing maintenance
    /// of and access to the document.
    pub fn custodian(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("custodian") {
            return Some(Reference { value: val });
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

    /// The status of the underlying document.
    pub fn doc_status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("docStatus") {
            return Some(string);
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

    /// A set of Security-Tag codes specifying the level of privacy/security of the
    /// Document. Note that DocumentReference.meta.security contains the security labels
    /// of the "reference" to the document, while DocumentReference.securityLabel
    /// contains a snapshot of the security labels on the document the reference refers
    /// to.
    pub fn security_label(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("securityLabel") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specifies the particular kind of document referenced  (e.g. History and
    /// Physical, Discharge Summary, Progress Note). This usually equates to the purpose
    /// of making the document referenced.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
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

    /// The clinical context in which the document was prepared.
    pub fn context(&self) -> Option<DocumentReference_Context> {
        if let Some(val) = self.value.get("context") {
            return Some(DocumentReference_Context { value: val });
        }
        return None;
    }

    /// Identifies who is responsible for adding the information to the document.
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

    /// When the document reference was created.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
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

    /// The document and format referenced. There may be multiple content element
    /// repetitions, each with a different format.
    pub fn content(&self) -> Vec<DocumentReference_Content> {
        self.value
            .get("content")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| DocumentReference_Content { value: e })
            .collect::<Vec<_>>()
    }

    /// A categorization for the type of document referenced - helps for indexing and
    /// searching. This may be implied by or derived from the code specified in the
    /// DocumentReference.type.
    pub fn category(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("category") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Other identifiers associated with the document, including version independent
    /// identifiers.
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

    /// Extensions for docStatus
    pub fn _doc_status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_docStatus") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Who or what the document is about. The document can be about a person, (patient
    /// or healthcare practitioner), a device (e.g. a machine) or even a group of
    /// subjects (such as a document about a herd of farm animals, or a set of patients
    /// that share a common exposure).
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.master_identifier() {
            _val.validate();
        }
        if let Some(_val) = self.relates_to() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.authenticator() {
            _val.validate();
        }
        if let Some(_val) = self.custodian() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.doc_status() {}
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.security_label() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.context() {
            _val.validate();
        }
        if let Some(_val) = self.author() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        let _ = self.content().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._doc_status() {
            _val.validate();
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum DocumentReferenceStatus {
    Current,
    Superseded,
    EnteredInError,
}

impl DocumentReferenceStatus {
    pub fn from_string(string: &str) -> Option<DocumentReferenceStatus> {
        match string {
            "current" => Some(DocumentReferenceStatus::Current),
            "superseded" => Some(DocumentReferenceStatus::Superseded),
            "entered-in-error" => Some(DocumentReferenceStatus::EnteredInError),
            _ => None,
        }
    }
}
