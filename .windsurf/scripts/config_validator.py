#!/usr/bin/env python3
"""
Configuration validator for FastRamdisk crando.toml files.

This script validates crando.toml configuration files against the expected
schema and provides helpful error messages for invalid configurations.
"""

import argparse
import sys
from pathlib import Path
from typing import Dict, Any, List, Optional
try:
    import tomli
except ImportError:
    try:
        import tomllib as tomli
    except ImportError:
        print("Error: tomli or tomllib required. Install with: pip install tomli")
        sys.exit(1)


class ConfigValidationError(Exception):
    """Configuration validation error."""
    pass


def validate_mount_point(value: Any) -> Optional[str]:
    """Validate mount point configuration."""
    if not isinstance(value, str):
        raise ConfigValidationError("mount_point must be a string")
    if not value:
        raise ConfigValidationError("mount_point cannot be empty")
    if not (value.endswith(':') or value.endswith('\\') or value.endswith('/')):
        raise ConfigValidationError(f"mount_point must be a drive letter with colon (e.g., 'R:') or end with path separator")
    return value


def validate_max_fs_size(value: Any) -> int:
    """Validate maximum filesystem size."""
    if not isinstance(value, int):
        if isinstance(value, str):
            try:
                value = int(value)
            except ValueError:
                raise ConfigValidationError("max_fs_size must be an integer")
        else:
            raise ConfigValidationError("max_fs_size must be an integer")
    if value <= 0:
        raise ConfigValidationError("max_fs_size must be positive")
    if value > 1024 * 1024 * 1024 * 1024:  # 1TB
        raise ConfigValidationError("max_fs_size exceeds maximum allowed size (1TB)")
    return value


def validate_volume_label(value: Any) -> str:
    """Validate volume label."""
    if not isinstance(value, str):
        raise ConfigValidationError("volume_label must be a string")
    if len(value) > 32:
        raise ConfigValidationError("volume_label must be 32 characters or less")
    return value


def validate_case_insensitive(value: Any) -> bool:
    """Validate case_insensitive flag."""
    if not isinstance(value, bool):
        raise ConfigValidationError("case_insensitive must be a boolean")
    return value


def validate_flush_on_cleanup(value: Any) -> bool:
    """Validate flush_on_cleanup flag."""
    if not isinstance(value, bool):
        raise ConfigValidationError("flush_on_cleanup must be a boolean")
    return value


def validate_disable_ramdisk(value: Any) -> bool:
    """Validate disable_ramdisk flag."""
    if not isinstance(value, bool):
        raise ConfigValidationError("disable_ramdisk must be a boolean")
    return value


VALIDATORS = {
    'mount_point': validate_mount_point,
    'max_fs_size': validate_max_fs_size,
    'volume_label': validate_volume_label,
    'case_insensitive': validate_case_insensitive,
    'flush_on_cleanup': validate_flush_on_cleanup,
    'disable_ramdisk': validate_disable_ramdisk,
}

DEFAULTS = {
    'mount_point': 'R:',
    'max_fs_size': 2 * 1024 * 1024 * 1024,  # 2GB
    'volume_label': 'FastRamdisk',
    'case_insensitive': True,
    'flush_on_cleanup': True,
    'disable_ramdisk': False,
}


def validate_config(config: Dict[str, Any]) -> List[str]:
    """Validate configuration dictionary."""
    errors = []
    warnings = []

    for key, validator in VALIDATORS.items():
        if key in config:
            try:
                validator(config[key])
            except ConfigValidationError as e:
                errors.append(f"{key}: {str(e)}")
        else:
            warnings.append(f"{key}: not specified, will use default ({DEFAULTS[key]})")

    # Check for unknown keys
    known_keys = set(VALIDATORS.keys())
    unknown_keys = set(config.keys()) - known_keys
    if unknown_keys:
        for key in unknown_keys:
            warnings.append(f"{key}: unknown configuration key")

    return errors, warnings


def load_config(config_path: Path) -> Dict[str, Any]:
    """Load configuration from TOML file."""
    with open(config_path, 'rb') as f:
        return tomli.load(f)


def print_validation_result(errors: List[str], warnings: List[str]):
    """Print validation results."""
    if errors:
        print("\n❌ Configuration errors:")
        for error in errors:
            print(f"  - {error}")
    else:
        print("\n✓ Configuration is valid")

    if warnings:
        print("\n⚠ Warnings:")
        for warning in warnings:
            print(f"  - {warning}")


def print_config_summary(config: Dict[str, Any]):
    """Print configuration summary."""
    print("\n=== Configuration Summary ===")
    for key in VALIDATORS.keys():
        value = config.get(key, DEFAULTS[key])
        print(f"{key}: {value}")


def main():
    parser = argparse.ArgumentParser(description='Validate crando.toml configuration')
    parser.add_argument('config', type=Path, help='Path to crando.toml file')
    parser.add_argument('--strict', action='store_true',
                        help='Treat warnings as errors')
    parser.add_argument('--summary', action='store_true',
                        help='Print configuration summary')

    args = parser.parse_args()

    if not args.config.exists():
        print(f"Error: Configuration file not found: {args.config}")
        sys.exit(1)

    try:
        config = load_config(args.config)
    except Exception as e:
        print(f"Error: Failed to parse configuration: {e}")
        sys.exit(1)

    errors, warnings = validate_config(config)

    if args.summary:
        print_config_summary(config)

    print_validation_result(errors, warnings)

    if errors or (args.strict and warnings):
        sys.exit(1)


if __name__ == '__main__':
    main()
