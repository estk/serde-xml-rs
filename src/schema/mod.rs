// default unqualified
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AttributeForm {
    Qualified,
    Unqualified,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Block {
    // #all
    Extension,
    Restriction,
    Substitution,
}
#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct URI(String);

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct ID(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum Final {
    // #all
    Extension,
    Restriction,
    List,
    Union,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct QName(String);

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    id: Option<String>,
    name: Option<String>,
    r#abstract: Option<bool>,
    r#type: Option<QName>,
    substitution_group: Option<QName>,
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,
    r#ref: Option<String>,
    #[serde(rename = "$value")]
    body: Option<Vec<ElementBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ElementBody {
    Annotation(Annotation),
    ComplexType(ComplexType),
    Key(Key),
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    name: Option<String>,
    #[serde(rename = "$value")]
    body: Option<Vec<KeyBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum KeyBody {
    Selector { xpath: String },
    Field { xpath: String },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum MaxOccurs {
    Bounded(u32),
    Unbounded(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SimpleType {
    name: Option<QName>,
    #[serde(rename = "$value")]
    body: Option<Vec<SimpleBody>>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SimpleBody {
    Annotation(Annotation),
    Documentation(Documentation),
    Restriction(Restriction),
    Union(Union),
    List(List),
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct List {
    item_type: Option<QName>,
    #[serde(rename = "$value")]
    body: Option<Vec<SimpleType>>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Union {
    member_types: Option<String>,
    #[serde(rename = "$value")]
    body: Option<Vec<SimpleType>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComplexType {
    name: Option<QName>,
    mixed: Option<bool>,
    r#abstract: Option<bool>,
    r#type: Option<QName>,
    #[serde(rename = "$value")]
    body: Option<Vec<ComplexBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ComplexBody {
    Annotation(Annotation),
    Sequence(Sequence),
    Attribute(Attribute),
    AnyAttribute(AnyAttribute),
    ComplexContent(ComplexContent),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct ComplexContent {
    #[serde(rename = "$value")]
    restriction: Option<Vec<ComplexContentBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ComplexContentBody {
    Restriction(Restriction),
    Extension(Extension),
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Extension {
    base: QName,
    #[serde(rename = "$value")]
    body: Option<Vec<ExtensionBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ExtensionBody {
    Group(Group),
    Attribute(Attribute),
    AttributeGroup(AttributeGroup),
    Sequence(Sequence),
    Choice(Choice),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Restriction {
    base: String,
    #[serde(rename = "$value")]
    body: Option<Vec<RestrictionBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum RestrictionBody {
    AnyAttribute(AnyAttribute),
    MinLength { value: u32 },
    Enumeration(Enumeration),
    Sequence(Sequence),
    Attribute(Attribute),
    Group(Group),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Enumeration {
    value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AnyAttribute {
    namespace: Option<String>,
    process_contents: Option<String>,

    #[serde(rename = "$value")]
    body: Option<Vec<Element>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Attribute {
    name: Option<String>,
    r#type: Option<QName>,
    r#use: Option<String>,
    r#ref: Option<String>,
    default: Option<String>,
    fixed: Option<String>,

    #[serde(rename = "$value")]
    body: Option<Vec<SimpleType>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SequenceBody {
    Any(Any),
    Annotation(Annotation),
    Element(Element),
    Group(Group),
    Sequence(Sequence),
    Choice(Choice),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Any {
    namespace: Option<String>,
    process_contents: Option<String>,
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,
    #[serde(rename = "$value")]
    body: Option<Vec<SequenceBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Import {
    namespace: URI,
    schema_location: URI,
    #[serde(rename = "$value")]
    body: Option<Vec<Annotation>>,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum SchemaBody {
    Include,
    Import(Import),
    Redefine,
    Override,
    Annotation(Annotation),
    DefaultOpenContent,
    SimpleType(SimpleType),
    ComplexType(ComplexType),
    Group(Group),
    AttributeGroup(AttributeGroup),
    Element(Element),
    Attribute,
    Notation(Notation),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Notation {
    name: String,
    public: String,
    system: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct AttributeGroup {
    name: Option<String>,
    r#ref: Option<String>,
    #[serde(rename = "$value")]
    body: Option<Vec<AttributeGroupBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AttributeGroupBody {
    Annotation(Annotation),
    Attribute(Attribute),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Group {
    name: Option<String>,
    r#ref: Option<String>,
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,
    #[serde(rename = "$value")]
    body: Option<Vec<GroupBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum GroupBody {
    Annotation(Annotation),
    Choice(Choice),
    Sequence(Sequence),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Choice {
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,
    #[serde(rename = "$value")]
    body: Option<Vec<ChoiceBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum ChoiceBody {
    Any(Any),
    Annotation(Annotation),
    Element(Element),
    Group(Group),
    Sequence(Sequence),
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub struct Documentation(String);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub enum AnnotationBody {
    AppInfo,
    Documentation(Documentation),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    namespace: Option<String>,
    #[serde(rename = "$value")]
    body: Option<Vec<AnnotationBody>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Schema {
    // default unqualified
    attribute_form_default: Option<AttributeForm>,
    // default empty
    // could be #all, which would block all
    block_default: Option<Vec<Block>>,
    // TODO
    default_attributes: Option<String>,
    // default ##local
    // ##defaultNamespace | ##targetNamespace | ##local)
    xpath_default_namespace: Option<URI>,
    // default unqualified
    element_form_default: Option<AttributeForm>,
    // default empty
    final_default: Option<Vec<Final>>,
    id: Option<ID>,
    target_namespace: Option<URI>,
    // TODO
    version: Option<String>,
    // TODO
    #[serde(rename = "lang")]
    xml_lang: Option<String>,

    #[serde(rename = "$value")]
    body: Option<Vec<SchemaBody>>,
}
