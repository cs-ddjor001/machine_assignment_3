use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = 4.0 / 3.0;
    let b = a - 1_f64;
    let c = b * b * b;
    let eps = (1_f64 - c).abs();
    let sqrt_eps = eps.sqrt();

    println!("Approximate machine epsilon: {}", eps);
    println!("Square root of epsilon: {}", sqrt_eps);

    println!("|  h   |       x       | Approx. f'(x) |  Known f'(x)  |  Abs. Error   |");
    println!("|:----:|--------------:|--------------:|--------------:|--------------:|");

    let x: f64 = 1.0;
    let f_x = x.sin();
    let f_prime_x = x.cos();

    let mut data_points = vec![];

    for i in 1..31 {
        let h = 2.0_f64.powi(-i);
        let approx_f_prime_x = ((x + h).sin() - f_x) / h;
        let abs_error = (f_prime_x - approx_f_prime_x).abs();
        data_points.push((h, abs_error));

        println!(
            "|2^-{:02} |    {:.8} |    {:.8} |    {:.8} |    {:.8} |",
            i, x, approx_f_prime_x, f_prime_x, abs_error
        );
    }

    let root = BitMapBackend::new("log_log_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Log-Log Plot of h vs. Absolute Error", ("sans-serif", 20))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (1e-10f64..1.0f64).log_scale(),
            (1e-16f64..1.0f64).log_scale(),
        )?;

    chart
        .configure_mesh()
        .x_desc("h (step size)")
        .y_desc("Absolute Error")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart
        .draw_series(LineSeries::new(data_points, &RED))?
        .label("Error Curve")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .draw()?;

    root.present()?;
    println!("Plot saved as log_log_plot.png");

    Ok(())
}
