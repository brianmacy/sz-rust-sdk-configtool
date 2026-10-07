//! Coverage-closing tests for the feature / element / attribute / data-source
//! modules plus the shared error, filter and export surfaces.
//!
//! Every test runs the real library against the real Senzing v4 template
//! (`tests/fixtures/g2config_template.json`) or a minimal inline config, and every
//! error path asserts the exact [`SzErrorKind`] (and the message where stable).

use serde_json::{Value, json};
use sz_configtool_lib::attributes::{
    AddAttributeParams, SetAttributeParams, add_attribute, get_attribute, list_attributes,
    set_attribute,
};
use sz_configtool_lib::datasources::{
    AddDataSourceParams, SetDataSourceParams, add_data_source, delete_data_source, get_data_source,
    list_data_sources, set_data_source,
};
use sz_configtool_lib::elements::{
    AddElementParams, AddElementToFeatureParams, SetElementParams, SetFeatureElementParams,
    add_element, add_element_to_feature, delete_element, delete_element_from_feature, get_element,
    list_elements, set_element, set_feature_element,
};
use sz_configtool_lib::features::{
    AddFeatureComparisonParams, AddFeatureDistinctCallElementParams, AddFeatureParams,
    GetFeatureComparisonParams, SetFeatureParams, add_feature, add_feature_comparison,
    add_feature_comparison_element, add_feature_distinct_call_element, build_feature_json,
    delete_feature, delete_feature_comparison, delete_feature_comparison_element, get_feature,
    get_feature_class, get_feature_comparison, list_feature_classes, list_feature_comparisons,
    list_features, set_feature, update_feature_version,
};
use sz_configtool_lib::filter::to_python_repr_string;
use sz_configtool_lib::{
    SzConfigError, SzErrorKind, ValidationFailure, ValidationReason, render_config,
};

const BAD_JSON: &str = "not json";
const EMPTY_G2: &str = r#"{"G2_CONFIG": {}}"#;

/// Load the real Senzing v4 template fixture.
fn template() -> String {
    let path = format!(
        "{}/tests/fixtures/g2config_template.json",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).expect("template fixture readable")
}

fn g2(config: &str) -> Value {
    serde_json::from_str::<Value>(config).expect("valid JSON")["G2_CONFIG"].clone()
}

/// Unwrap an error, failing the test if the call unexpectedly succeeded.
fn err_of<T: std::fmt::Debug>(r: Result<T, SzConfigError>) -> SzConfigError {
    r.expect_err("expected an error")
}

/// Assert the exact error kind and, when given, the exact bare message.
fn assert_err<T: std::fmt::Debug>(
    r: Result<T, SzConfigError>,
    kind: SzErrorKind,
    msg: Option<&str>,
) {
    let e = err_of(r);
    assert_eq!(e.kind(), kind, "unexpected error: {e:?}");
    if let Some(m) = msg {
        assert_eq!(e.message(), m);
    }
}

fn assert_json_parse<T: std::fmt::Debug>(r: Result<T, SzConfigError>) {
    assert_err(r, SzErrorKind::JsonParse, None);
}

fn assert_missing_section<T: std::fmt::Debug>(r: Result<T, SzConfigError>, section: &str) {
    assert_err(r, SzErrorKind::MissingSection, Some(section));
}

// ============================================================================
// error.rs
// ============================================================================

#[test]
fn test_error_constructors_and_display() {
    let cases = [
        (
            SzConfigError::json_parse("bad"),
            SzErrorKind::JsonParse,
            "JSON parse error: bad",
        ),
        (SzConfigError::not_on_call("x"), SzErrorKind::NotOnCall, "x"),
        (
            SzConfigError::not_in_feature("y"),
            SzErrorKind::NotInFeature,
            "y",
        ),
        (
            SzConfigError::already_present("z"),
            SzErrorKind::AlreadyPresent,
            "z",
        ),
        (
            SzConfigError::not_implemented("w"),
            SzErrorKind::NotImplemented,
            "Not implemented: w",
        ),
        (
            SzConfigError::MissingSection("CFG_X".into()),
            SzErrorKind::MissingSection,
            "Missing config section: CFG_X",
        ),
    ];
    for (err, kind, display) in cases {
        assert_eq!(err.kind(), kind);
        assert_eq!(err.to_string(), display);
    }
    assert_eq!(SzConfigError::not_in_feature("bare").message(), "bare");
}

#[test]
fn test_error_validation_display_without_offending_value() {
    let err = SzConfigError::ValidationErrors(vec![
        ValidationFailure::new("behavior", ValidationReason::Missing, None),
        ValidationFailure::new(
            "sendToRedo",
            ValidationReason::OutOfDomain,
            Some("x".into()),
        ),
    ]);
    assert_eq!(
        err.to_string(),
        "Invalid input: behavior MISSING; sendToRedo OUT_OF_DOMAIN 'x'"
    );
}

#[test]
fn test_error_from_serde_json_error() {
    let serde_err = serde_json::from_str::<Value>(BAD_JSON).unwrap_err();
    let expected = serde_err.to_string();
    let err: SzConfigError = serde_err.into();
    assert_eq!(err.kind(), SzErrorKind::JsonParse);
    assert_eq!(err.message(), expected);
}

// ============================================================================
// filter.rs / export.rs
// ============================================================================

#[test]
fn test_python_repr_escapes_control_characters() {
    let v = json!("a\\b\nc\rd\te");
    assert_eq!(to_python_repr_string(&v), r"'a\\b\nc\rd\te'");
}

#[test]
fn test_render_config_rejects_bad_json() {
    assert_json_parse(render_config(BAD_JSON, 2));
}

// ============================================================================
// datasources.rs
// ============================================================================

#[test]
fn test_datasource_params_builders_and_try_from() {
    let p = AddDataSourceParams {
        code: "X",
        ..Default::default()
    }
    .with_id(1500);
    assert_eq!(p.id, Some(1500));

    let j = json!({"code": "CUST", "retentionLevel": "Forget", "id": 1234});
    let p = AddDataSourceParams::try_from(&j).unwrap();
    assert_eq!(
        (p.code, p.retention_level, p.id),
        ("CUST", Some("Forget"), Some(1234))
    );
    let p = SetDataSourceParams::try_from(&j).unwrap();
    assert_eq!((p.code, p.retention_level), ("CUST", Some("Forget")));

    let missing = json!({});
    assert_err(
        AddDataSourceParams::try_from(&missing),
        SzErrorKind::MissingField,
        Some("code"),
    );
    assert_err(
        SetDataSourceParams::try_from(&missing),
        SzErrorKind::MissingField,
        Some("code"),
    );
}

#[test]
fn test_add_data_source_paths() {
    let cfg = template();
    let p = |code| AddDataSourceParams {
        code,
        ..Default::default()
    };
    assert_json_parse(add_data_source(BAD_JSON, p("A")));
    assert_missing_section(add_data_source(EMPTY_G2, p("A")), "CFG_DSRC");
    assert_err(
        add_data_source(&cfg, p("test")),
        SzErrorKind::AlreadyExists,
        Some("Data source already exists: TEST"),
    );
    assert_err(
        add_data_source(
            &cfg,
            AddDataSourceParams {
                retention_level: Some("sometimes"),
                ..p("NEWDS")
            },
        ),
        SzErrorKind::InvalidInput,
        Some("Invalid RETENTIONLEVEL value 'sometimes'. Must be 'Remember' or 'Forget'"),
    );
    let out = add_data_source(
        &cfg,
        AddDataSourceParams {
            retention_level: Some("remember"),
            ..p("NEWDS")
        },
    )
    .unwrap();
    assert_eq!(
        get_data_source(&out, "NEWDS").unwrap()["RETENTION_LEVEL"],
        json!("Remember")
    );
}

