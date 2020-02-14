#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Reference::Reference;
use serde_json::value::Value;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_ReferenceSeq<'a> {
    pub value: &'a Value,
}

impl MolecularSequence_ReferenceSeq<'_> {
    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for genomeBuild
    pub fn _genome_build(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_genomeBuild") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The Genome Build used for reference, following GRCh build versions e.g. 'GRCh
    /// 37'.  Version number must be included if a versioned release of a primary build
    /// was used.
    pub fn genome_build(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("genomeBuild") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for strand
    pub fn _strand(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_strand") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Start position of the window on the reference sequence. If the coordinate system
    /// is either 0-based or 1-based, then start position is inclusive.
    pub fn window_start(&self) -> Option<i64> {
        if let Some(val) = self.value.get("windowStart") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// A string like "ACGT".
    pub fn reference_seq_string(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("referenceSeqString") {
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

    /// Extensions for referenceSeqString
    pub fn _reference_seq_string(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_referenceSeqString") {
            return Some(Element { value: val });
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

    /// Extensions for windowEnd
    pub fn _window_end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_windowEnd") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Structural unit composed of a nucleic acid molecule which controls its own
    /// replication through the interaction of specific proteins at one or more origins
    /// of replication
    /// ([SO:0000340](http://www.sequenceontology.org/browser/current_svn/term/SO:000034
    /// 0)).
    pub fn chromosome(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("chromosome") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// An absolute reference to a strand. The Watson strand is the strand whose 5'-end
    /// is on the short arm of the chromosome, and the Crick strand as the one whose
    /// 5'-end is on the long arm.
    pub fn strand(&self) -> Option<MolecularSequence_ReferenceSeqStrand> {
        if let Some(Value::String(val)) = self.value.get("strand") {
            return Some(MolecularSequence_ReferenceSeqStrand::from_string(&val).unwrap());
        }
        return None;
    }

    /// Extensions for windowStart
    pub fn _window_start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_windowStart") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A pointer to another MolecularSequence entity as reference sequence.
    pub fn reference_seq_pointer(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("referenceSeqPointer") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Extensions for orientation
    pub fn _orientation(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_orientation") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Reference identifier of reference sequence submitted to NCBI. It must match the
    /// type in the MolecularSequence.type field. For example, the prefix, “NG_”
    /// identifies reference sequence for genes, “NM_” for messenger RNA transcripts,
    /// and “NP_” for amino acid sequences.
    pub fn reference_seq_id(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("referenceSeqId") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// End position of the window on the reference sequence. If the coordinate system
    /// is 0-based then end is exclusive and does not include the last position. If the
    /// coordinate system is 1-base, then end is inclusive and includes the last
    /// position.
    pub fn window_end(&self) -> Option<i64> {
        if let Some(val) = self.value.get("windowEnd") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// A relative reference to a DNA strand based on gene orientation. The strand that
    /// contains the open reading frame of the gene is the "sense" strand, and the
    /// opposite complementary strand is the "antisense" strand.
    pub fn orientation(&self) -> Option<MolecularSequence_ReferenceSeqOrientation> {
        if let Some(Value::String(val)) = self.value.get("orientation") {
            return Some(MolecularSequence_ReferenceSeqOrientation::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._genome_build() {
            _val.validate();
        }
        if let Some(_val) = self.genome_build() {}
        if let Some(_val) = self._strand() {
            _val.validate();
        }
        if let Some(_val) = self.window_start() {}
        if let Some(_val) = self.reference_seq_string() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._reference_seq_string() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._window_end() {
            _val.validate();
        }
        if let Some(_val) = self.chromosome() {
            _val.validate();
        }
        if let Some(_val) = self.strand() {}
        if let Some(_val) = self._window_start() {
            _val.validate();
        }
        if let Some(_val) = self.reference_seq_pointer() {
            _val.validate();
        }
        if let Some(_val) = self._orientation() {
            _val.validate();
        }
        if let Some(_val) = self.reference_seq_id() {
            _val.validate();
        }
        if let Some(_val) = self.window_end() {}
        if let Some(_val) = self.orientation() {}
        return true;
    }
}

#[derive(Debug)]
pub enum MolecularSequence_ReferenceSeqStrand {
    Watson,
    Crick,
}

impl MolecularSequence_ReferenceSeqStrand {
    pub fn from_string(string: &str) -> Option<MolecularSequence_ReferenceSeqStrand> {
        match string {
            "watson" => Some(MolecularSequence_ReferenceSeqStrand::Watson),
            "crick" => Some(MolecularSequence_ReferenceSeqStrand::Crick),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MolecularSequence_ReferenceSeqOrientation {
    Sense,
    Antisense,
}

impl MolecularSequence_ReferenceSeqOrientation {
    pub fn from_string(string: &str) -> Option<MolecularSequence_ReferenceSeqOrientation> {
        match string {
            "sense" => Some(MolecularSequence_ReferenceSeqOrientation::Sense),
            "antisense" => Some(MolecularSequence_ReferenceSeqOrientation::Antisense),
            _ => None,
        }
    }
}
