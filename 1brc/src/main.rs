use polars::prelude::*;
use std::time::Instant;

fn main() -> PolarsResult<()> {
    let now = Instant::now();
    let schema = Schema::from_iter(vec![
        Field::new("city", DataType::String),
        Field::new("temparature", DataType::Float64),
    ]);

    let df = CsvReader::from_path("C:/Users/bhagirathi/1brc/1brc/measurements.txt")?
        .with_separator(b';')
        .has_header(false)
        .with_schema(Some(Arc::new(schema)))
        .finish()?;

    println!("{}", df.head(Some(5)));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}
