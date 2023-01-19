from kitsuyui_rust_playground import my_calc


def test_my_calc() -> None:
    assert my_calc(1, 2, 3) == '9'