#[test]
fn test_delete_get_list_data_source_errors() {
    let cfg = template();
    assert_json_parse(delete_data_source(BAD_JSON, "A"));
    assert_missing_section(delete_data_source(EMPTY_G2, "A"), "CFG_DSRC");
    assert_err(
        delete_data_source(&cfg, "nope"),
        SzErrorKind::NotFound,
        Some("Data source not found: NOPE"),
    );
    assert_err(
        delete_data_source(&cfg, "TEST"),
        SzErrorKind::InvalidInput,
        Some("The TEST data source cannot be deleted"),
    );
    assert_json_parse(get_data_source(BAD_JSON, "A"));
    assert_missing_section(get_data_source(EMPTY_G2, "A"), "CFG_DSRC");
    assert_missing_section(list_data_sources(EMPTY_G2), "CFG_DSRC");
}

#[test]
fn test_set_data_source_paths() {
    let cfg = template();
    let p = |code, retention_level| SetDataSourceParams {
        code,
        retention_level,
    };
    assert_json_parse(set_data_source(BAD_JSON, p("TEST", None)));
    assert_missing_section(set_data_source(EMPTY_G2, p("TEST", None)), "CFG_DSRC");
    assert_err(
        set_data_source(&cfg, p("nope", None)),
        SzErrorKind::NotFound,
        Some("Data source not found: NOPE"),
    );
    let out = set_data_source(&cfg, p("test", Some("Forget"))).unwrap();
    assert_eq!(
        get_data_source(&out, "TEST").unwrap()["RETENTION_LEVEL"],
        json!("Forget")
    );
    // No retention supplied: config is unchanged.
    let out = set_data_source(&cfg, p("TEST", None)).unwrap();
    assert_eq!(g2(&out)["CFG_DSRC"], g2(&cfg)["CFG_DSRC"]);
}

// ============================================================================
// attributes.rs
// ============================================================================

#[test]
fn test_attribute_params_try_from() {
    let full = json!({
        "attribute": "A", "feature": "F", "element": "E", "class": "OTHER",
        "default": "d", "internal": "Yes", "required": "No", "id": 7
    });
    let p = AddAttributeParams::try_from(&full).unwrap();
    assert_eq!(
        (p.attribute, p.feature, p.element, p.class),
        ("A", "F", "E", "OTHER")
    );
    assert_eq!(
        (p.default_value, p.internal, p.required, p.id),
        (Some("d"), Some("Yes"), Some("No"), Some(7))
    );
    let s = SetAttributeParams::try_from(&full).unwrap();
    assert_eq!(
        (s.attribute, s.internal, s.required, s.default_value),
        ("A", Some("Yes"), Some("No"), Some("d"))
    );

    for field in ["attribute", "feature", "element", "class"] {
        let mut j = full.clone();
        j.as_object_mut().unwrap().remove(field);
        assert_err(
            AddAttributeParams::try_from(&j),
            SzErrorKind::MissingField,
            Some(field),
        );
    }
    assert_err(
        SetAttributeParams::try_from(&json!({})),
        SzErrorKind::MissingField,
        Some("attribute"),
    );
}

fn attr_params<'a>(
    attribute: &'a str,
    feature: &'a str,
    element: &'a str,
) -> AddAttributeParams<'a> {
    AddAttributeParams {
        attribute,
        feature,
        element,
        class: "IDENTIFIER",
        default_value: None,
        internal: None,
        required: None,
        id: None,
    }
}

#[test]
fn test_add_attribute_error_paths() {
    let cfg = template();
    let good = || attr_params("MY_ATTR", "EMAIL", "ADDR");
    assert_json_parse(add_attribute(BAD_JSON, good()));
    assert_err(
        add_attribute(
            &cfg,
            AddAttributeParams {
                class: "BOGUS",
                ..good()
            },
        ),
        SzErrorKind::InvalidInput,
        Some(
            "Invalid attribute class 'BOGUS'. Must be one of: NAME, ATTRIBUTE, IDENTIFIER, \
             ADDRESS, PHONE, RELATIONSHIP, OTHER",
        ),
    );
    assert_missing_section(add_attribute(EMPTY_G2, good()), "CFG_ATTR");
    assert_err(
        add_attribute(&cfg, attr_params("email_address", "EMAIL", "ADDR")),
        SzErrorKind::AlreadyExists,
        Some("Attribute: EMAIL_ADDRESS"),
    );
    assert_err(
        add_attribute(&cfg, attr_params("MY_ATTR", "NO_FEAT", "ADDR")),
        SzErrorKind::NotFound,
        Some("Feature 'NO_FEAT' not found"),
    );
    assert_err(
        add_attribute(&cfg, attr_params("MY_ATTR", "EMAIL", "NO_ELEM")),
        SzErrorKind::NotFound,
        Some("Element 'NO_ELEM' not found"),
    );
    assert_err(
        add_attribute(
            &cfg,
            AddAttributeParams {
                required: Some("maybe"),
                ..good()
            },
        ),
        SzErrorKind::InvalidInput,
        Some("Invalid REQUIRED value 'maybe'. Must be one of: Yes, No, Any, Desired"),
    );
    assert_err(
        add_attribute(
            &cfg,
            AddAttributeParams {
                internal: Some("maybe"),
                ..good()
            },
        ),
        SzErrorKind::InvalidInput,
        Some("Invalid INTERNAL value 'maybe'. Must be 'Yes' or 'No'"),
    );
}

#[test]
fn test_add_attribute_required_domain_normalized() {
    let cfg = template();
    for (input, expected) in [("yes", "Yes"), ("any", "Any"), ("desired", "Desired")] {
        let (_, attr) = add_attribute(
            &cfg,
            AddAttributeParams {
                required: Some(input),
                ..attr_params("MY_ATTR", "EMAIL", "ADDR")
            },
        )
        .unwrap();
        assert_eq!(attr["FELEM_REQ"], json!(expected));
    }
}

#[test]
fn test_get_and_list_attributes() {
    let cfg = template();
    assert_eq!(
        get_attribute(&cfg, "email_address").unwrap()["ATTR_ID"],
        json!(1511)
    );
    assert_json_parse(get_attribute(BAD_JSON, "X"));
    assert_missing_section(get_attribute(EMPTY_G2, "X"), "CFG_ATTR");
    assert_err(
        get_attribute(&cfg, "nope"),
        SzErrorKind::NotFound,
        Some("Attribute not found: NOPE"),
    );

    let list = list_attributes(&cfg).unwrap();
    let email = list
        .iter()
        .find(|a| a["attribute"] == json!("EMAIL_ADDRESS"))
        .unwrap();
    assert_eq!(
        email,
        &json!({"id": 1511, "attribute": "EMAIL_ADDRESS", "class": "IDENTIFIER",
                "feature": "EMAIL", "element": "ADDR", "required": "Yes",
                "default": null, "internal": "No"})
    );
    // A row with every column absent renders the documented defaults.
    let sparse = r#"{"G2_CONFIG": {"CFG_ATTR": [{}]}}"#;
    assert_eq!(
        list_attributes(sparse).unwrap(),
        vec![
            json!({"id": 0, "attribute": "", "class": "", "feature": null,
                    "element": null, "required": "", "default": null, "internal": ""})
        ]
    );
    assert_json_parse(list_attributes(BAD_JSON));
    assert_missing_section(list_attributes(EMPTY_G2), "CFG_ATTR");
}

#[test]
fn test_set_attribute_paths() {
    let cfg = template();
    let p = |internal, required, default_value| SetAttributeParams {
        attribute: "email_address",
        internal,
        required,
        default_value,
    };
    assert_json_parse(set_attribute(BAD_JSON, p(None, None, None)));
    assert_missing_section(set_attribute(EMPTY_G2, p(None, None, None)), "CFG_ATTR");
    assert_err(
        set_attribute(
            &cfg,
            SetAttributeParams {
                attribute: "nope",
                ..Default::default()
            },
        ),
        SzErrorKind::NotFound,
        Some("Attribute not found: NOPE"),
    );
    assert_err(
        set_attribute(&cfg, p(Some("bad"), None, None)),
        SzErrorKind::InvalidInput,
        Some("Invalid INTERNAL value 'bad'. Must be 'Yes' or 'No'"),
    );
    assert_err(
        set_attribute(&cfg, p(None, Some("bad"), None)),
        SzErrorKind::InvalidInput,
        Some("Invalid REQUIRED value 'bad'. Must be one of: Yes, No, Any, Desired"),
    );
    let cases = [
        ("yes", "yes", "Yes", "Yes"),
        ("no", "no", "No", "No"),
        ("NO", "any", "No", "Any"),
        ("YES", "desired", "Yes", "Desired"),
    ];
    // Only INTERNAL supplied: REQUIRED and DEFAULT_VALUE are left untouched.
    let out = set_attribute(&cfg, p(Some("yes"), None, None)).unwrap();
    let attr = get_attribute(&out, "EMAIL_ADDRESS").unwrap();
    assert_eq!(attr["INTERNAL"], json!("Yes"));
    assert_eq!(attr["FELEM_REQ"], json!("Yes"));
    assert_eq!(attr["DEFAULT_VALUE"], json!(null));
    for (internal, required, exp_internal, exp_required) in cases {
        let out = set_attribute(&cfg, p(Some(internal), Some(required), Some("dflt"))).unwrap();
        let attr = get_attribute(&out, "EMAIL_ADDRESS").unwrap();
        assert_eq!(attr["INTERNAL"], json!(exp_internal));
        assert_eq!(attr["FELEM_REQ"], json!(exp_required));
        assert_eq!(attr["DEFAULT_VALUE"], json!("dflt"));
    }
}

