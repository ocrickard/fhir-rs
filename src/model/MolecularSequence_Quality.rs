#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MolecularSequence_Roc::MolecularSequence_Roc;
use crate::model::Quantity::Quantity;
use serde_json::value::Value;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_Quality<'a> {
    pub value: &'a Value,
}

impl MolecularSequence_Quality<'_> {
    /// False negatives, i.e. the number of sites in the Truth Call Set for which there
    /// is no path through the Query Call Set that is consistent with all of the alleles
    /// at this site, or sites for which there is an inaccurate genotype call for the
    /// event. Sites with correct variant but incorrect genotype are counted here.
    pub fn truth_f_n(&self) -> Option<f64> {
        if let Some(val) = self.value.get("truthFN") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// INDEL / SNP / Undefined variant.
    pub fn fhir_type(&self) -> Option<MolecularSequence_QualityType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(MolecularSequence_QualityType::from_string(&val).unwrap());
        }
        return None;
    }

    /// Harmonic mean of Recall and Precision, computed as: 2 * precision * recall /
    /// (precision + recall).
    pub fn f_score(&self) -> Option<f64> {
        if let Some(val) = self.value.get("fScore") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for queryTP
    pub fn _query_t_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_queryTP") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// TRUTH.TP / (TRUTH.TP + TRUTH.FN).
    pub fn recall(&self) -> Option<f64> {
        if let Some(val) = self.value.get("recall") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for recall
    pub fn _recall(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recall") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// True positives, from the perspective of the query data, i.e. the number of sites
    /// in the Query Call Set for which there are paths through the Truth Call Set that
    /// are consistent with all of the alleles at this site, and for which there is an
    /// accurate genotype call for the event.
    pub fn query_t_p(&self) -> Option<f64> {
        if let Some(val) = self.value.get("queryTP") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// End position of the sequence. If the coordinate system is 0-based then end is
    /// exclusive and does not include the last position. If the coordinate system is 1-
    /// base, then end is inclusive and includes the last position.
    pub fn end(&self) -> Option<i64> {
        if let Some(val) = self.value.get("end") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// Extensions for precision
    pub fn _precision(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_precision") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Receiver Operator Characteristic (ROC) Curve  to give sensitivity/specificity
    /// tradeoff.
    pub fn roc(&self) -> Option<MolecularSequence_Roc> {
        if let Some(val) = self.value.get("roc") {
            return Some(MolecularSequence_Roc { value: val });
        }
        return None;
    }

    /// Extensions for queryFP
    pub fn _query_f_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_queryFP") {
            return Some(Element { value: val });
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

    /// Extensions for fScore
    pub fn _f_score(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fScore") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The number of false positives where the non-REF alleles in the Truth and Query
    /// Call Sets match (i.e. cases where the truth is 1/1 and the query is 0/1 or
    /// similar).
    pub fn gt_f_p(&self) -> Option<f64> {
        if let Some(val) = self.value.get("gtFP") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// QUERY.TP / (QUERY.TP + QUERY.FP).
    pub fn precision(&self) -> Option<f64> {
        if let Some(val) = self.value.get("precision") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// False positives, i.e. the number of sites in the Query Call Set for which there
    /// is no path through the Truth Call Set that is consistent with this site. Sites
    /// with correct variant but incorrect genotype are counted here.
    pub fn query_f_p(&self) -> Option<f64> {
        if let Some(val) = self.value.get("queryFP") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Gold standard sequence used for comparing against.
    pub fn standard_sequence(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("standardSequence") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Start position of the sequence. If the coordinate system is either 0-based or 1-
    /// based, then start position is inclusive.
    pub fn start(&self) -> Option<i64> {
        if let Some(val) = self.value.get("start") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// The score of an experimentally derived feature such as a p-value
    /// ([SO:0001685](http://www.sequenceontology.org/browser/current_svn/term/SO:000168
    /// 5)).
    pub fn score(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("score") {
            return Some(Quantity { value: val });
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

    /// Extensions for truthTP
    pub fn _truth_t_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_truthTP") {
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

    /// Extensions for start
    pub fn _start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_start") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Which method is used to get sequence quality.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// True positives, from the perspective of the truth data, i.e. the number of sites
    /// in the Truth Call Set for which there are paths through the Query Call Set that
    /// are consistent with all of the alleles at this site, and for which there is an
    /// accurate genotype call for the event.
    pub fn truth_t_p(&self) -> Option<f64> {
        if let Some(val) = self.value.get("truthTP") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Extensions for gtFP
    pub fn _gt_f_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gtFP") {
            return Some(Element { value: val });
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

    /// Extensions for truthFN
    pub fn _truth_f_n(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_truthFN") {
            return Some(Element { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.truth_f_n() {}
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.f_score() {}
        if let Some(_val) = self._query_t_p() {
            _val.validate();
        }
        if let Some(_val) = self.recall() {}
        if let Some(_val) = self._recall() {
            _val.validate();
        }
        if let Some(_val) = self.query_t_p() {}
        if let Some(_val) = self.end() {}
        if let Some(_val) = self._precision() {
            _val.validate();
        }
        if let Some(_val) = self.roc() {
            _val.validate();
        }
        if let Some(_val) = self._query_f_p() {
            _val.validate();
        }
        if let Some(_val) = self._end() {
            _val.validate();
        }
        if let Some(_val) = self._f_score() {
            _val.validate();
        }
        if let Some(_val) = self.gt_f_p() {}
        if let Some(_val) = self.precision() {}
        if let Some(_val) = self.query_f_p() {}
        if let Some(_val) = self.standard_sequence() {
            _val.validate();
        }
        if let Some(_val) = self.start() {}
        if let Some(_val) = self.score() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._truth_t_p() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._start() {
            _val.validate();
        }
        if let Some(_val) = self.method() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self.truth_t_p() {}
        if let Some(_val) = self._gt_f_p() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._truth_f_n() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum MolecularSequence_QualityType {
    Indel,
    Snp,
    Unknown,
}

impl MolecularSequence_QualityType {
    pub fn from_string(string: &str) -> Option<MolecularSequence_QualityType> {
        match string {
            "indel" => Some(MolecularSequence_QualityType::Indel),
            "snp" => Some(MolecularSequence_QualityType::Snp),
            "unknown" => Some(MolecularSequence_QualityType::Unknown),
            _ => None,
        }
    }
}
