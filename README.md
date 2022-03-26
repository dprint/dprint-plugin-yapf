# dprint-plugin-yapf

[![CI](https://github.com/dprint/dprint-plugin-yapf/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-yapf/actions?query=workflow%3ACI)

Wrapper around [yapf](https://github.com/google/yapf) in order to use it as a dprint plugin.

## DEPRECATED - Use dprint-plugin-exec instead

This plugin is deprecated and won't receive updates anymore. You can still format with yapf via dprint's cli though (and with many other formatting CLIs)! See [dprint-plugin-exec](https://github.com/dprint/dprint-plugin-exec/) and search for yapf.

## Old Archived Instructions

### Install

1. Install [dprint](https://dprint.dev/install/)
2. Follow instructions at https://github.com/dprint/dprint-plugin-yapf/releases/

### Configuration

See yapf's configuration [here](https://github.com/google/yapf#knobs).

```jsonc
{
  // ...etc...
  "yapf": {
    "based_on_style": "pep8",
    "spaces_before_comment": 4
  }
}
```
