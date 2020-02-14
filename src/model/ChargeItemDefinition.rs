#![allow(unused_imports, non_camel_case_types)]

use crate::model::ChargeItemDefinition_Applicability::ChargeItemDefinition_Applicability;
use crate::model::ChargeItemDefinition_PropertyGroup::ChargeItemDefinition_PropertyGroup;
use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Period::Period;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::value::Value;

/// The ChargeItemDefinition resource provides the properties that apply to the
/// (billing) codes necessary to calculate costs and prices. The properties may
/// differ largely depending on type and realm, therefore this resource gives only a
/// rough structure and requires profiling for each type of billing code system.

#[derive(Debug)]
pub struct ChargeItemDefinition<'a> {
    pub value: &'a Value,
}

impl ChargeItemDefinition<'_> {
    /// The date  (and optionally time) when the charge item definition was published.
    /// The date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the charge item definition changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
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

    /// Extensions for lastReviewDate
    pub fn _last_review_date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_lastReviewDate") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// An absolute URI that is used to identify this charge item definition when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this charge item
    /// definition is (or will be) published. This URL can be the target of a canonical
    /// reference. It SHALL remain the same when the charge item definition is stored on
    /// different servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// Expressions that describe applicability criteria for the billing code.
    pub fn applicability(&self) -> Option<Vec<ChargeItemDefinition_Applicability>> {
        if let Some(Value::Array(val)) = self.value.get("applicability") {
            return Some(
                val.into_iter()
                    .map(|e| ChargeItemDefinition_Applicability { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for derivedFromUri
    pub fn _derived_from_uri(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_derivedFromUri") {
            return Some(
                val.into_iter()
                    .map(|e| Element { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A free text natural language description of the charge item definition from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
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

    /// A larger definition of which this particular definition is a component or step.
    pub fn part_of(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("partOf") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
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

    /// The period during which the charge item definition content was or is planned to
    /// be in active use.
    pub fn effective_period(&self) -> Option<Period> {
        if let Some(val) = self.value.get("effectivePeriod") {
            return Some(Period { value: val });
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

    /// Extensions for title
    pub fn _title(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_title") {
            return Some(Element { value: val });
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

    /// A legal or geographic region in which the charge item definition is intended to
    /// be used.
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
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

    /// A formal identifier that is used to identify this charge item definition when it
    /// is represented in other formats, or referenced in a specification, model, design
    /// or an instance.
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

    /// Extensions for version
    pub fn _version(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_version") {
            return Some(Element { value: val });
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

    /// The name of the organization or individual that published the charge item
    /// definition.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
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

    /// A short, descriptive, user-friendly title for the charge item definition.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
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

    /// The current state of the ChargeItemDefinition.
    pub fn status(&self) -> Option<ChargeItemDefinitionStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ChargeItemDefinitionStatus::from_string(&val).unwrap());
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

    /// A Boolean value to indicate that this charge item definition is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended to be
    /// used for genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate charge item
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

    /// Group of properties which are applicable under the same conditions. If no
    /// applicability rules are established for the group, then all properties always
    /// apply.
    pub fn property_group(&self) -> Option<Vec<ChargeItemDefinition_PropertyGroup>> {
        if let Some(Value::Array(val)) = self.value.get("propertyGroup") {
            return Some(
                val.into_iter()
                    .map(|e| ChargeItemDefinition_PropertyGroup { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The identifier that is used to identify this version of the charge item
    /// definition when it is referenced in a specification, model, design or instance.
    /// This is an arbitrary value managed by the charge item definition author and is
    /// not expected to be globally unique. For example, it might be a timestamp (e.g.
    /// yyyymmdd) if a managed version is not available. There is also no expectation
    /// that versions can be placed in a lexicographical sequence. To provide a version
    /// consistent with the Decision Support Service specification, use the format
    /// Major.Minor.Revision (e.g. 1.0.0). For more information on versioning knowledge
    /// assets, refer to the Decision Support Service specification. Note that a version
    /// is required for non-experimental active assets.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    /// A copyright statement relating to the charge item definition and/or its
    /// contents. Copyright statements are generally legal restrictions on the use and
    /// publishing of the charge item definition.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// As new versions of a protocol or guideline are defined, allows identification of
    /// what versions are replaced by a new instance.
    pub fn replaces(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("replaces") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The URL pointing to an externally-defined charge item definition that is adhered
    /// to in whole or in part by this definition.
    pub fn derived_from_uri(&self) -> Option<Vec<&str>> {
        if let Some(Value::Array(val)) = self.value.get("derivedFromUri") {
            return Some(
                val.into_iter()
                    .map(|e| e.as_str().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The defined billing details in this resource pertain to the given product
    /// instance(s).
    pub fn instance(&self) -> Option<Vec<Reference>> {
        if let Some(Value::Array(val)) = self.value.get("instance") {
            return Some(
                val.into_iter()
                    .map(|e| Reference { value: e })
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

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
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

    /// The defined billing details in this resource pertain to the given billing code.
    pub fn code(&self) -> Option<CodeableConcept> {
        if let Some(val) = self.value.get("code") {
            return Some(CodeableConcept { value: val });
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

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.date() {}
        if let Some(_val) = self._copyright() {
            _val.validate();
        }
        if let Some(_val) = self._last_review_date() {
            _val.validate();
        }
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.applicability() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._derived_from_uri() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self._url() {
            _val.validate();
        }
        if let Some(_val) = self.part_of() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.meta() {
            _val.validate();
        }
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.text() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.contact() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.effective_period() {
            _val.validate();
        }
        if let Some(_val) = self._experimental() {
            _val.validate();
        }
        if let Some(_val) = self._title() {
            _val.validate();
        }
        if let Some(_val) = self._publisher() {
            _val.validate();
        }
        if let Some(_val) = self.jurisdiction() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._language() {
            _val.validate();
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.identifier() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._version() {
            _val.validate();
        }
        if let Some(_val) = self._implicit_rules() {
            _val.validate();
        }
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self._description() {
            _val.validate();
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.last_review_date() {}
        if let Some(_val) = self.contained() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.use_context() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.property_group() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.version() {}
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.replaces() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.derived_from_uri() {
            _val.into_iter().for_each(|_e| {});
        }
        if let Some(_val) = self.instance() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._status() {
            _val.validate();
        }
        if let Some(_val) = self._date() {
            _val.validate();
        }
        if let Some(_val) = self._approval_date() {
            _val.validate();
        }
        if let Some(_val) = self.code() {
            _val.validate();
        }
        if let Some(_val) = self.approval_date() {}
        return true;
    }
}

#[derive(Debug)]
pub enum ChargeItemDefinitionStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl ChargeItemDefinitionStatus {
    pub fn from_string(string: &str) -> Option<ChargeItemDefinitionStatus> {
        match string {
            "draft" => Some(ChargeItemDefinitionStatus::Draft),
            "active" => Some(ChargeItemDefinitionStatus::Active),
            "retired" => Some(ChargeItemDefinitionStatus::Retired),
            "unknown" => Some(ChargeItemDefinitionStatus::Unknown),
            _ => None,
        }
    }
}
