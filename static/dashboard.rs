use plotters::prelude::*;

use crate::handler::energizz::dashboard; // Importe a estrutura necessária aqui

pub fn energizz(data: &dashboard) -> Result<(), Box<dyn std::error::Error>> {
    // Implemente a criação do gráfico com base nos dados recebidos.
    // Exemplo de código para criar um gráfico de linha:

    let root = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Temperature Chart", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..10f32, 0f32..100f32)?;

    chart.configure_mesh()
        .y_labels(5)
        .x_labels(10)
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let data_points: Vec<(f32, f32)> = data
        .temperature_data
        .iter()
        .enumerate()
        .map(|(x, y)| (x as f32, *y))
        .collect();

    chart.draw_series(LineSeries::new(data_points, &RED))?;
    Ok(())
}
