#![allow(unused_imports, non_camel_case_types)]

use crate::model::Coding::Coding;
use crate::model::Element::Element;
use crate::model::Extension::Extension;
use crate::model::Questionnaire_AnswerOption::Questionnaire_AnswerOption;
use crate::model::Questionnaire_EnableWhen::Questionnaire_EnableWhen;
use crate::model::Questionnaire_Initial::Questionnaire_Initial;
use serde_json::value::Value;

/// A structured set of questions intended to guide the collection of answers from
/// end-users. Questionnaires provide detailed control over order, presentation,
/// phraseology and grouping to allow coherent, consistent data collection.

#[derive(Debug)]
pub struct Questionnaire_Item<'a> {
    pub value: &'a Value,
}

impl Questionnaire_Item<'_> {
    /// An indication, if true, that the item must be present in a "completed"
    /// QuestionnaireResponse.  If false, the item may be skipped when answering the
    /// questionnaire.
    pub fn required(&self) -> Option<bool> {
        if let Some(val) = self.value.get("required") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for required
    pub fn _required(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_required") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for linkId
    pub fn _link_id(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_linkId") {
            return Some(Element { value: val });
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

    /// Extensions for definition
    pub fn _definition(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_definition") {
            return Some(Element { value: val });
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

    /// An indication, if true, that the item may occur multiple times in the response,
    /// collecting multiple answers for questions or multiple sets of answers for
    /// groups.
    pub fn repeats(&self) -> Option<bool> {
        if let Some(val) = self.value.get("repeats") {
            return Some(val.as_bool().unwrap());
        }
        return None;
    }

    /// Extensions for repeats
    pub fn _repeats(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_repeats") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Text, questions and other groups to be nested beneath a question or group.
    pub fn item(&self) -> Option<Vec<Questionnaire_Item>> {
        if let Some(Value::Array(val)) = self.value.get("item") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_Item { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// One of the permitted answers for a "choice" or "open-choice" question.
    pub fn answer_option(&self) -> Option<Vec<Questionnaire_AnswerOption>> {
        if let Some(Value::Array(val)) = self.value.get("answerOption") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_AnswerOption { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// Extensions for enableBehavior
    pub fn _enable_behavior(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_enableBehavior") {
            return Some(Element { value: val });
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

    /// Extensions for text
    pub fn _text(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_text") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for readOnly
    pub fn _read_only(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_readOnly") {
            return Some(Element { value: val });
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

    /// A constraint indicating that this item should only be enabled (displayed/allow
    /// answers to be captured) when the specified condition is true.
    pub fn enable_when(&self) -> Option<Vec<Questionnaire_EnableWhen>> {
        if let Some(Value::Array(val)) = self.value.get("enableWhen") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_EnableWhen { value: e })
                    .collect::<Vec<_>>(),
            );
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

    /// Extensions for type
    pub fn _type(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_type") {
            return Some(Element { value: val });
        }
        return None;
    }

    /// Extensions for maxLength
    pub fn _max_length(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_maxLength") {
            return Some(Element { value: val });
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
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
                    .map(|e| Extension { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    /// A terminology code that corresponds to this group or question (e.g. a code from
    /// LOINC, which defines many questions and answers).
    pub fn code(&self) -> Option<Vec<Coding>> {
        if let Some(Value::Array(val)) = self.value.get("code") {
            return Some(
                val.into_iter()
                    .map(|e| Coding { value: e })
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

    /// Extensions for prefix
    pub fn _prefix(&self) -> Option<Element> {
        if let Some(val) = self.value.get("_prefix") {
            return Some(Element { value: val });
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

    /// Controls how multiple enableWhen values are interpreted -  whether all or any
    /// must be true.
    pub fn enable_behavior(&self) -> Option<Questionnaire_ItemEnableBehavior> {
        if let Some(Value::String(val)) = self.value.get("enableBehavior") {
            return Some(Questionnaire_ItemEnableBehavior::from_string(&val).unwrap());
        }
        return None;
    }

    /// One or more values that should be pre-populated in the answer when initially
    /// rendering the questionnaire for user input.
    pub fn initial(&self) -> Option<Vec<Questionnaire_Initial>> {
        if let Some(Value::Array(val)) = self.value.get("initial") {
            return Some(
                val.into_iter()
                    .map(|e| Questionnaire_Initial { value: e })
                    .collect::<Vec<_>>(),
            );
        }
        return None;
    }

    pub fn validate(&self) -> bool {
        if let Some(_val) = self.required() {}
        if let Some(_val) = self._required() {
            _val.validate();
        }
        if let Some(_val) = self._link_id() {
            _val.validate();
        }
        if let Some(_val) = self.fhir_type() {}
        if let Some(_val) = self._definition() {
            _val.validate();
        }
        if let Some(_val) = self.prefix() {}
        if let Some(_val) = self.repeats() {}
        if let Some(_val) = self._repeats() {
            _val.validate();
        }
        if let Some(_val) = self.item() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.answer_option() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self._enable_behavior() {
            _val.validate();
        }
        if let Some(_val) = self.answer_value_set() {}
        if let Some(_val) = self.definition() {}
        if let Some(_val) = self._text() {
            _val.validate();
        }
        if let Some(_val) = self._read_only() {
            _val.validate();
        }
        if let Some(_val) = self.id() {}
        if let Some(_val) = self.enable_when() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.max_length() {}
        if let Some(_val) = self._type() {
            _val.validate();
        }
        if let Some(_val) = self._max_length() {
            _val.validate();
        }
        if let Some(_val) = self.extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.read_only() {}
        if let Some(_val) = self.modifier_extension() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.code() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        if let Some(_val) = self.link_id() {}
        if let Some(_val) = self._prefix() {
            _val.validate();
        }
        if let Some(_val) = self.text() {}
        if let Some(_val) = self.enable_behavior() {}
        if let Some(_val) = self.initial() {
            _val.into_iter().for_each(|e| {
                e.validate();
            });
        }
        return true;
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
}