// ============================================================================
// elements.rs
// ============================================================================

#[test]
fn test_element_param_builders_and_try_from() {
    let p = AddElementToFeatureParams::new("F", "E")
        .with_display_delim("|")
        .with_derived("Yes");
    assert_eq!((p.display_delim, p.derived), (Some("|"), Some("Yes")));

    let j = json!({"featureCode": "F", "elementCode": "E", "displayLevel": 0,
                   "displayDelim": ",", "derived": "No", "execOrder": 4});
    let p = AddElementToFeatureParams::try_from(&j).unwrap();
    assert_eq!(
        (
            p.feature_code,
            p.element_code,
            p.display_level,
            p.display_delim,
            p.derived
        ),
        ("F", "E", Some(0), Some(","), Some("No"))
    );
    let alias = json!({"feature": "F2", "element": "E2"});
    let p = AddElementToFeatureParams::try_from(&alias).unwrap();
    assert_eq!((p.feature_code, p.element_code), ("F2", "E2"));
    assert_err(
        AddElementToFeatureParams::try_from(&json!({"element": "E"})),
        SzErrorKind::MissingField,
        Some("featureCode"),
    );
    assert_err(
        AddElementToFeatureParams::try_from(&json!({"feature": "F"})),
        SzErrorKind::MissingField,
        Some("elementCode"),
    );

    let p = SetFeatureElementParams::try_from(&j).unwrap();
    assert_eq!(
        (
            p.feature_code,
            p.element_code,
            p.exec_order,
            p.display_level
        ),
        (Some("F"), Some("E"), Some(4), Some(0))
    );
    assert_eq!((p.display_delim, p.derived), (Some(","), Some("No")));
    assert_err(
        SetFeatureElementParams::try_from(&json!({"elementCode": "E"})),
        SzErrorKind::MissingField,
        Some("featureCode"),
    );
    assert_err(
        SetFeatureElementParams::try_from(&json!({"featureCode": "F"})),
        SzErrorKind::MissingField,
        Some("elementCode"),
    );

    let e = json!({"code": "C", "description": "D", "dataType": "number", "id": 9});
    let p = AddElementParams::try_from(&e).unwrap();
    assert_eq!(
        (p.code, p.description, p.data_type, p.id),
        ("C", Some("D"), Some("number"), Some(9))
    );
    let p = SetElementParams::try_from(&e).unwrap();
    assert_eq!(
        (p.code, p.description, p.data_type),
        ("C", Some("D"), Some("number"))
    );
    assert_err(
        AddElementParams::try_from(&json!({})),
        SzErrorKind::MissingField,
        Some("code"),
    );
    assert_err(
        SetElementParams::try_from(&json!({})),
        SzErrorKind::MissingField,
        Some("code"),
    );
}

fn elem_params<'a>(code: &'a str, data_type: Option<&'a str>) -> AddElementParams<'a> {
    AddElementParams {
        code,
        description: None,
        data_type,
        id: None,
    }
}

#[test]
fn test_add_element_paths() {
    let cfg = template();
    assert_json_parse(add_element(BAD_JSON, elem_params("X", None)));
    assert_missing_section(add_element(EMPTY_G2, elem_params("X", None)), "CFG_FELEM");
    assert_err(
        add_element(&cfg, elem_params("make", None)),
        SzErrorKind::AlreadyExists,
        Some("Element already exists: MAKE"),
    );
    assert_err(
        add_element(&cfg, elem_params("NEW_EL", Some("blob"))),
        SzErrorKind::InvalidInput,
        Some("Invalid DATATYPE value 'blob'. Must be one of: string, number, date, datetime, json"),
    );
    for dt in ["Date", "DATETIME", "json"] {
        let out = add_element(&cfg, elem_params("NEW_EL", Some(dt))).unwrap();
        assert_eq!(
            get_element(&out, "new_el").unwrap()["datatype"],
            json!(dt.to_lowercase())
        );
    }
}

#[test]
fn test_delete_element_template_paths() {
    let cfg = template();
    assert_json_parse(delete_element(BAD_JSON, "X"));
    assert_missing_section(delete_element(EMPTY_G2, "X"), "CFG_FELEM");
    assert_err(
        delete_element(&cfg, "nope"),
        SzErrorKind::NotFound,
        Some("Element does not exist"),
    );
    assert_err(
        delete_element(&cfg, "user_name"),
        SzErrorKind::InvalidInput,
        Some("Element linked to the following feature(s): EMAIL"),
    );
    let out = delete_element(&cfg, "make").unwrap();
    assert_err(
        get_element(&out, "MAKE"),
        SzErrorKind::NotFound,
        Some("Element not found: MAKE"),
    );
}

#[test]
fn test_delete_element_structural_edge_cases() {
    // Element row without FELEM_ID.
    let no_id = r#"{"G2_CONFIG": {"CFG_FELEM": [{"FELEM_CODE": "E"}], "CFG_FBOM": []}}"#;
    assert_err(
        delete_element(no_id, "E"),
        SzErrorKind::MissingField,
        Some("FELEM_ID"),
    );
    // Missing CFG_FBOM.
    let no_fbom = r#"{"G2_CONFIG": {"CFG_FELEM": [{"FELEM_ID": 1, "FELEM_CODE": "E"}]}}"#;
    assert_missing_section(delete_element(no_fbom, "E"), "CFG_FBOM");
    // FBOM rows that reference the element but cannot be resolved to a feature
    // (no FTYPE_ID, an unknown FTYPE_ID, or no CFG_FTYPE at all) do not count as
    // linkage, so the delete succeeds.
    let dangling = [
        r#"{"G2_CONFIG": {"CFG_FELEM": [{"FELEM_ID": 1, "FELEM_CODE": "E"}],
            "CFG_FBOM": [{"FELEM_ID": 1}], "CFG_FTYPE": []}}"#,
        r#"{"G2_CONFIG": {"CFG_FELEM": [{"FELEM_ID": 1, "FELEM_CODE": "E"}],
            "CFG_FBOM": [{"FELEM_ID": 1, "FTYPE_ID": 9}],
            "CFG_FTYPE": [{"FTYPE_ID": 8, "FTYPE_CODE": "F"}]}}"#,
        r#"{"G2_CONFIG": {"CFG_FELEM": [{"FELEM_ID": 1, "FELEM_CODE": "E"}],
            "CFG_FBOM": [{"FELEM_ID": 1, "FTYPE_ID": 9}]}}"#,
    ];
    for cfg in dangling {
        let out = delete_element(cfg, "e").unwrap();
        assert_eq!(g2(&out)["CFG_FELEM"], json!([]));
    }
}

