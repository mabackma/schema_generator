use schema_generator::number::Number;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_u8() {
        assert_eq!(Number::Int(42).as_u8(), Some(42));
        assert_eq!(Number::Float(42.9).as_u8(), Some(42));
        assert_eq!(Number::Int(-1).as_u8(), None);
        assert_eq!(Number::Int(300).as_u8(), None); // Out of u8 range
    }

    #[test]
    fn test_as_u16() {
        assert_eq!(Number::Int(1000).as_u16(), Some(1000));
        assert_eq!(Number::Float(1000.5).as_u16(), Some(1000));
        assert_eq!(Number::Int(-1).as_u16(), None);
        assert_eq!(Number::Int(70000).as_u16(), None); // Out of u16 range
    }

    #[test]
    fn test_as_u32() {
        assert_eq!(Number::Int(1_000_000).as_u32(), Some(1_000_000));
        assert_eq!(Number::Float(1_000_000.5).as_u32(), Some(1_000_000));
        assert_eq!(Number::Int(-1).as_u32(), None);
    }

    #[test]
    fn test_as_i16() {
        assert_eq!(Number::Int(30000).as_i16(), Some(30000));
        assert_eq!(Number::Float(-30000.5).as_i16(), Some(-30000));
        assert_eq!(Number::Int(40000).as_i16(), None); // Out of i16 range
        assert_eq!(Number::Int(-40000).as_i16(), None); // Out of i16 range
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(Number::Int(2_000_000).as_i32(), Some(2_000_000));
        assert_eq!(Number::Float(-2_000_000.5).as_i32(), Some(-2_000_000));
        assert_eq!(Number::Int(i64::MAX).as_i32(), None); // Too large for i32
        assert_eq!(Number::Int(i64::MIN).as_i32(), None); // Too small for i32
    }

    #[test]
    fn test_as_i64() {
        assert_eq!(Number::Int(i64::MAX).as_i64(), Some(i64::MAX));
        assert_eq!(Number::Int(i64::MIN).as_i64(), Some(i64::MIN));
        assert_eq!(Number::Float(12345.9).as_i64(), Some(12345)); // Floats truncate
        assert_eq!(Number::Float(-12345.9).as_i64(), Some(-12345));
    }

    #[test]
    fn test_as_f32() {
        assert_eq!(Number::Int(42).as_f32(), Some(42.0));
        assert_eq!(Number::Float(42.5).as_f32(), Some(42.5));
        assert_eq!(Number::Float(f64::MAX).as_f32(), None); // Too large for f32
    }

    #[test]
    fn test_as_f64() {
        assert_eq!(Number::Int(42).as_f64(), 42.0);
        assert_eq!(Number::Float(42.5).as_f64(), 42.5);
        assert_eq!(Number::Float(f64::MAX).as_f64(), f64::MAX);
    }
}
