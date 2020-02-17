#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::SubstanceNucleicAcid_Subunit::SubstanceNucleicAcid_Subunit;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.

#[derive(Debug)]
pub struct SubstanceNucleicAcid<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl SubstanceNucleicAcid<'_> {
    pub fn new(value: &Value) -> SubstanceNucleicAcid {
        SubstanceNucleicAcid {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for areaOfHybridisation
    pub fn _area_of_hybridisation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_areaOfHybridisation") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for numberOfSubunits
    pub fn _number_of_subunits(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_numberOfSubunits") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The area of hybridisation shall be described if applicable for double stranded
    /// RNA or DNA. The number associated with the subunit followed by the number
    /// associated to the residue shall be specified in increasing order. The underscore
    /// “” shall be used as separator as follows: “Subunitnumber Residue”.
    pub fn area_of_hybridisation(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("areaOfHybridisation") {
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
                    .map(|e| ResourceList {
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

    /// The number of linear sequences of nucleotides linked through phosphodiester
    /// bonds shall be described. Subunits would be strands of nucleic acids that are
    /// tightly associated typically through Watson-Crick base pairing. NOTE: If not
    /// specified in the reference source, the assumption is that there is 1 subunit.
    pub fn number_of_subunits(&self) -> Option<i64> {
        if let Some(val) = self.value.get("numberOfSubunits") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// (TBC).
    pub fn oligo_nucleotide_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("oligoNucleotideType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The type of the sequence shall be specified based on a controlled vocabulary.
    pub fn sequence_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("sequenceType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Subunits are listed in order of decreasing length; sequences of the same length
    /// will be ordered by molecular weight; subunits that have identical sequences will
    /// be repeated multiple times.
    pub fn subunit(&self) -> Option<Vec<SubstanceNucleicAcid_Subunit>> {
        if let Some(Value::Array(val)) = self.value.get("subunit") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceNucleicAcid_Subunit {
                        value: Cow::Borrowed(e),
                    })
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._area_of_hybridisation() {
            if !_val.validate() {
                return false;
            }
        }
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
        if let Some(_val) = self._number_of_subunits() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.area_of_hybridisation() {}
        if let Some(_val) = self.contained() {
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
        if let Some(_val) = self.number_of_subunits() {}
        if let Some(_val) = self.oligo_nucleotide_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.sequence_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.subunit() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct SubstanceNucleicAcidBuilder {
    pub(crate) value: Value,
}

impl SubstanceNucleicAcidBuilder {
    pub fn build(&self) -> SubstanceNucleicAcid {
        SubstanceNucleicAcid {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: SubstanceNucleicAcid) -> SubstanceNucleicAcidBuilder {
        SubstanceNucleicAcidBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> SubstanceNucleicAcidBuilder {
        let mut __value: Value = json!({});
        return SubstanceNucleicAcidBuilder { value: __value };
    }

    pub fn _area_of_hybridisation<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["_areaOfHybridisation"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _number_of_subunits<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["_numberOfSubunits"] = json!(val.value);
        return self;
    }

    pub fn area_of_hybridisation<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["areaOfHybridisation"] = json!(val);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn number_of_subunits<'a>(&'a mut self, val: i64) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["numberOfSubunits"] = json!(val);
        return self;
    }

    pub fn oligo_nucleotide_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["oligoNucleotideType"] = json!(val.value);
        return self;
    }

    pub fn sequence_type<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["sequenceType"] = json!(val.value);
        return self;
    }

    pub fn subunit<'a>(
        &'a mut self,
        val: Vec<SubstanceNucleicAcid_Subunit>,
    ) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["subunit"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut SubstanceNucleicAcidBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}
