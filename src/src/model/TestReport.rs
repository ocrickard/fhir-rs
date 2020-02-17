#![allow(unused_imports, non_camel_case_types)]

use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Identifier::Identifier;
use crate::model::Meta::Meta;
use crate::model::Narrative::Narrative;
use crate::model::Reference::Reference;
use crate::model::ResourceList::ResourceList;
use crate::model::TestReport_Participant::TestReport_Participant;
use crate::model::TestReport_Setup::TestReport_Setup;
use crate::model::TestReport_Teardown::TestReport_Teardown;
use crate::model::TestReport_Test::TestReport_Test;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A summary of information based on the results of executing a TestScript.

#[derive(Debug)]
pub struct TestReport<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl TestReport<'_> {
    pub fn new(value: &Value) -> TestReport {
        TestReport {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
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

    /// Extensions for result
    pub fn _result(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_result") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for score
    pub fn _score(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_score") {
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

    /// Extensions for tester
    pub fn _tester(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_tester") {
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

    /// Identifier for the TestScript assigned for external purposes outside the context
    /// of FHIR.
    pub fn identifier(&self) -> Option<Identifier> {
        if let Some(val) = self.value.get("identifier") {
            return Some(Identifier {
                value: Cow::Borrowed(val),
            });
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

    /// When the TestScript was executed and this TestReport was generated.
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

    /// A free text natural language name identifying the executed TestScript.
    pub fn name(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("name") {
            return Some(string);
        }
        return None;
    }

    /// A participant in the test execution, either the execution engine, a client, or a
    /// server.
    pub fn participant(&self) -> Option<Vec<TestReport_Participant>> {
        if let Some(Value::Array(val)) = self.value.get("participant") {
            return Some(
                val.into_iter()
                    .map(|e| TestReport_Participant {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// The overall result from the execution of the TestScript.
    pub fn result(&self) -> Option<TestReportResult> {
        if let Some(Value::String(val)) = self.value.get("result") {
            return Some(TestReportResult::from_string(&val).unwrap());
        }
        return None;
    }

    /// The final score (percentage of tests passed) resulting from the execution of the
    /// TestScript.
    pub fn score(&self) -> Option<f64> {
        if let Some(val) = self.value.get("score") {
            return Some(val.as_f64().unwrap());
        }
        return None;
    }

    /// The results of the series of required setup operations before the tests were
    /// executed.
    pub fn setup(&self) -> Option<TestReport_Setup> {
        if let Some(val) = self.value.get("setup") {
            return Some(TestReport_Setup {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// The current state of this test report.
    pub fn status(&self) -> Option<TestReportStatus> {
        if let Some(Value::String(val)) = self.value.get("status") {
            return Some(TestReportStatus::from_string(&val).unwrap());
        }
        return None;
    }

    /// The results of the series of operations required to clean up after all the tests
    /// were executed (successfully or otherwise).
    pub fn teardown(&self) -> Option<TestReport_Teardown> {
        if let Some(val) = self.value.get("teardown") {
            return Some(TestReport_Teardown {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// A test executed from the test script.
    pub fn test(&self) -> Option<Vec<TestReport_Test>> {
        if let Some(Value::Array(val)) = self.value.get("test") {
            return Some(
                val.into_iter()
                    .map(|e| TestReport_Test {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Ideally this is an absolute URL that is used to identify the version-specific
    /// TestScript that was executed, matching the `TestScript.url`.
    pub fn test_script(&self) -> Reference {
        Reference {
            value: Cow::Borrowed(&self.value["testScript"]),
        }
    }

    /// Name of the tester producing this report (Organization or individual).
    pub fn tester(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("tester") {
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
        if let Some(_val) = self._implicit_rules() {
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
        if let Some(_val) = self._result() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._score() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._status() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._tester() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.contained() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.identifier() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.implicit_rules() {}
        if let Some(_val) = self.issued() {}
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
        if let Some(_val) = self.name() {}
        if let Some(_val) = self.participant() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.result() {}
        if let Some(_val) = self.score() {}
        if let Some(_val) = self.setup() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.status() {}
        if let Some(_val) = self.teardown() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.test() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if !self.test_script().validate() {
            return false;
        }
        if let Some(_val) = self.tester() {}
        if let Some(_val) = self.text() {
            if !_val.validate() {
                return false;
            }
        }
        return true;
    }
}

#[derive(Debug)]
pub struct TestReportBuilder {
    pub(crate) value: Value,
}

impl TestReportBuilder {
    pub fn build(&self) -> TestReport {
        TestReport {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: TestReport) -> TestReportBuilder {
        TestReportBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new(test_script: Reference) -> TestReportBuilder {
        let mut __value: Value = json!({});
        __value["testScript"] = json!(test_script.value);
        return TestReportBuilder { value: __value };
    }

    pub fn _implicit_rules<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_implicitRules"] = json!(val.value);
        return self;
    }

    pub fn _issued<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_issued"] = json!(val.value);
        return self;
    }

    pub fn _language<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_language"] = json!(val.value);
        return self;
    }

    pub fn _name<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_name"] = json!(val.value);
        return self;
    }

    pub fn _result<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_result"] = json!(val.value);
        return self;
    }

    pub fn _score<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_score"] = json!(val.value);
        return self;
    }

    pub fn _status<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_status"] = json!(val.value);
        return self;
    }

    pub fn _tester<'a>(&'a mut self, val: Element) -> &'a mut TestReportBuilder {
        self.value["_tester"] = json!(val.value);
        return self;
    }

    pub fn contained<'a>(&'a mut self, val: Vec<ResourceList>) -> &'a mut TestReportBuilder {
        self.value["contained"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestReportBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut TestReportBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn identifier<'a>(&'a mut self, val: Identifier) -> &'a mut TestReportBuilder {
        self.value["identifier"] = json!(val.value);
        return self;
    }

    pub fn implicit_rules<'a>(&'a mut self, val: &str) -> &'a mut TestReportBuilder {
        self.value["implicitRules"] = json!(val);
        return self;
    }

    pub fn issued<'a>(&'a mut self, val: &str) -> &'a mut TestReportBuilder {
        self.value["issued"] = json!(val);
        return self;
    }

    pub fn language<'a>(&'a mut self, val: &str) -> &'a mut TestReportBuilder {
        self.value["language"] = json!(val);
        return self;
    }

    pub fn meta<'a>(&'a mut self, val: Meta) -> &'a mut TestReportBuilder {
        self.value["meta"] = json!(val.value);
        return self;
    }

    pub fn modifier_extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut TestReportBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn name<'a>(&'a mut self, val: &str) -> &'a mut TestReportBuilder {
        self.value["name"] = json!(val);
        return self;
    }

    pub fn participant<'a>(
        &'a mut self,
        val: Vec<TestReport_Participant>,
    ) -> &'a mut TestReportBuilder {
        self.value["participant"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn result<'a>(&'a mut self, val: TestReportResult) -> &'a mut TestReportBuilder {
        self.value["result"] = json!(val.to_string());
        return self;
    }

    pub fn score<'a>(&'a mut self, val: f64) -> &'a mut TestReportBuilder {
        self.value["score"] = json!(val);
        return self;
    }

    pub fn setup<'a>(&'a mut self, val: TestReport_Setup) -> &'a mut TestReportBuilder {
        self.value["setup"] = json!(val.value);
        return self;
    }

    pub fn status<'a>(&'a mut self, val: TestReportStatus) -> &'a mut TestReportBuilder {
        self.value["status"] = json!(val.to_string());
        return self;
    }

    pub fn teardown<'a>(&'a mut self, val: TestReport_Teardown) -> &'a mut TestReportBuilder {
        self.value["teardown"] = json!(val.value);
        return self;
    }

    pub fn test<'a>(&'a mut self, val: Vec<TestReport_Test>) -> &'a mut TestReportBuilder {
        self.value["test"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn tester<'a>(&'a mut self, val: &str) -> &'a mut TestReportBuilder {
        self.value["tester"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: Narrative) -> &'a mut TestReportBuilder {
        self.value["text"] = json!(val.value);
        return self;
    }
}