#[test]
fn test_get_list_set_element_paths() {
    let cfg = template();
    assert_eq!(get_element(&cfg, "make").unwrap()["element"], json!("MAKE"));
    assert_json_parse(get_element(BAD_JSON, "X"));
    assert_missing_section(get_element(EMPTY_G2, "X"), "CFG_FELEM");

    let list = list_elements(&cfg).unwrap();
    assert_eq!(list.len(), g2(&cfg)["CFG_FELEM"].as_array().unwrap().len());
    let codes: Vec<&str> = list
        .iter()
        .map(|e| e["element"].as_str().unwrap())
        .collect();
    let mut sorted = codes.clone();
    sorted.sort();
    assert_eq!(codes, sorted);
    assert_json_parse(list_elements(BAD_JSON));
    assert_missing_section(list_elements(EMPTY_G2), "CFG_FELEM");

    let sp = |code, description, data_type| SetElementParams {
        code,
        description,
        data_type,
    };
    assert_json_parse(set_element(BAD_JSON, sp("MAKE", None, None)));
    assert_missing_section(set_element(EMPTY_G2, sp("MAKE", None, None)), "CFG_FELEM");
    assert_err(
        set_element(&cfg, sp("nope", None, None)),
        SzErrorKind::NotFound,
        Some("Element: NOPE"),
    );
    let out = set_element(&cfg, sp("make", Some("Vehicle make"), Some("number"))).unwrap();
    let row = g2(&out)["CFG_FELEM"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["FELEM_CODE"] == json!("MAKE"))
        .cloned()
        .unwrap();
    assert_eq!(row["FELEM_DESC"], json!("Vehicle make"));
    assert_eq!(row["DATA_TYPE"], json!("number"));
    let unchanged = set_element(&cfg, sp("MAKE", None, None)).unwrap();
    assert_eq!(g2(&unchanged)["CFG_FELEM"], g2(&cfg)["CFG_FELEM"]);
}

const NO_FBOM: &str = r#"{"G2_CONFIG": {
    "CFG_FTYPE": [{"FTYPE_ID": 1, "FTYPE_CODE": "F"}],
    "CFG_FELEM": [{"FELEM_ID": 2, "FELEM_CODE": "E"}],
    "CFG_DFUNC": [{"DFUNC_ID": 3, "DFUNC_CODE": "D"}]
}}"#;

#[test]
fn test_set_feature_element_error_paths() {
    let cfg = template();
    assert_err(
        set_feature_element(
            &cfg,
            SetFeatureElementParams {
                element_code: Some("ADDR"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("feature_code"),
    );
    assert_err(
        set_feature_element(
            &cfg,
            SetFeatureElementParams {
                feature_code: Some("EMAIL"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("element_code"),
    );
    assert_missing_section(
        set_feature_element(NO_FBOM, SetFeatureElementParams::new("F", "E")),
        "CFG_FBOM",
    );
    assert_err(
        set_feature_element(
            &cfg,
            SetFeatureElementParams::new("EMAIL", "ADDR").with_display_level(-1),
        ),
        SzErrorKind::InvalidInput,
        Some("Invalid DISPLAY_LEVEL value '-1'. Must be a non-negative integer"),
    );
    assert_err(
        set_feature_element(
            &cfg,
            SetFeatureElementParams::new("EMAIL", "ADDR").with_derived("maybe"),
        ),
        SzErrorKind::InvalidInput,
        Some("Invalid DERIVED value 'maybe'. Must be 'Yes' or 'No'"),
    );
}

#[test]
fn test_add_and_delete_element_to_feature_error_paths() {
    let cfg = template();
    assert_err(
        add_element_to_feature(&cfg, AddElementToFeatureParams::new("NO_FEAT", "MAKE")),
        SzErrorKind::NotFound,
        Some("Feature 'NO_FEAT' not found"),
    );
    assert_err(
        add_element_to_feature(&cfg, AddElementToFeatureParams::new("EMAIL", "NO_ELEM")),
        SzErrorKind::NotFound,
        Some("Element 'NO_ELEM' not found"),
    );
    assert_err(
        add_element_to_feature(
            &cfg,
            AddElementToFeatureParams::new("EMAIL", "MAKE").with_derived("maybe"),
        ),
        SzErrorKind::InvalidInput,
        Some("Invalid DERIVED value 'maybe'. Must be 'Yes' or 'No'"),
    );
    assert_missing_section(
        add_element_to_feature(NO_FBOM, AddElementToFeatureParams::new("F", "E")),
        "CFG_FBOM",
    );

    assert_err(
        delete_element_from_feature(&cfg, "NO_FEAT", "MAKE"),
        SzErrorKind::NotFound,
        Some("Feature 'NO_FEAT' not found"),
    );
    assert_err(
        delete_element_from_feature(&cfg, "EMAIL", "NO_ELEM"),
        SzErrorKind::NotFound,
        Some("Element 'NO_ELEM' not found"),
    );
    assert_missing_section(delete_element_from_feature(NO_FBOM, "F", "E"), "CFG_FBOM");
}

// ============================================================================
// features.rs — parameter structs
// ============================================================================

#[test]
fn test_add_feature_params_try_from() {
    let j = json!({
        "feature": "F", "elementList": ["E"], "class": "OTHER", "behavior": "FM",
        "candidates": "Yes", "anonymize": "No", "derived": "No", "history": "Yes",
        "matchKey": "No", "standardize": "PARSE_NAME", "expression": "",
        "comparison": "STR_COMP", "version": 2, "rtypeId": 3, "id": 1500
    });
    let p = AddFeatureParams::try_from(&j).unwrap();
    assert_eq!((p.feature, p.element_list), ("F", &json!(["E"])));
    assert_eq!(
        (p.class, p.behavior, p.candidates, p.anonymize, p.derived),
        (
            Some("OTHER"),
            Some("FM"),
            Some("Yes"),
            Some("No"),
            Some("No")
        )
    );
    assert_eq!((p.history, p.matchkey), (Some("Yes"), Some("No")));
    // Empty function codes are filtered out to None.
    assert_eq!(
        (p.standardize, p.expression, p.comparison),
        (Some("PARSE_NAME"), None, Some("STR_COMP"))
    );
    assert_eq!(
        (p.version, p.rtype_id, p.id),
        (Some(2), Some(3), Some(1500))
    );

    assert_err(
        AddFeatureParams::try_from(&json!({"elementList": []})),
        SzErrorKind::MissingField,
        Some("feature"),
    );
    assert_err(
        AddFeatureParams::try_from(&json!({"feature": "F"})),
        SzErrorKind::MissingField,
        Some("elementList"),
    );
}

#[test]
fn test_set_feature_params_new_and_try_from() {
    let p = SetFeatureParams::new("NAME");
    assert_eq!(p.feature, "NAME");
    assert!(p.candidates.is_none() && p.class.is_none());

    let j = json!({
        "feature": "F", "candidates": "Yes", "anonymize": "No", "derived": "No",
        "history": "Yes", "matchKey": "Confirm", "behavior": "F1", "class": "OTHER",
        "version": 4, "rtypeId": 5
    });
    let p = SetFeatureParams::try_from(&j).unwrap();
    assert_eq!(
        (p.feature, p.candidates, p.anonymize, p.derived, p.history),
        ("F", Some("Yes"), Some("No"), Some("No"), Some("Yes"))
    );
    assert_eq!(
        (p.matchkey, p.behavior, p.class, p.version, p.rtype_id),
        (Some("Confirm"), Some("F1"), Some("OTHER"), Some(4), Some(5))
    );
    assert_err(
        SetFeatureParams::try_from(&json!({})),
        SzErrorKind::MissingField,
        Some("feature"),
    );
}

#[test]
fn test_feature_comparison_params() {
    let p = AddFeatureComparisonParams::new("F", "E")
        .with_display_level(0)
        .with_display_delim("|")
        .with_derived("Yes");
    assert_eq!(
        (p.display_level, p.display_delim, p.derived),
        (Some(0), Some("|"), Some("Yes"))
    );

    let j = json!({"featureCode": "F", "elementCode": "E", "execOrder": 2,
                   "displayLevel": 1, "displayDelim": ",", "derived": "No"});
    let p = AddFeatureComparisonParams::try_from(&j).unwrap();
    assert_eq!(
        (p.feature_code, p.element_code, p.exec_order),
        (Some("F"), Some("E"), Some(2))
    );
    assert_eq!(
        (p.display_level, p.display_delim, p.derived),
        (Some(1), Some(","), Some("No"))
    );
    let g = GetFeatureComparisonParams::try_from(&j).unwrap();
    assert_eq!((g.feature_code, g.element_code), (Some("F"), Some("E")));
    let g = GetFeatureComparisonParams::new("F1", "E1");
    assert_eq!((g.feature_code, g.element_code), (Some("F1"), Some("E1")));

    let only_elem = json!({"elementCode": "E"});
    let only_feat = json!({"featureCode": "F"});
    assert_err(
        AddFeatureComparisonParams::try_from(&only_elem),
        SzErrorKind::MissingField,
        Some("featureCode"),
    );
    assert_err(
        AddFeatureComparisonParams::try_from(&only_feat),
        SzErrorKind::MissingField,
        Some("elementCode"),
    );
    assert_err(
        GetFeatureComparisonParams::try_from(&only_elem),
        SzErrorKind::MissingField,
        Some("featureCode"),
    );
    assert_err(
        GetFeatureComparisonParams::try_from(&only_feat),
        SzErrorKind::MissingField,
        Some("elementCode"),
    );

    let d = AddFeatureDistinctCallElementParams::new("F", "D")
        .with_element_code("E")
        .with_exec_order(3);
    assert_eq!((d.element_code, d.exec_order), (Some("E"), Some(3)));
}

// ============================================================================
// features.rs — add_feature
// ============================================================================

fn feat<'a>(feature: &'a str, element_list: &'a Value) -> AddFeatureParams<'a> {
    AddFeatureParams::new(feature, element_list)
}

#[test]
fn test_add_feature_structural_errors() {
    let cfg = template();
    let elems = json!(["MAKE"]);
    assert_json_parse(add_feature(BAD_JSON, feat("X", &elems)));
    assert_missing_section(add_feature(EMPTY_G2, feat("X", &elems)), "CFG_FTYPE");
    assert_err(
        add_feature(&cfg, feat("email", &elems)),
        SzErrorKind::AlreadyExists,
        Some("Feature already exists: EMAIL"),
    );
    let not_array = json!("MAKE");
    assert_err(
        add_feature(&cfg, feat("X", &not_array)),
        SzErrorKind::InvalidInput,
        Some("elementList must be an array"),
    );
    let empty = json!([]);
    assert_err(
        add_feature(&cfg, feat("X", &empty)),
        SzErrorKind::InvalidInput,
        Some("elementList must contain at least one element"),
    );
    assert_err(
        add_feature(&cfg, feat("X", &elems).with_id(14)),
        SzErrorKind::AlreadyExists,
        Some("The specified ID 14 is already taken"),
    );
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                behavior: Some("BOGUS"),
                ..feat("X", &elems)
            },
        ),
        SzErrorKind::InvalidInput,
        None,
    );
    let no_fclass = r#"{"G2_CONFIG": {"CFG_FTYPE": []}}"#;
    assert_missing_section(add_feature(no_fclass, feat("X", &elems)), "CFG_FCLASS");
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                class: Some("NO_CLASS"),
                ..feat("X", &elems)
            },
        ),
        SzErrorKind::NotFound,
        Some("Feature class: NO_CLASS"),
    );
}

