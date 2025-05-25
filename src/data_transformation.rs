use polars::prelude::*;

pub fn column_selection() -> PolarsResult<DataFrame> {
    let df = df! {
        "date" => ["2023-01-01", "2023-01-02", "2023-01-03", "2023-01-04", "2023-01-05"],
        "temperature" => [20.5, 22.1, 19.8, 23.4, 21.2],
        "humidity" => [65, 70, 68, 72, 66],
        "city" => ["Tokyo", "Tokyo", "Osaka", "Osaka", "Tokyo"],
    }?;
    
    // 特定の列を選択
    let selected = df.select(["temperature", "humidity"])?;
    println!("選択されたデータ:");
    println!("{}", selected);
    
    Ok(selected)
}

pub fn data_filtering() -> PolarsResult<DataFrame> {
    let df = df! {
        "temperature" => [20.5, 22.1, 19.8, 23.4, 21.2],
        "humidity" => [65, 70, 68, 72, 66],
        "city" => ["Tokyo", "Tokyo", "Osaka", "Osaka", "Tokyo"],
    }?;
    
    // 温度が20度以上のデータをフィルタリング
    let mask = df.column("temperature")?
        .f64()?
        .gt(20.0);
    
    let filtered = df.filter(&mask)?;
    println!("フィルタリング後のデータ:");
    println!("{}", filtered);
    
    Ok(filtered)
}