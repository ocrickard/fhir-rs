#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::MeasureReport_Group::MeasureReport_Group;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// The MeasureReport resource contains the results of the calculation of a measure;
/// and optionally a reference to the resources involved in that calculation.

#[derive(Debug)]
pub struct MeasureReport<'a> {
    pub value: &'a Value,
}

impl MeasureReport<'_> {
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A reference to the Measure that was calculated to produce this report.
    pub fn measure(&self) -> &str {
        self.value.get("measure").unwrap().as_str().unwrap()
    }

    /// A reference to a Bundle containing the Resources that were used in the
    /// calculation of this measure.
    pub fn evaluated_resource(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("evaluatedResource") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The results of the calculation, one for each population group in the measure.
    pub fn group(&self) -> Option<Vec<MeasureReport_Group>> {
        if let Some(Value::Array(val)) = self.value.get("group") {
            return Some(
                val.into_iter()
                    .map(|e| MeasureReport_Group { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The individual, location, or organization that is reporting the data.
    pub fn reporter(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("reporter") {
            return Some(Reference { value: val });
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// The type of measure report. This may be an individual report, which provides the
    /// score for the measure for an individual member of the population; a subject-
    /// listing, which returns the list of members that meet the various
    /// criteria in the measure; a summary report, which returns a population count for
    /// each of the criteria in the measure; or a data-collection, which enables the
    /// MeasureReport to be used to exchange the data-of-interest for a quality measure.
    pub fn fhir_type(&self) -> Option<MeasureReportType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(MeasureReportType::from_string(&val).unwrap());
        }
        return None;
    }

    /// The date this measure report was generated.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
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

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The reporting period for which the report was calculated.
    pub fn period(&self) -> Period {
        Period {
            value: &self.value["period"],
        }
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

    /// The MeasureReport status. No data will be available until the MeasureReport
    /// status is complete.
    pub fn status(&self) -> Option<MeasureReportStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(MeasureReportStatus::from_string(&val).unwrap());
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

    /// A formal identifier that is used to identify this MeasureReport when it is
    /// represented in other formats or referenced in a specification, model, design or
    /// an instance.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier { value: e })
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

    /// Optional subject identifying the individual or individuals the report is for.
    pub fn subject(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subject") {
            return Some(Reference { value: val });
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

    /// Whether improvement in the measure is noted by an increase or decrease in the
    /// measure score.
    pub fn improvement_notation(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("improvementNotation") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._type() {
            _val.validate();
        }
        let _ = self.measure();
        if let Some(_val) = self.evaluated_resource() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.group() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.reporter() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        let _ = self.period().validate();
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.subject() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.improvement_notation() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum MeasureReportType {
    Individual,
    SubjectList,
    Summary,
    DataCollection,
}

impl MeasureReportType {
    pub fn from_string(string: &str) -> Option<MeasureReportType> {
        match string {
            "individual" => Some(MeasureReportType::Individual),
            "subject-list" => Some(MeasureReportType::SubjectList),
            "summary" => Some(MeasureReportType::Summary),
            "data-collection" => Some(MeasureReportType::DataCollection),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum MeasureReportStatus {
    Complete,
    Pending,
    Error,
}

impl MeasureReportStatus {
    pub fn from_string(string: &str) -> Option<MeasureReportStatus> {
        match string {
            "complete" => Some(MeasureReportStatus::Complete),
            "pending" => Some(MeasureReportStatus::Pending),
            "error" => Some(MeasureReportStatus::Error),
            _ => None,
        }
    }
}
