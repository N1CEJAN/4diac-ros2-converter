use std::fs::File;
use std::string::ToString;

use xmltree::{Element, EmitterConfig, XMLNode};

use crate::business::error::Result;
use crate::core::dtp::*;

pub fn write(data_type: DataType, to_directory: &str) -> Result<()> {
    let data_type_name = data_type.name().to_string();
    let custom_data_type_element = create_data_type_element(data_type);

    let mut config = EmitterConfig::new();
    config.perform_escaping = false;
    config.perform_indent = true;
    let file = File::create(format!("{to_directory}{data_type_name}.dtp"))?;
    custom_data_type_element.write_with_config(file, config)?;
    Ok(())
}

fn create_data_type_element(data_type: DataType) -> Element {
    let mut data_type_element = Element::new(XML_TAG_DATA_TYPE);
    data_type_element
        .attributes
        .insert(XML_ATTRIBUTE_NAME.to_string(), data_type.name().to_string());
    if let Some(comment) = data_type.comment() {
        data_type_element
            .attributes
            .insert(XML_ATTRIBUTE_COMMENT.to_string(), comment.clone());
    }
    data_type_element
        .children
        .push(match data_type.data_type_kind() {
            DataTypeKind::StructuredType(structured_type) => {
                create_structured_type_element(structured_type)
            }
        });
    data_type_element
}

fn create_structured_type_element(structured_type: &StructuredType) -> XMLNode {
    let mut structured_type_element = Element::new(XML_TAG_STRUCTURED_TYPE);
    if let Some(comment) = structured_type.comment() {
        structured_type_element
            .attributes
            .insert(XML_ATTRIBUTE_COMMENT.to_string(), comment.clone());
    }
    structured_type_element.children.append(
        &mut structured_type
            .children()
            .iter()
            .map(|structured_type_child| match structured_type_child {
                StructuredTypeChild::VarDeclaration(var_declaration) => {
                    create_var_declaration_element(var_declaration)
                }
            })
            .collect(),
    );
    XMLNode::Element(structured_type_element)
}

fn create_var_declaration_element(var_declaration: &VarDeclaration) -> XMLNode {
    let mut var_declaration_element = Element::new(XML_TAG_VAR_DECLARATION);
    var_declaration_element.attributes.insert(
        XML_ATTRIBUTE_NAME.to_string(),
        var_declaration.name().to_string(),
    );
    var_declaration_element.attributes.insert(
        XML_ATTRIBUTE_TYPE.to_string(),
        base_type_to_string(var_declaration.base_type()),
    );
    if let Some(array_size) = var_declaration.array_size() {
        var_declaration_element.attributes.insert(
            XML_ATTRIBUTE_ARRAY_SIZE.to_string(),
            match array_size {
                ArraySize::Dynamic => String::from('*'),
                ArraySize::Static(Capacity::InPlace(capacity)) => format!("0..{}", capacity - 1),
                ArraySize::Static(Capacity::Shifted(start, end)) => format!("{start}..{end}"),
            },
        );
    }
    if let Some(initial_value) = var_declaration.initial_value() {
        var_declaration_element.attributes.insert(
            XML_ATTRIBUTE_INITIAL_VALUE.to_string(),
            initial_value_to_string(initial_value),
        );
    }
    if let Some(comment) = var_declaration.comment() {
        var_declaration_element
            .attributes
            .insert(XML_ATTRIBUTE_COMMENT.to_string(), comment.clone());
    }
    XMLNode::Element(var_declaration_element)
}

