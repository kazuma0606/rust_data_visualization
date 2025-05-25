use plotters::prelude::*;
use polars::prelude::*;

pub fn create_line_chart(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("line_chart.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let temperatures: Vec<f64> = df.column("temperature")?
        .f64()?
        .into_no_null_iter()
        .collect();
    
    let mut chart = ChartBuilder::on(&root)
        .caption("温度の推移", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..temperatures.len(), 15f64..25f64)?;
    
    chart.configure_mesh().draw()?;
    
    chart.draw_series(LineSeries::new(
        temperatures.iter().enumerate().map(|(i, &temp)| (i, temp)),
        &RED,
    ))?
    .label("温度")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
    
    chart.configure_series_labels().draw()?;
    root.present()?;
    
    println!("線グラフを 'line_chart.png' に保存しました");
    Ok(())
}

pub fn create_scatter_plot(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("scatter_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let temperatures: Vec<f64> = df.column("temperature")?
        .f64()?
        .into_no_null_iter()
        .collect();
    
    let humidities: Vec<i32> = df.column("humidity")?
        .i32()?
        .into_no_null_iter()
        .collect();
    
    let mut chart = ChartBuilder::on(&root)
        .caption("温度と湿度の関係", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(15f64..25f64, 60i32..80i32)?;
    
    chart.configure_mesh()
        .x_desc("温度 (°C)")
        .y_desc("湿度 (%)")
        .draw()?;
    
    chart.draw_series(
        temperatures.iter().zip(humidities.iter())
            .map(|(&temp, &hum)| Circle::new((temp, hum), 5, BLUE.filled()))
    )?
    .label("データポイント")
    .legend(|(x, y)| Circle::new((x + 10, y), 3, BLUE.filled()));
    
    chart.configure_series_labels().draw()?;
    root.present()?;
    
    println!("散布図を 'scatter_plot.png' に保存しました");
    Ok(())
}

// pub fn create_bar_chart(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
//     let root = BitMapBackend::new("bar_chart.png", (800, 600)).into_drawing_area();
//     root.fill(&WHITE)?;
    
//     let cities: Vec<String> = df.column("city")?
//         .str()?
//         .into_no_null_iter()
//         .map(|s| s.to_string())
//         .collect();
    
//     let avg_temps: Vec<f64> = df.column("avg_temp")?
//         .f64()?
//         .into_no_null_iter()
//         .collect();
    
//     let mut chart = ChartBuilder::on(&root)
//         .caption("都市別平均温度", ("sans-serif", 40))
//         .margin(10)
//         .x_label_area_size(40)
//         .y_label_area_size(40)
//         .build_cartesian_2d(0..cities.len(), 15f64..25f64)?;
    
//     chart.configure_mesh()
//         .x_desc("都市")
//         .y_desc("平均温度 (°C)")
//         .x_label_formatter(&|x| cities.get(*x).unwrap_or(&String::new()).clone())
//         .draw()?;
    
//     chart.draw_series(
//         avg_temps.iter().enumerate()
//             .map(|(i, &temp)| Rectangle::new([(i, 0.0), (i, temp)], GREEN.filled()))
//     )?
//     .label("平均温度")
//     .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], GREEN.filled()));
    
//     chart.configure_series_labels().draw()?;
//     root.present()?;
    
//     println!("棒グラフを 'bar_chart.png' に保存しました");
//     Ok(())
// }