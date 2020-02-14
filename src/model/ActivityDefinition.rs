#![allow(unused_imports, non_camel_case_types)]

use crate::model::ActivityDefinition_DynamicValue::ActivityDefinition_DynamicValue;
use crate::model::ActivityDefinition_Participant::ActivityDefinition_Participant;
use crate::model::Age::Age;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Dosage::Dosage;
use crate::model::Duration::Duration;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Quantity::Quantity;
use crate::model::Range::Range;
use crate::model::Reference::Reference;
use crate::model::RelatedArtifact::RelatedArtifact;
use crate::model::ResourceList::ResourceList;
use crate::model::Timing::Timing;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// This resource allows for the definition of some activity to be performed,
/// independent of a particular patient, practitioner, or other performance context.

#[derive(Debug)]
pub struct ActivityDefinition<'a> {
    pub value: &'a Value,
}

impl ActivityDefinition<'_> {
    /// A profile to which the target of the activity definition is expected to conform.
    pub fn profile(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("profile") {
            return Some(string);
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn timing_date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("timingDateTime") {
            return Some(string);
        }
        return None;
    }

    /// Indicates the sites on the subject's body where the procedure should be
    /// performed (I.e. the target sites).
    pub fn body_site(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("bodySite") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period during which the activity definition content was or is planned to be
    /// in active use.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// A description of the kind of resource the activity definition is representing.
    /// For example, a MedicationRequest, a ServiceRequest, or a CommunicationRequest.
    /// Typically, but not always, this is a Request resource.
    pub fn kind(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("kind") {
            return Some(string);
        }
        return None;
    }

    /// Explanation of why this activity definition is needed and why it has been
    /// designed as it has.
    pub fn purpose(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("purpose") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the facility where the activity will occur; e.g. home, hospital,
    /// specific clinic, etc.
    pub fn location(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("location") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Defines specimen requirements for the action to be performed, such as required
    /// specimens for a lab test.
    pub fn specimen_requirement(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("specimenRequirement") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_purpose") {
            return Some(Element { value: val });
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

    /// Extensions for doNotPerform
    pub fn _do_not_perform(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_doNotPerform") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An absolute URI that is used to identify this activity definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this activity definition
    /// is (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the activity definition is stored on different
    /// servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The status of this activity definition. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<ActivityDefinitionStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ActivityDefinitionStatus::from_string(&val).unwrap());
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

    /// Identifies the food, drug or other product being consumed or supplied in the
    /// activity.
    pub fn product_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("productReference") {
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

    /// Set this to true if the definition is to indicate that a particular activity
    /// should NOT be performed. If true, this element should be interpreted to
    /// reinforce a negative coding. For example NPO as a code with a doNotPerform of
    /// true would still indicate to NOT perform the action.
    pub fn do_not_perform(&self) -> Option<bool> {
        if let Some(val) = self.value.get("doNotPerform") {
            return Some(val.as_bool().unwrap());
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

    /// A reference to a Library resource containing any formal logic used by the
    /// activity definition.
    pub fn library(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("library") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn timing_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("timingPeriod") {
            return Some(Period { value: val });
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

    /// A natural language name identifying the activity definition. This name should be
    /// usable as an identifier for the module by machine processing applications such
    /// as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// Defines the observations that are expected to be produced by the action.
    pub fn observation_result_requirement(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("observationResultRequirement") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn timing_duration(&self) -> Option<Duration> {
        if let Some(val) = self.value.get("timingDuration") {
            return Some(Duration { value: val });
        }
        return None;
    }

    /// A legal or geographic region in which the activity definition is intended to be
    /// used.
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

    /// A short, descriptive, user-friendly title for the activity definition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Identifies the quantity expected to be consumed at once (per dose, per meal,
    /// etc.).
    pub fn quantity(&self) -> Option<Quantity> {
        if let Some(val) = self.value.get("quantity") {
            return Some(Quantity { value: val });
        }
        return None;
    }

    /// Detailed description of the type of activity; e.g. What lab test, what
    /// procedure, what kind of encounter.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Extensions for timingDateTime
    pub fn _timing_date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_timingDateTime") {
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

    /// Provides detailed dosage instructions in the same way that they are described
    /// for MedicationRequest resources.
    pub fn dosage(&self) -> Option<Vec<Dosage>> {
        if let Some(Value::Array(val)) = self.value.get("dosage") {
            return Some(
                val.into_iter()
                    .map(|e| Dosage { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Descriptive topics related to the content of the activity. Topics provide a
    /// high-level categorization of the activity that can be useful for filtering and
    /// searching.
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

    /// Extensions for intent
    pub fn _intent(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_intent") {
            return Some(Element { value: val });
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

    /// An explanatory or alternate title for the activity definition giving additional
    /// information about its content.
    pub fn subtitle(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subtitle") {
            return Some(string);
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// A Boolean value to indicate that this activity definition is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended to be
    /// used for genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// A code or group definition that describes the intended subject of the activity
    /// being defined.
    pub fn subject_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("subjectReference") {
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

    /// A free text natural language description of the activity definition from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// The name of the organization or individual that published the activity
    /// definition.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate activity
    /// definition instances.
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

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
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

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The identifier that is used to identify this version of the activity definition
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the activity definition author and is not expected to
    /// be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence. To provide a version consistent with
    /// the Decision Support Service specification, use the format Major.Minor.Revision
    /// (e.g. 1.0.0). For more information on versioning knowledge assets, refer to the
    /// Decision Support Service specification. Note that a version is required for non-
    /// experimental active assets.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for priority
    pub fn _priority(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_priority") {
            return Some(Element { value: val });
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

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for approvalDate
    pub fn _approval_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_approvalDate") {
            return Some(Element { value: val });
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

    /// Extensions for kind
    pub fn _kind(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_kind") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Indicates the level of authority/intentionality associated with the activity and
    /// where the request should fit into the workflow chain.
    pub fn intent(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("intent") {
            return Some(string);
        }
        return None;
    }

    /// Indicates how quickly the activity  should be addressed with respect to other
    /// requests.
    pub fn priority(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("priority") {
            return Some(string);
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn timing_age(&self) -> Option<Age> {
        if let Some(val) = self.value.get("timingAge") {
            return Some(Age { value: val });
        }
        return None;
    }

    /// Defines observation requirements for the action to be performed, such as body
    /// weight or surface area.
    pub fn observation_requirement(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("observationRequirement") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code or group definition that describes the intended subject of the activity
    /// being defined.
    pub fn subject_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("subjectCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Identifies the food, drug or other product being consumed or supplied in the
    /// activity.
    pub fn product_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("productCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn timing_range(&self) -> Option<Range> {
        if let Some(val) = self.value.get("timingRange") {
            return Some(Range { value: val });
        }
        return None;
    }

    /// Dynamic values that will be evaluated to produce values for elements of the
    /// resulting resource. For example, if the dosage of a medication must be computed
    /// based on the patient's weight, a dynamic value would be used to specify an
    /// expression that calculated the weight, and the path on the request resource that
    /// would contain the result.
    pub fn dynamic_value(&self) -> Option<Vec<ActivityDefinition_DynamicValue>> {
        if let Some(Value::Array(val)) = self.value.get("dynamicValue") {
            return Some(
                val.into_iter()
                    .map(|e| ActivityDefinition_DynamicValue { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A detailed description of how the activity definition is used from a clinical
    /// perspective.
    pub fn usage(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("usage") {
            return Some(string);
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

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element { value: val });
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

    /// A reference to a StructureMap resource that defines a transform that can be
    /// executed to produce the intent resource using the ActivityDefinition instance as
    /// the input.
    pub fn transform(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("transform") {
            return Some(string);
        }
        return None;
    }

    /// Indicates who should participate in performing the action described.
    pub fn participant(&self) -> Option<Vec<ActivityDefinition_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| ActivityDefinition_Participant { value: e })
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

    /// The date  (and optionally time) when the activity definition was published. The
    /// date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the activity definition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for subtitle
    pub fn _subtitle(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subtitle") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// The period, timing or frequency upon which the described activity is to occur.
    pub fn timing_timing(&self) -> Option<Timing> {
        if let Some(val) = self.value.get("timingTiming") {
            return Some(Timing { value: val });
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

    /// The date on which the resource content was approved by the publisher. Approval
    /// happens once when the content is officially approved for usage.
    pub fn approval_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("approvalDate") {
            return Some(string);
        }
        return None;
    }

    /// A copyright statement relating to the activity definition and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the activity definition.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for usage
    pub fn _usage(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_usage") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// A formal identifier that is used to identify this activity definition when it is
    /// represented in other formats, or referenced in a specification, model, design or
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.profile() {}
        if let Some(_val) = self.timing_date_time() {}
        if let Some(_val) = self.body_site() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.effective_period() {
            _val.validate();
        }
        if let Some(_val) = self.kind() {}
        if let Some(_val) = self.purpose() {}
        if let Some(_val) = self.location() {
            _val.validate();
        }
        if let Some(_val) = self.specimen_requirement() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._purpose() {
            _val.validate();
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self._do_not_perform() {
            _val.validate();
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.product_reference() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self.do_not_perform() {}
        if let Some(_val) = self.endorser() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.library() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.timing_period() {
            _val.validate();
        }
        if let Some(_val) = self.last_review_date() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.observation_result_requirement() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.timing_duration() {
            _val.validate();
        }
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.quantity() {
            _val.validate();
        }
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self._timing_date_time() {
            _val.validate();
        }
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.dosage() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.topic() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._intent() {
            _val.validate();
        }
        if let Some(_val) = self._copyright() {
            _val.validate();
        }
        if let Some(_val) = self.subtitle() {}
        if let Some(_val) = self.editor() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.subject_reference() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self._experimental() {
            _val.validate();
        }
        if let Some(_val) = self._priority() {
            _val.validate();
        }
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self._approval_date() {
            _val.validate();
        }
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._kind() {
            _val.validate();
        }
        if let Some(_val) = self.intent() {}
        if let Some(_val) = self.priority() {}
        if let Some(_val) = self.timing_age() {
            _val.validate();
        }
        if let Some(_val) = self.observation_requirement() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subject_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.product_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.timing_range() {
            _val.validate();
        }
        if let Some(_val) = self.dynamic_value() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.usage() {}
        if let Some(_val) = self.author() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.related_artifact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self._last_review_date() {
            _val.validate();
        }
        if let Some(_val) = self.transform() {}
        if let Some(_val) = self.participant() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.reviewer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.date() {}
        if let Some(_val) = self._subtitle() {
            _val.validate();
        }
        if let Some(_val) = self.timing_timing() {
            _val.validate();
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.approval_date() {}
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self._usage() {
            _val.validate();
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
    }
}

#[derive(Debug)]
pub enum ActivityDefinitionStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl ActivityDefinitionStatus {
    pub fn from_string(string: &str) -> Option<ActivityDefinitionStatus> {
        match string {
            "draft" => Some(ActivityDefinitionStatus::Draft),
            "active" => Some(ActivityDefinitionStatus::Active),
            "retired" => Some(ActivityDefinitionStatus::Retired),
            "unknown" => Some(ActivityDefinitionStatus::Unknown),
            _ => None,
        }
    }
}
