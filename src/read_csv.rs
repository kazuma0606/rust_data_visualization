#![allow(unused)]
use polars::prelude::*;
use std::error::Error;

pub fn read_csv_data() -> Result<DataFrame, Box<dyn Error>> {
    // Define the path to the CSV file
    // サンプルデータの作成（実際のプロジェクトではCSVファイルから読み込み）
    let df = df! {
        "date" => ["2023-01-01", "2023-01-02", "2023-01-03", "2023-01-04", "2023-01-05"],
        "temperature" => [20.5, 22.1, 19.8, 23.4, 21.2],
        "humidity" => [65, 70, 68, 72, 66],
        "city" => ["Tokyo", "Tokyo", "Osaka", "Osaka", "Tokyo"],
    }?;

    // Print the DataFrame to verify it was read correctly
    println!("{:?}", df);

    // Return the DataFrame
    Ok(df)
}

pub fn read_from_csv() -> PolarsResult<DataFrame> {
    CsvReadOptions::default()
    .with_has_header(true)
    .try_into_reader_with_file_path(Some("sample.csv".into()))?
    .finish()
}

pub fn explore_data(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    // データの形状を確認
    println!("データ形状: {:?}", df.shape());
    
    // 最初の5行を表示
    println!("最初の3行:");
    println!("{}", df.head(Some(3)));
    
    // データ型を確認
    println!("データ型:");
    for (name, dtype) in df.schema().iter() {
        println!("{}: {:?}", name, dtype);
    }
    
    // 列名を取得
    println!("列名: {:?}", df.get_column_names());
    
    // 基本的な統計情報を手動で計算
    println!("基本情報:");
    for col_name in df.get_column_names() {
        if let Ok(series) = df.column(col_name) {
            println!("{}: 件数={}, Null数={}", col_name, series.len(), series.null_count());
        }
    }
    
    Ok(())
}