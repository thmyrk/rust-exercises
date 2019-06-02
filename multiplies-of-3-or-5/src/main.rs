//  If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//  Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in.
//  Note: If the number is a multiple of both 3 and 5, only count it once.
//
//  I. if 3 < number; add it; add 3 to checked number; loop; else break
//  II. iterations = number / 3; iterations.times += 3
//  III. sequence: An = 3n; A1 = 3 * 1; Alast = 3 * (number / 3); Sum = ((A1 + Alast) / 2) * (number / 3)
//       Bn = 3*5*n;

fn solution(num: i32) -> i32 {
    let threes_sum = sum_of_sequence(num, 3);
    let fives_sum = sum_of_sequence(num, 5);
    let common_numbers_sum = sum_of_sequence(num, 15);

    threes_sum + fives_sum - common_numbers_sum
}

fn sum_of_sequence(max: i32, a: i32) -> i32 {
    let number_of_elements_in_sequence = (num - 1) / 3;
    let first_number_of_sequence = 3 * 1;
    let last_number_of_sequence = 3 * number_of_elements_in_sequence;

    (first_number_of_sequence + last_number_of_sequence) * 10 / 2 * number_of_elements_in_sequence / 10;
}

#[test]
fn run_tests() {
    assert_eq!(solution(10), 23);
    assert_eq!(solution(11), 33); // 3 + 6 + 9 + 5 + 10
    assert_eq!(solution(6), 8);
    assert_eq!(solution(33), 225);
}
