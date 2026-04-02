pub fn second_largest(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 { 
        return None; 
    }

    let mut largest = i32::MIN;
    let mut second = i32::MIN;

    for &num in numbers {
        if num > largest {
            second = largest;
            largest = num;
        } else if num < largest && num > second {
            second = num;
        }
    }

    if second == i32::MIN { 
        None
    } else { 
        Some(second)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_same() {
        let result = second_largest(&[1, 1, 1]);
        assert_eq!(result, None);
    }

    #[test]
    fn test_reg_value() {
        let result = second_largest(&[1, 2, 3, 2]); 
        assert_eq!(result, Some(2)); 
    }


    #[test]
    fn test_pos_neg_values() {
        let result = second_largest(&[-1, 1]); 
        assert_eq!(result, Some(-1)); 
    }

    #[test]
    fn test_neg_neg_values() {
        let result = second_largest(&[-2, -3]); 
        assert_eq!(result, Some(-3)); 
    }

     #[test]
    fn test_two_value_same() {
        let result = second_largest(&[1, 1, 3, 3]); 
        assert_eq!(result, Some(1))
    }

    #[test] 
    fn test_big_value_first_with_dup_big_value() {
        let result = second_largest(&[5, 3, 2, 5]); 
        assert_eq!(result, Some(3)); 
    }

    #[test]
    fn test_empty_array() {
        let result = second_largest(&[]); 
        assert_eq!(result, None)
    }

    #[test]
    fn test_single_valued_array() {
        let result = second_largest(&[2]); 
        assert_eq!(result, None)
    }
}