import polars as pl
from polars.utils.udfs import _get_shared_lib_location

# print(str(list(Path(__file__).parent.glob("*.so"))))
# print(__file__)
# lib_path = list(Path(__file__).parent.parent.glob("target/*/*/*.so"))
# lib = str(lib_path[0])
# print(lib_path)
lib = _get_shared_lib_location(__file__)
print("Lib:", lib)


@pl.api.register_expr_namespace("stringstuff")
class Stringstuff:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def process(self) -> pl.Expr:
        return self._expr._register_plugin(
            lib=lib,
            symbol="process",
            is_elementwise=True,
        )


data = pl.DataFrame({"a": ["foo", "bar", "ham"], "b": [1, 2, 3]})
print(data.select(pl.col("a").stringstuff.process()))
