import kitsuyui_rust_playground as krp


def test_multiply_sum() -> None:
    assert krp.multiply_sum(1, 2, 3) == "9"


def test_exports() -> None:
    assert krp.__all__ == ["multiply_sum"]
