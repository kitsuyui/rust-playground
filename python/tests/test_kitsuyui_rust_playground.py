import kitsuyui_rust_playground as krp


def test_my_calc() -> None:
    assert krp.my_calc(1, 2, 3) == "9"


def test_exports() -> None:
    assert krp.__all__ == ["my_calc"]
