#!/usr/bin/env python3
"""
Refactor all function calls to use (config, params) pattern.
"""

import re
import sys
from pathlib import Path

def refactor_add_attribute(content):
    """Refactor add_attribute(config, attr, params) -> add_attribute(config, params)"""
    # Pattern: add_attribute(config, attr, params { ... })
    pattern = r'add_attribute\(\s*(\w+),\s*(\w+),\s*crate::attributes::AddAttributeParams\s*\{'

    def replacer(match):
        config_var = match.group(1)
        attr_var = match.group(2)
        return f'add_attribute({config_var}, crate::attributes::AddAttributeParams {{\n                    attribute: {attr_var},'

    return re.sub(pattern, replacer, content)

def refactor_add_feature(content):
    """Refactor add_feature(config, feature, params) -> add_feature(config, params)"""
    pattern = r'add_feature\(\s*(\w+),\s*(\w+),\s*crate::features::AddFeatureParams\s*\{'

    def replacer(match):
        config_var = match.group(1)
        feature_var = match.group(2)
        return f'add_feature({config_var}, crate::features::AddFeatureParams {{\n                    feature: {feature_var},'

    return re.sub(pattern, replacer, content)

def refactor_set_feature(content):
    """Refactor set_feature(config, feature, params) -> set_feature(config, params)"""
    pattern = r'set_feature\(\s*(\w+),\s*(\w+),\s*crate::features::SetFeatureParams\s*\{'

    def replacer(match):
        config_var = match.group(1)
        feature_var = match.group(2)
        return f'set_feature({config_var}, crate::features::SetFeatureParams {{\n                    feature: {feature_var},'

    return re.sub(pattern, replacer, content)

def refactor_add_data_source(content):
    """Refactor add_data_source(config, code, params) -> add_data_source(config, params)"""
    pattern = r'add_data_source\(\s*(\w+),\s*(\w+),\s*crate::datasources::AddDataSourceParams\s*\{'

    def replacer(match):
        config_var = match.group(1)
        code_var = match.group(2)
        return f'add_data_source({config_var}, crate::datasources::AddDataSourceParams {{\n                    code: {code_var},'

    return re.sub(pattern, replacer, content)

def refactor_set_feature_element(content):
    """Refactor set_feature_element(config, ftype_id, felem_id, params) -> set_feature_element(config, params)"""
    pattern = r'set_feature_element\(\s*(\w+),\s*(\w+),\s*(\w+),\s*crate::elements::SetFeatureElementParams\s*\{'

    def replacer(match):
        config_var = match.group(1)
        ftype_var = match.group(2)
        felem_var = match.group(3)
        return f'set_feature_element({config_var}, crate::elements::SetFeatureElementParams {{\n                    ftype_id: {ftype_var},\n                    felem_id: {felem_var},'

    return re.sub(pattern, replacer, content)

def refactor_add_feature_comparison(content):
    """Refactor add_feature_comparison(config, ftype_id, felem_id, params) -> add_feature_comparison(config, params)"""
    pattern = r'add_feature_comparison\(\s*(\w+),\s*(\w+),\s*(\w+),\s*crate::features::AddFeatureComparisonParams\s*\{'

    def replacer(match):
        config_var = match.group(1)
        ftype_var = match.group(2)
        felem_var = match.group(3)
        return f'add_feature_comparison({config_var}, crate::features::AddFeatureComparisonParams {{\n                    ftype_id: {ftype_var},\n                    felem_id: {felem_var},'

    return re.sub(pattern, replacer, content)

def refactor_file(filepath):
    """Apply all refactorings to a file"""
    print(f"Refactoring {filepath}...")

    with open(filepath, 'r') as f:
        content = f.read()

    original = content

    # Apply all refactorings
    content = refactor_add_attribute(content)
    content = refactor_add_feature(content)
    content = refactor_set_feature(content)
    content = refactor_add_data_source(content)
    content = refactor_set_feature_element(content)
    content = refactor_add_feature_comparison(content)

    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"  ✓ Modified {filepath}")
        return True
    else:
        print(f"  - No changes in {filepath}")
        return False

def main():
    root = Path("/Users/brianmacy/open_dev/sz-rust-sdk-configtool")

    # Files to refactor
    files_to_refactor = [
        root / "src" / "command_processor.rs",
        root / "src" / "ffi.rs",
        root / "tests" / "lib_tests.rs",
        root / "examples" / "basic_usage.rs",
        root / "examples" / "datasource_management.rs",
        root / "tests" / "test_set_feature_extended.rs",
    ]

    modified_count = 0
    for filepath in files_to_refactor:
        if filepath.exists():
            if refactor_file(filepath):
                modified_count += 1
        else:
            print(f"⚠ File not found: {filepath}")

    print(f"\nRefactored {modified_count} files")

if __name__ == "__main__":
    main()
