import kitsuyui_rust_playground as krp
import pytest


def test_multiply_sum() -> None:
    assert krp.multiply_sum(1, 2, 3) == "9"


def test_multiply_sum_overflow_raises_overflow_error() -> None:
    with pytest.raises(OverflowError):
        krp.multiply_sum(2**62, 2**62, 1)


def test_exports() -> None:
    assert krp.__all__ == ["multiply_sum"]
