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
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a
/// policy or agreement.

#[derive(Debug)]
pub struct Contract<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Contract<'_> {
    pub fn new(value: &Value) -> Contract {
        Contract {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for alias
    pub fn _alias(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_alias") {
            return Some(
                val.into_iter()
                    .map(|e| Element {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for implicitRules
    pub fn _implicit_rules(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_implicitRules") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for instantiatesUri
    pub fn _instantiates_uri(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_instantiatesUri") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for issued
    pub fn _issued(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_issued") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for name
    pub fn _name(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_name") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for status
    pub fn _status(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_status") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for subtitle
    pub fn _subtitle(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_subtitle") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for url
    pub fn _url(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_url") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Relevant time or time-period when this Contract is applicable.
    pub fn applies(&self) -> Option<Period> {
        if let Some(val) = self.value.get("applies") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The individual or organization that authored the Contract definition,
    /// derivative, or instance in any legal state.
    pub fn author(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("author") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| ResourceList {
                        value: Cow::Borrowed(e),
                    })
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
            return Some(Contract_ContentDefinition {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The minimal content derived from the basal information source at a specific
    /// stage in its lifecycle.
    pub fn content_derivative(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("contentDerivative") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Event resulting in discontinuation or termination of this Contract instance by
    /// one or more parties to the contract.
    pub fn expiration_type(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("expirationType") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Contract_Friendly {
                        value: Cow::Borrowed(e),
                    })
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

    /// Unique identifier for this Contract or a derivative that references a Source
    /// Contract.
    pub fn identifier(&self) -> Option<Vec<Identifier>> {
        if let Some(Value::Array(val)) = self.value.get("identifier") {
            return Some(
                val.into_iter()
                    .map(|e| Identifier {
                        value: Cow::Borrowed(e),
                    })
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

    /// The URL pointing to a FHIR-defined Contract Definition that is adhered to in
    /// whole or part by this Contract.
    pub fn instantiates_canonical(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("instantiatesCanonical") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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

    /// When this  Contract was issued.
    pub fn issued(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("issued") {
            return Some(string);
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

    /// List of Legal expressions or representations of this Contract.
    pub fn legal(&self) -> Option<Vec<Contract_Legal>> {
        if let Some(Value::Array(val)) = self.value.get("legal") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Legal {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Legal states of the formation of a legal instrument, which is a formally
    /// executed written document that can be formally attributed to its author, records
    /// and formally expresses a legally enforceable act, process, or contractual duty,
    /// obligation, or right, and therefore evidences that act, process, or agreement.
    pub fn legal_state(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("legalState") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Legally binding Contract: This is the signed and legally recognized
    /// representation of the Contract, which is considered the "source of truth" and
    /// which would be the basis for legal action related to enforcement of this
    /// Contract.
    pub fn legally_binding_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("legallyBindingAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Legally binding Contract: This is the signed and legally recognized
    /// representation of the Contract, which is considered the "source of truth" and
    /// which would be the basis for legal action related to enforcement of this
    /// Contract.
    pub fn legally_binding_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("legallyBindingReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The metadata about the resource. This is content that is maintained by the
    /// infrastructure. Changes to the content might not always be associated with
    /// version changes to the resource.
    pub fn meta(&self) -> Option<Meta> {
        if let Some(val) = self.value.get("meta") {
            return Some(Meta {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
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

    /// Links to Provenance records for past versions of this Contract definition,
    /// derivative, or instance, which identify key state transitions or updates that
    /// are likely to be relevant to a user looking at the current version of the
    /// Contract.  The Provence.entity indicates the target that was changed in the
    /// update. http://build.fhir.org/provenance-definitions.html#Provenance.entity.
    pub fn relevant_history(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("relevantHistory") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// List of Computable Policy Rule Language Representations of this Contract.
    pub fn rule(&self) -> Option<Vec<Contract_Rule>> {
        if let Some(Value::Array(val)) = self.value.get("rule") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Rule {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A selector of legal concerns for this Contract definition, derivative, or
    /// instance in any legal state.
    pub fn scope(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("scope") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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
                    .map(|e| Contract_Signer {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Sites in which the contract is complied with,  exercised, or in force.
    pub fn site(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("site") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
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

    /// Sub-category for the Contract that distinguishes the kinds of systems that would
    /// be interested in the Contract within the context of the Contract's scope.
    pub fn sub_type(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("subType") {
            return Some(
                val.into_iter()
                    .map(|e| CodeableConcept {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
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

    /// Information that may be needed by/relevant to the performer in their execution
    /// of this term action.
    pub fn supporting_info(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("supportingInfo") {
            return Some(
                val.into_iter()
                    .map(|e| Reference {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// One or more Contract Provisions, which may be related and conveyed as a group,
    /// and may contain nested groups.
    pub fn term(&self) -> Option<Vec<Contract_Term>> {
        if let Some(Value::Array(val)) = self.value.get("term") {
            return Some(
                val.into_iter()
                    .map(|e| Contract_Term {
                        value: Cow::Borrowed(e),
                    })
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
            return Some(Narrative {
                value: Cow::Borrowed(val),
            });
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

    /// Narrows the range of legal concerns to focus on the achievement of specific
    /// contractual objectives.
    pub fn topic_codeable_concept(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("topicCodeableConcept") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Narrows the range of legal concerns to focus on the achievement of specific
    /// contractual objectives.
    pub fn topic_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("topicReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
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
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// An edition identifier used for business purposes to label business significant
    /// variants.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._alias() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._instantiates_uri() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._issued() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._subtitle() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._title() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._url() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._version() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.alias() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.applies() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.author() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.authority() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.content_definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.content_derivative() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.domain() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.expiration_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.friendly() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.instantiates_canonical() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.instantiates_uri() {}
        if let Some(_val) = self.issued() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.legal() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.legal_state() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.legally_binding_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.legally_binding_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.meta() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.relevant_history() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.rule() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.scope() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.signer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.site() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.sub_type() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.subject() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.subtitle() {}
        if let Some(_val) = self.supporting_info() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.term() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.topic_codeable_concept() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.topic_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.fhir_type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ContractBuilder {
    pub(crate) value: Value,
}

impl ContractBuilder {
    pub fn build(&self) -> Contract {
        Contract {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Contract) -> ContractBuilder {
        ContractBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ContractBuilder {
        let mut __value: Value = json!({});
        return ContractBuilder { value: __value };
    }

    pub fn _alias<'a>(&'a mut self, val: Vec<Element>) -> &'a mut ContractBuilder {
        self.value["_alias"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _instantiates_uri<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_instantiatesUri"] = json!(val.value);
        return self;
    }

    pub fn _issued<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_issued"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _subtitle<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_subtitle"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut ContractBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn alias<'a>(&'a mut self, val: Vec<&str>) -> &'a mut ContractBuilder {
        self.value["alias"] = json!(val);
        return self;
    }

    pub fn applies<'a>(&'a mut self, val: Period) -> &'a mut ContractBuilder {
        self.value["applies"] = json!(val.value);
        return self;
    }

    pub fn author<'a>(&'a mut self, val: Reference) -> &'a mut ContractBuilder {
        self.value["author"] = json!(val.value);
        return self;
    }

    pub fn authority<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ContractBuilder {
        self.value["authority"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ContractBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn content_definition<'a>(
        &'a mut self,
        val: Contract_ContentDefinition,
    ) -> &'a mut ContractBuilder {
        self.value["contentDefinition"] = json!(val.value);
        return self;
    }

    pub fn content_derivative<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ContractBuilder {
        self.value["contentDerivative"] = json!(val.value);
        return self;
    }

    pub fn domain<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ContractBuilder {
        self.value["domain"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn expiration_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ContractBuilder {
        self.value["expirationType"] = json!(val.value);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ContractBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn friendly<'a>(&'a mut self, val: Vec<Contract_Friendly>) -> &'a mut ContractBuilder {
        self.value["friendly"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ContractBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn instantiates_canonical<'a>(&'a mut self, val: Reference) -> &'a mut ContractBuilder {
        self.value["instantiatesCanonical"] = json!(val.value);
        return self;
    }

    pub fn instantiates_uri<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["instantiatesUri"] = json!(val);
        return self;
    }

    pub fn issued<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["issued"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn legal<'a>(&'a mut self, val: Vec<Contract_Legal>) -> &'a mut ContractBuilder {
        self.value["legal"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn legal_state<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ContractBuilder {
        self.value["legalState"] = json!(val.value);
        return self;
    }

    pub fn legally_binding_attachment<'a>(
        &'a mut self,
        val: Attachment,
    ) -> &'a mut ContractBuilder {
        self.value["legallyBindingAttachment"] = json!(val.value);
        return self;
    }

    pub fn legally_binding_reference<'a>(&'a mut self, val: Reference) -> &'a mut ContractBuilder {
        self.value["legallyBindingReference"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ContractBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ContractBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn relevant_history<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ContractBuilder {
        self.value["relevantHistory"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn rule<'a>(&'a mut self, val: Vec<Contract_Rule>) -> &'a mut ContractBuilder {
        self.value["rule"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn scope<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ContractBuilder {
        self.value["scope"] = json!(val.value);
        return self;
    }

    pub fn signer<'a>(&'a mut self, val: Vec<Contract_Signer>) -> &'a mut ContractBuilder {
        self.value["signer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn site<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ContractBuilder {
        self.value["site"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn sub_type<'a>(&'a mut self, val: Vec<CodeableConcept>) -> &'a mut ContractBuilder {
        self.value["subType"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn subject<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ContractBuilder {
        self.value["subject"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn subtitle<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["subtitle"] = json!(val);
        return self;
    }

    pub fn supporting_info<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ContractBuilder {
        self.value["supportingInfo"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn term<'a>(&'a mut self, val: Vec<Contract_Term>) -> &'a mut ContractBuilder {
        self.value["term"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ContractBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn topic_codeable_concept<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut ContractBuilder {
        self.value["topicCodeableConcept"] = json!(val.value);
        return self;
    }

    pub fn topic_reference<'a>(&'a mut self, val: Reference) -> &'a mut ContractBuilder {
        self.value["topicReference"] = json!(val.value);
        return self;
    }

    pub fn fhir_type<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ContractBuilder {
        self.value["type"] = json!(val.value);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut ContractBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}
