fn calculate<T, U>(height: T, weight: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    let h = height.into() / 100.0;
    let bmi = weight.into() / (h * h);
    (bmi * 10.0).round() / 10.0
}

// https://rust-lang.tw/book-tw/ch11-03-test-organization.html
// 單元測試常見的做法是：
// 1. 單元測試放在 src 目錄中每個你要測試的程式同個檔案下
// 2. 在每個檔案建立一個模組 tests 來包含測試函式
// 3. 用 cfg(test) 來詮釋模組，因為單元測試與程式碼位於相同的檔案下，
// 你需要使用 #[cfg(test)] 來指明它們不應該被（cargo build）包含在編譯結果

#[cfg(test)]
mod tests {
    // 下層模組 tests 可以使用該項目以上的模組 bmi
    // 使用 super:: 來引用上層模組
    use super::calculate;

    #[test]
    fn test_calculate() {
        // 使用絕對路徑引用 bmi 模組
        // let result = crate::bmi::calculate(180, 65);

        // 使用 super:: 來引用上層模組
        // let result = super::calculate(180, 65);

        // 下層模組可以使用上層模組的私有函式
        let result = calculate(180, 65);
        assert_eq!(20.1, result);
    }
}
