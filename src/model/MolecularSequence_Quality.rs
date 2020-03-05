#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::MolecularSequence_Roc::MolecularSequence_Roc;
use crate::model::Quantity::Quantity;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Raw data describing a biological sequence.

#[derive(Debug)]
pub struct MolecularSequence_Quality<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl MolecularSequence_Quality<'_> {
    pub fn new(value: &Value) -> MolecularSequence_Quality {
        MolecularSequence_Quality {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for end
    pub fn _end(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_end") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for fScore
    pub fn _f_score(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_fScore") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for gtFP
    pub fn _gt_f_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_gtFP") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for precision
    pub fn _precision(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_precision") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for queryFP
    pub fn _query_f_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_queryFP") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for queryTP
    pub fn _query_t_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_queryTP") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for recall
    pub fn _recall(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_recall") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for start
    pub fn _start(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_start") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for truthFN
    pub fn _truth_f_n(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_truthFN") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for truthTP
    pub fn _truth_t_p(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_truthTP") {
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

    /// End position of the sequence. If the coordinate system is 0-based then end is
    /// exclusive and does not include the last position. If the coordinate system is 1-
    /// base, then end is inclusive and includes the last position.
    pub fn end(&self) -> Option<i64> {
        if let Some(val) = self.value.get("end") {
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// The number of false positives where the non-REF alleles in the Truth and Query
    /// Call Sets match (i.e. cases where the truth is 1/1 and the query is 0/1 or
    /// similar).
    pub fn gt_f_p(&self) -> Option<f64> {
        if let Some(val) = self.value.get("gtFP") {
            return Some(val.as_f64().unwrap());
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

    /// Which method is used to get sequence quality.
    pub fn method(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("method") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// TRUTH.TP / (TRUTH.TP + TRUTH.FN).
    pub fn recall(&self) -> Option<f64> {
        if let Some(val) = self.value.get("recall") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// Receiver Operator Characteristic (ROC) Curve  to give sensitivity/specificity
    /// tradeoff.
    pub fn roc(&self) -> Option<MolecularSequence_Roc> {
        if let Some(val) = self.value.get("roc") {
            return Some(MolecularSequence_Roc {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The score of an experimentally derived feature such as a p-value
    /// ([SO:0001685](http://www.sequenceontology.org/browser/current_svn/term/SO:000168
    /// 5)).
    pub fn score(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("score") {
            return Some(Quantity {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Gold standard sequence used for comparing against.
    pub fn standard_sequence(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("standardSequence") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// INDEL / SNP / Undefined variant.
    pub fn fhir_type(&self) -> Option<MolecularSequence_QualityType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(MolecularSequence_QualityType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._end() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._f_score() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._gt_f_p() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._precision() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._query_f_p() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._query_t_p() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._recall() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._start() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._truth_f_n() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._truth_t_p() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.end() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.f_score() {}
        if let Some(_val) = self.gt_f_p() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.method() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.precision() {}
        if let Some(_val) = self.query_f_p() {}
        if let Some(_val) = self.query_t_p() {}
        if let Some(_val) = self.recall() {}
        if let Some(_val) = self.roc() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.score() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.standard_sequence() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.start() {}
        if let Some(_val) = self.truth_f_n() {}
        if let Some(_val) = self.truth_t_p() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct MolecularSequence_QualityBuilder {
    pub(crate) value: Value,
}

impl MolecularSequence_QualityBuilder {
    pub fn build(&self) -> MolecularSequence_Quality {
        MolecularSequence_Quality {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: MolecularSequence_Quality) -> MolecularSequence_QualityBuilder {
        MolecularSequence_QualityBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> MolecularSequence_QualityBuilder {
        let mut __value: Value = json!({});
        return MolecularSequence_QualityBuilder { value: __value };
    }

    pub fn _end<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_end"] = json!(val.value);
        return self;
    }

    pub fn _f_score<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_fScore"] = json!(val.value);
        return self;
    }

    pub fn _gt_f_p<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_gtFP"] = json!(val.value);
        return self;
    }

    pub fn _precision<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_precision"] = json!(val.value);
        return self;
    }

    pub fn _query_f_p<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_queryFP"] = json!(val.value);
        return self;
    }

    pub fn _query_t_p<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_queryTP"] = json!(val.value);
        return self;
    }

    pub fn _recall<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_recall"] = json!(val.value);
        return self;
    }

    pub fn _start<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_start"] = json!(val.value);
        return self;
    }

    pub fn _truth_f_n<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_truthFN"] = json!(val.value);
        return self;
    }

    pub fn _truth_t_p<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_truthTP"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn end<'a>(&'a mut self, val: i64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["end"] = json!(val);
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn f_score<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["fScore"] = json!(val);
        return self;
    }

    pub fn gt_f_p<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["gtFP"] = json!(val);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn method<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["method"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn precision<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["precision"] = json!(val);
        return self;
    }

    pub fn query_f_p<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["queryFP"] = json!(val);
        return self;
    }

    pub fn query_t_p<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["queryTP"] = json!(val);
        return self;
    }

    pub fn recall<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["recall"] = json!(val);
        return self;
    }

    pub fn roc<'a>(
        &'a mut self,
        val: MolecularSequence_Roc,
    ) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["roc"] = json!(val.value);
        return self;
    }

    pub fn score<'a>(&'a mut self, val: Quantity) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["score"] = json!(val.value);
        return self;
    }

    pub fn standard_sequence<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["standardSequence"] = json!(val.value);
        return self;
    }

    pub fn start<'a>(&'a mut self, val: i64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["start"] = json!(val);
        return self;
    }

    pub fn truth_f_n<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["truthFN"] = json!(val);
        return self;
    }

    pub fn truth_t_p<'a>(&'a mut self, val: f64) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["truthTP"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: MolecularSequence_QualityType,
    ) -> &'a mut MolecularSequence_QualityBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
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

    pub fn to_string(&self) -> String {
        match self {
            MolecularSequence_QualityType::Indel => "indel".to_string(),
            MolecularSequence_QualityType::Snp => "snp".to_string(),
            MolecularSequence_QualityType::Unknown => "unknown".to_string(),
        }
    }
}
