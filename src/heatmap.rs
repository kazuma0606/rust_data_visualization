use plotters::prelude::*;

pub fn create_heatmap() -> Result<(), Box<dyn std::error::Error>> {
    // 相関行列データの作成（実際のプロジェクトでは計算により取得）
    let correlation_data = vec![
        vec![1.0, 0.8, -0.6, 0.3],
        vec![0.8, 1.0, -0.4, 0.5],
        vec![-0.6, -0.4, 1.0, -0.2],
        vec![0.3, 0.5, -0.2, 1.0],
    ];
    
    let labels = vec!["温度", "湿度", "風速", "気圧"];
    
    let root = BitMapBackend::new("heatmap.png", (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    
    let mut chart = ChartBuilder::on(&root)
        .caption("相関行列ヒートマップ", ("sans-serif", 30))
        .margin(50)
        .build_cartesian_2d(0..4, 0..4)?;
    
    // ヒートマップを描画
    for (i, row) in correlation_data.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            let color_intensity = ((value + 1.0) / 2.0 * 255.0) as u8;
            let color = RGBColor(255 - color_intensity, 100, color_intensity);
            
            chart.draw_series(std::iter::once(Rectangle::new(
                [(j as i32, i as i32), (j as i32 + 1, i as i32 + 1)],
                color.filled(),
            )))?;
            
            // 値をテキストで表示
            chart.draw_series(std::iter::once(Text::new(
                format!("{:.2}", value),
                (j as i32, i as i32),
                ("sans-serif", 15).into_font().color(&BLACK),
            )))?;
        }
    }
    
    // 修正版：正しい型シグネチャ
    chart.configure_mesh()
        .x_label_formatter(&|x: &i32| -> String {
            labels.get(*x as usize).unwrap_or(&"").to_string()
        })
        .y_label_formatter(&|y: &i32| -> String {
            labels.get(*y as usize).unwrap_or(&"").to_string()
        })
        .draw()?;
    
    root.present()?;
    println!("ヒートマップを 'heatmap.png' に保存しました");
    Ok(())
}