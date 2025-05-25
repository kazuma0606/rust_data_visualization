use plotters::prelude::*;
use polars::prelude::*;

pub fn time_series_analysis() -> Result<(), Box<dyn std::error::Error>> {
    // 時系列データの作成
    let dates = vec![
        "2023-01-01", "2023-01-02", "2023-01-03", "2023-01-04", "2023-01-05",
        "2023-01-06", "2023-01-07", "2023-01-08", "2023-01-09", "2023-01-10"
    ];
    
    let values = vec![100.0, 105.2, 98.7, 110.1, 107.3, 112.8, 108.9, 115.2, 119.7, 122.4];
    
    let df = df! {
        "date" => dates,
        "value" => values,
    }?;
    
    // 手動で移動平均を計算する方法
    let df_with_ma = calculate_moving_average(&df, 3)?;
    
    println!("移動平均付きデータ:");
    println!("{}", df_with_ma);
    
    // 可視化
    create_time_series_chart(&df_with_ma)?;
    
    Ok(())
}

// 手動で移動平均を計算する関数
// より直感的な実装
fn calculate_moving_average(df: &DataFrame, window: usize) -> PolarsResult<DataFrame> {
    if window == 0 {
        return Err(PolarsError::InvalidOperation("Window size cannot be zero".into()));
    }
    
    let values: Vec<f64> = df.column("value")?
        .f64()?
        .into_no_null_iter()
        .collect();
    
    let mut ma_values = Vec::new();
    
    for i in 0..values.len() {
        if i < window - 1 {
            // 最初の (window-1) 個の要素は移動平均を計算できない
            ma_values.push(None);
        } else {
            // window個の要素の平均を計算
            let mut sum = 0.0;
            for j in 0..window {
                sum += values[i - j];
            }
            ma_values.push(Some(sum / window as f64));
        }
    }
    
    // 元のDataFrameに移動平均列を追加
    let ma_series = Series::new(PlSmallStr::from("ma_3"), ma_values);
    let mut result_df = df.clone();
    result_df.with_column(ma_series)?;
    
    Ok(result_df)
}
pub fn create_time_series_chart(df: &DataFrame) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("time_series.png", (1000, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let values: Vec<f64> = df.column("value")?
        .f64()?
        .into_no_null_iter()
        .collect();
    
    let ma_values: Vec<Option<f64>> = df.column("ma_3")?
        .f64()?
        .into_iter()
        .collect();
    
    let mut chart = ChartBuilder::on(&root)
        .caption("時系列データと移動平均", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..values.len(), 90f64..130f64)?;
    
    chart.configure_mesh()
        .x_desc("日")
        .y_desc("値")
        .draw()?;
    
    // 元データをプロット
    chart.draw_series(LineSeries::new(
        values.iter().enumerate().map(|(i, &val)| (i, val)),
        &BLUE,
    ))?
    .label("元データ")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));
    
    // 移動平均をプロット
    chart.draw_series(LineSeries::new(
        ma_values.iter().enumerate()
            .filter_map(|(i, &ma)| ma.map(|val| (i, val))),
        &RED,
    ))?
    .label("3日移動平均")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
    
    chart.configure_series_labels().draw()?;
    root.present()?;
    
    println!("時系列グラフを 'time_series.png' に保存しました");
    Ok(())
}