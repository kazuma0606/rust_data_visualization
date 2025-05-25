use polars::prelude::*;

pub fn lazy_operations() -> PolarsResult<DataFrame> {
    let df = df! {
        "date" => ["2023-01-01", "2023-01-02", "2023-01-03", "2023-01-04", "2023-01-05"],
        "temperature" => [20.5, 22.1, 19.8, 23.4, 21.2],
        "humidity" => [65, 70, 68, 72, 66],
        "city" => ["Tokyo", "Tokyo", "Osaka", "Osaka", "Tokyo"],
    }?;
    
    // Lazy評価でデータ変換
    let result = df.lazy()
        .with_columns([
            // 摂氏から華氏に変換
            (col("temperature") * lit(9.0) / lit(5.0) + lit(32.0)).alias("temp_fahrenheit"),
            // 快適性指数の計算（簡単な例）
            (col("temperature") * lit(0.7) + col("humidity") * lit(0.3)).alias("comfort_index"),
        ])
        .filter(col("temperature").gt(lit(20.0)))
        .sort(vec!["date"], SortMultipleOptions::default())
        .collect()?;
    
    Ok(result)
}

pub fn group_analysis() -> PolarsResult<DataFrame> {
    let df = df! {
        "city" => ["Tokyo", "Tokyo", "Osaka", "Osaka", "Tokyo", "Osaka"],
        "temperature" => [20.5, 22.1, 19.8, 23.4, 21.2, 24.1],
        "humidity" => [65, 70, 68, 72, 66, 74],
    }?;
    
    // 都市別の統計
    let grouped = df.lazy()
        .group_by([col("city")])
        .agg([
            col("temperature").mean().alias("avg_temp"),
            col("temperature").max().alias("max_temp"),
            col("temperature").min().alias("min_temp"),
            col("humidity").mean().alias("avg_humidity"),
            col("temperature").count().alias("count"),
        ])
        .collect()?;
    
    Ok(grouped)
}