#[test]
fn test_add_feature_flag_domains() {
    let cfg = template();
    let elems = json!(["MAKE"]);
    let base = || feat("NEW_FEAT", &elems);
    type Setter = for<'a> fn(AddFeatureParams<'a>, &'a str) -> AddFeatureParams<'a>;
    let fields: [(&str, &str, Setter); 4] = [
        ("CANDIDATES", "USED_FOR_CAND", |p, v| AddFeatureParams {
            candidates: Some(v),
            ..p
        }),
        ("ANONYMIZE", "ANONYMIZE", |p, v| AddFeatureParams {
            anonymize: Some(v),
            ..p
        }),
        ("DERIVED", "DERIVED", |p, v| AddFeatureParams {
            derived: Some(v),
            ..p
        }),
        ("HISTORY", "PERSIST_HISTORY", |p, v| AddFeatureParams {
            history: Some(v),
            ..p
        }),
    ];
    for (name, column, set) in fields {
        for (input, expected) in [("yes", "Yes"), ("NO", "No")] {
            let out = add_feature(&cfg, set(base(), input)).unwrap();
            let row = new_ftype_row(&out, "NEW_FEAT");
            assert_eq!(row[column], json!(expected), "{name}={input}");
        }
        assert_err(
            add_feature(&cfg, set(base(), "maybe")),
            SzErrorKind::InvalidInput,
            Some(format!("Invalid {name} value 'maybe'. Must be 'Yes' or 'No'").as_str()),
        );
    }
    for (input, expected) in [
        ("yes", "Yes"),
        ("no", "No"),
        ("confirm", "Confirm"),
        ("DENIAL", "Denial"),
    ] {
        let out = add_feature(
            &cfg,
            AddFeatureParams {
                matchkey: Some(input),
                ..base()
            },
        )
        .unwrap();
        assert_eq!(
            new_ftype_row(&out, "NEW_FEAT")["SHOW_IN_MATCH_KEY"],
            json!(expected)
        );
    }
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                matchkey: Some("maybe"),
                ..base()
            },
        ),
        SzErrorKind::InvalidInput,
        Some("Invalid MATCHKEY value 'maybe'. Must be one of: Yes, No, Confirm, Denial"),
    );
}

fn new_ftype_row(config: &str, code: &str) -> Value {
    g2(config)["CFG_FTYPE"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["FTYPE_CODE"] == json!(code))
        .cloned()
        .expect("feature row present")
}

#[test]
fn test_add_feature_function_lookup_errors() {
    let cfg = template();
    let elems = json!([{"element": "MAKE", "expressed": "Yes", "compared": "Yes"}]);
    let base = || feat("NEW_FEAT", &elems);
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                standardize: Some("NO_SFUNC"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        None,
    );
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                expression: Some("NO_EFUNC"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        None,
    );
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                comparison: Some("NO_CFUNC"),
                ..base()
            },
        ),
        SzErrorKind::NotFound,
        None,
    );
}

#[test]
fn test_add_feature_requires_marked_elements_for_functions() {
    let cfg = template();
    // A bare-string element alongside an unmarked object: neither is marked.
    let unmarked = json!(["MAKE", {"element": "MODEL"}]);
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                expression: Some("EXPRESS_BOM"),
                ..feat("NEW_FEAT", &unmarked)
            },
        ),
        SzErrorKind::InvalidInput,
        Some("No elements marked \"expressed\" for expression routine"),
    );
    assert_err(
        add_feature(
            &cfg,
            AddFeatureParams {
                comparison: Some("STR_COMP"),
                ..feat("NEW_FEAT", &unmarked)
            },
        ),
        SzErrorKind::InvalidInput,
        Some("No elements marked \"compared\" for comparison routine"),
    );
}

#[test]
fn test_add_feature_with_all_functions_on_template() {
    let cfg = template();
    // Upper-case keys exercise the alternate spellings; "display" toggles the
    // backwards-compatible yes/no path.
    let elems = json!([
        "MAKE",
        {"ELEMENT": "MODEL", "EXPRESSED": "Yes", "COMPARED": "Yes", "DISPLAY": "no",
         "DISPLAYDELIM": "|", "DERIVED": "yes"},
        {"element": "BRAND_NEW_ELEM", "expressed": "yes", "compared": "yes", "display": "Yes"}
    ]);
    let out = add_feature(
        &cfg,
        AddFeatureParams {
            standardize: Some("PARSE_NAME"),
            expression: Some("EXPRESS_BOM"),
            comparison: Some("STR_COMP"),
            ..feat("NEW_FEAT", &elems)
        },
    )
    .unwrap();
    let f = get_feature(&out, "new_feat").unwrap();
    assert_eq!(f["standardize"], json!("PARSE_NAME"));
    assert_eq!(f["expression"], json!("EXPRESS_BOM"));
    assert_eq!(f["comparison"], json!("STR_COMP"));
    assert_eq!(f["matchKey"], json!("Yes")); // default when comparison is set
    assert_eq!(
        f["elementList"],
        json!([
            {"element": "MAKE", "expressed": "No", "compared": "No", "derived": "No", "display": "Yes"},
            {"element": "MODEL", "expressed": "Yes", "compared": "Yes", "derived": "Yes", "display": "No"},
            {"element": "BRAND_NEW_ELEM", "expressed": "Yes", "compared": "Yes", "derived": "No", "display": "Yes"}
        ])
    );
}

