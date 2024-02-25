use crate::utils::print_pass;

const NAME: &str = "plus-one";


pub fn multiply(num1: String, num2: String) -> String {
    let len1 = num1.len();
    let len2 = num2.len();
    let digits = {
        let mut digits = vec![0; len1 + len2];
        for (idx1, ch1) in num1.chars().rev().enumerate() {
            for (idx2, ch2) in num2.chars().rev().enumerate() {
                let n1 = ch1 as i32 - '0' as i32;
                let n2 = ch2 as i32 - '0' as i32;
                let res = n1 * n2 + digits[idx1 + idx2];
                digits[idx1 + idx2] = res % 10;
                digits[idx1 + idx2 + 1] += res / 10;
            }
        }       
        while digits.len() > 1 && digits.last() == Some(&0){
            digits.pop();
        }
        digits
    };
    digits.into_iter().rev().map(|d| d.to_string()).collect::<String>()
}

pub fn main() {
    let result = multiply(String::from("123"), String::from("456"));
    assert_eq!(result, String::from("56088"));
    print_pass(NAME)
}
