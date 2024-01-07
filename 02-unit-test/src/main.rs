#[cfg(test)]
mod bmi_test {
    use crate::calculate;

    #[test]
    fn test_calculate() {
        let result = calculate(180, 65);
        assert_eq!(20.1, result);
    }
}

fn calculate<T, U>(height: T, weight: U) -> f64
where
    T: Into<f64>,
    U: Into<f64>,
{
    let h = height.into() / 100.0;
    let bmi = weight.into() / (h * h);
    (bmi * 10.0).round() / 10.0
}

fn main() {}
