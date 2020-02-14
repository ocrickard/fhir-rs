#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::SubstanceNucleicAcid_Linkage::SubstanceNucleicAcid_Linkage;
use crate::model::SubstanceNucleicAcid_Sugar::SubstanceNucleicAcid_Sugar;
use serde_json::value::Value;

/// Nucleic acids are defined by three distinct elements: the base, sugar and
/// linkage. Individual substance/moiety IDs will be created for each of these
/// elements. The nucleotide sequence will be always entered in the 5’-3’ direction.

#[derive(Debug)]
pub struct SubstanceNucleicAcid_Subunit<'a> {
    pub value: &'a Value,
}

impl SubstanceNucleicAcid_Subunit<'_> {
    /// Actual nucleotide sequence notation from 5' to 3' end using standard single
    /// letter codes. In addition to the base sequence, sugar and type of phosphate or
    /// non-phosphate linkage should also be captured.
    pub fn sequence(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("sequence") {
            return Some(string);
        }
        return None;
    }

    /// (TBC).
    pub fn sequence_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("sequenceAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Extensions for subunit
    pub fn _subunit(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subunit") {
            return Some(Element { value: val });
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

    /// Index of linear sequences of nucleic acids in order of decreasing length.
    /// Sequences of the same length will be ordered by molecular weight. Subunits that
    /// have identical sequences will be repeated and have sequential subscripts.
    pub fn subunit(&self) -> Option<i64> {
        if let Some(val) = self.value.get("subunit") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for length
    pub fn _length(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_length") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The nucleotide present at the 5’ terminal shall be specified based on a
    /// controlled vocabulary. Since the sequence is represented from the 5' to the 3'
    /// end, the 5’ prime nucleotide is the letter at the first position in the
    /// sequence. A separate representation would be redundant.
    pub fn five_prime(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("fivePrime") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The nucleotide present at the 3’ terminal shall be specified based on a
    /// controlled vocabulary. Since the sequence is represented from the 5' to the 3'
    /// end, the 5’ prime nucleotide is the letter at the last position in the sequence.
    /// A separate representation would be redundant.
    pub fn three_prime(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("threePrime") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// 5.3.6.8.1 Sugar ID (Mandatory).
    pub fn sugar(&self) -> Option<Vec<SubstanceNucleicAcid_Sugar>> {
        if let Some(Value::Array(val)) = self.value.get("sugar") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceNucleicAcid_Sugar { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The linkages between sugar residues will also be captured.
    pub fn linkage(&self) -> Option<Vec<SubstanceNucleicAcid_Linkage>> {
        if let Some(Value::Array(val)) = self.value.get("linkage") {
            return Some(
                val.into_iter()
                    .map(|e| SubstanceNucleicAcid_Linkage { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for sequence
    pub fn _sequence(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_sequence") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The length of the sequence shall be captured.
    pub fn length(&self) -> Option<i64> {
        if let Some(val) = self.value.get("length") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.sequence() {}
        if let Some(_val) = self.sequence_attachment() {
            _val.validate();
        }
        if let Some(_val) = self._subunit() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subunit() {}
        if let Some(_val) = self._length() {
            _val.validate();
        }
        if let Some(_val) = self.five_prime() {
            _val.validate();
        }
        if let Some(_val) = self.three_prime() {
            _val.validate();
        }
        if let Some(_val) = self.sugar() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.linkage() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._sequence() {
            _val.validate();
        }
        if let Some(_val) = self.length() {}
        return true;
    }
}
