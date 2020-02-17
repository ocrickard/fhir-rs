#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Questionnaire_AnswerOption::Questionnaire_AnswerOption;
use crate::model::Questionnaire_EnableWhen::Questionnaire_EnableWhen;
use crate::model::Questionnaire_Initial::Questionnaire_Initial;
use serde_json::json;
use serde_json::value::Value;
use std::borrow::Cow;

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.

#[derive(Debug)]
pub struct Questionnaire_Item<'a> {
    pub(crate) value: Cow<'a, Value>,
}

impl Questionnaire_Item<'_> {
    pub fn new(value: &Value) -> Questionnaire_Item {
        Questionnaire_Item {
            value: Cow::Borrowed(value),
        }
    }

    pub fn to_json(&self) -> Value {
        (*self.value).clone()
    }

    /// Extensions for definition
    pub fn _definition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definition") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for enableBehavior
    pub fn _enable_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_enableBehavior") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_linkId") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for maxLength
    pub fn _max_length(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxLength") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for prefix
    pub fn _prefix(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_prefix") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for readOnly
    pub fn _read_only(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_readOnly") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for repeats
    pub fn _repeats(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_repeats") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for required
    pub fn _required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_required") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element {
                value: Cow::Borrowed(val),
            });
        }
        return None;
    }

    /// One of the permitted answers for a "choice" or "open-choice" question.
    pub fn answer_option(&self) -> Option<Vec<Questionnaire_AnswerOption>> {
        if let Some(Value::Array(val)) = self.value.get("answerOption") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_AnswerOption {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A reference to a value set containing a list of codes representing permitted
    /// answers for a "choice" or "open-choice" question.
    pub fn answer_value_set(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("answerValueSet") {
            return Some(string);
        }
        return None;
    }

    /// A terminology code that corresponds to this group or question (e.g. a code from
    /// LOINC, which defines many questions and answers).
    pub fn code(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| Coding {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// This element is a URI that refers to an [[[ElementDefinition]]] that provides
    /// information about this item, including information that might otherwise be
    /// included in the instance of the Questionnaire resource. A detailed description
    /// of the construction of the URI is shown in Comments, below. If this element is
    /// present then the following element values MAY be derived from the Element
    /// Definition if the corresponding elements of this Questionnaire resource instance
    /// have no value:    * code (ElementDefinition.code)   * type
    /// (ElementDefinition.type)   * required (ElementDefinition.min)   * repeats
    /// (ElementDefinition.max)   * maxLength (ElementDefinition.maxLength)   *
    /// answerValueSet (ElementDefinition.binding)  * options
    /// (ElementDefinition.binding).
    pub fn definition(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("definition") {
            return Some(string);
        }
        return None;
    }

    /// Controls how multiple enableWhen values are interpreted -  whether all or any
    /// must be true.
    pub fn enable_behavior(&self) -> Option<Questionnaire_ItemEnableBehavior> {
        if let Some(Value::String(val)) = self.value.get("enableBehavior") {
            return Some(Questionnaire_ItemEnableBehavior::from_string(&val).unwrap());
        }
        return None;
    }

    /// A constraint indicating that this item should only be enabled (displayed/allow
    /// answers to be captured) when the specified condition is true.
    pub fn enable_when(&self) -> Option<Vec<Questionnaire_EnableWhen>> {
        if let Some(Value::Array(val)) = self.value.get("enableWhen") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_EnableWhen {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element. To make the use of extensions safe and manageable,
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

    /// Unique id for the element within a resource (for internal references). This may
    /// be any string value that does not contain spaces.
    pub fn id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("id") {
            return Some(string);
        }
        return None;
    }

    /// One or more values that should be pre-populated in the answer when initially
    /// rendering the questionnaire for user input.
    pub fn initial(&self) -> Option<Vec<Questionnaire_Initial>> {
        if let Some(Value::Array(val)) = self.value.get("initial") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_Initial {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Text, questions and other groups to be nested beneath a question or group.
    pub fn item(&self) -> Option<Vec<Questionnaire_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_Item {
                        value: Cow::Borrowed(e),
                    })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// An identifier that is unique within the Questionnaire allowing linkage to the
    /// equivalent item in a QuestionnaireResponse resource.
    pub fn link_id(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("linkId") {
            return Some(string);
        }
        return None;
    }

    /// The maximum number of characters that are permitted in the answer to be
    /// considered a "valid" QuestionnaireResponse.
    pub fn max_length(&self) -> Option<i64> {
        if let Some(val) = self.value.get("maxLength") {
            return Some(val.as_i64().unwrap());
        }
        return None;
    }

    /// May be used to represent additional information that is not part of the basic
    /// definition of the element and that modifies the understanding of the element in
    /// which it is contained and/or the understanding of the containing element's
    /// descendants. Usually modifier elements provide negation or qualification. To
    /// make the use of extensions safe and manageable, there is a strict set of
    /// governance applied to the definition and use of extensions. Though any
    /// implementer can define an extension, there is a set of requirements that SHALL
    /// be met as part of the definition of the extension. Applications processing a
    /// resource are required to check for modifier extensions.    Modifier extensions
    /// SHALL NOT change the meaning of any elements on Resource or DomainResource
    /// (including cannot change the meaning of modifierExtension itself).
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

    /// A short label for a particular group, question or set of display text within the
    /// questionnaire used for reference by the individual completing the questionnaire.
    pub fn prefix(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("prefix") {
            return Some(string);
        }
        return None;
    }

    /// An indication, when true, that the value cannot be changed by a human respondent
    /// to the Questionnaire.
    pub fn read_only(&self) -> Option<bool> {
        if let Some(val) = self.value.get("readOnly") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// An indication, if true, that the item may occur multiple times in the response,
    /// collecting multiple answers for questions or multiple sets of answers for
    /// groups.
    pub fn repeats(&self) -> Option<bool> {
        if let Some(val) = self.value.get("repeats") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// An indication, if true, that the item must be present in a "completed"
    /// QuestionnaireResponse.  If false, the item may be skipped when answering the
    /// questionnaire.
    pub fn required(&self) -> Option<bool> {
        if let Some(val) = self.value.get("required") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// The name of a section, the text of a question or text content for a display
    /// item.
    pub fn text(&self) -> Option<&str> {
        if let Some(Value::String(string)) = self.value.get("text") {
            return Some(string);
        }
        return None;
    }

    /// The type of questionnaire item this is - whether text for display, a grouping of
    /// other items or a particular type of data to be captured (string, integer, coded
    /// choice, etc.).
    pub fn fhir_type(&self) -> Option<Questionnaire_ItemType> {
        if let Some(Value::String(val)) = self.value.get("type") {
            return Some(Questionnaire_ItemType::from_string(&val).unwrap());
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self._definition() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._enable_behavior() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._link_id() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._max_length() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._prefix() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._read_only() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._repeats() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._required() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._text() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self._type() {
            if !_val.validate() {
                return false;
            }
        }
        if let Some(_val) = self.answer_option() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.answer_value_set() {}
        if let Some(_val) = self.code() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self.enable_behavior() {}
        if let Some(_val) = self.enable_when() {
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
        if let Some(_val) = self.initial() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.item() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.link_id() {}
        if let Some(_val) = self.max_length() {}
        if let Some(_val) = self.modifier_extension() {
            if !_val.into_iter().map(|e| e.validate()).all(|x| x == true) {
                return false;
            }
        }
        if let Some(_val) = self.prefix() {}
        if let Some(_val) = self.read_only() {}
        if let Some(_val) = self.repeats() {}
        if let Some(_val) = self.required() {}
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.fhir_type() {}
        return true;
    }
}

#[derive(Debug)]
pub struct Questionnaire_ItemBuilder {
    pub(crate) value: Value,
}

impl Questionnaire_ItemBuilder {
    pub fn build(&self) -> Questionnaire_Item {
        Questionnaire_Item {
            value: Cow::Owned(self.value.clone()),
        }
    }

    pub fn with(existing: Questionnaire_Item) -> Questionnaire_ItemBuilder {
        Questionnaire_ItemBuilder {
            value: (*existing.value).clone(),
        }
    }

    pub fn new() -> Questionnaire_ItemBuilder {
        let mut __value: Value = json!({});
        return Questionnaire_ItemBuilder { value: __value };
    }

    pub fn _definition<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_definition"] = json!(val.value);
        return self;
    }

    pub fn _enable_behavior<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_enableBehavior"] = json!(val.value);
        return self;
    }

    pub fn _link_id<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_linkId"] = json!(val.value);
        return self;
    }

    pub fn _max_length<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_maxLength"] = json!(val.value);
        return self;
    }

    pub fn _prefix<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_prefix"] = json!(val.value);
        return self;
    }

    pub fn _read_only<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_readOnly"] = json!(val.value);
        return self;
    }

    pub fn _repeats<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_repeats"] = json!(val.value);
        return self;
    }

    pub fn _required<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_required"] = json!(val.value);
        return self;
    }

    pub fn _text<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_text"] = json!(val.value);
        return self;
    }

    pub fn _type<'a>(&'a mut self, val: Element) -> &'a mut Questionnaire_ItemBuilder {
        self.value["_type"] = json!(val.value);
        return self;
    }

    pub fn answer_option<'a>(
        &'a mut self,
        val: Vec<Questionnaire_AnswerOption>,
    ) -> &'a mut Questionnaire_ItemBuilder {
        self.value["answerOption"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn answer_value_set<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_ItemBuilder {
        self.value["answerValueSet"] = json!(val);
        return self;
    }

    pub fn code<'a>(&'a mut self, val: Vec<Coding>) -> &'a mut Questionnaire_ItemBuilder {
        self.value["code"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn definition<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_ItemBuilder {
        self.value["definition"] = json!(val);
        return self;
    }

    pub fn enable_behavior<'a>(
        &'a mut self,
        val: Questionnaire_ItemEnableBehavior,
    ) -> &'a mut Questionnaire_ItemBuilder {
        self.value["enableBehavior"] = json!(val.to_string());
        return self;
    }

    pub fn enable_when<'a>(
        &'a mut self,
        val: Vec<Questionnaire_EnableWhen>,
    ) -> &'a mut Questionnaire_ItemBuilder {
        self.value["enableWhen"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn extension<'a>(&'a mut self, val: Vec<Extension>) -> &'a mut Questionnaire_ItemBuilder {
        self.value["extension"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn id<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_ItemBuilder {
        self.value["id"] = json!(val);
        return self;
    }

    pub fn initial<'a>(
        &'a mut self,
        val: Vec<Questionnaire_Initial>,
    ) -> &'a mut Questionnaire_ItemBuilder {
        self.value["initial"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn item<'a>(
        &'a mut self,
        val: Vec<Questionnaire_Item>,
    ) -> &'a mut Questionnaire_ItemBuilder {
        self.value["item"] = json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn link_id<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_ItemBuilder {
        self.value["linkId"] = json!(val);
        return self;
    }

    pub fn max_length<'a>(&'a mut self, val: i64) -> &'a mut Questionnaire_ItemBuilder {
        self.value["maxLength"] = json!(val);
        return self;
    }

    pub fn modifier_extension<'a>(
        &'a mut self,
        val: Vec<Extension>,
    ) -> &'a mut Questionnaire_ItemBuilder {
        self.value["modifierExtension"] =
            json!(val.into_iter().map(|e| e.value).collect::<Vec<_>>());
        return self;
    }

    pub fn prefix<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_ItemBuilder {
        self.value["prefix"] = json!(val);
        return self;
    }

    pub fn read_only<'a>(&'a mut self, val: bool) -> &'a mut Questionnaire_ItemBuilder {
        self.value["readOnly"] = json!(val);
        return self;
    }

    pub fn repeats<'a>(&'a mut self, val: bool) -> &'a mut Questionnaire_ItemBuilder {
        self.value["repeats"] = json!(val);
        return self;
    }

    pub fn required<'a>(&'a mut self, val: bool) -> &'a mut Questionnaire_ItemBuilder {
        self.value["required"] = json!(val);
        return self;
    }

    pub fn text<'a>(&'a mut self, val: &str) -> &'a mut Questionnaire_ItemBuilder {
        self.value["text"] = json!(val);
        return self;
    }

    pub fn fhir_type<'a>(
        &'a mut self,
        val: Questionnaire_ItemType,
    ) -> &'a mut Questionnaire_ItemBuilder {
        self.value["type"] = json!(val.to_string());
        return self;
    }
}

