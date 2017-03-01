#[derive(Debug)]
struct BigDecimal {
    int_value: u64,
    scale: i32,
    precision: u8
}

impl BigDecimal {
    fn add(&self, augend: &BigDecimal) -> BigDecimal {
        let scale_diff = self.scale - augend.scale;
        let (sum_value, scale) = if scale_diff == 0 {
            (self.int_value + augend.int_value, self.scale)
        } else if scale_diff < 0 {
            (self.multiply_power_ten(scale_diff.abs() as u32) + augend.int_value, augend.scale)
        } else {
            (self.int_value + augend.multiply_power_ten(scale_diff as u32), self.scale)
        };
        BigDecimal{
            int_value: sum_value,
            scale: scale,
            precision: (sum_value as f64).log(10.0).floor() as u8 + 1
        }
    }

    fn multiply_power_ten(&self, n: u32) -> u64 {
        if n <= 0 {
            self.int_value
        } else {
            self.int_value * self.ten_to_ten(n)
        }
    }

    fn ten_to_ten(&self, n: u32) -> u64 {
        (10 as u64).pow(n)
    }
}

impl PartialEq for BigDecimal {
    fn eq(&self, other: &BigDecimal) -> bool {
        self.int_value == other.int_value &&
            self.precision == other.precision &&
            self.scale == other.scale
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn big_decimal_add() {
        let bd1 = BigDecimal{ int_value: 19, scale: 0, precision: 2 };
        let bd2 = BigDecimal{ int_value: 19, scale: 1, precision: 2 };
        let expected = BigDecimal{ int_value: 209, scale: 1, precision: 3 };
        assert_eq!(expected, bd1.add(&bd2));
    }
}
