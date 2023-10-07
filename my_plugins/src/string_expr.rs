use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

#[polars_expr(output_type=Utf8)]
fn process(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].utf8()?;

    let mut result: Vec<Option<String>> = Vec::new();

    for value in ca {
        match value {
            Some(v) => {
                result.push(Some(v.replace("a", "b")));
            }
            None => {
                result.push(None);
            }
        }
    }

    Ok(Series::new(ca.name(), result))
}