#[derive(Debug)]
pub enum Questionnaire_ItemEnableBehavior {
    All,
    Any,
}

impl Questionnaire_ItemEnableBehavior {
    pub fn from_string(string: &str) -> Option<Questionnaire_ItemEnableBehavior> {
        match string {
            "all" => Some(Questionnaire_ItemEnableBehavior::All),
            "any" => Some(Questionnaire_ItemEnableBehavior::Any),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Questionnaire_ItemEnableBehavior::All => "all".to_string(),
            Questionnaire_ItemEnableBehavior::Any => "any".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Questionnaire_ItemType {
    Group,
    Display,
    Boolean,
    Decimal,
    Integer,
    Date,
    DateTime,
    Time,
    String,
    Text,
    Url,
    Choice,
    OpenChoice,
    Attachment,
    Reference,
    Quantity,
}

impl Questionnaire_ItemType {
    pub fn from_string(string: &str) -> Option<Questionnaire_ItemType> {
        match string {
            "group" => Some(Questionnaire_ItemType::Group),
            "display" => Some(Questionnaire_ItemType::Display),
            "boolean" => Some(Questionnaire_ItemType::Boolean),
            "decimal" => Some(Questionnaire_ItemType::Decimal),
            "integer" => Some(Questionnaire_ItemType::Integer),
            "date" => Some(Questionnaire_ItemType::Date),
            "dateTime" => Some(Questionnaire_ItemType::DateTime),
            "time" => Some(Questionnaire_ItemType::Time),
            "string" => Some(Questionnaire_ItemType::String),
            "text" => Some(Questionnaire_ItemType::Text),
            "url" => Some(Questionnaire_ItemType::Url),
            "choice" => Some(Questionnaire_ItemType::Choice),
            "open-choice" => Some(Questionnaire_ItemType::OpenChoice),
            "attachment" => Some(Questionnaire_ItemType::Attachment),
            "reference" => Some(Questionnaire_ItemType::Reference),
            "quantity" => Some(Questionnaire_ItemType::Quantity),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Questionnaire_ItemType::Group => "group".to_string(),
            Questionnaire_ItemType::Display => "display".to_string(),
            Questionnaire_ItemType::Boolean => "boolean".to_string(),
            Questionnaire_ItemType::Decimal => "decimal".to_string(),
            Questionnaire_ItemType::Integer => "integer".to_string(),
            Questionnaire_ItemType::Date => "date".to_string(),
            Questionnaire_ItemType::DateTime => "dateTime".to_string(),
            Questionnaire_ItemType::Time => "time".to_string(),
            Questionnaire_ItemType::String => "string".to_string(),
            Questionnaire_ItemType::Text => "text".to_string(),
            Questionnaire_ItemType::Url => "url".to_string(),
            Questionnaire_ItemType::Choice => "choice".to_string(),
            Questionnaire_ItemType::OpenChoice => "open-choice".to_string(),
            Questionnaire_ItemType::Attachment => "attachment".to_string(),
            Questionnaire_ItemType::Reference => "reference".to_string(),
            Questionnaire_ItemType::Quantity => "quantity".to_string(),
        }
    }
}
