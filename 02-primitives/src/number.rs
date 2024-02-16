#[cfg(test)]
mod tests {
    #[test]
    fn floating_point_not_precise() {
        assert_eq!(0.30000000000000004_f32, 0.1_f32 + 0.2_f32);
        assert_eq!(0.30000000000000004_f64, 0.1_f64 + 0.2_f64);
    }
}
