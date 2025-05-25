use polars::prelude::*;

pub fn calculate_statistics(df: &DataFrame) -> PolarsResult<()> {
    // 数値列の統計を計算
    let numeric_cols = ["temperature", "humidity"];
    
    for col_name in numeric_cols {
        if let Ok(_series) = df.column(col_name) {
            println!("=== {} の統計情報 ===", col_name);
            
            // 個別の統計量を計算
            let stats = df.clone().lazy()
                .select([
                    col(col_name).count().alias("count"),
                    col(col_name).mean().alias("mean"),
                    col(col_name).median().alias("median"),
                    col(col_name).std(1).alias("std"),
                    col(col_name).min().alias("min"),
                    col(col_name).max().alias("max"),
                ])
                .collect()?;
            
            println!("{}", stats);
        }
    }
    
    Ok(())
}