fn base_type_to_string(base_type: &BaseType) -> String {
    match base_type {
        BaseType::BOOL => "BOOL".to_string(),
        BaseType::SINT => "SINT".to_string(),
        BaseType::INT => "INT".to_string(),
        BaseType::DINT => "DINT".to_string(),
        BaseType::LINT => "LINT".to_string(),
        BaseType::USINT => "USINT".to_string(),
        BaseType::UINT => "UINT".to_string(),
        BaseType::UDINT => "UDINT".to_string(),
        BaseType::ULINT => "ULINT".to_string(),
        BaseType::REAL => "REAL".to_string(),
        BaseType::LREAL => "LREAL".to_string(),
        BaseType::BYTE => "BYTE".to_string(),
        BaseType::WORD => "WORD".to_string(),
        BaseType::DWORD => "DWORD".to_string(),
        BaseType::LWORD => "LWORD".to_string(),
        BaseType::CHAR => "CHAR".to_string(),
        BaseType::STRING => "STRING".to_string(),
        BaseType::WSTRING => "WSTRING".to_string(),
        BaseType::TIME => "TIME".to_string(),
        BaseType::DATE => "DATE".to_string(),
        BaseType::TIME_OF_DAY => "TIME_OF_DAY".to_string(),
        BaseType::TOD => "TOD".to_string(),
        BaseType::DATE_AND_TIME => "DATE_AND_TIME".to_string(),
        BaseType::DT => "DT".to_string(),
        BaseType::Custom(type_name) => type_name.clone(),
    }
}

fn initial_value_to_string(initial_value: &InitialValue) -> String {
    match initial_value {
        InitialValue::BOOL(bool) => {
            if *bool {
                "TRUE".to_string()
            } else {
                "FALSE".to_string()
            }
        }
        InitialValue::SINT(value) => int_literal_as_string(value),
        InitialValue::INT(value) => value.to_string(),
        InitialValue::DINT(value) => value.to_string(),
        InitialValue::LINT(value) => value.to_string(),
        InitialValue::USINT(value) => value.to_string(),
        InitialValue::UINT(value) => value.to_string(),
        InitialValue::UDINT(value) => value.to_string(),
        InitialValue::ULINT(value) => value.to_string(),
        InitialValue::REAL(value) => value.to_string(),
        InitialValue::LREAL(value) => value.to_string(),
        InitialValue::BYTE(value) => format!("16#{:02X}", value),
        InitialValue::WORD(value) => format!("16#{:04X}", value),
        InitialValue::DWORD(value) => format!("16#{:08X}", value),
        InitialValue::LWORD(value) => format!("16#{:016X}", value),
        InitialValue::CHAR(value) => format!("'${:02X}'", value),
        InitialValue::STRING(value) => format!("'{value}'"),
        InitialValue::WSTRING(value) => format!("&quot;{value}&quot;"),
        InitialValue::TIME(value) => value.to_string(),
        InitialValue::DATE(value) => value.to_string(),
        InitialValue::TIME_OF_DAY(value) => value.to_string(),
        InitialValue::TOD(value) => value.to_string(),
        InitialValue::DATE_AND_TIME(value) => value.to_string(),
        InitialValue::DT(value) => value.to_string(),
        InitialValue::Array(values) => array_of_initial_values_as_string(values),
    }
}

fn int_literal_as_string(int_literal: &IntLiteral) -> String {
    let mut string = String::new();
    if let Some(type_name) = int_literal.int_type.as_ref() {
        string.push_str(match type_name {
            IntTypeName::SignedIntTypeName(type_name) => match type_name {
                SignedIntTypeName::SINT => "SINT#",
                SignedIntTypeName::INT => "INT#",
                SignedIntTypeName::DINT => "DINT#",
                SignedIntTypeName::LINT => "LINT#",
            },
            IntTypeName::UnsignedIntTypeName(type_name) => match type_name {
                UnsignedIntTypeName::USINT => "USINT#",
                UnsignedIntTypeName::UINT => "UINT#",
                UnsignedIntTypeName::UDINT => "UDINT#",
                UnsignedIntTypeName::ULINT => "ULINT#",
            },
        })
    }
    string.push_str(
        &(match &int_literal.e_int_literal {
            EIntLiteral::DecimalInt => format!("{}", int_literal.value),
            EIntLiteral::BinaryInt => format!("2#{}", int_literal.value),
            EIntLiteral::OctalInt => format!("8#{}", int_literal.value),
            EIntLiteral::HexalInt => format!("16#{}", int_literal.value),
        }),
    );
    string
}

fn array_of_initial_values_as_string(values: &[InitialValue]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(initial_value_to_string)
            .collect::<Vec<String>>()
            .join(", ")
    )
}
