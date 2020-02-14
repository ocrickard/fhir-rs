#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::DetectedIssue_Evidence::DetectedIssue_Evidence;
use crate::model::DetectedIssue_Mitigation::DetectedIssue_Mitigation;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Indicates an actual or potential clinical issue with or between one or more
/// active or proposed clinical actions for a patient; e.g. Drug-drug interaction,
/// Ineffective treatment frequency, Procedure-condition conflict, etc.

#[derive(Debug)]
pub struct DetectedIssue<'a> {
    pub value: &'a Value,
}

impl DetectedIssue<'_> {
    /// Extensions for detail
    pub fn _detail(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_detail") {
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

    /// Indicates the degree of importance associated with the identified issue based on
    /// the potential impact on the patient.
    pub fn severity(&self) -> Option<DetectedIssueSeverity> {
        if let Some(Value::String(val)) = self.value.get("severity") {
            return Some(DetectedIssueSeverity::from_string(&val).unwrap());
        }
        return None;
    }

    /// A textual explanation of the detected issue.
    pub fn detail(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("detail") {
            return Some(string);
        }
        return None;
    }

    /// Business identifier associated with the detected issue record.
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

    /// Extensions for identifiedDateTime
    pub fn _identified_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_identifiedDateTime") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Individual or device responsible for the issue being raised.  For example, a
    /// decision support application or a pharmacist conducting a medication review.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference { value: val });
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

    /// The literature, knowledge-base or similar reference that describes the
    /// propensity for the detected issue identified.
    pub fn reference(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("reference") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the resource representing the current activity or proposed activity
    /// that is potentially problematic.
    pub fn implicated(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("implicated") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for reference
    pub fn _reference(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_reference") {
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

    /// The date or period when the detected issue was initially identified.
    pub fn identified_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("identifiedDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the general type of issue identified.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Indicates the patient whose record the detected issue is associated with.
    pub fn patient(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("patient") {
            return Some(Reference { value: val });
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

    /// Indicates the status of the detected issue.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
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

    /// Extensions for severity
    pub fn _severity(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_severity") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date or period when the detected issue was initially identified.
    pub fn identified_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("identifiedPeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Supporting evidence or manifestations that provide the basis for identifying the
    /// detected issue such as a GuidanceResponse or MeasureReport.
    pub fn evidence(&self) -> Option<Vec<DetectedIssue_Evidence>> {
        if let Some(Value::Array(val)) = self.value.get("evidence") {
            return Some(
                val.into_iter()
                    .map(|e| DetectedIssue_Evidence { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Indicates an action that has been taken or is committed to reduce or eliminate
    /// the likelihood of the risk identified by the detected issue from manifesting.
    /// Can also reflect an observation of known mitigating factors that may
    /// reduce/eliminate the need for any action.
    pub fn mitigation(&self) -> Option<Vec<DetectedIssue_Mitigation>> {
        if let Some(Value::Array(val)) = self.value.get("mitigation") {
            return Some(
                val.into_iter()
                    .map(|e| DetectedIssue_Mitigation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._detail() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.severity() {}
        if let Some(_val) = self.detail() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._identified_date_time() {
            _val.validate();
        }
        if let Some(_val) = self.author() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reference() {}
        if let Some(_val) = self.implicated() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._reference() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.identified_date_time() {}
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self.patient() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._severity() {
            _val.validate();
        }
        if let Some(_val) = self.identified_period() {
            _val.validate();
        }
        if let Some(_val) = self.evidence() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.mitigation() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum DetectedIssueSeverity {
    High,
    Moderate,
    Low,
}

impl DetectedIssueSeverity {
    pub fn from_string(string: &str) -> Option<DetectedIssueSeverity> {
        match string {
            "high" => Some(DetectedIssueSeverity::High),
            "moderate" => Some(DetectedIssueSeverity::Moderate),
            "low" => Some(DetectedIssueSeverity::Low),
            _ => None,
        }
    }
}
