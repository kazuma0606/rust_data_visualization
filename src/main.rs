mod read_csv;
mod data_transformation;
mod lazy_operation;
mod calc_statistics;
mod data_visualization;
mod time_series;
mod heatmap;

fn main() {
    println!("Polars + plotters データ解析チュートリアル開始");

    // 1. データの読み込みと探索
    println!("");
    println!("1. データの読み込みと探索");
    let df = read_csv::read_csv_data().expect("CSVデータの読み込みに失敗しました");
    // println!("データフレームの内容:\n{:?}", df);
    read_csv::explore_data(&df).expect("データの探索に失敗しました");

    // 2. 列選択とフィルタリング
    println!("");
    println!("2. 列選択とフィルタリング");
    data_transformation::column_selection().expect("列選択に失敗しました");
    data_transformation::data_filtering().expect("データのフィルタリングに失敗しました");
    
    // 3. Lazy評価でのデータ変換
    println!("");
    let transformed_df = lazy_operation::lazy_operations().expect("Lazy評価でのデータ変換に失敗しました");
    println!("変換後のデータ:");
    println!("{:?}", transformed_df);

    // 4. グループ分析
    println!("");
    println!("4. グループ分析");
    let grouped_df = lazy_operation::group_analysis().expect("グループ分析に失敗しました");
    println!("都市別統計:");
    println!("{}", grouped_df);

    // 5. 統計情報の計算
    println!("");
    println!("5. 統計情報の計算");
    calc_statistics::calculate_statistics(&df).expect("統計情報の計算に失敗しました");

    // 6. データの可視化
    println!("");
    println!("6. データの可視化");
    data_visualization::create_line_chart(&df).expect("データの可視化に失敗しました");
    // data_visualization::create_bar_chart(&df).expect("棒グラフの作成に失敗しました");
    data_visualization::create_scatter_plot(&df).expect("散布図の作成に失敗しました");
    
    // 7. 時系列解析
    println!("");
    println!("7. 時系列解析");
    time_series::time_series_analysis().expect("時系列解析に失敗しました");
    
    // 8. ヒートマップ
    println!("");
    println!("8. ヒートマップの作成");
    heatmap::create_heatmap().expect("ヒートマップの作成に失敗しました");
    println!("データ解析チュートリアル完了");
}
