#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::MolecularSequence_Quality::MolecularSequence_Quality;
use crate::model::MolecularSequence_ReferenceSeq::MolecularSequence_ReferenceSeq;
use crate::model::MolecularSequence_Repository::MolecularSequence_Repository;
use crate::model::MolecularSequence_StructureVariant::MolecularSequence_StructureVariant;
use crate::model::MolecularSequence_Variant::MolecularSequence_Variant;
use crate::model::Narrative::Narrative;
use crate::model::Quantity::Quantity;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MolecularSequence<'_> {
    pub fn new(value: &Value) -> MolecularSequence {
        MolecularSequence {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for coordinateSystem
    pub fn _coordinate_system(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_coordinateSystem") {
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

    /// Extensions for observedSeq
    pub fn _observed_seq(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_observedSeq") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for readCoverage
    pub fn _read_coverage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_readCoverage") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
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

    /// Whether the sequence is numbered starting at 0 (0-based numbering or
    /// coordinates, inclusive start, exclusive end) or starting at 1 (1-based
    /// numbering, inclusive start and inclusive end).
    pub fn coordinate_system(&self) -> Option<i64> {
        if let Some(val) = self.value.get("coordinateSystem") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The method for sequencing, for example, chip information.
    pub fn device(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("device") {
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A unique identifier for this particular sequence instance. This is a FHIR-
    /// defined id.
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

    /// Sequence that was observed. It is the result marked by referenceSeq along with
    /// variant records on referenceSeq. This shall start from referenceSeq.windowStart
    /// and end by referenceSeq.windowEnd.
    pub fn observed_seq(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("observedSeq") {
            return Some(string);
        }
        return None;
    }

    /// The patient whose sequencing results are described by this resource.
    pub fn patient(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("patient") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The organization or lab that should be responsible for this result.
    pub fn performer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("performer") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Pointer to next atomic sequence which at most contains one variant.
    pub fn pointer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("pointer") {
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

    /// An experimental feature attribute that defines the quality of the feature in a
    /// quantitative way, such as a phred quality score
    /// ([SO:0001686](http://www.sequenceontology.org/browser/current_svn/term/SO:000168
    /// 6)).
    pub fn quality(&self) -> Option<Vec<MolecularSequence_Quality>> {
        if let Some(Value::Array(val)) = self.value.get("quality") {
            return Some(
                val.into_iter()
                    .map(|e| MolecularSequence_Quality {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The number of copies of the sequence of interest. (RNASeq).
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Coverage (read depth or depth) is the average number of reads representing a
    /// given nucleotide in the reconstructed sequence.
    pub fn read_coverage(&self) -> Option<i64> {
        if let Some(val) = self.value.get("readCoverage") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// A sequence that is used as a reference to describe variants that are present in
    /// a sequence analyzed.
    pub fn reference_seq(&self) -> Option<MolecularSequence_ReferenceSeq> {
        if let Some(val) = self.value.get("referenceSeq") {
            return Some(MolecularSequence_ReferenceSeq {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Configurations of the external repository. The repository shall store target's
    /// observedSeq or records related with target's observedSeq.
    pub fn repository(&self) -> Option<Vec<MolecularSequence_Repository>> {
        if let Some(Value::Array(val)) = self.value.get("repository") {
            return Some(
                val.into_iter()
                    .map(|e| MolecularSequence_Repository {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Specimen used for sequencing.
    pub fn specimen(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("specimen") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Information about chromosome structure variation.
    pub fn structure_variant(&self) -> Option<Vec<MolecularSequence_StructureVariant>> {
        if let Some(Value::Array(val)) = self.value.get("structureVariant") {
            return Some(
                val.into_iter()
                    .map(|e| MolecularSequence_StructureVariant {
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

    /// Amino Acid Sequence/ DNA Sequence / RNA Sequence.
    pub fn fhir_type(&self) -> Option<MolecularSequenceType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(MolecularSequenceType::from_string(&val).unwrap());
        }
        return None;
    }

    /// The definition of variant here originates from Sequence ontology
    /// ([variant_of](http://www.sequenceontology.org/browser/current_svn/term/variant_o
    /// f)). This element can represent amino acid or nucleic sequence change(including
    /// insertion,deletion,SNP,etc.)  It can represent some complex mutation or segment
    /// variation with the assist of CIGAR string.
    pub fn variant(&self) -> Option<Vec<MolecularSequence_Variant>> {
        if let Some(Value::Array(val)) = self.value.get("variant") {
            return Some(
                val.into_iter()
                    .map(|e| MolecularSequence_Variant {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._coordinate_system() {
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
        if let Some(_val) = self._observed_seq() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._read_coverage() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.coordinate_system() {}
        if let Some(_val) = self.device() {
            if !_val.validate() {
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
        if let Some(_val) = self.observed_seq() {}
        if let Some(_val) = self.patient() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.pointer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quality() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.quantity() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.read_coverage() {}
        if let Some(_val) = self.reference_seq() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.repository() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.specimen() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.structure_variant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.variant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct MolecularSequenceBuilder {
    pub(crate) value: Value,
}

impl MolecularSequenceBuilder {
    pub fn build(&self) -> MolecularSequence {
        MolecularSequence {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MolecularSequence) -> MolecularSequenceBuilder {
        MolecularSequenceBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MolecularSequenceBuilder {
        let mut __value: Value = json!({});
        return MolecularSequenceBuilder { value: __value };
    }

    pub fn _coordinate_system<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequenceBuilder {
        self.value["_coordinateSystem"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequenceBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequenceBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _observed_seq<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequenceBuilder {
        self.value["_observedSeq"] = json!(val.value);
        return self;
    }

    pub fn _read_coverage<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequenceBuilder {
        self.value["_readCoverage"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequenceBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut MolecularSequenceBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn coordinate_system<'a>(&'a mut self, val: i64) -> &'a mut MolecularSequenceBuilder {
        self.value["coordinateSystem"] = json!(val);
        return self;
    }

    pub fn device<'a>(&'a mut self, val: Reference) -> &'a mut MolecularSequenceBuilder {
        self.value["device"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut MolecularSequenceBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MolecularSequenceBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut MolecularSequenceBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut MolecularSequenceBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut MolecularSequenceBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut MolecularSequenceBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MolecularSequenceBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn observed_seq<'a>(&'a mut self, val: &str) -> &'a mut MolecularSequenceBuilder {
        self.value["observedSeq"] = json!(val);
        return self;
    }

    pub fn patient<'a>(&'a mut self, val: Reference) -> &'a mut MolecularSequenceBuilder {
        self.value["patient"] = json!(val.value);
        return self;
    }

    pub fn performer<'a>(&'a mut self, val: Reference) -> &'a mut MolecularSequenceBuilder {
        self.value["performer"] = json!(val.value);
        return self;
    }

    pub fn pointer<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut MolecularSequenceBuilder {
        self.value["pointer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quality<'a>(
        &'a mut self,
        val: Vec<MolecularSequence_Quality>,
    ) -> &'a mut MolecularSequenceBuilder {
        self.value["quality"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn quantity<'a>(&'a mut self, val: Quantity) -> &'a mut MolecularSequenceBuilder {
        self.value["quantity"] = json!(val.value);
        return self;
    }

    pub fn read_coverage<'a>(&'a mut self, val: i64) -> &'a mut MolecularSequenceBuilder {
        self.value["readCoverage"] = json!(val);
        return self;
    }

    pub fn reference_seq<'a>(
        &'a mut self,
        val: MolecularSequence_ReferenceSeq,
    ) -> &'a mut MolecularSequenceBuilder {
        self.value["referenceSeq"] = json!(val.value);
        return self;
    }

    pub fn repository<'a>(
        &'a mut self,
        val: Vec<MolecularSequence_Repository>,
    ) -> &'a mut MolecularSequenceBuilder {
        self.value["repository"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn specimen<'a>(&'a mut self, val: Reference) -> &'a mut MolecularSequenceBuilder {
        self.value["specimen"] = json!(val.value);
        return self;
    }

    pub fn structure_variant<'a>(
        &'a mut self,
        val: Vec<MolecularSequence_StructureVariant>,
    ) -> &'a mut MolecularSequenceBuilder {
        self.value["structureVariant"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut MolecularSequenceBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: MolecularSequenceType,
    ) -> &'a mut MolecularSequenceBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }

    pub fn variant<'a>(
        &'a mut self,
        val: Vec<MolecularSequence_Variant>,
    ) -> &'a mut MolecularSequenceBuilder {
        self.value["variant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum MolecularSequenceType {
    Aa,
    Dna,
    Rna,
}

impl MolecularSequenceType {
    pub fn from_string(string: &str) -> Option<MolecularSequenceType> {
        match string {
            "aa" => Some(MolecularSequenceType::Aa),
            "dna" => Some(MolecularSequenceType::Dna),
            "rna" => Some(MolecularSequenceType::Rna),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            MolecularSequenceType::Aa => "aa".to_string(),
            MolecularSequenceType::Dna => "dna".to_string(),
            MolecularSequenceType::Rna => "rna".to_string(),
        }
    }
}