#[test]
fn test_add_feature_element_list_item_errors() {
    let cfg = template();
    let missing_code = json!([{"expressed": "Yes"}]);
    assert_err(
        add_feature(&cfg, feat("NEW_FEAT", &missing_code)),
        SzErrorKind::InvalidInput,
        Some("Missing element code in elementList item 1"),
    );
    let bad_item = json!(["MAKE", 42]);
    assert_err(
        add_feature(&cfg, feat("NEW_FEAT", &bad_item)),
        SzErrorKind::InvalidInput,
        Some("Invalid element in elementList item 2"),
    );
}

#[test]
fn test_add_feature_missing_call_and_element_sections() {
    let base = r#"{"G2_CONFIG": {
        "CFG_FTYPE": [],
        "CFG_FCLASS": [{"FCLASS_ID": 9, "FCLASS_CODE": "OTHER"}],
        "CFG_SFUNC": [{"SFUNC_ID": 1, "SFUNC_CODE": "S"}],
        "CFG_EFUNC": [{"EFUNC_ID": 1, "EFUNC_CODE": "X"}],
        "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "C"}]
    }}"#;
    let elems = json!([{"element": "E", "expressed": "Yes", "compared": "Yes"}]);
    let p = |s, x, c| AddFeatureParams {
        standardize: s,
        expression: x,
        comparison: c,
        ..feat("F", &elems)
    };
    assert_missing_section(add_feature(base, p(Some("S"), None, None)), "CFG_SFCALL");
    assert_missing_section(add_feature(base, p(None, Some("X"), None)), "CFG_EFCALL");
    assert_missing_section(add_feature(base, p(None, None, Some("C"))), "CFG_CFCALL");
    assert_missing_section(add_feature(base, p(None, None, None)), "CFG_FELEM");

    // An existing element row with no FELEM_ID is structurally invalid.
    let bad_felem = r#"{"G2_CONFIG": {
        "CFG_FTYPE": [],
        "CFG_FCLASS": [{"FCLASS_ID": 9, "FCLASS_CODE": "OTHER"}],
        "CFG_FELEM": [{"FELEM_CODE": "E"}]
    }}"#;
    assert_err(
        add_feature(bad_felem, p(None, None, None)),
        SzErrorKind::InvalidStructure,
        Some("Invalid FELEM_ID"),
    );

    // EFBOM / CFBOM / FBOM sections absent: rows are silently not written.
    let calls_only = r#"{"G2_CONFIG": {
        "CFG_FTYPE": [],
        "CFG_FCLASS": [{"FCLASS_ID": 9, "FCLASS_CODE": "OTHER"}],
        "CFG_FELEM": [],
        "CFG_EFUNC": [{"EFUNC_ID": 1, "EFUNC_CODE": "X"}],
        "CFG_CFUNC": [{"CFUNC_ID": 1, "CFUNC_CODE": "C"}],
        "CFG_EFCALL": [], "CFG_CFCALL": []
    }}"#;
    let out = add_feature(calls_only, p(None, Some("X"), Some("C"))).unwrap();
    let g = g2(&out);
    assert_eq!(g["CFG_FTYPE"].as_array().unwrap().len(), 1);
    for absent in ["CFG_EFBOM", "CFG_CFBOM", "CFG_FBOM"] {
        assert!(g[absent].is_null(), "{absent} must not be written");
    }
}

// ============================================================================
// features.rs — delete / get / list / set
// ============================================================================

#[test]
fn test_delete_feature_cascades_on_template() {
    let cfg = template();
    let before = g2(&cfg);
    let passport_id = new_ftype_row(&cfg, "PASSPORT")["FTYPE_ID"]
        .as_i64()
        .unwrap();
    let out = delete_feature(&cfg, &passport_id.to_string()).unwrap();
    let after = g2(&out);
    for section in [
        "CFG_FBOM",
        "CFG_SFCALL",
        "CFG_EFCALL",
        "CFG_CFCALL",
        "CFG_DFCALL",
    ] {
        let has = |g: &Value| {
            g[section]
                .as_array()
                .unwrap()
                .iter()
                .any(|r| r["FTYPE_ID"].as_i64() == Some(passport_id))
        };
        assert!(has(&before), "{section} had PASSPORT rows");
        assert!(!has(&after), "{section} PASSPORT rows removed");
    }
    assert!(
        !after["CFG_ATTR"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["FTYPE_CODE"] == json!("PASSPORT"))
    );
    assert_err(
        get_feature(&out, "PASSPORT"),
        SzErrorKind::NotFound,
        Some("Feature: PASSPORT"),
    );
}

#[test]
fn test_delete_feature_error_paths() {
    let cfg = template();
    assert_json_parse(delete_feature(BAD_JSON, "X"));
    assert_missing_section(delete_feature(EMPTY_G2, "5"), "CFG_FTYPE");
    assert_err(
        delete_feature(&cfg, "nope"),
        SzErrorKind::NotFound,
        Some("Feature not found"),
    );
    assert_err(
        delete_feature(&cfg, "99999"),
        SzErrorKind::NotFound,
        Some("Feature: 99999"),
    );
    // An id-addressed row without an FTYPE_CODE cannot be resolved.
    let no_code = r#"{"G2_CONFIG": {"CFG_FTYPE": [{"FTYPE_ID": 5}]}}"#;
    assert_err(
        delete_feature(no_code, "5"),
        SzErrorKind::NotFound,
        Some("Feature: 5"),
    );
}

#[test]
fn test_delete_feature_tolerates_rows_without_call_ids() {
    // BOM rows without a call id and call rows without their id are tolerated.
    let cfg = r#"{"G2_CONFIG": {
        "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "F"}],
        "CFG_ATTR": [{"ATTR_CODE": "A"}, {"ATTR_CODE": "B", "FTYPE_CODE": "f"}],
        "CFG_EFCALL": [{"FTYPE_ID": 5}, {"FTYPE_ID": 5, "EFCALL_ID": 1}],
        "CFG_EFBOM": [{"FTYPE_ID": 5}, {"EFCALL_ID": 1}, {"EFCALL_ID": 2}],
        "CFG_CFCALL": [{"FTYPE_ID": 5}, {"FTYPE_ID": 5, "CFCALL_ID": 1}],
        "CFG_CFBOM": [{"FTYPE_ID": 5}, {"CFCALL_ID": 1}, {"CFCALL_ID": 2}],
        "CFG_DFCALL": [{"FTYPE_ID": 5}, {"FTYPE_ID": 5, "DFCALL_ID": 1}],
        "CFG_DFBOM": [{"FTYPE_ID": 5}, {"DFCALL_ID": 1}, {"DFCALL_ID": 2}]
    }}"#;
    let out = delete_feature(cfg, "f").unwrap();
    let g = g2(&out);
    assert_eq!(g["CFG_FTYPE"], json!([]));
    assert_eq!(g["CFG_ATTR"], json!([{"ATTR_CODE": "A"}]));
    for (bom, key) in [
        ("CFG_EFBOM", "EFCALL_ID"),
        ("CFG_CFBOM", "CFCALL_ID"),
        ("CFG_DFBOM", "DFCALL_ID"),
    ] {
        assert_eq!(g[bom], json!([{"FTYPE_ID": 5}, {key: 2}]), "{bom}");
    }
    for call in ["CFG_EFCALL", "CFG_CFCALL", "CFG_DFCALL"] {
        assert_eq!(g[call], json!([]), "{call}");
    }
}

