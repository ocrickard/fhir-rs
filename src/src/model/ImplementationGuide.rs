#![allow(unused_imports, non_camel_case_types)]

use crate::model::CodeableConcept::CodeableConcept;
use crate::model::ContactDetail::ContactDetail;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::ImplementationGuide_Definition::ImplementationGuide_Definition;
use crate::model::ImplementationGuide_DependsOn::ImplementationGuide_DependsOn;
use crate::model::ImplementationGuide_Global::ImplementationGuide_Global;
use crate::model::ImplementationGuide_Manifest::ImplementationGuide_Manifest;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::ResourceList::ResourceList;
use crate::model::UsageContext::UsageContext;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A set of rules of how a particular interoperability or standards problem is
/// solved - typically through the use of FHIR resources. This resource is used to
/// gather all the parts of an implementation guide into a logical whole and to
/// publish a computable definition of all the parts.

#[derive(Debug)]
pub struct ImplementationGuide<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl ImplementationGuide<'_> {
    pub fn new(value: &Value) -> ImplementationGuide {
        ImplementationGuide {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for copyright
    pub fn _copyright(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_copyright") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for date
    pub fn _date(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_date") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for description
    pub fn _description(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_description") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for experimental
    pub fn _experimental(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_experimental") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for fhirVersion
    pub fn _fhir_version(&self) -> Option<Vec<Element>> {
        if let Some(Value::Array(val)) = self.value.get("_fhirVersion") {
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

    /// Extensions for language
    pub fn _language(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_language") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for license
    pub fn _license(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_license") {
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

    /// Extensions for packageId
    pub fn _package_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_packageId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for publisher
    pub fn _publisher(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_publisher") {
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

    /// Contact details to assist a user in finding and communicating with the
    /// publisher.
    pub fn contact(&self) -> Option<Vec<ContactDetail>> {
        if let Some(Value::Array(val)) = self.value.get("contact") {
            return Some(
                val.into_iter()
                    .map(|e| ContactDetail {
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

    /// A copyright statement relating to the implementation guide and/or its contents.
    /// Copyright statements are generally legal restrictions on the use and publishing
    /// of the implementation guide.
    pub fn copyright(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("copyright") {
            return Some(string);
        }
        return None;
    }

    /// The date  (and optionally time) when the implementation guide was published. The
    /// date must change when the business version changes and it must change if the
    /// status code changes. In addition, it should change when the substantive content
    /// of the implementation guide changes.
    pub fn date(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("date") {
            return Some(string);
        }
        return None;
    }

    /// The information needed by an IG publisher tool to publish the whole
    /// implementation guide.
    pub fn definition(&self) -> Option<ImplementationGuide_Definition> {
        if let Some(val) = self.value.get("definition") {
            return Some(ImplementationGuide_Definition {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Another implementation guide that this implementation depends on. Typically, an
    /// implementation guide uses value sets, profiles etc.defined in other
    /// implementation guides.
    pub fn depends_on(&self) -> Option<Vec<ImplementationGuide_DependsOn>> {
        if let Some(Value::Array(val)) = self.value.get("dependsOn") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_DependsOn {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A free text natural language description of the implementation guide from a
    /// consumer's perspective.
    pub fn description(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("description") {
            return Some(string);
        }
        return None;
    }

    /// A Boolean value to indicate that this implementation guide is authored for
    /// testing purposes (or education/evaluation/marketing) and is not intended to be
    /// used for genuine usage.
    pub fn experimental(&self) -> Option<bool> {
        if let Some(val) = self.value.get("experimental") {
            return Some(val.as_bool().unwrap());
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

    /// A set of profiles that all resources covered by this implementation guide must
    /// conform to.
    pub fn global(&self) -> Option<Vec<ImplementationGuide_Global>> {
        if let Some(Value::Array(val)) = self.value.get("global") {
            return Some(
                val.into_iter()
                    .map(|e| ImplementationGuide_Global {
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

    /// A legal or geographic region in which the implementation guide is intended to be
    /// used.
    pub fn jurisdiction(&self) -> Option<Vec<CodeableConcept>> {
        if let Some(Value::Array(val)) = self.value.get("jurisdiction") {
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

    /// The base language in which the resource is written.
    pub fn language(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("language") {
            return Some(string);
        }
        return None;
    }

    /// The license that applies to this Implementation Guide, using an SPDX license
    /// code, or 'not-open-source'.
    pub fn license(&self) -> Option<ImplementationGuideLicense> {
        if let Some(Value::String(val)) = self.value.get("license") {
            return Some(ImplementationGuideLicense::from_string(&val).unwrap());
        }
        return None;
    }

    /// Information about an assembled implementation guide, created by the publication
    /// tooling.
    pub fn manifest(&self) -> Option<ImplementationGuide_Manifest> {
        if let Some(val) = self.value.get("manifest") {
            return Some(ImplementationGuide_Manifest {
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

    /// A natural language name identifying the implementation guide. This name should
    /// be usable as an identifier for the module by machine processing applications
    /// such as code generation.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// The NPM package name for this Implementation Guide, used in the NPM package
    /// distribution, which is the primary mechanism by which FHIR based tooling manages
    /// IG dependencies. This value must be globally unique, and should be assigned with
    /// care.
    pub fn package_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("packageId") {
            return Some(string);
        }
        return None;
    }

    /// The name of the organization or individual that published the implementation
    /// guide.
    pub fn publisher(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("publisher") {
            return Some(string);
        }
        return None;
    }

    /// The status of this implementation guide. Enables tracking the life-cycle of the
    /// content.
    pub fn status(&self) -> Option<ImplementationGuideStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(ImplementationGuideStatus::from_string(&val).unwrap());
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

    /// A short, descriptive, user-friendly title for the implementation guide.
    pub fn title(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("title") {
            return Some(string);
        }
        return None;
    }

    /// An absolute URI that is used to identify this implementation guide when it is
    /// referenced in a specification, model, design or an instance; also called its
    /// canonical identifier. This SHOULD be globally unique and SHOULD be a literal
    /// address at which at which an authoritative instance of this implementation guide
    /// is (or will be) published. This URL can be the target of a canonical reference.
    /// It SHALL remain the same when the implementation guide is stored on different
    /// servers.
    pub fn url(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("url") {
            return Some(string);
        }
        return None;
    }

    /// The content was developed with a focus and intent of supporting the contexts
    /// that are listed. These contexts may be general categories (gender, age, ...) or
    /// may be references to specific programs (insurance plans, studies, ...) and may
    /// be used to assist with indexing and searching for appropriate implementation
    /// guide instances.
    pub fn use_context(&self) -> Option<Vec<UsageContext>> {
        if let Some(Value::Array(val)) = self.value.get("useContext") {
            return Some(
                val.into_iter()
                    .map(|e| UsageContext {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The identifier that is used to identify this version of the implementation guide
    /// when it is referenced in a specification, model, design or instance. This is an
    /// arbitrary value managed by the implementation guide author and is not expected
    /// to be globally unique. For example, it might be a timestamp (e.g. yyyymmdd) if a
    /// managed version is not available. There is also no expectation that versions can
    /// be placed in a lexicographical sequence.
    pub fn version(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("version") {
            return Some(string);
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._copyright() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._date() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._description() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._experimental() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._fhir_version() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
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
        if let Some(_val) = self._license() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._name() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._package_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._publisher() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
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
        if let Some(_val) = self.contact() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.copyright() {}
        if let Some(_val) = self.date() {}
        if let Some(_val) = self.definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.depends_on() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.description() {}
        if let Some(_val) = self.experimental() {}
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.global() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.jurisdiction() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.language() {}
        if let Some(_val) = self.license() {}
        if let Some(_val) = self.manifest() {
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
        if let Some(_val) = self.package_id() {}
        if let Some(_val) = self.publisher() {}
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.title() {}
        if let Some(_val) = self.url() {}
        if let Some(_val) = self.use_context() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.version() {}
        return true;
    }
}

#[derive(Debug)]
pub struct ImplementationGuideBuilder {
    pub(crate) value: Value,
}

impl ImplementationGuideBuilder {
    pub fn build(&self) -> ImplementationGuide {
        ImplementationGuide {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: ImplementationGuide) -> ImplementationGuideBuilder {
        ImplementationGuideBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> ImplementationGuideBuilder {
        let mut __value: Value = json!({});
        return ImplementationGuideBuilder { value: __value };
    }

    pub fn _copyright<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_copyright"] = json!(val.value);
        return self;
    }

    pub fn _date<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_date"] = json!(val.value);
        return self;
    }

    pub fn _description<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_description"] = json!(val.value);
        return self;
    }

    pub fn _experimental<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_experimental"] = json!(val.value);
        return self;
    }

    pub fn _fhir_version<'a>(
        &'a mut self,
        val: Vec<Element>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["_fhirVersion"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _license<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_license"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _package_id<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_packageId"] = json!(val.value);
        return self;
    }

    pub fn _publisher<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_publisher"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _title<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_title"] = json!(val.value);
        return self;
    }

    pub fn _url<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_url"] = json!(val.value);
        return self;
    }

    pub fn _version<'a>(&'a mut self, val: Element) -> &'a mut ImplementationGuideBuilder {
        self.value["_version"] = json!(val.value);
        return self;
    }

    pub fn contact<'a>(
        &'a mut self,
        val: Vec<ContactDetail>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["contact"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn contained<'a>(
        &'a mut self,
        val: Vec<ResourceList>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn copyright<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["copyright"] = json!(val);
        return self;
    }

    pub fn date<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["date"] = json!(val);
        return self;
    }

    pub fn definition<'a>(
        &'a mut self,
        val: ImplementationGuide_Definition,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["definition"] = json!(val.value);
        return self;
    }

    pub fn depends_on<'a>(
        &'a mut self,
        val: Vec<ImplementationGuide_DependsOn>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["dependsOn"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn description<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["description"] = json!(val);
        return self;
    }

    pub fn experimental<'a>(&'a mut self, val: bool) -> &'a mut ImplementationGuideBuilder {
        self.value["experimental"] = json!(val);
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut ImplementationGuideBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn global<'a>(
        &'a mut self,
        val: Vec<ImplementationGuide_Global>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["global"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn jurisdiction<'a>(
        &'a mut self,
        val: Vec<CodeableConcept>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["jurisdiction"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn license<'a>(
        &'a mut self,
        val: ImplementationGuideLicense,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["license"] = json!(val.to_string());
        return self;
    }

    pub fn manifest<'a>(
        &'a mut self,
        val: ImplementationGuide_Manifest,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["manifest"] = json!(val.value);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut ImplementationGuideBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn package_id<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["packageId"] = json!(val);
        return self;
    }

    pub fn publisher<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["publisher"] = json!(val);
        return self;
    }

    pub fn status<'a>(
        &'a mut self,
        val: ImplementationGuideStatus,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut ImplementationGuideBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }

    pub fn title<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["title"] = json!(val);
        return self;
    }

    pub fn url<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["url"] = json!(val);
        return self;
    }

    pub fn use_context<'a>(
        &'a mut self,
        val: Vec<UsageContext>,
    ) -> &'a mut ImplementationGuideBuilder {
        self.value["useContext"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn version<'a>(&'a mut self, val: &str) -> &'a mut ImplementationGuideBuilder {
        self.value["version"] = json!(val);
        return self;
    }
}

#[derive(Debug)]
pub enum ImplementationGuideLicense {
    NotOpenSource,
    Fhir0bsd,
    AAL,
    Abstyles,
    Adobe2006,
    AdobeGlyph,
    ADSL,
    Afl11,
    Afl12,
    Afl20,
    Afl21,
    Afl30,
    Afmparse,
    Agpl10Only,
    Agpl10OrLater,
    Agpl30Only,
    Agpl30OrLater,
    Aladdin,
    AMDPLPA,
    AML,
    AMPAS,
    AntlrPd,
    Apache10,
    Apache11,
    Apache20,
    APAFML,
    Apl10,
    Apsl10,
    Apsl11,
    Apsl12,
    Apsl20,
    Artistic10Cl8,
    Artistic10Perl,
    Artistic10,
    Artistic20,
    Bahyph,
    Barr,
    Beerware,
    Bittorrent10,
    Bittorrent11,
    Borceux,
    Bsd1Clause,
    Bsd2ClauseFreebsd,
    Bsd2ClauseNetbsd,
    Bsd2ClausePatent,
    Bsd2Clause,
    Bsd3ClauseAttribution,
    Bsd3ClauseClear,
    Bsd3ClauseLbnl,
    Bsd3ClauseNoNuclearLicense2014,
    Bsd3ClauseNoNuclearLicense,
    Bsd3ClauseNoNuclearWarranty,
    Bsd3Clause,
    Bsd4ClauseUc,
    Bsd4Clause,
    BsdProtection,
    BsdSourceCode,
    Bsl10,
    Bzip2105,
    Bzip2106,
    Caldera,
    Catosl11,
    CcBy10,
    CcBy20,
    CcBy25,
    CcBy30,
    CcBy40,
    CcByNc10,
    CcByNc20,
    CcByNc25,
    CcByNc30,
    CcByNc40,
    CcByNcNd10,
    CcByNcNd20,
    CcByNcNd25,
    CcByNcNd30,
    CcByNcNd40,
    CcByNcSa10,
    CcByNcSa20,
    CcByNcSa25,
    CcByNcSa30,
    CcByNcSa40,
    CcByNd10,
    CcByNd20,
    CcByNd25,
    CcByNd30,
    CcByNd40,
    CcBySa10,
    CcBySa20,
    CcBySa25,
    CcBySa30,
    CcBySa40,
    Cc010,
    Cddl10,
    Cddl11,
    CdlaPermissive10,
    CdlaSharing10,
    Cecill10,
    Cecill11,
    Cecill20,
    Cecill21,
    CecillB,
    CecillC,
    ClArtistic,
    CnriJython,
    CnriPythonGplCompatible,
    CnriPython,
    Condor11,
    Cpal10,
    Cpl10,
    Cpol102,
    Crossword,
    CrystalStacker,
    CuaOpl10,
    Cube,
    Curl,
    DFsl10,
    Diffmark,
    DOC,
    Dotseqn,
    DSDP,
    Dvipdfm,
    Ecl10,
    Ecl20,
    Efl10,
    Efl20,
    EGenix,
    Entessa,
    Epl10,
    Epl20,
    Erlpl11,
    EUDatagrid,
    Eupl10,
    Eupl11,
    Eupl12,
    Eurosym,
    Fair,
    Frameworx10,
    FreeImage,
    FSFAP,
    FSFUL,
    FSFULLR,
    FTL,
    Gfdl11Only,
    Gfdl11OrLater,
    Gfdl12Only,
    Gfdl12OrLater,
    Gfdl13Only,
    Gfdl13OrLater,
    Giftware,
    GL2PS,
    Glide,
    Glulxe,
    Gnuplot,
    Gpl10Only,
    Gpl10OrLater,
    Gpl20Only,
    Gpl20OrLater,
    Gpl30Only,
    Gpl30OrLater,
    Gsoap13b,
    HaskellReport,
    HPND,
    IbmPibs,
    ICU,
    IJG,
    ImageMagick,
    IMatix,
    Imlib2,
    InfoZip,
    IntelAcpi,
    Intel,
    Interbase10,
    IPA,
    Ipl10,
    ISC,
    Jasper20,
    JSON,
    Lal12,
    Lal13,
    Latex2e,
    Leptonica,
    Lgpl20Only,
    Lgpl20OrLater,
    Lgpl21Only,
    Lgpl21OrLater,
    Lgpl30Only,
    Lgpl30OrLater,
    LGPLLR,
    Libpng,
    Libtiff,
    LiliqP11,
    LiliqR11,
    LiliqRplus11,
    LinuxOpenib,
    Lpl10,
    Lpl102,
    Lppl10,
    Lppl11,
    Lppl12,
    Lppl13a,
    Lppl13c,
    MakeIndex,
    MirOS,
    Mit0,
    MitAdvertising,
    MitCmu,
    MitEnna,
    MitFeh,
    MIT,
    MITNFA,
    Motosoto,
    Mpich2,
    Mpl10,
    Mpl11,
    Mpl20NoCopyleftException,
    Mpl20,
    MsPl,
    MsRl,
    MTLL,
    Multics,
    Mup,
    Nasa13,
    Naumen,
    Nbpl10,
    NCSA,
    NetSnmp,
    NetCDF,
    Newsletr,
    NGPL,
    Nlod10,
    NLPL,
    Nokia,
    NOSL,
    Noweb,
    Npl10,
    Npl11,
    Nposl30,
    NRL,
    NTP,
    OcctPl,
    Oclc20,
    Odbl10,
    Ofl10,
    Ofl11,
    OGTSL,
    Oldap11,
    Oldap12,
    Oldap13,
    Oldap14,
    Oldap201,
    Oldap20,
    Oldap21,
    Oldap221,
    Oldap222,
    Oldap22,
    Oldap23,
    Oldap24,
    Oldap25,
    Oldap26,
    Oldap27,
    Oldap28,
    OML,
    OpenSSL,
    Opl10,
    OsetPl21,
    Osl10,
    Osl11,
    Osl20,
    Osl21,
    Osl30,
    Pddl10,
    Php30,
    Php301,
    Plexus,
    PostgreSQL,
    Psfrag,
    Psutils,
    Python20,
    Qhull,
    Qpl10,
    Rdisc,
    Rhecos11,
    Rpl11,
    Rpl15,
    Rpsl10,
    RsaMd,
    RSCPL,
    Ruby,
    SaxPd,
    Saxpath,
    SCEA,
    Sendmail,
    SgiB10,
    SgiB11,
    SgiB20,
    Simpl20,
    Sissl12,
    SISSL,
    Sleepycat,
    SMLNJ,
    SMPPL,
    SNIA,
    Spencer86,
    Spencer94,
    Spencer99,
    Spl10,
    Sugarcrm113,
    SWL,
    TCL,
    TcpWrappers,
    TMate,
    Torque11,
    TOSL,
    UnicodeDfs2015,
    UnicodeDfs2016,
    UnicodeTou,
    Unlicense,
    Upl10,
    Vim,
    VOSTROM,
    Vsl10,
    W3c19980720,
    W3c20150513,
    W3C,
    Watcom10,
    Wsuipa,
    WTFPL,
    X11,
    Xerox,
    Xfree8611,
    Xinetd,
    Xnet,
    Xpp,
    XSkat,
    Ypl10,
    Ypl11,
    Zed,
    Zend20,
    Zimbra13,
    Zimbra14,
    ZlibAcknowledgement,
    Zlib,
    Zpl11,
    Zpl20,
    Zpl21,
}

impl ImplementationGuideLicense {
    pub fn from_string(string: &str) -> Option<ImplementationGuideLicense> {
        match string {
            "not-open-source" => Some(ImplementationGuideLicense::NotOpenSource),
            "0BSD" => Some(ImplementationGuideLicense::Fhir0bsd),
            "AAL" => Some(ImplementationGuideLicense::AAL),
            "Abstyles" => Some(ImplementationGuideLicense::Abstyles),
            "Adobe-2006" => Some(ImplementationGuideLicense::Adobe2006),
            "Adobe-Glyph" => Some(ImplementationGuideLicense::AdobeGlyph),
            "ADSL" => Some(ImplementationGuideLicense::ADSL),
            "AFL-1.1" => Some(ImplementationGuideLicense::Afl11),
            "AFL-1.2" => Some(ImplementationGuideLicense::Afl12),
            "AFL-2.0" => Some(ImplementationGuideLicense::Afl20),
            "AFL-2.1" => Some(ImplementationGuideLicense::Afl21),
            "AFL-3.0" => Some(ImplementationGuideLicense::Afl30),
            "Afmparse" => Some(ImplementationGuideLicense::Afmparse),
            "AGPL-1.0-only" => Some(ImplementationGuideLicense::Agpl10Only),
            "AGPL-1.0-or-later" => Some(ImplementationGuideLicense::Agpl10OrLater),
            "AGPL-3.0-only" => Some(ImplementationGuideLicense::Agpl30Only),
            "AGPL-3.0-or-later" => Some(ImplementationGuideLicense::Agpl30OrLater),
            "Aladdin" => Some(ImplementationGuideLicense::Aladdin),
            "AMDPLPA" => Some(ImplementationGuideLicense::AMDPLPA),
            "AML" => Some(ImplementationGuideLicense::AML),
            "AMPAS" => Some(ImplementationGuideLicense::AMPAS),
            "ANTLR-PD" => Some(ImplementationGuideLicense::AntlrPd),
            "Apache-1.0" => Some(ImplementationGuideLicense::Apache10),
            "Apache-1.1" => Some(ImplementationGuideLicense::Apache11),
            "Apache-2.0" => Some(ImplementationGuideLicense::Apache20),
            "APAFML" => Some(ImplementationGuideLicense::APAFML),
            "APL-1.0" => Some(ImplementationGuideLicense::Apl10),
            "APSL-1.0" => Some(ImplementationGuideLicense::Apsl10),
            "APSL-1.1" => Some(ImplementationGuideLicense::Apsl11),
            "APSL-1.2" => Some(ImplementationGuideLicense::Apsl12),
            "APSL-2.0" => Some(ImplementationGuideLicense::Apsl20),
            "Artistic-1.0-cl8" => Some(ImplementationGuideLicense::Artistic10Cl8),
            "Artistic-1.0-Perl" => Some(ImplementationGuideLicense::Artistic10Perl),
            "Artistic-1.0" => Some(ImplementationGuideLicense::Artistic10),
            "Artistic-2.0" => Some(ImplementationGuideLicense::Artistic20),
            "Bahyph" => Some(ImplementationGuideLicense::Bahyph),
            "Barr" => Some(ImplementationGuideLicense::Barr),
            "Beerware" => Some(ImplementationGuideLicense::Beerware),
            "BitTorrent-1.0" => Some(ImplementationGuideLicense::Bittorrent10),
            "BitTorrent-1.1" => Some(ImplementationGuideLicense::Bittorrent11),
            "Borceux" => Some(ImplementationGuideLicense::Borceux),
            "BSD-1-Clause" => Some(ImplementationGuideLicense::Bsd1Clause),
            "BSD-2-Clause-FreeBSD" => Some(ImplementationGuideLicense::Bsd2ClauseFreebsd),
            "BSD-2-Clause-NetBSD" => Some(ImplementationGuideLicense::Bsd2ClauseNetbsd),
            "BSD-2-Clause-Patent" => Some(ImplementationGuideLicense::Bsd2ClausePatent),
            "BSD-2-Clause" => Some(ImplementationGuideLicense::Bsd2Clause),
            "BSD-3-Clause-Attribution" => Some(ImplementationGuideLicense::Bsd3ClauseAttribution),
            "BSD-3-Clause-Clear" => Some(ImplementationGuideLicense::Bsd3ClauseClear),
            "BSD-3-Clause-LBNL" => Some(ImplementationGuideLicense::Bsd3ClauseLbnl),
            "BSD-3-Clause-No-Nuclear-License-2014" => {
                Some(ImplementationGuideLicense::Bsd3ClauseNoNuclearLicense2014)
            }
            "BSD-3-Clause-No-Nuclear-License" => {
                Some(ImplementationGuideLicense::Bsd3ClauseNoNuclearLicense)
            }
            "BSD-3-Clause-No-Nuclear-Warranty" => {
                Some(ImplementationGuideLicense::Bsd3ClauseNoNuclearWarranty)
            }
            "BSD-3-Clause" => Some(ImplementationGuideLicense::Bsd3Clause),
            "BSD-4-Clause-UC" => Some(ImplementationGuideLicense::Bsd4ClauseUc),
            "BSD-4-Clause" => Some(ImplementationGuideLicense::Bsd4Clause),
            "BSD-Protection" => Some(ImplementationGuideLicense::BsdProtection),
            "BSD-Source-Code" => Some(ImplementationGuideLicense::BsdSourceCode),
            "BSL-1.0" => Some(ImplementationGuideLicense::Bsl10),
            "bzip2-1.0.5" => Some(ImplementationGuideLicense::Bzip2105),
            "bzip2-1.0.6" => Some(ImplementationGuideLicense::Bzip2106),
            "Caldera" => Some(ImplementationGuideLicense::Caldera),
            "CATOSL-1.1" => Some(ImplementationGuideLicense::Catosl11),
            "CC-BY-1.0" => Some(ImplementationGuideLicense::CcBy10),
            "CC-BY-2.0" => Some(ImplementationGuideLicense::CcBy20),
            "CC-BY-2.5" => Some(ImplementationGuideLicense::CcBy25),
            "CC-BY-3.0" => Some(ImplementationGuideLicense::CcBy30),
            "CC-BY-4.0" => Some(ImplementationGuideLicense::CcBy40),
            "CC-BY-NC-1.0" => Some(ImplementationGuideLicense::CcByNc10),
            "CC-BY-NC-2.0" => Some(ImplementationGuideLicense::CcByNc20),
            "CC-BY-NC-2.5" => Some(ImplementationGuideLicense::CcByNc25),
            "CC-BY-NC-3.0" => Some(ImplementationGuideLicense::CcByNc30),
            "CC-BY-NC-4.0" => Some(ImplementationGuideLicense::CcByNc40),
            "CC-BY-NC-ND-1.0" => Some(ImplementationGuideLicense::CcByNcNd10),
            "CC-BY-NC-ND-2.0" => Some(ImplementationGuideLicense::CcByNcNd20),
            "CC-BY-NC-ND-2.5" => Some(ImplementationGuideLicense::CcByNcNd25),
            "CC-BY-NC-ND-3.0" => Some(ImplementationGuideLicense::CcByNcNd30),
            "CC-BY-NC-ND-4.0" => Some(ImplementationGuideLicense::CcByNcNd40),
            "CC-BY-NC-SA-1.0" => Some(ImplementationGuideLicense::CcByNcSa10),
            "CC-BY-NC-SA-2.0" => Some(ImplementationGuideLicense::CcByNcSa20),
            "CC-BY-NC-SA-2.5" => Some(ImplementationGuideLicense::CcByNcSa25),
            "CC-BY-NC-SA-3.0" => Some(ImplementationGuideLicense::CcByNcSa30),
            "CC-BY-NC-SA-4.0" => Some(ImplementationGuideLicense::CcByNcSa40),
            "CC-BY-ND-1.0" => Some(ImplementationGuideLicense::CcByNd10),
            "CC-BY-ND-2.0" => Some(ImplementationGuideLicense::CcByNd20),
            "CC-BY-ND-2.5" => Some(ImplementationGuideLicense::CcByNd25),
            "CC-BY-ND-3.0" => Some(ImplementationGuideLicense::CcByNd30),
            "CC-BY-ND-4.0" => Some(ImplementationGuideLicense::CcByNd40),
            "CC-BY-SA-1.0" => Some(ImplementationGuideLicense::CcBySa10),
            "CC-BY-SA-2.0" => Some(ImplementationGuideLicense::CcBySa20),
            "CC-BY-SA-2.5" => Some(ImplementationGuideLicense::CcBySa25),
            "CC-BY-SA-3.0" => Some(ImplementationGuideLicense::CcBySa30),
            "CC-BY-SA-4.0" => Some(ImplementationGuideLicense::CcBySa40),
            "CC0-1.0" => Some(ImplementationGuideLicense::Cc010),
            "CDDL-1.0" => Some(ImplementationGuideLicense::Cddl10),
            "CDDL-1.1" => Some(ImplementationGuideLicense::Cddl11),
            "CDLA-Permissive-1.0" => Some(ImplementationGuideLicense::CdlaPermissive10),
            "CDLA-Sharing-1.0" => Some(ImplementationGuideLicense::CdlaSharing10),
            "CECILL-1.0" => Some(ImplementationGuideLicense::Cecill10),
            "CECILL-1.1" => Some(ImplementationGuideLicense::Cecill11),
            "CECILL-2.0" => Some(ImplementationGuideLicense::Cecill20),
            "CECILL-2.1" => Some(ImplementationGuideLicense::Cecill21),
            "CECILL-B" => Some(ImplementationGuideLicense::CecillB),
            "CECILL-C" => Some(ImplementationGuideLicense::CecillC),
            "ClArtistic" => Some(ImplementationGuideLicense::ClArtistic),
            "CNRI-Jython" => Some(ImplementationGuideLicense::CnriJython),
            "CNRI-Python-GPL-Compatible" => {
                Some(ImplementationGuideLicense::CnriPythonGplCompatible)
            }
            "CNRI-Python" => Some(ImplementationGuideLicense::CnriPython),
            "Condor-1.1" => Some(ImplementationGuideLicense::Condor11),
            "CPAL-1.0" => Some(ImplementationGuideLicense::Cpal10),
            "CPL-1.0" => Some(ImplementationGuideLicense::Cpl10),
            "CPOL-1.02" => Some(ImplementationGuideLicense::Cpol102),
            "Crossword" => Some(ImplementationGuideLicense::Crossword),
            "CrystalStacker" => Some(ImplementationGuideLicense::CrystalStacker),
            "CUA-OPL-1.0" => Some(ImplementationGuideLicense::CuaOpl10),
            "Cube" => Some(ImplementationGuideLicense::Cube),
            "curl" => Some(ImplementationGuideLicense::Curl),
            "D-FSL-1.0" => Some(ImplementationGuideLicense::DFsl10),
            "diffmark" => Some(ImplementationGuideLicense::Diffmark),
            "DOC" => Some(ImplementationGuideLicense::DOC),
            "Dotseqn" => Some(ImplementationGuideLicense::Dotseqn),
            "DSDP" => Some(ImplementationGuideLicense::DSDP),
            "dvipdfm" => Some(ImplementationGuideLicense::Dvipdfm),
            "ECL-1.0" => Some(ImplementationGuideLicense::Ecl10),
            "ECL-2.0" => Some(ImplementationGuideLicense::Ecl20),
            "EFL-1.0" => Some(ImplementationGuideLicense::Efl10),
            "EFL-2.0" => Some(ImplementationGuideLicense::Efl20),
            "eGenix" => Some(ImplementationGuideLicense::EGenix),
            "Entessa" => Some(ImplementationGuideLicense::Entessa),
            "EPL-1.0" => Some(ImplementationGuideLicense::Epl10),
            "EPL-2.0" => Some(ImplementationGuideLicense::Epl20),
            "ErlPL-1.1" => Some(ImplementationGuideLicense::Erlpl11),
            "EUDatagrid" => Some(ImplementationGuideLicense::EUDatagrid),
            "EUPL-1.0" => Some(ImplementationGuideLicense::Eupl10),
            "EUPL-1.1" => Some(ImplementationGuideLicense::Eupl11),
            "EUPL-1.2" => Some(ImplementationGuideLicense::Eupl12),
            "Eurosym" => Some(ImplementationGuideLicense::Eurosym),
            "Fair" => Some(ImplementationGuideLicense::Fair),
            "Frameworx-1.0" => Some(ImplementationGuideLicense::Frameworx10),
            "FreeImage" => Some(ImplementationGuideLicense::FreeImage),
            "FSFAP" => Some(ImplementationGuideLicense::FSFAP),
            "FSFUL" => Some(ImplementationGuideLicense::FSFUL),
            "FSFULLR" => Some(ImplementationGuideLicense::FSFULLR),
            "FTL" => Some(ImplementationGuideLicense::FTL),
            "GFDL-1.1-only" => Some(ImplementationGuideLicense::Gfdl11Only),
            "GFDL-1.1-or-later" => Some(ImplementationGuideLicense::Gfdl11OrLater),
            "GFDL-1.2-only" => Some(ImplementationGuideLicense::Gfdl12Only),
            "GFDL-1.2-or-later" => Some(ImplementationGuideLicense::Gfdl12OrLater),
            "GFDL-1.3-only" => Some(ImplementationGuideLicense::Gfdl13Only),
            "GFDL-1.3-or-later" => Some(ImplementationGuideLicense::Gfdl13OrLater),
            "Giftware" => Some(ImplementationGuideLicense::Giftware),
            "GL2PS" => Some(ImplementationGuideLicense::GL2PS),
            "Glide" => Some(ImplementationGuideLicense::Glide),
            "Glulxe" => Some(ImplementationGuideLicense::Glulxe),
            "gnuplot" => Some(ImplementationGuideLicense::Gnuplot),
            "GPL-1.0-only" => Some(ImplementationGuideLicense::Gpl10Only),
            "GPL-1.0-or-later" => Some(ImplementationGuideLicense::Gpl10OrLater),
            "GPL-2.0-only" => Some(ImplementationGuideLicense::Gpl20Only),
            "GPL-2.0-or-later" => Some(ImplementationGuideLicense::Gpl20OrLater),
            "GPL-3.0-only" => Some(ImplementationGuideLicense::Gpl30Only),
            "GPL-3.0-or-later" => Some(ImplementationGuideLicense::Gpl30OrLater),
            "gSOAP-1.3b" => Some(ImplementationGuideLicense::Gsoap13b),
            "HaskellReport" => Some(ImplementationGuideLicense::HaskellReport),
            "HPND" => Some(ImplementationGuideLicense::HPND),
            "IBM-pibs" => Some(ImplementationGuideLicense::IbmPibs),
            "ICU" => Some(ImplementationGuideLicense::ICU),
            "IJG" => Some(ImplementationGuideLicense::IJG),
            "ImageMagick" => Some(ImplementationGuideLicense::ImageMagick),
            "iMatix" => Some(ImplementationGuideLicense::IMatix),
            "Imlib2" => Some(ImplementationGuideLicense::Imlib2),
            "Info-ZIP" => Some(ImplementationGuideLicense::InfoZip),
            "Intel-ACPI" => Some(ImplementationGuideLicense::IntelAcpi),
            "Intel" => Some(ImplementationGuideLicense::Intel),
            "Interbase-1.0" => Some(ImplementationGuideLicense::Interbase10),
            "IPA" => Some(ImplementationGuideLicense::IPA),
            "IPL-1.0" => Some(ImplementationGuideLicense::Ipl10),
            "ISC" => Some(ImplementationGuideLicense::ISC),
            "JasPer-2.0" => Some(ImplementationGuideLicense::Jasper20),
            "JSON" => Some(ImplementationGuideLicense::JSON),
            "LAL-1.2" => Some(ImplementationGuideLicense::Lal12),
            "LAL-1.3" => Some(ImplementationGuideLicense::Lal13),
            "Latex2e" => Some(ImplementationGuideLicense::Latex2e),
            "Leptonica" => Some(ImplementationGuideLicense::Leptonica),
            "LGPL-2.0-only" => Some(ImplementationGuideLicense::Lgpl20Only),
            "LGPL-2.0-or-later" => Some(ImplementationGuideLicense::Lgpl20OrLater),
            "LGPL-2.1-only" => Some(ImplementationGuideLicense::Lgpl21Only),
            "LGPL-2.1-or-later" => Some(ImplementationGuideLicense::Lgpl21OrLater),
            "LGPL-3.0-only" => Some(ImplementationGuideLicense::Lgpl30Only),
            "LGPL-3.0-or-later" => Some(ImplementationGuideLicense::Lgpl30OrLater),
            "LGPLLR" => Some(ImplementationGuideLicense::LGPLLR),
            "Libpng" => Some(ImplementationGuideLicense::Libpng),
            "libtiff" => Some(ImplementationGuideLicense::Libtiff),
            "LiLiQ-P-1.1" => Some(ImplementationGuideLicense::LiliqP11),
            "LiLiQ-R-1.1" => Some(ImplementationGuideLicense::LiliqR11),
            "LiLiQ-Rplus-1.1" => Some(ImplementationGuideLicense::LiliqRplus11),
            "Linux-OpenIB" => Some(ImplementationGuideLicense::LinuxOpenib),
            "LPL-1.0" => Some(ImplementationGuideLicense::Lpl10),
            "LPL-1.02" => Some(ImplementationGuideLicense::Lpl102),
            "LPPL-1.0" => Some(ImplementationGuideLicense::Lppl10),
            "LPPL-1.1" => Some(ImplementationGuideLicense::Lppl11),
            "LPPL-1.2" => Some(ImplementationGuideLicense::Lppl12),
            "LPPL-1.3a" => Some(ImplementationGuideLicense::Lppl13a),
            "LPPL-1.3c" => Some(ImplementationGuideLicense::Lppl13c),
            "MakeIndex" => Some(ImplementationGuideLicense::MakeIndex),
            "MirOS" => Some(ImplementationGuideLicense::MirOS),
            "MIT-0" => Some(ImplementationGuideLicense::Mit0),
            "MIT-advertising" => Some(ImplementationGuideLicense::MitAdvertising),
            "MIT-CMU" => Some(ImplementationGuideLicense::MitCmu),
            "MIT-enna" => Some(ImplementationGuideLicense::MitEnna),
            "MIT-feh" => Some(ImplementationGuideLicense::MitFeh),
            "MIT" => Some(ImplementationGuideLicense::MIT),
            "MITNFA" => Some(ImplementationGuideLicense::MITNFA),
            "Motosoto" => Some(ImplementationGuideLicense::Motosoto),
            "mpich2" => Some(ImplementationGuideLicense::Mpich2),
            "MPL-1.0" => Some(ImplementationGuideLicense::Mpl10),
            "MPL-1.1" => Some(ImplementationGuideLicense::Mpl11),
            "MPL-2.0-no-copyleft-exception" => {
                Some(ImplementationGuideLicense::Mpl20NoCopyleftException)
            }
            "MPL-2.0" => Some(ImplementationGuideLicense::Mpl20),
            "MS-PL" => Some(ImplementationGuideLicense::MsPl),
            "MS-RL" => Some(ImplementationGuideLicense::MsRl),
            "MTLL" => Some(ImplementationGuideLicense::MTLL),
            "Multics" => Some(ImplementationGuideLicense::Multics),
            "Mup" => Some(ImplementationGuideLicense::Mup),
            "NASA-1.3" => Some(ImplementationGuideLicense::Nasa13),
            "Naumen" => Some(ImplementationGuideLicense::Naumen),
            "NBPL-1.0" => Some(ImplementationGuideLicense::Nbpl10),
            "NCSA" => Some(ImplementationGuideLicense::NCSA),
            "Net-SNMP" => Some(ImplementationGuideLicense::NetSnmp),
            "NetCDF" => Some(ImplementationGuideLicense::NetCDF),
            "Newsletr" => Some(ImplementationGuideLicense::Newsletr),
            "NGPL" => Some(ImplementationGuideLicense::NGPL),
            "NLOD-1.0" => Some(ImplementationGuideLicense::Nlod10),
            "NLPL" => Some(ImplementationGuideLicense::NLPL),
            "Nokia" => Some(ImplementationGuideLicense::Nokia),
            "NOSL" => Some(ImplementationGuideLicense::NOSL),
            "Noweb" => Some(ImplementationGuideLicense::Noweb),
            "NPL-1.0" => Some(ImplementationGuideLicense::Npl10),
            "NPL-1.1" => Some(ImplementationGuideLicense::Npl11),
            "NPOSL-3.0" => Some(ImplementationGuideLicense::Nposl30),
            "NRL" => Some(ImplementationGuideLicense::NRL),
            "NTP" => Some(ImplementationGuideLicense::NTP),
            "OCCT-PL" => Some(ImplementationGuideLicense::OcctPl),
            "OCLC-2.0" => Some(ImplementationGuideLicense::Oclc20),
            "ODbL-1.0" => Some(ImplementationGuideLicense::Odbl10),
            "OFL-1.0" => Some(ImplementationGuideLicense::Ofl10),
            "OFL-1.1" => Some(ImplementationGuideLicense::Ofl11),
            "OGTSL" => Some(ImplementationGuideLicense::OGTSL),
            "OLDAP-1.1" => Some(ImplementationGuideLicense::Oldap11),
            "OLDAP-1.2" => Some(ImplementationGuideLicense::Oldap12),
            "OLDAP-1.3" => Some(ImplementationGuideLicense::Oldap13),
            "OLDAP-1.4" => Some(ImplementationGuideLicense::Oldap14),
            "OLDAP-2.0.1" => Some(ImplementationGuideLicense::Oldap201),
            "OLDAP-2.0" => Some(ImplementationGuideLicense::Oldap20),
            "OLDAP-2.1" => Some(ImplementationGuideLicense::Oldap21),
            "OLDAP-2.2.1" => Some(ImplementationGuideLicense::Oldap221),
            "OLDAP-2.2.2" => Some(ImplementationGuideLicense::Oldap222),
            "OLDAP-2.2" => Some(ImplementationGuideLicense::Oldap22),
            "OLDAP-2.3" => Some(ImplementationGuideLicense::Oldap23),
            "OLDAP-2.4" => Some(ImplementationGuideLicense::Oldap24),
            "OLDAP-2.5" => Some(ImplementationGuideLicense::Oldap25),
            "OLDAP-2.6" => Some(ImplementationGuideLicense::Oldap26),
            "OLDAP-2.7" => Some(ImplementationGuideLicense::Oldap27),
            "OLDAP-2.8" => Some(ImplementationGuideLicense::Oldap28),
            "OML" => Some(ImplementationGuideLicense::OML),
            "OpenSSL" => Some(ImplementationGuideLicense::OpenSSL),
            "OPL-1.0" => Some(ImplementationGuideLicense::Opl10),
            "OSET-PL-2.1" => Some(ImplementationGuideLicense::OsetPl21),
            "OSL-1.0" => Some(ImplementationGuideLicense::Osl10),
            "OSL-1.1" => Some(ImplementationGuideLicense::Osl11),
            "OSL-2.0" => Some(ImplementationGuideLicense::Osl20),
            "OSL-2.1" => Some(ImplementationGuideLicense::Osl21),
            "OSL-3.0" => Some(ImplementationGuideLicense::Osl30),
            "PDDL-1.0" => Some(ImplementationGuideLicense::Pddl10),
            "PHP-3.0" => Some(ImplementationGuideLicense::Php30),
            "PHP-3.01" => Some(ImplementationGuideLicense::Php301),
            "Plexus" => Some(ImplementationGuideLicense::Plexus),
            "PostgreSQL" => Some(ImplementationGuideLicense::PostgreSQL),
            "psfrag" => Some(ImplementationGuideLicense::Psfrag),
            "psutils" => Some(ImplementationGuideLicense::Psutils),
            "Python-2.0" => Some(ImplementationGuideLicense::Python20),
            "Qhull" => Some(ImplementationGuideLicense::Qhull),
            "QPL-1.0" => Some(ImplementationGuideLicense::Qpl10),
            "Rdisc" => Some(ImplementationGuideLicense::Rdisc),
            "RHeCos-1.1" => Some(ImplementationGuideLicense::Rhecos11),
            "RPL-1.1" => Some(ImplementationGuideLicense::Rpl11),
            "RPL-1.5" => Some(ImplementationGuideLicense::Rpl15),
            "RPSL-1.0" => Some(ImplementationGuideLicense::Rpsl10),
            "RSA-MD" => Some(ImplementationGuideLicense::RsaMd),
            "RSCPL" => Some(ImplementationGuideLicense::RSCPL),
            "Ruby" => Some(ImplementationGuideLicense::Ruby),
            "SAX-PD" => Some(ImplementationGuideLicense::SaxPd),
            "Saxpath" => Some(ImplementationGuideLicense::Saxpath),
            "SCEA" => Some(ImplementationGuideLicense::SCEA),
            "Sendmail" => Some(ImplementationGuideLicense::Sendmail),
            "SGI-B-1.0" => Some(ImplementationGuideLicense::SgiB10),
            "SGI-B-1.1" => Some(ImplementationGuideLicense::SgiB11),
            "SGI-B-2.0" => Some(ImplementationGuideLicense::SgiB20),
            "SimPL-2.0" => Some(ImplementationGuideLicense::Simpl20),
            "SISSL-1.2" => Some(ImplementationGuideLicense::Sissl12),
            "SISSL" => Some(ImplementationGuideLicense::SISSL),
            "Sleepycat" => Some(ImplementationGuideLicense::Sleepycat),
            "SMLNJ" => Some(ImplementationGuideLicense::SMLNJ),
            "SMPPL" => Some(ImplementationGuideLicense::SMPPL),
            "SNIA" => Some(ImplementationGuideLicense::SNIA),
            "Spencer-86" => Some(ImplementationGuideLicense::Spencer86),
            "Spencer-94" => Some(ImplementationGuideLicense::Spencer94),
            "Spencer-99" => Some(ImplementationGuideLicense::Spencer99),
            "SPL-1.0" => Some(ImplementationGuideLicense::Spl10),
            "SugarCRM-1.1.3" => Some(ImplementationGuideLicense::Sugarcrm113),
            "SWL" => Some(ImplementationGuideLicense::SWL),
            "TCL" => Some(ImplementationGuideLicense::TCL),
            "TCP-wrappers" => Some(ImplementationGuideLicense::TcpWrappers),
            "TMate" => Some(ImplementationGuideLicense::TMate),
            "TORQUE-1.1" => Some(ImplementationGuideLicense::Torque11),
            "TOSL" => Some(ImplementationGuideLicense::TOSL),
            "Unicode-DFS-2015" => Some(ImplementationGuideLicense::UnicodeDfs2015),
            "Unicode-DFS-2016" => Some(ImplementationGuideLicense::UnicodeDfs2016),
            "Unicode-TOU" => Some(ImplementationGuideLicense::UnicodeTou),
            "Unlicense" => Some(ImplementationGuideLicense::Unlicense),
            "UPL-1.0" => Some(ImplementationGuideLicense::Upl10),
            "Vim" => Some(ImplementationGuideLicense::Vim),
            "VOSTROM" => Some(ImplementationGuideLicense::VOSTROM),
            "VSL-1.0" => Some(ImplementationGuideLicense::Vsl10),
            "W3C-19980720" => Some(ImplementationGuideLicense::W3c19980720),
            "W3C-20150513" => Some(ImplementationGuideLicense::W3c20150513),
            "W3C" => Some(ImplementationGuideLicense::W3C),
            "Watcom-1.0" => Some(ImplementationGuideLicense::Watcom10),
            "Wsuipa" => Some(ImplementationGuideLicense::Wsuipa),
            "WTFPL" => Some(ImplementationGuideLicense::WTFPL),
            "X11" => Some(ImplementationGuideLicense::X11),
            "Xerox" => Some(ImplementationGuideLicense::Xerox),
            "XFree86-1.1" => Some(ImplementationGuideLicense::Xfree8611),
            "xinetd" => Some(ImplementationGuideLicense::Xinetd),
            "Xnet" => Some(ImplementationGuideLicense::Xnet),
            "xpp" => Some(ImplementationGuideLicense::Xpp),
            "XSkat" => Some(ImplementationGuideLicense::XSkat),
            "YPL-1.0" => Some(ImplementationGuideLicense::Ypl10),
            "YPL-1.1" => Some(ImplementationGuideLicense::Ypl11),
            "Zed" => Some(ImplementationGuideLicense::Zed),
            "Zend-2.0" => Some(ImplementationGuideLicense::Zend20),
            "Zimbra-1.3" => Some(ImplementationGuideLicense::Zimbra13),
            "Zimbra-1.4" => Some(ImplementationGuideLicense::Zimbra14),
            "zlib-acknowledgement" => Some(ImplementationGuideLicense::ZlibAcknowledgement),
            "Zlib" => Some(ImplementationGuideLicense::Zlib),
            "ZPL-1.1" => Some(ImplementationGuideLicense::Zpl11),
            "ZPL-2.0" => Some(ImplementationGuideLicense::Zpl20),
            "ZPL-2.1" => Some(ImplementationGuideLicense::Zpl21),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ImplementationGuideLicense::NotOpenSource => "not-open-source".to_string(),
            ImplementationGuideLicense::Fhir0bsd => "0BSD".to_string(),
            ImplementationGuideLicense::AAL => "AAL".to_string(),
            ImplementationGuideLicense::Abstyles => "Abstyles".to_string(),
            ImplementationGuideLicense::Adobe2006 => "Adobe-2006".to_string(),
            ImplementationGuideLicense::AdobeGlyph => "Adobe-Glyph".to_string(),
            ImplementationGuideLicense::ADSL => "ADSL".to_string(),
            ImplementationGuideLicense::Afl11 => "AFL-1.1".to_string(),
            ImplementationGuideLicense::Afl12 => "AFL-1.2".to_string(),
            ImplementationGuideLicense::Afl20 => "AFL-2.0".to_string(),
            ImplementationGuideLicense::Afl21 => "AFL-2.1".to_string(),
            ImplementationGuideLicense::Afl30 => "AFL-3.0".to_string(),
            ImplementationGuideLicense::Afmparse => "Afmparse".to_string(),
            ImplementationGuideLicense::Agpl10Only => "AGPL-1.0-only".to_string(),
            ImplementationGuideLicense::Agpl10OrLater => "AGPL-1.0-or-later".to_string(),
            ImplementationGuideLicense::Agpl30Only => "AGPL-3.0-only".to_string(),
            ImplementationGuideLicense::Agpl30OrLater => "AGPL-3.0-or-later".to_string(),
            ImplementationGuideLicense::Aladdin => "Aladdin".to_string(),
            ImplementationGuideLicense::AMDPLPA => "AMDPLPA".to_string(),
            ImplementationGuideLicense::AML => "AML".to_string(),
            ImplementationGuideLicense::AMPAS => "AMPAS".to_string(),
            ImplementationGuideLicense::AntlrPd => "ANTLR-PD".to_string(),
            ImplementationGuideLicense::Apache10 => "Apache-1.0".to_string(),
            ImplementationGuideLicense::Apache11 => "Apache-1.1".to_string(),
            ImplementationGuideLicense::Apache20 => "Apache-2.0".to_string(),
            ImplementationGuideLicense::APAFML => "APAFML".to_string(),
            ImplementationGuideLicense::Apl10 => "APL-1.0".to_string(),
            ImplementationGuideLicense::Apsl10 => "APSL-1.0".to_string(),
            ImplementationGuideLicense::Apsl11 => "APSL-1.1".to_string(),
            ImplementationGuideLicense::Apsl12 => "APSL-1.2".to_string(),
            ImplementationGuideLicense::Apsl20 => "APSL-2.0".to_string(),
            ImplementationGuideLicense::Artistic10Cl8 => "Artistic-1.0-cl8".to_string(),
            ImplementationGuideLicense::Artistic10Perl => "Artistic-1.0-Perl".to_string(),
            ImplementationGuideLicense::Artistic10 => "Artistic-1.0".to_string(),
            ImplementationGuideLicense::Artistic20 => "Artistic-2.0".to_string(),
            ImplementationGuideLicense::Bahyph => "Bahyph".to_string(),
            ImplementationGuideLicense::Barr => "Barr".to_string(),
            ImplementationGuideLicense::Beerware => "Beerware".to_string(),
            ImplementationGuideLicense::Bittorrent10 => "BitTorrent-1.0".to_string(),
            ImplementationGuideLicense::Bittorrent11 => "BitTorrent-1.1".to_string(),
            ImplementationGuideLicense::Borceux => "Borceux".to_string(),
            ImplementationGuideLicense::Bsd1Clause => "BSD-1-Clause".to_string(),
            ImplementationGuideLicense::Bsd2ClauseFreebsd => "BSD-2-Clause-FreeBSD".to_string(),
            ImplementationGuideLicense::Bsd2ClauseNetbsd => "BSD-2-Clause-NetBSD".to_string(),
            ImplementationGuideLicense::Bsd2ClausePatent => "BSD-2-Clause-Patent".to_string(),
            ImplementationGuideLicense::Bsd2Clause => "BSD-2-Clause".to_string(),
            ImplementationGuideLicense::Bsd3ClauseAttribution => {
                "BSD-3-Clause-Attribution".to_string()
            }
            ImplementationGuideLicense::Bsd3ClauseClear => "BSD-3-Clause-Clear".to_string(),
            ImplementationGuideLicense::Bsd3ClauseLbnl => "BSD-3-Clause-LBNL".to_string(),
            ImplementationGuideLicense::Bsd3ClauseNoNuclearLicense2014 => {
                "BSD-3-Clause-No-Nuclear-License-2014".to_string()
            }
            ImplementationGuideLicense::Bsd3ClauseNoNuclearLicense => {
                "BSD-3-Clause-No-Nuclear-License".to_string()
            }
            ImplementationGuideLicense::Bsd3ClauseNoNuclearWarranty => {
                "BSD-3-Clause-No-Nuclear-Warranty".to_string()
            }
            ImplementationGuideLicense::Bsd3Clause => "BSD-3-Clause".to_string(),
            ImplementationGuideLicense::Bsd4ClauseUc => "BSD-4-Clause-UC".to_string(),
            ImplementationGuideLicense::Bsd4Clause => "BSD-4-Clause".to_string(),
            ImplementationGuideLicense::BsdProtection => "BSD-Protection".to_string(),
            ImplementationGuideLicense::BsdSourceCode => "BSD-Source-Code".to_string(),
            ImplementationGuideLicense::Bsl10 => "BSL-1.0".to_string(),
            ImplementationGuideLicense::Bzip2105 => "bzip2-1.0.5".to_string(),
            ImplementationGuideLicense::Bzip2106 => "bzip2-1.0.6".to_string(),
            ImplementationGuideLicense::Caldera => "Caldera".to_string(),
            ImplementationGuideLicense::Catosl11 => "CATOSL-1.1".to_string(),
            ImplementationGuideLicense::CcBy10 => "CC-BY-1.0".to_string(),
            ImplementationGuideLicense::CcBy20 => "CC-BY-2.0".to_string(),
            ImplementationGuideLicense::CcBy25 => "CC-BY-2.5".to_string(),
            ImplementationGuideLicense::CcBy30 => "CC-BY-3.0".to_string(),
            ImplementationGuideLicense::CcBy40 => "CC-BY-4.0".to_string(),
            ImplementationGuideLicense::CcByNc10 => "CC-BY-NC-1.0".to_string(),
            ImplementationGuideLicense::CcByNc20 => "CC-BY-NC-2.0".to_string(),
            ImplementationGuideLicense::CcByNc25 => "CC-BY-NC-2.5".to_string(),
            ImplementationGuideLicense::CcByNc30 => "CC-BY-NC-3.0".to_string(),
            ImplementationGuideLicense::CcByNc40 => "CC-BY-NC-4.0".to_string(),
            ImplementationGuideLicense::CcByNcNd10 => "CC-BY-NC-ND-1.0".to_string(),
            ImplementationGuideLicense::CcByNcNd20 => "CC-BY-NC-ND-2.0".to_string(),
            ImplementationGuideLicense::CcByNcNd25 => "CC-BY-NC-ND-2.5".to_string(),
            ImplementationGuideLicense::CcByNcNd30 => "CC-BY-NC-ND-3.0".to_string(),
            ImplementationGuideLicense::CcByNcNd40 => "CC-BY-NC-ND-4.0".to_string(),
            ImplementationGuideLicense::CcByNcSa10 => "CC-BY-NC-SA-1.0".to_string(),
            ImplementationGuideLicense::CcByNcSa20 => "CC-BY-NC-SA-2.0".to_string(),
            ImplementationGuideLicense::CcByNcSa25 => "CC-BY-NC-SA-2.5".to_string(),
            ImplementationGuideLicense::CcByNcSa30 => "CC-BY-NC-SA-3.0".to_string(),
            ImplementationGuideLicense::CcByNcSa40 => "CC-BY-NC-SA-4.0".to_string(),
            ImplementationGuideLicense::CcByNd10 => "CC-BY-ND-1.0".to_string(),
            ImplementationGuideLicense::CcByNd20 => "CC-BY-ND-2.0".to_string(),
            ImplementationGuideLicense::CcByNd25 => "CC-BY-ND-2.5".to_string(),
            ImplementationGuideLicense::CcByNd30 => "CC-BY-ND-3.0".to_string(),
            ImplementationGuideLicense::CcByNd40 => "CC-BY-ND-4.0".to_string(),
            ImplementationGuideLicense::CcBySa10 => "CC-BY-SA-1.0".to_string(),
            ImplementationGuideLicense::CcBySa20 => "CC-BY-SA-2.0".to_string(),
            ImplementationGuideLicense::CcBySa25 => "CC-BY-SA-2.5".to_string(),
            ImplementationGuideLicense::CcBySa30 => "CC-BY-SA-3.0".to_string(),
            ImplementationGuideLicense::CcBySa40 => "CC-BY-SA-4.0".to_string(),
            ImplementationGuideLicense::Cc010 => "CC0-1.0".to_string(),
            ImplementationGuideLicense::Cddl10 => "CDDL-1.0".to_string(),
            ImplementationGuideLicense::Cddl11 => "CDDL-1.1".to_string(),
            ImplementationGuideLicense::CdlaPermissive10 => "CDLA-Permissive-1.0".to_string(),
            ImplementationGuideLicense::CdlaSharing10 => "CDLA-Sharing-1.0".to_string(),
            ImplementationGuideLicense::Cecill10 => "CECILL-1.0".to_string(),
            ImplementationGuideLicense::Cecill11 => "CECILL-1.1".to_string(),
            ImplementationGuideLicense::Cecill20 => "CECILL-2.0".to_string(),
            ImplementationGuideLicense::Cecill21 => "CECILL-2.1".to_string(),
            ImplementationGuideLicense::CecillB => "CECILL-B".to_string(),
            ImplementationGuideLicense::CecillC => "CECILL-C".to_string(),
            ImplementationGuideLicense::ClArtistic => "ClArtistic".to_string(),
            ImplementationGuideLicense::CnriJython => "CNRI-Jython".to_string(),
            ImplementationGuideLicense::CnriPythonGplCompatible => {
                "CNRI-Python-GPL-Compatible".to_string()
            }
            ImplementationGuideLicense::CnriPython => "CNRI-Python".to_string(),
            ImplementationGuideLicense::Condor11 => "Condor-1.1".to_string(),
            ImplementationGuideLicense::Cpal10 => "CPAL-1.0".to_string(),
            ImplementationGuideLicense::Cpl10 => "CPL-1.0".to_string(),
            ImplementationGuideLicense::Cpol102 => "CPOL-1.02".to_string(),
            ImplementationGuideLicense::Crossword => "Crossword".to_string(),
            ImplementationGuideLicense::CrystalStacker => "CrystalStacker".to_string(),
            ImplementationGuideLicense::CuaOpl10 => "CUA-OPL-1.0".to_string(),
            ImplementationGuideLicense::Cube => "Cube".to_string(),
            ImplementationGuideLicense::Curl => "curl".to_string(),
            ImplementationGuideLicense::DFsl10 => "D-FSL-1.0".to_string(),
            ImplementationGuideLicense::Diffmark => "diffmark".to_string(),
            ImplementationGuideLicense::DOC => "DOC".to_string(),
            ImplementationGuideLicense::Dotseqn => "Dotseqn".to_string(),
            ImplementationGuideLicense::DSDP => "DSDP".to_string(),
            ImplementationGuideLicense::Dvipdfm => "dvipdfm".to_string(),
            ImplementationGuideLicense::Ecl10 => "ECL-1.0".to_string(),
            ImplementationGuideLicense::Ecl20 => "ECL-2.0".to_string(),
            ImplementationGuideLicense::Efl10 => "EFL-1.0".to_string(),
            ImplementationGuideLicense::Efl20 => "EFL-2.0".to_string(),
            ImplementationGuideLicense::EGenix => "eGenix".to_string(),
            ImplementationGuideLicense::Entessa => "Entessa".to_string(),
            ImplementationGuideLicense::Epl10 => "EPL-1.0".to_string(),
            ImplementationGuideLicense::Epl20 => "EPL-2.0".to_string(),
            ImplementationGuideLicense::Erlpl11 => "ErlPL-1.1".to_string(),
            ImplementationGuideLicense::EUDatagrid => "EUDatagrid".to_string(),
            ImplementationGuideLicense::Eupl10 => "EUPL-1.0".to_string(),
            ImplementationGuideLicense::Eupl11 => "EUPL-1.1".to_string(),
            ImplementationGuideLicense::Eupl12 => "EUPL-1.2".to_string(),
            ImplementationGuideLicense::Eurosym => "Eurosym".to_string(),
            ImplementationGuideLicense::Fair => "Fair".to_string(),
            ImplementationGuideLicense::Frameworx10 => "Frameworx-1.0".to_string(),
            ImplementationGuideLicense::FreeImage => "FreeImage".to_string(),
            ImplementationGuideLicense::FSFAP => "FSFAP".to_string(),
            ImplementationGuideLicense::FSFUL => "FSFUL".to_string(),
            ImplementationGuideLicense::FSFULLR => "FSFULLR".to_string(),
            ImplementationGuideLicense::FTL => "FTL".to_string(),
            ImplementationGuideLicense::Gfdl11Only => "GFDL-1.1-only".to_string(),
            ImplementationGuideLicense::Gfdl11OrLater => "GFDL-1.1-or-later".to_string(),
            ImplementationGuideLicense::Gfdl12Only => "GFDL-1.2-only".to_string(),
            ImplementationGuideLicense::Gfdl12OrLater => "GFDL-1.2-or-later".to_string(),
            ImplementationGuideLicense::Gfdl13Only => "GFDL-1.3-only".to_string(),
            ImplementationGuideLicense::Gfdl13OrLater => "GFDL-1.3-or-later".to_string(),
            ImplementationGuideLicense::Giftware => "Giftware".to_string(),
            ImplementationGuideLicense::GL2PS => "GL2PS".to_string(),
            ImplementationGuideLicense::Glide => "Glide".to_string(),
            ImplementationGuideLicense::Glulxe => "Glulxe".to_string(),
            ImplementationGuideLicense::Gnuplot => "gnuplot".to_string(),
            ImplementationGuideLicense::Gpl10Only => "GPL-1.0-only".to_string(),
            ImplementationGuideLicense::Gpl10OrLater => "GPL-1.0-or-later".to_string(),
            ImplementationGuideLicense::Gpl20Only => "GPL-2.0-only".to_string(),
            ImplementationGuideLicense::Gpl20OrLater => "GPL-2.0-or-later".to_string(),
            ImplementationGuideLicense::Gpl30Only => "GPL-3.0-only".to_string(),
            ImplementationGuideLicense::Gpl30OrLater => "GPL-3.0-or-later".to_string(),
            ImplementationGuideLicense::Gsoap13b => "gSOAP-1.3b".to_string(),
            ImplementationGuideLicense::HaskellReport => "HaskellReport".to_string(),
            ImplementationGuideLicense::HPND => "HPND".to_string(),
            ImplementationGuideLicense::IbmPibs => "IBM-pibs".to_string(),
            ImplementationGuideLicense::ICU => "ICU".to_string(),
            ImplementationGuideLicense::IJG => "IJG".to_string(),
            ImplementationGuideLicense::ImageMagick => "ImageMagick".to_string(),
            ImplementationGuideLicense::IMatix => "iMatix".to_string(),
            ImplementationGuideLicense::Imlib2 => "Imlib2".to_string(),
            ImplementationGuideLicense::InfoZip => "Info-ZIP".to_string(),
            ImplementationGuideLicense::IntelAcpi => "Intel-ACPI".to_string(),
            ImplementationGuideLicense::Intel => "Intel".to_string(),
            ImplementationGuideLicense::Interbase10 => "Interbase-1.0".to_string(),
            ImplementationGuideLicense::IPA => "IPA".to_string(),
            ImplementationGuideLicense::Ipl10 => "IPL-1.0".to_string(),
            ImplementationGuideLicense::ISC => "ISC".to_string(),
            ImplementationGuideLicense::Jasper20 => "JasPer-2.0".to_string(),
            ImplementationGuideLicense::JSON => "JSON".to_string(),
            ImplementationGuideLicense::Lal12 => "LAL-1.2".to_string(),
            ImplementationGuideLicense::Lal13 => "LAL-1.3".to_string(),
            ImplementationGuideLicense::Latex2e => "Latex2e".to_string(),
            ImplementationGuideLicense::Leptonica => "Leptonica".to_string(),
            ImplementationGuideLicense::Lgpl20Only => "LGPL-2.0-only".to_string(),
            ImplementationGuideLicense::Lgpl20OrLater => "LGPL-2.0-or-later".to_string(),
            ImplementationGuideLicense::Lgpl21Only => "LGPL-2.1-only".to_string(),
            ImplementationGuideLicense::Lgpl21OrLater => "LGPL-2.1-or-later".to_string(),
            ImplementationGuideLicense::Lgpl30Only => "LGPL-3.0-only".to_string(),
            ImplementationGuideLicense::Lgpl30OrLater => "LGPL-3.0-or-later".to_string(),
            ImplementationGuideLicense::LGPLLR => "LGPLLR".to_string(),
            ImplementationGuideLicense::Libpng => "Libpng".to_string(),
            ImplementationGuideLicense::Libtiff => "libtiff".to_string(),
            ImplementationGuideLicense::LiliqP11 => "LiLiQ-P-1.1".to_string(),
            ImplementationGuideLicense::LiliqR11 => "LiLiQ-R-1.1".to_string(),
            ImplementationGuideLicense::LiliqRplus11 => "LiLiQ-Rplus-1.1".to_string(),
            ImplementationGuideLicense::LinuxOpenib => "Linux-OpenIB".to_string(),
            ImplementationGuideLicense::Lpl10 => "LPL-1.0".to_string(),
            ImplementationGuideLicense::Lpl102 => "LPL-1.02".to_string(),
            ImplementationGuideLicense::Lppl10 => "LPPL-1.0".to_string(),
            ImplementationGuideLicense::Lppl11 => "LPPL-1.1".to_string(),
            ImplementationGuideLicense::Lppl12 => "LPPL-1.2".to_string(),
            ImplementationGuideLicense::Lppl13a => "LPPL-1.3a".to_string(),
            ImplementationGuideLicense::Lppl13c => "LPPL-1.3c".to_string(),
            ImplementationGuideLicense::MakeIndex => "MakeIndex".to_string(),
            ImplementationGuideLicense::MirOS => "MirOS".to_string(),
            ImplementationGuideLicense::Mit0 => "MIT-0".to_string(),
            ImplementationGuideLicense::MitAdvertising => "MIT-advertising".to_string(),
            ImplementationGuideLicense::MitCmu => "MIT-CMU".to_string(),
            ImplementationGuideLicense::MitEnna => "MIT-enna".to_string(),
            ImplementationGuideLicense::MitFeh => "MIT-feh".to_string(),
            ImplementationGuideLicense::MIT => "MIT".to_string(),
            ImplementationGuideLicense::MITNFA => "MITNFA".to_string(),
            ImplementationGuideLicense::Motosoto => "Motosoto".to_string(),
            ImplementationGuideLicense::Mpich2 => "mpich2".to_string(),
            ImplementationGuideLicense::Mpl10 => "MPL-1.0".to_string(),
            ImplementationGuideLicense::Mpl11 => "MPL-1.1".to_string(),
            ImplementationGuideLicense::Mpl20NoCopyleftException => {
                "MPL-2.0-no-copyleft-exception".to_string()
            }
            ImplementationGuideLicense::Mpl20 => "MPL-2.0".to_string(),
            ImplementationGuideLicense::MsPl => "MS-PL".to_string(),
            ImplementationGuideLicense::MsRl => "MS-RL".to_string(),
            ImplementationGuideLicense::MTLL => "MTLL".to_string(),
            ImplementationGuideLicense::Multics => "Multics".to_string(),
            ImplementationGuideLicense::Mup => "Mup".to_string(),
            ImplementationGuideLicense::Nasa13 => "NASA-1.3".to_string(),
            ImplementationGuideLicense::Naumen => "Naumen".to_string(),
            ImplementationGuideLicense::Nbpl10 => "NBPL-1.0".to_string(),
            ImplementationGuideLicense::NCSA => "NCSA".to_string(),
            ImplementationGuideLicense::NetSnmp => "Net-SNMP".to_string(),
            ImplementationGuideLicense::NetCDF => "NetCDF".to_string(),
            ImplementationGuideLicense::Newsletr => "Newsletr".to_string(),
            ImplementationGuideLicense::NGPL => "NGPL".to_string(),
            ImplementationGuideLicense::Nlod10 => "NLOD-1.0".to_string(),
            ImplementationGuideLicense::NLPL => "NLPL".to_string(),
            ImplementationGuideLicense::Nokia => "Nokia".to_string(),
            ImplementationGuideLicense::NOSL => "NOSL".to_string(),
            ImplementationGuideLicense::Noweb => "Noweb".to_string(),
            ImplementationGuideLicense::Npl10 => "NPL-1.0".to_string(),
            ImplementationGuideLicense::Npl11 => "NPL-1.1".to_string(),
            ImplementationGuideLicense::Nposl30 => "NPOSL-3.0".to_string(),
            ImplementationGuideLicense::NRL => "NRL".to_string(),
            ImplementationGuideLicense::NTP => "NTP".to_string(),
            ImplementationGuideLicense::OcctPl => "OCCT-PL".to_string(),
            ImplementationGuideLicense::Oclc20 => "OCLC-2.0".to_string(),
            ImplementationGuideLicense::Odbl10 => "ODbL-1.0".to_string(),
            ImplementationGuideLicense::Ofl10 => "OFL-1.0".to_string(),
            ImplementationGuideLicense::Ofl11 => "OFL-1.1".to_string(),
            ImplementationGuideLicense::OGTSL => "OGTSL".to_string(),
            ImplementationGuideLicense::Oldap11 => "OLDAP-1.1".to_string(),
            ImplementationGuideLicense::Oldap12 => "OLDAP-1.2".to_string(),
            ImplementationGuideLicense::Oldap13 => "OLDAP-1.3".to_string(),
            ImplementationGuideLicense::Oldap14 => "OLDAP-1.4".to_string(),
            ImplementationGuideLicense::Oldap201 => "OLDAP-2.0.1".to_string(),
            ImplementationGuideLicense::Oldap20 => "OLDAP-2.0".to_string(),
            ImplementationGuideLicense::Oldap21 => "OLDAP-2.1".to_string(),
            ImplementationGuideLicense::Oldap221 => "OLDAP-2.2.1".to_string(),
            ImplementationGuideLicense::Oldap222 => "OLDAP-2.2.2".to_string(),
            ImplementationGuideLicense::Oldap22 => "OLDAP-2.2".to_string(),
            ImplementationGuideLicense::Oldap23 => "OLDAP-2.3".to_string(),
            ImplementationGuideLicense::Oldap24 => "OLDAP-2.4".to_string(),
            ImplementationGuideLicense::Oldap25 => "OLDAP-2.5".to_string(),
            ImplementationGuideLicense::Oldap26 => "OLDAP-2.6".to_string(),
            ImplementationGuideLicense::Oldap27 => "OLDAP-2.7".to_string(),
            ImplementationGuideLicense::Oldap28 => "OLDAP-2.8".to_string(),
            ImplementationGuideLicense::OML => "OML".to_string(),
            ImplementationGuideLicense::OpenSSL => "OpenSSL".to_string(),
            ImplementationGuideLicense::Opl10 => "OPL-1.0".to_string(),
            ImplementationGuideLicense::OsetPl21 => "OSET-PL-2.1".to_string(),
            ImplementationGuideLicense::Osl10 => "OSL-1.0".to_string(),
            ImplementationGuideLicense::Osl11 => "OSL-1.1".to_string(),
            ImplementationGuideLicense::Osl20 => "OSL-2.0".to_string(),
            ImplementationGuideLicense::Osl21 => "OSL-2.1".to_string(),
            ImplementationGuideLicense::Osl30 => "OSL-3.0".to_string(),
            ImplementationGuideLicense::Pddl10 => "PDDL-1.0".to_string(),
            ImplementationGuideLicense::Php30 => "PHP-3.0".to_string(),
            ImplementationGuideLicense::Php301 => "PHP-3.01".to_string(),
            ImplementationGuideLicense::Plexus => "Plexus".to_string(),
            ImplementationGuideLicense::PostgreSQL => "PostgreSQL".to_string(),
            ImplementationGuideLicense::Psfrag => "psfrag".to_string(),
            ImplementationGuideLicense::Psutils => "psutils".to_string(),
            ImplementationGuideLicense::Python20 => "Python-2.0".to_string(),
            ImplementationGuideLicense::Qhull => "Qhull".to_string(),
            ImplementationGuideLicense::Qpl10 => "QPL-1.0".to_string(),
            ImplementationGuideLicense::Rdisc => "Rdisc".to_string(),
            ImplementationGuideLicense::Rhecos11 => "RHeCos-1.1".to_string(),
            ImplementationGuideLicense::Rpl11 => "RPL-1.1".to_string(),
            ImplementationGuideLicense::Rpl15 => "RPL-1.5".to_string(),
            ImplementationGuideLicense::Rpsl10 => "RPSL-1.0".to_string(),
            ImplementationGuideLicense::RsaMd => "RSA-MD".to_string(),
            ImplementationGuideLicense::RSCPL => "RSCPL".to_string(),
            ImplementationGuideLicense::Ruby => "Ruby".to_string(),
            ImplementationGuideLicense::SaxPd => "SAX-PD".to_string(),
            ImplementationGuideLicense::Saxpath => "Saxpath".to_string(),
            ImplementationGuideLicense::SCEA => "SCEA".to_string(),
            ImplementationGuideLicense::Sendmail => "Sendmail".to_string(),
            ImplementationGuideLicense::SgiB10 => "SGI-B-1.0".to_string(),
            ImplementationGuideLicense::SgiB11 => "SGI-B-1.1".to_string(),
            ImplementationGuideLicense::SgiB20 => "SGI-B-2.0".to_string(),
            ImplementationGuideLicense::Simpl20 => "SimPL-2.0".to_string(),
            ImplementationGuideLicense::Sissl12 => "SISSL-1.2".to_string(),
            ImplementationGuideLicense::SISSL => "SISSL".to_string(),
            ImplementationGuideLicense::Sleepycat => "Sleepycat".to_string(),
            ImplementationGuideLicense::SMLNJ => "SMLNJ".to_string(),
            ImplementationGuideLicense::SMPPL => "SMPPL".to_string(),
            ImplementationGuideLicense::SNIA => "SNIA".to_string(),
            ImplementationGuideLicense::Spencer86 => "Spencer-86".to_string(),
            ImplementationGuideLicense::Spencer94 => "Spencer-94".to_string(),
            ImplementationGuideLicense::Spencer99 => "Spencer-99".to_string(),
            ImplementationGuideLicense::Spl10 => "SPL-1.0".to_string(),
            ImplementationGuideLicense::Sugarcrm113 => "SugarCRM-1.1.3".to_string(),
            ImplementationGuideLicense::SWL => "SWL".to_string(),
            ImplementationGuideLicense::TCL => "TCL".to_string(),
            ImplementationGuideLicense::TcpWrappers => "TCP-wrappers".to_string(),
            ImplementationGuideLicense::TMate => "TMate".to_string(),
            ImplementationGuideLicense::Torque11 => "TORQUE-1.1".to_string(),
            ImplementationGuideLicense::TOSL => "TOSL".to_string(),
            ImplementationGuideLicense::UnicodeDfs2015 => "Unicode-DFS-2015".to_string(),
            ImplementationGuideLicense::UnicodeDfs2016 => "Unicode-DFS-2016".to_string(),
            ImplementationGuideLicense::UnicodeTou => "Unicode-TOU".to_string(),
            ImplementationGuideLicense::Unlicense => "Unlicense".to_string(),
            ImplementationGuideLicense::Upl10 => "UPL-1.0".to_string(),
            ImplementationGuideLicense::Vim => "Vim".to_string(),
            ImplementationGuideLicense::VOSTROM => "VOSTROM".to_string(),
            ImplementationGuideLicense::Vsl10 => "VSL-1.0".to_string(),
            ImplementationGuideLicense::W3c19980720 => "W3C-19980720".to_string(),
            ImplementationGuideLicense::W3c20150513 => "W3C-20150513".to_string(),
            ImplementationGuideLicense::W3C => "W3C".to_string(),
            ImplementationGuideLicense::Watcom10 => "Watcom-1.0".to_string(),
            ImplementationGuideLicense::Wsuipa => "Wsuipa".to_string(),
            ImplementationGuideLicense::WTFPL => "WTFPL".to_string(),
            ImplementationGuideLicense::X11 => "X11".to_string(),
            ImplementationGuideLicense::Xerox => "Xerox".to_string(),
            ImplementationGuideLicense::Xfree8611 => "XFree86-1.1".to_string(),
            ImplementationGuideLicense::Xinetd => "xinetd".to_string(),
            ImplementationGuideLicense::Xnet => "Xnet".to_string(),
            ImplementationGuideLicense::Xpp => "xpp".to_string(),
            ImplementationGuideLicense::XSkat => "XSkat".to_string(),
            ImplementationGuideLicense::Ypl10 => "YPL-1.0".to_string(),
            ImplementationGuideLicense::Ypl11 => "YPL-1.1".to_string(),
            ImplementationGuideLicense::Zed => "Zed".to_string(),
            ImplementationGuideLicense::Zend20 => "Zend-2.0".to_string(),
            ImplementationGuideLicense::Zimbra13 => "Zimbra-1.3".to_string(),
            ImplementationGuideLicense::Zimbra14 => "Zimbra-1.4".to_string(),
            ImplementationGuideLicense::ZlibAcknowledgement => "zlib-acknowledgement".to_string(),
            ImplementationGuideLicense::Zlib => "Zlib".to_string(),
            ImplementationGuideLicense::Zpl11 => "ZPL-1.1".to_string(),
            ImplementationGuideLicense::Zpl20 => "ZPL-2.0".to_string(),
            ImplementationGuideLicense::Zpl21 => "ZPL-2.1".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum ImplementationGuideStatus {
    Draft,
    Active,
    Retired,
    Unknown,
}

impl ImplementationGuideStatus {
    pub fn from_string(string: &str) -> Option<ImplementationGuideStatus> {
        match string {
            "draft" => Some(ImplementationGuideStatus::Draft),
            "active" => Some(ImplementationGuideStatus::Active),
            "retired" => Some(ImplementationGuideStatus::Retired),
            "unknown" => Some(ImplementationGuideStatus::Unknown),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ImplementationGuideStatus::Draft => "draft".to_string(),
            ImplementationGuideStatus::Active => "active".to_string(),
            ImplementationGuideStatus::Retired => "retired".to_string(),
            ImplementationGuideStatus::Unknown => "unknown".to_string(),
        }
    }
}
