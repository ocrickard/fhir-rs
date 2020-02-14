#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Contract_ContentDefinition::Contract_ContentDefinition;
use crate::model::Contract_Friendly::Contract_Friendly;
use crate::model::Contract_Legal::Contract_Legal;
use crate::model::Contract_Rule::Contract_Rule;
use crate::model::Contract_Signer::Contract_Signer;
use crate::model::Contract_Term::Contract_Term;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::value::Value;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract<'a> {
    pub value: &'a Value,
}

impl Contract<'_> {
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

    /// Recognized governance framework or system operating with a circumscribed scope
    /// in accordance with specified principles, policies, processes or procedures for
    /// managing rights, actions, or behaviors of parties or principals relative to
    /// resources.
    pub fn domain(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("domain") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The status of the resource instance.
    pub fn status(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("status") {
            return Some(string);
        }
        return None;
    }

    /// Canonical identifier for this contract, represented as a URI (globally unique).
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// A formally or informally recognized grouping of people, principals,
    /// organizations, or jurisdictions formed for the purpose of achieving some form of
    /// collective action such as the promulgation, administration and enforcement of
    /// contracts and policies.
    pub fn authority(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("authority") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Alternative representation of the title for this Contract definition,
    /// derivative, or instance in any legal state., e.g., a domain specific contract
    /// number related to legislation.
    pub fn alias(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("alias") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A selector of legal concerns for this Contract definition, derivative, or
    /// instance in any legal state.
    pub fn scope(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("scope") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Narrows the range of legal concerns to focus on the achievement of specific
    /// contractual objectives.
    pub fn topic_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("topicReference") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// Links to Provenance records for past versions of this Contract definition,
    /// derivative, or instance, which identify key state transitions or updates that
    /// are likely to be relevant to a user looking at the current version of the
    /// Contract.  The Provence.entity indicates the target that was changed in the
    /// update. http://build.fhir.org/provenance-definitions.html#Provenance.entity.
    pub fn relevant_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("relevantHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Parties with legal standing in the Contract, including the principal parties,
    /// the grantor(s) and grantee(s), which are any person or organization bound by the
    /// contract, and any ancillary parties, which facilitate the execution of the
    /// contract such as a notary or witness.
    pub fn signer(&self) -> Option<Vec<Contract_Signer>> {
        if let Some(Value::Array(val)) = self.value.get("signer") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Signer { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The target entity impacted by or of interest to parties to the agreement.
    pub fn subject(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("subject") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An explanatory or alternate user-friendly title for this Contract definition,
    /// derivative, or instance in any legal state.t giving additional information about
    /// its content.
    pub fn subtitle(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("subtitle") {
            return Some(string);
        }
        return None;
    }

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instantiatesUri") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Sites in which the contract is complied with,  exercised, or in force.
    pub fn site(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("site") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The minimal content derived from the basal information source at a specific
    /// stage in its lifecycle.
    pub fn content_derivative(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("contentDerivative") {
            return Some(CodeableConcept { value: val });
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

    /// Legally binding Contract: This is the signed and legally recognized
    /// representation of the Contract, which is considered the "source of truth" and
    /// which would be the basis for legal action related to enforcement of this
    /// Contract.
    pub fn legally_binding_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("legallyBindingAttachment") {
            return Some(Attachment { value: val });
        }
        return None;
    }

    /// Legally binding Contract: This is the signed and legally recognized
    /// representation of the Contract, which is considered the "source of truth" and
    /// which would be the basis for legal action related to enforcement of this
    /// Contract.
    pub fn legally_binding_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("legallyBindingReference") {
            return Some(Reference { value: val });
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

    /// When this  Contract was issued.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
            return Some(string);
        }
        return None;
    }

    /// The URL pointing to a FHIR-defined Contract Definition that is adhered to in
    /// whole or part by this Contract.
    pub fn instantiates_canonical(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("instantiatesCanonical") {
            return Some(Reference { value: val });
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

    /// Sub-category for the Contract that distinguishes the kinds of systems that would
    /// be interested in the Contract within the context of the Contract's scope.
    pub fn sub_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subType") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept { value: e })
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

    /// The logical id of the resource, as used in the URL for the resource. Once
    /// assigned, this value never changes.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
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

    /// Legal states of the formation of a legal instrument, which is a formally
    /// executed written document that can be formally attributed to its author, records
    /// and formally expresses a legally enforceable act, process, or contractual duty,
    /// obligation, or right, and therefore evidences that act, process, or agreement.
    pub fn legal_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("legalState") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// List of Computable Policy Rule Language Representations of this Contract.
    pub fn rule(&self) -> Option<Vec<Contract_Rule>> {
        if let Some(Value::Array(val)) = self.value.get("rule") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Rule { value: e })
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

    /// Extensions for subtitle
    pub fn _subtitle(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subtitle") {
            return Some(Element { value: val });
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

    /// The individual or organization that authored the Contract definition,
    /// derivative, or instance in any legal state.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference { value: val });
        }
        return None;
    }

    /// List of Legal expressions or representations of this Contract.
    pub fn legal(&self) -> Option<Vec<Contract_Legal>> {
        if let Some(Value::Array(val)) = self.value.get("legal") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Legal { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Unique identifier for this Contract or a derivative that references a Source
    /// Contract.
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

    /// An edition identifier used for business purposes to label business significant
    /// variants.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// A natural language name identifying this Contract definition, derivative, or
    /// instance in any legal state. Provides additional information about its content.
    /// This name should be usable as an identifier for the module by machine processing
    /// applications such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
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

    /// One or more Contract Provisions, which may be related and conveyed as a group,
    /// and may contain nested groups.
    pub fn term(&self) -> Option<Vec<Contract_Term>> {
        if let Some(Value::Array(val)) = self.value.get("term") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Term { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Precusory content developed with a focus and intent of supporting the formation
    /// a Contract instance, which may be associated with and transformable into a
    /// Contract.
    pub fn content_definition(&self) -> Option<Contract_ContentDefinition> {
        if let Some(val) = self.value.get("contentDefinition") {
            return Some(Contract_ContentDefinition { value: val });
        }
        return None;
    }

    /// The "patient friendly language" versionof the Contract in whole or in parts.
    /// "Patient friendly language" means the representation of the Contract and
    /// Contract Provisions in a manner that is readily accessible and understandable by
    /// a layperson in accordance with best practices for communication styles that
    /// ensure that those agreeing to or signing the Contract understand the roles,
    /// actions, obligations, responsibilities, and implication of the agreement.
    pub fn friendly(&self) -> Option<Vec<Contract_Friendly>> {
        if let Some(Value::Array(val)) = self.value.get("friendly") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Friendly { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A high-level category for the legal instrument, whether constructed as a
    /// Contract definition, derivative, or instance in any legal state.  Provides
    /// additional information about its content within the context of the Contract's
    /// scope to distinguish the kinds of systems that would be interested in the
    /// contract.
    pub fn fhir_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("type") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// The URL pointing to an externally maintained definition that is adhered to in
    /// whole or in part by this Contract.
    pub fn instantiates_uri(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("instantiatesUri") {
            return Some(string);
        }
        return None;
    }

    /// Event resulting in discontinuation or termination of this Contract instance by
    /// one or more parties to the contract.
    pub fn expiration_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("expirationType") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// A short, descriptive, user-friendly title for this Contract definition,
    /// derivative, or instance in any legal state.t giving additional information about
    /// its content.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// Relevant time or time-period when this Contract is applicable.
    pub fn applies(&self) -> Option<Period> {
        if let Some(val) = self.value.get("applies") {
            return Some(Period { value: val });
        }
        return None;
    }

    /// Narrows the range of legal concerns to focus on the achievement of specific
    /// contractual objectives.
    pub fn topic_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("topicCodeableConcept") {
            return Some(CodeableConcept { value: val });
        }
        return None;
    }

    /// Information that may be needed by/relevant to the performer in their execution
    /// of this term action.
    pub fn supporting_info(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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

    /// Extensions for alias
    pub fn _alias(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_alias") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.domain() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.authority() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._issued() {
            _val.validate();
        }
        if let Some(_val) = self.alias() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.scope() {
            _val.validate();
        }
        if let Some(_val) = self.topic_reference() {
            _val.validate();
        }
        if let Some(_val) = self.relevant_history() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.signer() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subject() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.subtitle() {}
        if let Some(_val) = self._instantiates_uri() {
            _val.validate();
        }
        if let Some(_val) = self.site() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.content_derivative() {
            _val.validate();
        }
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self.legally_binding_attachment() {
            _val.validate();
        }
        if let Some(_val) = self.legally_binding_reference() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.issued() {}
        if let Some(_val) = self.instantiates_canonical() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.sub_type() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.id() {}
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.legal_state() {
            _val.validate();
        }
        if let Some(_val) = self.rule() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self._subtitle() {
            _val.validate();
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.author() {
            _val.validate();
        }
        if let Some(_val) = self.legal() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.name() {}
        if let Some(_val) = self._name() {
            _val.validate();
        }
        if let Some(_val) = self.term() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.content_definition() {
            _val.validate();
        }
        if let Some(_val) = self.friendly() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.fhir_type() {
            _val.validate();
        }
        if let Some(_val) = self.instantiates_uri() {}
        if let Some(_val) = self.expiration_type() {
            _val.validate();
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.applies() {
            _val.validate();
        }
        if let Some(_val) = self.topic_codeable_concept() {
            _val.validate();
        }
        if let Some(_val) = self.supporting_info() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self._alias() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        return true;
    }
}