#[test]
fn test_get_and_list_features() {
    let cfg = template();
    assert_json_parse(get_feature(BAD_JSON, "NAME"));
    let by_id = get_feature(&cfg, "14").unwrap();
    assert_eq!(by_id["feature"], json!("EMAIL"));
    assert_err(
        get_feature(&cfg, "99999"),
        SzErrorKind::NotFound,
        Some("Feature: 99999"),
    );
    assert_err(
        get_feature(&cfg, "nope"),
        SzErrorKind::NotFound,
        Some("Feature: NOPE"),
    );

    let list = list_features(&cfg).unwrap();
    assert_eq!(list.len(), g2(&cfg)["CFG_FTYPE"].as_array().unwrap().len());
    let ids: Vec<i64> = list.iter().map(|f| f["id"].as_i64().unwrap()).collect();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted);
    let email = list
        .iter()
        .find(|f| f["feature"] == json!("EMAIL"))
        .unwrap();
    assert_eq!(email, &by_id);
    assert_json_parse(list_features(BAD_JSON));
    assert_missing_section(list_features(EMPTY_G2), "CFG_FTYPE");
}

#[test]
fn test_build_feature_json_with_sparse_rows() {
    // Rows missing optional columns / referencing unknown ids fall back to the
    // documented defaults, and elements sort by EXEC_ORDER.
    let cfg = r#"{"G2_CONFIG": {
        "CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "F", "FCLASS_ID": 99}],
        "CFG_SFCALL": [{"FTYPE_ID": 5, "SFUNC_ID": 42}],
        "CFG_EFCALL": [{"FTYPE_ID": 5, "EFUNC_ID": 42}],
        "CFG_CFCALL": [{"FTYPE_ID": 5, "CFUNC_ID": 42}],
        "CFG_FBOM": [
            {"FTYPE_ID": 5, "FELEM_ID": 7, "EXEC_ORDER": 2, "DISPLAY_LEVEL": 0},
            {"FTYPE_ID": 5, "FELEM_ID": 8, "EXEC_ORDER": 1}
        ],
        "CFG_FELEM": [{"FELEM_ID": 8, "FELEM_CODE": "E8"}]
    }}"#;
    let f = get_feature(cfg, "F").unwrap();
    assert_eq!(f["class"], json!("OTHER"));
    assert_eq!(
        (&f["standardize"], &f["expression"], &f["comparison"]),
        (&json!(""), &json!(""), &json!(""))
    );
    assert_eq!(
        (
            &f["anonymize"],
            &f["candidates"],
            &f["matchKey"],
            &f["version"]
        ),
        (&json!(""), &json!(""), &json!(""), &json!(0))
    );
    assert_eq!(
        f["elementList"],
        json!([
            {"element": "E8", "expressed": "No", "compared": "No", "derived": "No", "display": "Yes"},
            {"element": "", "expressed": "No", "compared": "No", "derived": "No", "display": "No"}
        ])
    );
    // The public builder on the parsed config gives the same summary.
    let parsed: serde_json::Value = serde_json::from_str(cfg).unwrap();
    let ftype = &parsed["G2_CONFIG"]["CFG_FTYPE"][0];
    assert_eq!(build_feature_json(&parsed, ftype).unwrap(), f);
}

#[test]
fn test_set_feature_updates_every_field() {
    let cfg = template();
    let out = set_feature(
        &cfg,
        SetFeatureParams {
            feature: "14",
            candidates: Some("yes"),
            anonymize: Some("Yes"),
            derived: Some("Yes"),
            history: Some("No"),
            matchkey: Some("denial"),
            behavior: Some("FM"),
            class: Some("other"),
            version: Some(9),
            rtype_id: Some(4),
        },
    )
    .unwrap();
    let row = new_ftype_row(&out, "EMAIL");
    let expected = [
        ("USED_FOR_CAND", json!("Yes")),
        ("ANONYMIZE", json!("Yes")),
        ("DERIVED", json!("Yes")),
        ("PERSIST_HISTORY", json!("No")),
        ("SHOW_IN_MATCH_KEY", json!("Denial")),
        ("FTYPE_FREQ", json!("FM")),
        ("FCLASS_ID", json!(9)),
        ("VERSION", json!(9)),
        ("RTYPE_ID", json!(4)),
    ];
    for (col, val) in expected {
        assert_eq!(row[col], val, "{col}");
    }
}

#[test]
fn test_set_feature_unchanged_values_report_no_changes() {
    let cfg = template();
    // Every supplied value equals the current EMAIL row, so nothing changes.
    let same = SetFeatureParams {
        feature: "email",
        candidates: Some("No"),
        anonymize: Some("No"),
        derived: Some("No"),
        history: Some("Yes"),
        matchkey: Some("Yes"),
        behavior: Some("F1"),
        class: Some("ELECTED_ID"),
        version: Some(3),
        rtype_id: Some(0),
    };
    assert_err(
        set_feature(&cfg, same),
        SzErrorKind::InvalidInput,
        Some("No changes detected"),
    );
}

