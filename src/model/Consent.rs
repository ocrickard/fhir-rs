#![allow(unused_imports, non_camel_case_types)]

use crate::model::Attachment::Attachment;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::Consent_Policy::Consent_Policy;
use crate::model::Consent_Provision::Consent_Provision;
use crate::model::Consent_Verification::Consent_Verification;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A record of a healthcare consumer’s  choices, which permits or denies identified
/// recipient(s) or recipient role(s) to perform one or more actions within a given
/// policy context, for specific purposes and periods of time.

#[derive(Debug)]
pub struct Consent<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Consent<'_> {
    pub fn new(value: &Value) -> Consent {
        Consent {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for dateTime
    pub fn _date_time(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_dateTime") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
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

    /// A classification of the type of consents found in the statement. This element
    /// supports indexing and retrieval of consent statements.
    pub fn category(&self) -> Vec<CodeableConcept> {
        self.value
            .get("category")
            .unwrap()
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| CodeableConcept {
                value: Cow::Borrowed(e),
            })
            .collect::<Vec<_>>()
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

    /// When this  Consent was issued / created / indexed.
    pub fn date_time(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("dateTime") {
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
                    .map(|e| Extension {
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

    /// Unique identifier for this copy of the Consent Statement.
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
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

    /// The organization that manages the consent, and the framework within which it is
    /// executed.
    pub fn organization(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("organization") {
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

    /// The patient/healthcare consumer to whom this consent applies.
    pub fn patient(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("patient") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Either the Grantor, which is the entity responsible for granting the rights
    /// listed in a Consent Directive or the Grantee, which is the entity responsible
    /// for complying with the Consent Directive, including any obligations or
    /// limitations on authorizations and enforcement of prohibitions.
    pub fn performer(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("performer") {
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

    /// The references to the policies that are included in this consent scope. Policies
    /// may be organizational, but are often defined jurisdictionally, or in law.
    pub fn policy(&self) -> Option<Vec<Consent_Policy>> {
        if let Some(Value::Array(val)) = self.value.get("policy") {
            return Some(
                val.into_iter()
                    .map(|e| Consent_Policy {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to the specific base computable regulation or policy.
    pub fn policy_rule(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("policyRule") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// An exception to the base policy of this consent. An exception can be an addition
    /// or removal of access permissions.
    pub fn provision(&self) -> Option<Consent_Provision> {
        if let Some(val) = self.value.get("provision") {
            return Some(Consent_Provision {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A selector of the type of consent being presented: ADR, Privacy, Treatment,
    /// Research.  This list is now extensible.
    pub fn scope(&self) -> CodeableConcept {
        CodeableConcept {
            value: Cow::Borrowed(&self.value["scope"]),
        }
    }

    /// The source on which this consent statement is based. The source might be a
    /// scanned original paper form, or a reference to a consent that links back to such
    /// a source, a reference to a document repository (e.g. XDS) that stores the
    /// original consent document.
    pub fn source_attachment(&self) -> Option<Attachment> {
        if let Some(val) = self.value.get("sourceAttachment") {
            return Some(Attachment {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The source on which this consent statement is based. The source might be a
    /// scanned original paper form, or a reference to a consent that links back to such
    /// a source, a reference to a document repository (e.g. XDS) that stores the
    /// original consent document.
    pub fn source_reference(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("sourceReference") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Indicates the current state of this consent.
    pub fn status(&self) -> Option<ConsentStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ConsentStatus::from_string(&val).unwrap());
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

    /// Whether a treatment instruction (e.g. artificial respiration yes or no) was
    /// verified with the patient, his/her family or another authorized person.
    pub fn verification(&self) -> Option<Vec<Consent_Verification>> {
        if let Some(Value::Array(val)) = self.value.get("verification") {
            return Some(
                val.into_iter()
                    .map(|e| Consent_Verification {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._date_time() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._implicit_rules() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._language() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if !self
            .category()
            .into_iter()
            .map(|e| e.validate())
            .all(|x| x == true)
        {
            return false;
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.date_time() {}
        if let Some(_val) = self.extension() {
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
        if let Some(_val) = self.language() {}
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
        if let Some(_val) = self.organization() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.patient() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.performer() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.policy() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.policy_rule() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.provision() {
            if !_val.validate() {
                return false;
            }
        }
        if !self.scope().validate() {
            return false;
        }
        if let Some(_val) = self.source_attachment() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.source_reference() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.verification() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct ConsentBuilder {
    pub(crate) value: Value,
}

impl ConsentBuilder {
    pub fn build(&self) -> Consent {
        Consent {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Consent) -> ConsentBuilder {
        ConsentBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(category: Vec<CodeableConcept>, scope: CodeableConcept) -> ConsentBuilder {
        let mut __value: Value = json!({});
        __value["category"] = json!(category.into_iter().map(|e| e.value).collect::<Vec<_>>());
        __value["scope"] = json!(scope.value);
        return ConsentBuilder { value: __value };
    }

    pub fn _date_time<'a>(&'a mut self, val: Element) -> &'a mut ConsentBuilder {
        self.value["_dateTime"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ConsentBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ConsentBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ConsentBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut ConsentBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn date_time<'a>(&'a mut self, val: &str) -> &'a mut ConsentBuilder {
        self.value["dateTime"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConsentBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ConsentBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Vec<Identifier>) -> &'a mut ConsentBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ConsentBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ConsentBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ConsentBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ConsentBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn organization<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ConsentBuilder {
        self.value["organization"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn patient<'a>(&'a mut self, val: Reference) -> &'a mut ConsentBuilder {
        self.value["patient"] = json!(val.value);
        return self;
    }

    pub fn performer<'a>(&'a mut self, val: Vec<Reference>) -> &'a mut ConsentBuilder {
        self.value["performer"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn policy<'a>(&'a mut self, val: Vec<Consent_Policy>) -> &'a mut ConsentBuilder {
        self.value["policy"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn policy_rule<'a>(&'a mut self, val: CodeableConcept) -> &'a mut ConsentBuilder {
        self.value["policyRule"] = json!(val.value);
        return self;
    }

    pub fn provision<'a>(&'a mut self, val: Consent_Provision) -> &'a mut ConsentBuilder {
        self.value["provision"] = json!(val.value);
        return self;
    }

    pub fn source_attachment<'a>(&'a mut self, val: Attachment) -> &'a mut ConsentBuilder {
        self.value["sourceAttachment"] = json!(val.value);
        return self;
    }

    pub fn source_reference<'a>(&'a mut self, val: Reference) -> &'a mut ConsentBuilder {
        self.value["sourceReference"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: ConsentStatus) -> &'a mut ConsentBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ConsentBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn verification<'a>(
        &'a mut self,
        val: Vec<Consent_Verification>,
    ) -> &'a mut ConsentBuilder {
        self.value["verification"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }
}

#[derive(Debug)]
pub enum ConsentStatus {
    Draft,
    Proposed,
    Active,
    Rejected,
    Inactive,
    EnteredInError,
}

impl ConsentStatus {
    pub fn from_string(string: &str) -> Option<ConsentStatus> {
        match string {
            "draft" => Some(ConsentStatus::Draft),
            "proposed" => Some(ConsentStatus::Proposed),
            "active" => Some(ConsentStatus::Active),
            "rejected" => Some(ConsentStatus::Rejected),
            "inactive" => Some(ConsentStatus::Inactive),
            "entered-in-error" => Some(ConsentStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ConsentStatus::Draft => "draft".to_string(),
            ConsentStatus::Proposed => "proposed".to_string(),
            ConsentStatus::Active => "active".to_string(),
            ConsentStatus::Rejected => "rejected".to_string(),
            ConsentStatus::Inactive => "inactive".to_string(),
            ConsentStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}
