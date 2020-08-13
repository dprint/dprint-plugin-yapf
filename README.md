# dprint-plugin-yapf

[![CI](https://github.com/dprint/dprint-plugin-yapf/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-yapf/actions?query=workflow%3ACI)

Wrapper around [yapf](https://github.com/google/yapf) in order to use it as a dprint plugin.

## Install

1. Install [dprint](https://dprint.dev/install/)
2. Follow instructions at https://github.com/dprint/dprint-plugin-yapf/releases/

## Configuration

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
