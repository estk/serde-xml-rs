use heck::*;
use if_chain::if_chain;
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use std::str::FromStr;
use syn::{self, Ident};

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
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
#[serde(transparent)]
pub struct QName(String);

/// This corresponds to a field of a struct
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

impl Element {
    fn get_doc(&self) -> Option<String> {
        let es = self.body.as_ref()?;
        let mut documentation = None;

        // TODO clean this up with macro
        for e in es {
            if let ElementBody::Annotation(a) = e {
                if let Some(es) = a.body.as_ref() {
                    for e in es {
                        if let AnnotationBody::Documentation(doc) = e {
                            documentation = Some(doc.0.clone());
                        }
                    }
                }
            }
        }
        documentation
    }
}

impl CodeGenerator for Element {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        unimplemented!()
    }
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
impl CodeGenerator for SimpleType {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        unimplemented!()
    }
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

impl CodeGenerator for ComplexType {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let name = Ident::new(
            &self.name.as_ref().unwrap().0.to_camel_case(),
            Span::call_site(),
        );

        debug!("Building complex type: {}", name);

        let sequence: Vec<&Sequence> = self
            .body
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|e| match e {
                ComplexBody::Sequence(s) => Some(s),
                _ => None,
            })
            .collect();

        if sequence.is_empty() {
            quote!(
                struct #name {}
            )
        } else {
            let mut body_ctx = ctx.with_name(&format!("{}Body", name));
            let body = sequence.first().unwrap().codegen(&mut body_ctx);
            let defs = body_ctx.defs;

            quote!(
                struct #name {
                    body: #body,
                }
                #defs
            )
        }
    }
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
    MinInclusive { value: f32 },
    MaxInclusive { value: f32 },
    MinExclusive { value: f32 },
    MaxExclusive { value: f32 },
    MinLength { value: f32 },
    MaxLength { value: f32 },
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
pub struct Any {
    namespace: Option<String>,
    process_contents: Option<String>,
    min_occurs: Option<u32>,
    #[serde(default = Some(MaxOccurs::Bounded(1)))]
    max_occurs: Option<MaxOccurs>,

    #[serde(rename = "$value")]
    body: Option<Vec<Annotation>>,
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

impl CodeGenerator for Sequence {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let name = Ident::new(
            &ctx.name.as_ref().unwrap().to_camel_case(),
            Span::call_site(),
        );
        let variants: Vec<TokenStream> = self
            .body
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|e| match e {
                SequenceBody::Element(e) => Some(e),
                _ => None,
            })
            .map(|e| {
                let name = Ident::new(&e.name.as_ref().unwrap().to_camel_case(), Span::call_site());
                let mut qty_name_path = e
                    .r#type
                    .as_ref()
                    .unwrap()
                    .0
                    .split(':')
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>();

                qty_name_path.last_mut().map(|x| *x = x.to_camel_case());
                let ty_path = qty_name_path.join("::");

                let ty_name: syn::TypePath = syn::parse_str(&ty_path).unwrap();
                // let ty_name = Ident::new(&ty_path, Span::call_site());
                quote!(
                    #name(#ty_name)
                )
            })
            .collect();

        let mut variant_stream = TokenStream::new();
        for v in variants {
            variant_stream = quote!(
                #variant_stream
                #v,
            )
        }

        let companion_type = quote!(
            enum #name {
                #variant_stream
            }
        );
        ctx.defs.append_all(companion_type);
        quote!(
            Vec<#name>
        )
    }
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
pub struct Import {
    namespace: URI,
    schema_location: URI,
    #[serde(rename = "$value")]
    body: Option<Vec<Annotation>>,
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
impl CodeGenerator for SchemaBody {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let mut ts = TokenStream::new();
        let body = match self {
            Self::Include => TokenStream::new(),
            Self::Import(i) => TokenStream::new(),
            Self::Redefine => TokenStream::new(),
            Self::Override => TokenStream::new(),
            Self::Annotation(i) => TokenStream::new(),
            Self::DefaultOpenContent => TokenStream::new(),
            Self::SimpleType(i) => i.codegen(ctx),
            Self::ComplexType(i) => i.codegen(ctx),
            Self::Group(i) => TokenStream::new(),
            Self::AttributeGroup(i) => TokenStream::new(),
            Self::Element(i) => i.codegen(ctx),
            Self::Attribute => TokenStream::new(),
            Self::Notation(i) => TokenStream::new(),
        };
        ts.append_all(body);
        ts
    }
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
pub trait CodeGenerator {
    fn codegen(&self, ctx: &mut Context) -> TokenStream;
}

#[derive(Clone)]
pub struct Context {
    name: Option<String>,
    defs: TokenStream,
}
impl Context {
    pub fn with_name(&self, name: &str) -> Self {
        let mut me = self.clone();
        me.name.replace(name.to_string());
        me
    }
}

impl Default for Context {
    fn default() -> Self {
        Context {
            name: Some("".to_string()),
            defs: TokenStream::new(),
        }
    }
}

impl CodeGenerator for Schema {
    fn codegen(&self, ctx: &mut Context) -> TokenStream {
        let mut root = None;
        // let simple_types = vec![];
        let mut complex_types = vec![];
        if let Some(body) = self.body.as_ref() {
            for item in body {
                match item {
                    SchemaBody::Element(x) => root = Some(x),
                    SchemaBody::ComplexType(x) => complex_types.push(x),
                    _ => (),
                }
            }
        }

        let root = root.unwrap();
        let root_name_str = root.name.as_ref().unwrap();
        let root_name = Ident::new(&root_name_str, Span::call_site());
        let root_type_name_str = root.r#type.as_ref().unwrap().0.clone();
        let root_type_name = Ident::new(&root_type_name_str.to_camel_case(), Span::call_site());
        let root_doc_str = format!("/// {}", root.get_doc().unwrap());
        let root_doc = TokenStream::from_str(&root_doc_str).unwrap();
        let root_rename = quote!(
            #[serde(rename = #root_name_str)]
        );

        let mut defs = TokenStream::new();
        let mut root_ty = None;
        for t in complex_types {
            println!("ty name: {}", t.name.as_ref().unwrap().0);
            println!("ty search: {}", root_type_name_str);
            if_chain! {
                if let Some(n) = t.name.as_ref();
                if n.0 == root_type_name_str;
                then {
                    root_ty = Some(t);
                } else {
                    defs.append_all(t.codegen(ctx));
                }
            };
        }
        let root_def = root_ty.unwrap().codegen(ctx);
        quote!(
            mod #root_name {
                #root_doc
                #root_rename
                #root_def
                #defs
            }
        )
    }
}