#[derive(Debug)]
pub enum TestReportResult {
    Pass,
    Fail,
    Pending,
}

impl TestReportResult {
    pub fn from_string(string: &str) -> Option<TestReportResult> {
        match string {
            "pass" => Some(TestReportResult::Pass),
            "fail" => Some(TestReportResult::Fail),
            "pending" => Some(TestReportResult::Pending),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TestReportResult::Pass => "pass".to_string(),
            TestReportResult::Fail => "fail".to_string(),
            TestReportResult::Pending => "pending".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum TestReportStatus {
    Completed,
    InProgress,
    Waiting,
    Stopped,
    EnteredInError,
}

impl TestReportStatus {
    pub fn from_string(string: &str) -> Option<TestReportStatus> {
        match string {
            "completed" => Some(TestReportStatus::Completed),
            "in-progress" => Some(TestReportStatus::InProgress),
            "waiting" => Some(TestReportStatus::Waiting),
            "stopped" => Some(TestReportStatus::Stopped),
            "entered-in-error" => Some(TestReportStatus::EnteredInError),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            TestReportStatus::Completed => "completed".to_string(),
            TestReportStatus::InProgress => "in-progress".to_string(),
            TestReportStatus::Waiting => "waiting".to_string(),
            TestReportStatus::Stopped => "stopped".to_string(),
            TestReportStatus::EnteredInError => "entered-in-error".to_string(),
        }
    }
}
