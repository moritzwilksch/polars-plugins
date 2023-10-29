use polars::prelude::*;
use pyo3_polars::derive::polars_expr;

#[polars_expr(output_type=Utf8)]
fn process(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].utf8()?;

    let mut result: Vec<String> = Vec::new();

    for value in ca {
        match value {
            Some(v) => {
                result.push(v.replace("a", "b"));
            }
            None => {}
        }
    }

    Ok(result.into_iter().collect())
}
