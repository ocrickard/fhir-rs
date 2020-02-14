#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceSpecification_Code::SubstanceSpecification_Code;
use crate::model::SubstanceSpecification_Moiety::SubstanceSpecification_Moiety;
use crate::model::SubstanceSpecification_MolecularWeight::SubstanceSpecification_MolecularWeight;
use crate::model::SubstanceSpecification_Name::SubstanceSpecification_Name;
use crate::model::SubstanceSpecification_Property::SubstanceSpecification_Property;
use crate::model::SubstanceSpecification_Relationship::SubstanceSpecification_Relationship;
use crate::model::SubstanceSpecification_Structure::SubstanceSpecification_Structure;
use serde_json::value::Value;

/// The detailed description of a substance, typically at a level beyond what is
/// used for prescribing.

#[derive(Debug)]
pub struct SubstanceSpecification<'a> {
    pub value: &'a Value,
}

impl SubstanceSpecification<'_> {
    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// Extensions for comment
    pub fn _comment(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_comment") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Codes associated with the substance.
    pub fn code(&self) -> Option<Vec<SubstanceSpecification_Code>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Code { value: e })
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

    /// General specifications for this substance, including how it is related to other
    /// substances.
    pub fn property(&self) -> Option<Vec<SubstanceSpecification_Property>> {
        if let Some(Value::Array(val)) = self.value.get("property") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Property { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Structural information.
    pub fn structure(&self) -> Option<SubstanceSpecification_Structure> {
        if let Some(val) = self.value.get("structure") {
            return Some(SubstanceSpecification_Structure { value: val });
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

    /// Textual comment about this record of a substance.
    pub fn comment(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("comment") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Status of substance within the catalogue e.g. approved.
    pub fn status(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("status") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Material or taxonomic/anatomical source for the substance.
    pub fn source_material(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("sourceMaterial") {
            return Some(Reference { value: val });
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

    /// A link between this substance and another, with details of the relationship.
    pub fn relationship(&self) -> Option<Vec<SubstanceSpecification_Relationship>> {
        if let Some(Value::Array(val)) = self.value.get("relationship") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Relationship { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Names applicable to this substance.
    pub fn name(&self) -> Option<Vec<SubstanceSpecification_Name>> {
        if let Some(Value::Array(val)) = self.value.get("name") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Name { value: e })
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

    /// Moiety, for structural modifications.
    pub fn moiety(&self) -> Option<Vec<SubstanceSpecification_Moiety>> {
        if let Some(Value::Array(val)) = self.value.get("moiety") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_Moiety { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Identifier by which this substance is known.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier { value: val });
        }
        return None;
    }

    /// Data items specific to nucleic acids.
    pub fn nucleic_acid(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("nucleicAcid") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// High level categorization, e.g. polymer or nucleic acid.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// General information detailing this substance.
    pub fn reference_information(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("referenceInformation") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Data items specific to proteins.
    pub fn protein(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("protein") {
            return Some(Reference { value: val });
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

    /// If the substance applies to only human or veterinary use.
    pub fn domain(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("domain") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The molecular weight or weight range (for proteins, polymers or nucleic acids).
    pub fn molecular_weight(&self) -> Option<Vec<SubstanceSpecification_MolecularWeight>> {
        if let Some(Value::Array(val)) = self.value.get("molecularWeight") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceSpecification_MolecularWeight { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Supporting literature.
    pub fn source(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("source") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Data items specific to polymers.
    pub fn polymer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("polymer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Textual description of the substance.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self._comment() {
            _val.validate();
        }
        if let Some(_val) = self.code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.property() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.structure() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.comment() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.status() {
            _val.validate();
        }
        if let Some(_val) = self.source_material() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.relationship() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.name() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.moiety() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.validate();
        }
        if let Some(_val) = self.nucleic_acid() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.reference_information() {
            _val.validate();
        }
        if let Some(_val) = self.protein() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.domain() {
            _val.validate();
        }
        if let Some(_val) = self.molecular_weight() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.source() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.polymer() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        return true;
    }
}
