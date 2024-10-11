// src/data_processing.rs

use polars::prelude::*;

pub struct DataSummary {
    pub mean: f64,
    pub variance: f64,
}

pub fn process_data(file_path: &str, column_name: &str) -> DataSummary {
    // Read the CSV file into a DataFrame
    let df = CsvReader::from_path(file_path)
        .expect("Could not read CSV file")
        .infer_schema(None)
        .has_header(true)
        .finish()
        .expect("Failed to create DataFrame");

    // Select the specified column for analysis
    let series = df
        .column(column_name)
        .expect(&format!("Column '{}' not found", column_name));

    // Ensure the column is of a numeric type
    let series = series
        .cast(&DataType::Float64)
        .expect("Failed to cast column to Float64");

    // Compute mean
    let mean = series.mean().expect("Failed to compute mean");

    // Compute variance (ddof=1 for sample variance)
    let variance = series.var(1).expect("Failed to compute variance");

    println!("Mean of '{}': {}", column_name, mean);
    println!("Variance of '{}': {}", column_name, variance);

    DataSummary { mean, variance }
}