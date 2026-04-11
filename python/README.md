# kitsuyui-rust-playground

This repo is intended to show how to build rust-python binding packages.

```python
from kitsuyui_rust_playground import my_calc
my_calc(1, 2, 3)  # => '9'
```

`my_calc` is implemented by Rust (with PyO3). 
However it works as same as following python code:

```python
def my_calc(a: int, b: int, c: int) -> str:
    return str((a + b) * c)
```

## Development

Install development dependencies with [uv](https://docs.astral.sh/uv/).

```sh
cd python
uv sync --group dev
```

Run checks and tests with:

```sh
uv run poe check-all
uv run poe test
```

# LICENSE

The 3-Clause BSD License. See also LICENSE file.

- [PyO3](https://github.com/PyO3/pyo3) is licensed under the Apache-2.0 license.
- [Python](https://github.com/python/cpython) is licensed under the Python License.
