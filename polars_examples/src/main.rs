use polars::lazy::prelude::*;
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = df![
        "range" => [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        "left" => (0..10).map(|_| Some("foo")).collect::<Vec<_>>(),
        "right" => (0..10).map(|_| Some("bar")).collect::<Vec<_>>()
    ]?;

    let new = df
        .lazy()
        .with_column(
            when(col("range").gt_eq(lit(5)))
                .then(col("left"))
                .otherwise(col("right"))
                .alias("foo_or_bar"),
        )
        .collect()?;

    println!("{new:?}");
    Ok(())
}