#[test]
fn test_set_feature_error_paths() {
    let cfg = template();
    let p = |feature| SetFeatureParams::new(feature);
    assert_json_parse(set_feature(BAD_JSON, p("NAME")));
    assert_err(
        set_feature(&cfg, p("nope")),
        SzErrorKind::NotFound,
        Some("Feature not found"),
    );
    assert_missing_section(set_feature(EMPTY_G2, p("5")), "CFG_FTYPE");
    assert_err(
        set_feature(&cfg, p("99999")),
        SzErrorKind::NotFound,
        Some("Feature not found"),
    );
    assert_err(
        set_feature(
            &cfg,
            SetFeatureParams {
                candidates: Some("maybe"),
                ..p("EMAIL")
            },
        ),
        SzErrorKind::InvalidInput,
        Some(r#"CANDIDATES value must be in ["Yes", "No"]"#),
    );
    assert_err(
        set_feature(
            &cfg,
            SetFeatureParams {
                matchkey: Some("maybe"),
                ..p("EMAIL")
            },
        ),
        SzErrorKind::InvalidInput,
        Some(r#"MATCHKEY value must be in ["Yes", "No", "Confirm", "Denial"]"#),
    );
    assert_err(
        set_feature(
            &cfg,
            SetFeatureParams {
                behavior: Some("BOGUS"),
                ..p("EMAIL")
            },
        ),
        SzErrorKind::InvalidInput,
        None,
    );
    assert_err(
        set_feature(
            &cfg,
            SetFeatureParams {
                class: Some("NO_CLASS"),
                ..p("EMAIL")
            },
        ),
        SzErrorKind::NotFound,
        Some("Feature class: NO_CLASS"),
    );
    let no_fclass = r#"{"G2_CONFIG": {"CFG_FTYPE": [{"FTYPE_ID": 5, "FTYPE_CODE": "F"}]}}"#;
    assert_missing_section(
        set_feature(
            no_fclass,
            SetFeatureParams {
                class: Some("OTHER"),
                ..p("F")
            },
        ),
        "CFG_FCLASS",
    );
}

// ============================================================================
// features.rs — feature comparisons (CFG_FBOM) and distinct calls
// ============================================================================

#[test]
fn test_add_feature_comparison_error_paths() {
    let cfg = template();
    assert_err(
        add_feature_comparison(
            &cfg,
            AddFeatureComparisonParams {
                element_code: Some("MAKE"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("feature_code"),
    );
    assert_err(
        add_feature_comparison(
            &cfg,
            AddFeatureComparisonParams {
                feature_code: Some("EMAIL"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("element_code"),
    );
    assert_err(
        add_feature_comparison(&cfg, AddFeatureComparisonParams::new("NO_FEAT", "MAKE")),
        SzErrorKind::NotFound,
        Some("Feature 'NO_FEAT' not found"),
    );
    assert_err(
        add_feature_comparison(&cfg, AddFeatureComparisonParams::new("EMAIL", "NO_ELEM")),
        SzErrorKind::NotFound,
        Some("Element 'NO_ELEM' not found"),
    );
    assert_missing_section(
        add_feature_comparison(NO_FBOM, AddFeatureComparisonParams::new("F", "E")),
        "CFG_FBOM",
    );
    assert_err(
        add_feature_comparison(&cfg, AddFeatureComparisonParams::new("EMAIL", "ADDR")),
        SzErrorKind::AlreadyExists,
        Some(r#"Feature comparison: Some("EMAIL")+Some("ADDR")"#),
    );
}

#[test]
fn test_feature_comparison_round_trip_on_template() {
    let cfg = template();
    let out = add_feature_comparison_element(
        &cfg,
        AddFeatureComparisonParams::new("EMAIL", "MAKE").with_display_level(0),
    )
    .unwrap();
    let row =
        get_feature_comparison(&out, GetFeatureComparisonParams::new("email", "make")).unwrap();
    assert_eq!(row["DISPLAY_LEVEL"], json!(0));
    assert_eq!(row["EXEC_ORDER"], json!(27)); // template max is 26

    let deleted = delete_feature_comparison_element(&out, "EMAIL", "MAKE").unwrap();
    assert_eq!(g2(&deleted)["CFG_FBOM"], g2(&cfg)["CFG_FBOM"]);
    let deleted = delete_feature_comparison(&deleted, "EMAIL", "ADDR").unwrap();
    assert_err(
        get_feature_comparison(&deleted, GetFeatureComparisonParams::new("EMAIL", "ADDR")),
        SzErrorKind::NotFound,
        Some(r#"Feature comparison: Some("EMAIL")+Some("ADDR")"#),
    );
}

#[test]
fn test_delete_feature_comparison_error_paths() {
    let cfg = template();
    assert_err(
        delete_feature_comparison(&cfg, "NO_FEAT", "MAKE"),
        SzErrorKind::NotFound,
        Some("Feature 'NO_FEAT' not found"),
    );
    assert_err(
        delete_feature_comparison(&cfg, "EMAIL", "NO_ELEM"),
        SzErrorKind::NotFound,
        Some("Element 'NO_ELEM' not found"),
    );
    let make_id = g2(&cfg)["CFG_FELEM"]
        .as_array()
        .unwrap()
        .iter()
        .find(|e| e["FELEM_CODE"] == json!("MAKE"))
        .unwrap()["FELEM_ID"]
        .as_i64()
        .unwrap();
    assert_err(
        delete_feature_comparison(&cfg, "EMAIL", "MAKE"),
        SzErrorKind::NotFound,
        Some(format!("Feature comparison: FTYPE_ID=14, FELEM_ID={make_id}").as_str()),
    );
    // Without a CFG_FBOM section nothing can match.
    assert_err(
        delete_feature_comparison(NO_FBOM, "F", "E"),
        SzErrorKind::NotFound,
        Some("Feature comparison: FTYPE_ID=1, FELEM_ID=2"),
    );
}

#[test]
fn test_get_feature_comparison_error_paths() {
    let cfg = template();
    assert_err(
        get_feature_comparison(
            &cfg,
            GetFeatureComparisonParams {
                element_code: Some("ADDR"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("feature_code"),
    );
    assert_err(
        get_feature_comparison(
            &cfg,
            GetFeatureComparisonParams {
                feature_code: Some("EMAIL"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("element_code"),
    );
    assert_err(
        get_feature_comparison(&cfg, GetFeatureComparisonParams::new("NO_FEAT", "ADDR")),
        SzErrorKind::NotFound,
        Some("Feature 'NO_FEAT' not found"),
    );
    assert_err(
        get_feature_comparison(&cfg, GetFeatureComparisonParams::new("EMAIL", "NO_ELEM")),
        SzErrorKind::NotFound,
        Some("Element 'NO_ELEM' not found"),
    );
    assert_missing_section(
        get_feature_comparison(NO_FBOM, GetFeatureComparisonParams::new("F", "E")),
        "CFG_FBOM",
    );
}

#[test]
fn test_list_feature_comparisons() {
    let cfg = template();
    let list = list_feature_comparisons(&cfg).unwrap();
    assert_eq!(list.len(), g2(&cfg)["CFG_FBOM"].as_array().unwrap().len());
    let keys: Vec<(i64, i64)> = list
        .iter()
        .map(|r| {
            (
                r["FTYPE_ID"].as_i64().unwrap(),
                r["EXEC_ORDER"].as_i64().unwrap(),
            )
        })
        .collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
    assert_json_parse(list_feature_comparisons(BAD_JSON));
    assert_missing_section(list_feature_comparisons(EMPTY_G2), "CFG_FBOM");
}

#[test]
fn test_add_feature_distinct_call_element_paths() {
    let cfg = template();
    let p = AddFeatureDistinctCallElementParams::new;
    assert_err(
        add_feature_distinct_call_element(
            &cfg,
            AddFeatureDistinctCallElementParams {
                distinct_func_code: Some("PARTIAL_NAMES"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("feature_code"),
    );
    assert_err(
        add_feature_distinct_call_element(
            &cfg,
            AddFeatureDistinctCallElementParams {
                feature_code: Some("EMAIL"),
                ..Default::default()
            },
        ),
        SzErrorKind::MissingField,
        Some("distinct_func_code"),
    );
    assert_err(
        add_feature_distinct_call_element(&cfg, p("NO_FEAT", "PARTIAL_NAMES")),
        SzErrorKind::NotFound,
        Some("Feature 'NO_FEAT' not found"),
    );
    assert_err(
        add_feature_distinct_call_element(&cfg, p("EMAIL", "NO_DFUNC")),
        SzErrorKind::NotFound,
        None,
    );
    assert_err(
        add_feature_distinct_call_element(
            &cfg,
            p("EMAIL", "PARTIAL_NAMES").with_element_code("NO_ELEM"),
        ),
        SzErrorKind::NotFound,
        Some("Element 'NO_ELEM' not found"),
    );
    assert_missing_section(
        add_feature_distinct_call_element(NO_FBOM, p("F", "D")),
        "CFG_DFCALL",
    );

    let out = add_feature_distinct_call_element(
        &cfg,
        p("EMAIL", "partial_names").with_element_code("ADDR"),
    )
    .unwrap();
    let added = g2(&out)["CFG_DFCALL"].as_array().unwrap().len();
    assert_eq!(added, g2(&cfg)["CFG_DFCALL"].as_array().unwrap().len() + 1);
    assert_err(
        add_feature_distinct_call_element(&out, p("EMAIL", "PARTIAL_NAMES")),
        SzErrorKind::AlreadyExists,
        Some(r#"Feature distinct call element: Some("EMAIL")+Some("PARTIAL_NAMES")"#),
    );
}

// ============================================================================
// features.rs — feature classes and version
// ============================================================================

#[test]
fn test_list_and_get_feature_classes() {
    let cfg = template();
    let list = list_feature_classes(&cfg).unwrap();
    let ids: Vec<i64> = list
        .iter()
        .map(|c| c["FCLASS_ID"].as_i64().unwrap())
        .collect();
    assert_eq!(ids, (1..=12).collect::<Vec<_>>());
    assert_json_parse(list_feature_classes(BAD_JSON));
    assert_missing_section(list_feature_classes(EMPTY_G2), "CFG_FCLASS");

    assert_eq!(
        get_feature_class(&cfg, "5").unwrap()["FCLASS_CODE"],
        json!("PHONE")
    );
    assert_eq!(
        get_feature_class(&cfg, "issued_id").unwrap()["FCLASS_ID"],
        json!(7)
    );
    assert_err(
        get_feature_class(&cfg, "999"),
        SzErrorKind::NotFound,
        Some("Feature class: 999"),
    );
    assert_err(
        get_feature_class(&cfg, "nope"),
        SzErrorKind::NotFound,
        Some("Feature class: NOPE"),
    );
    assert_json_parse(get_feature_class(BAD_JSON, "1"));
    assert_missing_section(get_feature_class(EMPTY_G2, "1"), "CFG_FCLASS");
}

#[test]
fn test_update_feature_version() {
    let cfg = template();
    let out = update_feature_version(&cfg, "12").unwrap();
    assert_eq!(
        g2(&out)["CONFIG_BASE_VERSION"]["COMPATIBILITY_VERSION"]["FEATURE_VERSION"],
        json!("12")
    );
    assert_json_parse(update_feature_version(BAD_JSON, "1"));
    assert_missing_section(
        update_feature_version(EMPTY_G2, "1"),
        "COMPATIBILITY_VERSION",
    );
    // A top-level array (valid JSON, not a config object) is a missing
    // section, not a panic (JSON IndexMut on an array used to panic).
    assert_missing_section(update_feature_version("[]", "1"), "COMPATIBILITY_VERSION");
}
