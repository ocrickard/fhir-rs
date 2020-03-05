#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::CoverageEligibilityResponse_Error::CoverageEligibilityResponse_Error;
use crate::model::CoverageEligibilityResponse_Insurance::CoverageEligibilityResponse_Insurance;
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

/// This resource provides eligibility and plan details from the processing of an
/// CoverageEligibilityRequest resource.

#[derive(Debug)]
pub struct CoverageEligibilityResponse<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl CoverageEligibilityResponse<'_> {
    pub fn new(value: &Value) -> CoverageEligibilityResponse {
        CoverageEligibilityResponse {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for created
    pub fn _created(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_created") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for disposition
    pub fn _disposition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_disposition") {
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

    /// Extensions for outcome
    pub fn _outcome(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_outcome") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for preAuthRef
    pub fn _pre_auth_ref(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_preAuthRef") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for purpose
    pub fn _purpose(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_purpose") {
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

    /// Extensions for servicedDate
    pub fn _serviced_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_servicedDate") {
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

    /// The date this resource was created.
    pub fn created(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("created") {
            return Some(string);
        }
        return None;
    }

    /// A human readable description of the status of the adjudication.
    pub fn disposition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("disposition") {
            return Some(string);
        }
        return None;
    }

    /// Errors encountered during the processing of the request.
    pub fn error(&self) -> Option<Vec<CoverageEligibilityResponse_Error>> {
        if let Some(Value::Array(val)) = self.value.get("error") {
            return Some(
                val.into_iter()
                    .map(|e| CoverageEligibilityResponse_Error {
                        value: Cow::Borrowed(e),
                    })
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
                    .map(|e| Extension {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A code for the form to be used for printing the content.
    pub fn form(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("form") {
            return Some(CodeableConcept {
                value: Cow::Borrowed(val),
            });
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

    /// A unique identifier assigned to this coverage eligiblity request.
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

    /// Financial instruments for reimbursement for the health care products and
    /// services.
    pub fn insurance(&self) -> Option<Vec<CoverageEligibilityResponse_Insurance>> {
        if let Some(Value::Array(val)) = self.value.get("insurance") {
            return Some(
                val.into_iter()
                    .map(|e| CoverageEligibilityResponse_Insurance {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The Insurer who issued the coverage in question and is the author of the
    /// response.
    pub fn insurer(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["insurer"]),
        }
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

    /// The outcome of the request processing.
    pub fn outcome(&self) -> Option<CoverageEligibilityResponseOutcome> {
        if let Some(Value::String(val)) = self.value.get("outcome") {
            return Some(CoverageEligibilityResponseOutcome::from_string(&val).unwrap());
        }
        return None;
    }

    /// The party who is the beneficiary of the supplied coverage and for whom
    /// eligibility is sought.
    pub fn patient(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["patient"]),
        }
    }

    /// A reference from the Insurer to which these services pertain to be used on
    /// further communication and as proof that the request occurred.
    pub fn pre_auth_ref(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("preAuthRef") {
            return Some(string);
        }
        return None;
    }

    /// Reference to the original request resource.
    pub fn request(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["request"]),
        }
    }

    /// The provider which is responsible for the request.
    pub fn requestor(&self) -> Option<Reference> {
        if let Some(val) = self.value.get("requestor") {
            return Some(Reference {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The date or dates when the enclosed suite of services were performed or
    /// completed.
    pub fn serviced_date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("servicedDate") {
            return Some(string);
        }
        return None;
    }

    /// The date or dates when the enclosed suite of services were performed or
    /// completed.
    pub fn serviced_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("servicedPeriod") {
            return Some(Period {
                value: Cow::Borrowed(val),
            });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._created() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._disposition() {
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
        if let Some(_val) = self._outcome() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._pre_auth_ref() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._purpose() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self._serviced_date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.created() {}
        if let Some(_val) = self.disposition() {}
        if let Some(_val) = self.error() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.form() {
            if !_val.validate() {
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
        if let Some(_val) = self.insurance() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.insurer().validate() {
            return false;
        }
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
        if let Some(_val) = self.outcome() {}
        if !self.patient().validate() {
            return false;
        }
        if let Some(_val) = self.pre_auth_ref() {}
        if !self.request().validate() {
            return false;
        }
        if let Some(_val) = self.requestor() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.serviced_date() {}
        if let Some(_val) = self.serviced_period() {
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
        return true;
    }
}

#[derive(Debug)]
pub struct CoverageEligibilityResponseBuilder {
    pub(crate) value: Value,
}

impl CoverageEligibilityResponseBuilder {
    pub fn build(&self) -> CoverageEligibilityResponse {
        CoverageEligibilityResponse {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: CoverageEligibilityResponse) -> CoverageEligibilityResponseBuilder {
        CoverageEligibilityResponseBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(
        insurer: Reference,
        patient: Reference,
        request: Reference,
    ) -> CoverageEligibilityResponseBuilder {
        let mut __value: Value = json!({});
        __value["insurer"] = json!(insurer.value);
        __value["patient"] = json!(patient.value);
        __value["request"] = json!(request.value);
        return CoverageEligibilityResponseBuilder { value: __value };
    }

    pub fn _created<'a>(&'a mut self, val: Element) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_created"] = json!(val.value);
        return self;
    }

    pub fn _disposition<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_disposition"] = json!(val.value);
        return self;
    }

    pub fn _implicit_rules<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _outcome<'a>(&'a mut self, val: Element) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_outcome"] = json!(val.value);
        return self;
    }

    pub fn _pre_auth_ref<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_preAuthRef"] = json!(val.value);
        return self;
    }

    pub fn _purpose<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_purpose"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _serviced_date<'a>(
        &'a mut self,
        val: Element,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_servicedDate"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn created<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["created"] = json!(val);
        return self;
    }

    pub fn disposition<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["disposition"] = json!(val);
        return self;
    }

    pub fn error<'a>(
        &'a mut self,
        val: Vec<CoverageEligibilityResponse_Error>,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["error"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn form<'a>(
        &'a mut self,
        val: CodeableConcept,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["form"] = json!(val.value);
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(
        &'a mut self,
        val: Vec<Identifier>,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["identifier"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn implicit_rules<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn insurance<'a>(
        &'a mut self,
        val: Vec<CoverageEligibilityResponse_Insurance>,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["insurance"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn outcome<'a>(
        &'a mut self,
        val: CoverageEligibilityResponseOutcome,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["outcome"] = json!(val.to_string());
        return self;
    }

    pub fn pre_auth_ref<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["preAuthRef"] = json!(val);
        return self;
    }

    pub fn requestor<'a>(
        &'a mut self,
        val: Reference,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["requestor"] = json!(val.value);
        return self;
    }

    pub fn serviced_date<'a>(
        &'a mut self,
        val: &str,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["servicedDate"] = json!(val);
        return self;
    }

    pub fn serviced_period<'a>(
        &'a mut self,
        val: Period,
    ) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["servicedPeriod"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: &str) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["status"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut CoverageEligibilityResponseBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum CoverageEligibilityResponseOutcome {
    Queued,
    Complete,
    Error,
    Partial,
}

impl CoverageEligibilityResponseOutcome {
    pub fn from_string(string: &str) -> Option<CoverageEligibilityResponseOutcome> {
        match string {
            "queued" => Some(CoverageEligibilityResponseOutcome::Queued),
            "complete" => Some(CoverageEligibilityResponseOutcome::Complete),
            "error" => Some(CoverageEligibilityResponseOutcome::Error),
            "partial" => Some(CoverageEligibilityResponseOutcome::Partial),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            CoverageEligibilityResponseOutcome::Queued => "queued".to_string(),
            CoverageEligibilityResponseOutcome::Complete => "complete".to_string(),
            CoverageEligibilityResponseOutcome::Error => "error".to_string(),
            CoverageEligibilityResponseOutcome::Partial => "partial".to_string(),
        }
    }
}
