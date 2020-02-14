#![allow(unused_imports, non_camel_case_types)]

use crate::model::Annotation::Annotation;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::EffectEvidenceSynthesis_Certainty::EffectEvidenceSynthesis_Certainty;
use crate::model::EffectEvidenceSynthesis_EffectEstimate::EffectEvidenceSynthesis_EffectEstimate;
use crate::model::EffectEvidenceSynthesis_ResultsByExposure::EffectEvidenceSynthesis_ResultsByExposure;
use crate::model::EffectEvidenceSynthesis_SampleSize::EffectEvidenceSynthesis_SampleSize;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// The EffectEvidenceSynthesis resource describes the difference in an outcome
/// between exposures states in a population where the effect estimate is derived
/// from a combination of research studies.

#[derive(Debug)]
pub struct EffectEvidenceSynthesis<'a> {
    pub value: &'a Value,
}

impl EffectEvidenceSynthesis<'_> {
    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A legal or geographic region in which the effect evidence synthesis is intended
    /// to be used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The date  (and optionally time) when the effect evidence synthesis was
    /// published. The date must change when the business version changes and it must
    /// change if the status code changes. In addition, it should change when the
    /// substantive content of the effect evidence synthesis changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
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

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A reference to a EvidenceVariable resource that defines the comparison exposure
    /// for the research.
    pub fn exposure_alternative(&self) -> Reference {
        Reference {
            value: &self.value["exposureAlternative"],
        }
    }

    /// Extensions for approvalDate
    pub fn _approval_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_approvalDate") {
            return Some(Element { value: val });
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

    /// A short, descriptive, user-friendly title for the effect evidence synthesis.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
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

    /// A formal identifier that is used to identify this effect evidence synthesis when
    /// it is represented in other formats, or referenced in a specification, model,
    /// design or an instance.
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A human-readable string to clarify or explain concepts about the resource.
    pub fn note(&self) -> Option<Vec<Annotation>> {
        if let Some(Value::Array(val)) = self.value.get("note") {
            return Some(
                val.into_iter()
                    .map(|e| Annotation { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Related artifacts such as additional documentation, justification, or
    /// bibliographic references.
    pub fn related_artifact(&self) -> Option<Vec<RelatedArtifact>> {
        if let Some(Value::Array(val)) = self.value.get("relatedArtifact") {
            return Some(
                val.into_iter()
                    .map(|e| RelatedArtifact { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a EvidenceVariable resource that defines the population for the
    /// research.
    pub fn population(&self) -> Reference {
        Reference {
            value: &self.value["population"],
        }
    }

    /// A description of the results for each exposure considered in the effect
    /// estimate.
    pub fn results_by_exposure(&self) -> Option<Vec<EffectEvidenceSynthesis_ResultsByExposure>> {
        if let Some(Value::Array(val)) = self.value.get("resultsByExposure") {
            return Some(
                val.into_iter()
                    .map(|e| EffectEvidenceSynthesis_ResultsByExposure { value: e })
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

    /// A free text natural language description of the effect evidence synthesis from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// The status of this effect evidence synthesis. Enables tracking the life-cycle of
    /// the content.
    pub fn status(&self) -> Option<EffectEvidenceSynthesisStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(EffectEvidenceSynthesisStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// A natural language name identifying the effect evidence synthesis. This name
    /// should be usable as an identifier for the module by machine processing
    /// applications such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An individiual or organization primarily involved in the creation and
    /// maintenance of the content.
    pub fn author(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("author") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An absolute URI that is used to identify this effect evidence synthesis when it
    /// is referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this effect evidence
    /// synthesis is (or will be) published. This URL can be the target of a canonical
    /// reference. It SHALL remain the same when the effect evidence synthesis is stored
    /// on different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A reference to a EvidenceVariable resomece that defines the outcome for the
    /// research.
    pub fn outcome(&self) -> Reference {
        Reference {
            value: &self.value["outcome"],
        }
    }

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The name of the organization or individual that published the effect evidence
    /// synthesis.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// The identifier that is used to identify this version of the effect evidence
    /// synthesis when it is referenced in a specification, model, design or instance.
    /// This is an arbitrary value managed by the effect evidence synthesis author and
    /// is not expected to be globally unique. For example, it might be a timestamp
    /// (e.g. yyyymmdd) if a managed version is not available. There is also no
    /// expectation that versions can be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date on which the resource content was last reviewed. Review happens
    /// periodically after approval but does not change the original approval date.
    pub fn last_review_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("lastReviewDate") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An individual or organization primarily responsible for internal coherence of
    /// the content.
    pub fn editor(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("editor") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate effect evidence
    /// synthesis instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An individual or organization responsible for officially endorsing the content
    /// for use in some setting.
    pub fn endorser(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("endorser") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An individual or organization primarily responsible for review of some aspect of
    /// the content.
    pub fn reviewer(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("reviewer") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail { value: e })
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

    /// Extensions for lastReviewDate
    pub fn _last_review_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastReviewDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Type of study eg randomized trial.
    pub fn study_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("studyType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A description of the size of the sample involved in the synthesis.
    pub fn sample_size(&self) -> Option<EffectEvidenceSynthesis_SampleSize> {
        if let Some(val) = self.value.get("sampleSize") {
            return Some(EffectEvidenceSynthesis_SampleSize { value: val });
        }
        return None;
    }

    /// The estimated effect of the exposure variant.
    pub fn effect_estimate(&self) -> Option<Vec<EffectEvidenceSynthesis_EffectEstimate>> {
        if let Some(Value::Array(val)) = self.value.get("effectEstimate") {
            return Some(
                val.into_iter()
                    .map(|e| EffectEvidenceSynthesis_EffectEstimate { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A description of the certainty of the effect estimate.
    pub fn certainty(&self) -> Option<Vec<EffectEvidenceSynthesis_Certainty>> {
        if let Some(Value::Array(val)) = self.value.get("certainty") {
            return Some(
                val.into_iter()
                    .map(|e| EffectEvidenceSynthesis_Certainty { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period during which the effect evidence synthesis content was or is planned
    /// to be in active use.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// A reference to a EvidenceVariable resource that defines the exposure for the
    /// research.
    pub fn exposure(&self) -> Reference {
        Reference {
            value: &self.value["exposure"],
        }
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// A copyright statement relating to the effect evidence synthesis and/or its
    /// contents. Copyright statements are generally legal restrictions on the use and
    /// publishing of the effect evidence synthesis.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// Descriptive topics related to the content of the EffectEvidenceSynthesis. Topics
    /// provide a high-level categorization grouping types of EffectEvidenceSynthesiss
    /// that can be useful for filtering and searching.
    pub fn topic(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("topic") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Type of synthesis eg meta-analysis.
    pub fn synthesis_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("synthesisType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        let _ = self.exposure_alternative().validate();
        if let Some(_val) = self._approval_date() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.note() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.related_artifact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        let _ = self.population().validate();
        if let Some(_val) = self.results_by_exposure() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._copyright() {
            _val.validate();
        }
        if let Some(_val) = self.author() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        let _ = self.outcome().validate();
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.last_review_date() {}
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.editor() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.endorser() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reviewer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._last_review_date() {
            _val.validate();
        }
        if let Some(_val) = self.study_type() {
            _val.validate();
        }
        if let Some(_val) = self.sample_size() {
            _val.validate();
        }
        if let Some(_val) = self.effect_estimate() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.certainty() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.effective_period() {
            _val.validate();
        }
        let _ = self.exposure().validate();
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self.approval_date() {}
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.topic() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.synthesis_type() {
            _val.validate();
        }
        return true;
    }
}

#[derive(Debug)]
pub enum EffectEvidenceSynthesisStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl EffectEvidenceSynthesisStatus {
    pub fn from_string(string: &str) -> Option<EffectEvidenceSynthesisStatus> {
        match string {
            "draft" => Some(EffectEvidenceSynthesisStatus::Draft),
            "active" => Some(EffectEvidenceSynthesisStatus::Active),
            "retired" => Some(EffectEvidenceSynthesisStatus::Retired),
            "unknown" => Some(EffectEvidenceSynthesisStatus::Unknown),
            _ => None,
        }
    }
}
