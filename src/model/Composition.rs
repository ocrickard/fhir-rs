#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Composition_Attester::Composition_Attester;
use crate::model::Composition_Event::Composition_Event;
use crate::model::Composition_RelatesTo::Composition_RelatesTo;
use crate::model::Composition_Section::Composition_Section;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// A set of healthcare-related information that is assembled together into a single
/// logical package that provides a single coherent statement of meaning,
/// establishes its own context and that has clinical attestation with regard to who
/// is making the statement. A Composition defines the structure and narrative
/// content necessary for a document. However, a Composition alone does not
/// constitute a document. Rather, the Composition must be the first entry in a
/// Bundle where Bundle.type=document, and any other resources referenced from
/// Composition must be included as subsequent entries in the Bundle (for example
/// Patient, Practitioner, Encounter, etc.).

#[derive(Debug)]
pub struct Composition<'a> {
    pub value: &'a Value,
}

impl Composition<'_> {
    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Identifies who is responsible for the information in the composition, not
    /// necessarily who typed it in.
    pub fn author(&self) -> Vec<Reference> {
        self.value
            .get("author")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Reference { value: e })
            .collect::<Vec<_>>()
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
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

    /// A participant who has attested to the accuracy of the composition/document.
    pub fn attester(&self) -> Option<Vec<Composition_Attester>> {
        if let Some(Value::Array(val)) = self.value.get("attester") {
            return Some(
                val.into_iter()
                    .map(|e| Composition_Attester { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Identifies the organization or group who is responsible for ongoing maintenance
    /// of and access to the composition/document information.
    pub fn custodian(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("custodian") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Relationships that this composition has with other compositions or documents
    /// that already exist.
    pub fn relates_to(&self) -> Option<Vec<Composition_RelatesTo>> {
        if let Some(Value::Array(val)) = self.value.get("relatesTo") {
            return Some(
                val.into_iter()
                    .map(|e| Composition_RelatesTo { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for confidentiality
    pub fn _confidentiality(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_confidentiality") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The clinical service, such as a colonoscopy or an appendectomy, being
    /// documented.
    pub fn event(&self) -> Option<Vec<Composition_Event>> {
        if let Some(Value::Array(val)) = self.value.get("event") {
            return Some(
                val.into_iter()
                    .map(|e| Composition_Event { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Official human-readable label for the composition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// The workflow/clinical status of this composition. The status is a marker for the
    /// clinical standing of the document.
    pub fn status(&self) -> Option<CompositionStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(CompositionStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The composition editing time, when the composition was last logically changed by
    /// the author.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// A categorization for the type of the composition - helps for indexing and
    /// searching. This may be implied by or derived from the code specified in the
    /// Composition Type.
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

    /// A version-independent identifier for the Composition. This identifier stays
    /// constant as the composition is changed over time.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
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

    /// Describes the clinical encounter or type of care this documentation is
    /// associated with.
    pub fn encounter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("encounter") {
            return Some(Reference { value: val });
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

    /// Who or what the composition is about. The composition can be about a person,
    /// (patient or healthcare practitioner), a device (e.g. a machine) or even a group
    /// of subjects (such as a document about a herd of livestock, or a set of patients
    /// that share a common exposure).
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
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

    /// The code specifying the level of confidentiality of the Composition.
    pub fn confidentiality(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("confidentiality") {
            return Some(string);
        }
        return None;
    }

    /// The root of the sections that make up the composition.
    pub fn section(&self) -> Option<Vec<Composition_Section>> {
        if let Some(Value::Array(val)) = self.value.get("section") {
            return Some(
                val.into_iter()
                    .map(|e| Composition_Section { value: e })
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

    /// Specifies the particular kind of composition (e.g. History and Physical,
    /// Discharge Summary, Progress Note). This usually equates to the purpose of making
    /// the composition.
    pub fn fhir_type(&self) -> CodeableConcept {
        CodeableConcept {
            value: &self.value["type"],
        }
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.language() {}
        let _ = self.author().into_iter().for_each(|e| {
            e.validate();
        });
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.attester() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.custodian() {
            _val.validate();
        }
        if let Some(_val) = self.relates_to() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._confidentiality() {
            _val.validate();
        }
        if let Some(_val) = self.event() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.category() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.encounter() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.confidentiality() {}
        if let Some(_val) = self.section() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.fhir_type().validate();
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum CompositionStatus {
    Preliminary,
    Final,
    Amended,
    EnteredInError,
}

impl CompositionStatus {
    pub fn from_string(string: &str) -> Option<CompositionStatus> {
        match string {
            "preliminary" => Some(CompositionStatus::Preliminary),
            "final" => Some(CompositionStatus::Final),
            "amended" => Some(CompositionStatus::Amended),
            "entered-in-error" => Some(CompositionStatus::EnteredInError),
            _ => None,
        }
    }
}
