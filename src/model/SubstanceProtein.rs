#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceProtein_Subunit::SubstanceProtein_Subunit;
use serde_json::value::Value;

/// A SubstanceProtein is defined as a single unit of a linear amino acid sequence,
/// or a combination of subunits that are either covalently linked or have a defined
/// invariant stoichiometric relationship. This includes all synthetic, recombinant
/// and purified SubstanceProteins of defined sequence, whether the use is
/// therapeutic or prophylactic. This set of elements will be used to describe
/// albumins, coagulation factors, cytokines, growth factors,
/// peptide/SubstanceProtein hormones, enzymes, toxins, toxoids, recombinant
/// vaccines, and immunomodulators.

#[derive(Debug)]
pub struct SubstanceProtein<'a> {
    pub value: &'a Value,
}

impl SubstanceProtein<'_> {
    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for disulfideLinkage
    pub fn _disulfide_linkage(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_disulfideLinkage") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// This subclause refers to the description of each subunit constituting the
    /// SubstanceProtein. A subunit is a linear sequence of amino acids linked through
    /// peptide bonds. The Subunit information shall be provided when the finished
    /// SubstanceProtein is a complex of multiple sequences; subunits are not used to
    /// delineate domains within a single sequence. Subunits are listed in order of
    /// decreasing length; sequences of the same length will be ordered by decreasing
    /// molecular weight; subunits that have identical sequences will be repeated
    /// multiple times.
    pub fn subunit(&self) -> Option<Vec<SubstanceProtein_Subunit>> {
        if let Some(Value::Array(val)) = self.value.get("subunit") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceProtein_Subunit { value: e })
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

    /// The disulphide bond between two cysteine residues either on the same subunit or
    /// on two different subunits shall be described. The position of the disulfide
    /// bonds in the SubstanceProtein shall be listed in increasing order of subunit
    /// number and position within subunit followed by the abbreviation of the amino
    /// acids involved. The disulfide linkage positions shall actually contain the amino
    /// acid Cysteine at the respective positions.
    pub fn disulfide_linkage(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("disulfideLinkage") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for numberOfSubunits
    pub fn _number_of_subunits(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfSubunits") {
            return Some(Element { value: val });
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

    /// The SubstanceProtein descriptive elements will only be used when a complete or
    /// partial amino acid sequence is available or derivable from a nucleic acid
    /// sequence.
    pub fn sequence_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sequenceType") {
            return Some(CodeableConcept { value: val });
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

    /// Number of linear sequences of amino acids linked through peptide bonds. The
    /// number of subunits constituting the SubstanceProtein shall be described. It is
    /// possible that the number of subunits can be variable.
    pub fn number_of_subunits(&self) -> Option<i64> {
        if let Some(val) = self.value.get("numberOfSubunits") {
            return Some(val.as_i64().unwrap());
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
        if let Some(_val) = self.language() {}
        if let Some(_val) = self._disulfide_linkage() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subunit() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.disulfide_linkage() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self._number_of_subunits() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
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
        if let Some(_val) = self.sequence_type() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.number_of_subunits() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        return true;
    }
}
