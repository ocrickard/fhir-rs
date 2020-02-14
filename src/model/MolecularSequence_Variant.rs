#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_Variant<'a> {
    pub value: &'a Value,
}

impl MolecularSequence_Variant<'_> {
    /// Extensions for observedAllele
    pub fn _observed_allele(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_observedAllele") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extended CIGAR string for aligning the sequence with reference bases. See
    /// detailed documentation
    /// [here](http://support.illumina.com/help/SequencingAnalysisWorkflow/Content/Vault
    /// /Informatics/Sequencing_Analysis/CASAVA/swSEQ_mCA_ExtendedCIGARFormat.htm).
    pub fn cigar(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("cigar") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An allele is one of a set of coexisting sequence variants of a gene
    /// ([SO:0001023](http://www.sequenceontology.org/browser/current_svn/term/SO:000102
    /// 3)).  Nucleotide(s)/amino acids from start position of sequence to stop position
    /// of sequence on the positive (+) strand of the observed  sequence. When the
    /// sequence  type is DNA, it should be the sequence on the positive (+) strand.
    /// This will lay in the range between variant.start and variant.end.
    pub fn observed_allele(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("observedAllele") {
            return Some(string);
        }
        return None;
    }

    /// An allele is one of a set of coexisting sequence variants of a gene
    /// ([SO:0001023](http://www.sequenceontology.org/browser/current_svn/term/SO:000102
    /// 3)). Nucleotide(s)/amino acids from start position of sequence to stop position
    /// of sequence on the positive (+) strand of the reference sequence. When the
    /// sequence  type is DNA, it should be the sequence on the positive (+) strand.
    /// This will lay in the range between variant.start and variant.end.
    pub fn reference_allele(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("referenceAllele") {
            return Some(string);
        }
        return None;
    }

    /// Start position of the variant on the  reference sequence. If the coordinate
    /// system is either 0-based or 1-based, then start position is inclusive.
    pub fn start(&self) -> Option<i64> {
        if let Some(val) = self.value.get("start") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// Extensions for start
    pub fn _start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_start") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for referenceAllele
    pub fn _reference_allele(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_referenceAllele") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for cigar
    pub fn _cigar(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_cigar") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A pointer to an Observation containing variant information.
    pub fn variant_pointer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("variantPointer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// End position of the variant on the reference sequence. If the coordinate system
    /// is 0-based then end is exclusive and does not include the last position. If the
    /// coordinate system is 1-base, then end is inclusive and includes the last
    /// position.
    pub fn end(&self) -> Option<i64> {
        if let Some(val) = self.value.get("end") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._observed_allele() {
            _val.validate();
        }
        if let Some(_val) = self.cigar() {}
        if let Some(_val) = self._end() {
            _val.validate();
        }
        if let Some(_val) = self.observed_allele() {}
        if let Some(_val) = self.reference_allele() {}
        if let Some(_val) = self.start() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._start() {
            _val.validate();
        }
        if let Some(_val) = self._reference_allele() {
            _val.validate();
        }
        if let Some(_val) = self._cigar() {
            _val.validate();
        }
        if let Some(_val) = self.variant_pointer() {
            _val.validate();
        }
        if let Some(_val) = self.end() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}
