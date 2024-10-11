// src/visualization.rs

use crate::data_processing::DataSummary;
use plotters::prelude::*;

pub fn create_charts(summary: &DataSummary, column_name: &str) {
    let output_path = format!("output/{}_summary_chart.svg", column_name);

    let root = SVGBackend::new(&output_path, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    // Determine the maximum value for scaling the chart
    let max_value = summary.mean.max(summary.variance) * 1.2;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Statistical Summary of '{}'", column_name),
            ("sans-serif", 40).into_font(),
        )
        .margin(10)
        .set_left_and_bottom_label_area_size(50)
        .build_cartesian_2d(0..3, 0.0..max_value)
        .unwrap();

    chart
        .configure_mesh()
        .disable_mesh()
        .x_labels(2)
        .x_desc("Statistic")
        .y_desc("Value")
        .x_label_formatter(&|x| match *x {
            1 => "Mean".to_string(),
            2 => "Variance".to_string(),
            _ => "".to_string(),
        })
        .draw()
        .unwrap();

    // Plot mean as a bar
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(1 - 0.25, 0.0), (1 + 0.25, summary.mean)],
            RED.filled(),
        )))
        .unwrap()
        .label("Mean")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], RED.filled()));

    // Plot variance as a bar
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(2 - 0.25, 0.0), (2 + 0.25, summary.variance)],
            BLUE.filled(),
        )))
        .unwrap()
        .label("Variance")
        .legend(|(x, y)| Rectangle::new([(x, y - 5), (x + 10, y + 5)], BLUE.filled()));

    // Draw the legend
    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()
        .unwrap();

    println!("Chart saved to {}", output_path);
}