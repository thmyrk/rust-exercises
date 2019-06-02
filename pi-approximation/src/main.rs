// The aim of the kata is to try to show how difficult it can be to calculate decimals of an irrational number with a certain precision.
// We have chosen to get a few decimals of the number "pi" using the following infinite series (Leibniz 1646–1716):

// PI / 4 = 1 - 1/3 + 1/5 - 1/7 + ... which gives an approximation of PI / 4.

// http://en.wikipedia.org/wiki/Leibniz_formula_for_%CF%80
// To have a measure of the difficulty we will count how many iterations are needed to calculate PI with a given precision.
// There are several ways to determine the precision of the calculus but to keep things easy we will calculate to within epsilon
// of your language Math::PI constant. In other words we will stop the iterative process when the absolute value of the difference
// between our calculation and the Math::PI constant of the given language is less than epsilon.
// Your function returns an array or an arryList or a string or a tuple depending on the language (See sample tests) where your approximation of PI has 10 decimals
// In Haskell you can use the function "trunc10Dble" (see "Your solution"); in Clojure you can use the function "round" (see "Your solution");
// in OCaml or Rust the function "rnd10" (see "Your solution") in order to avoid discusssions about the result.

fn rnd10(f: f64) -> f64 { (f * 1e10).round() / 1e10 }

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut i: i32 = 1;
    let mut result: f64 = 1.0;
    while (std::f64::consts::PI - result * 4.0).abs() > epsilon {
        result += (-1.0_f64).powi(i) / (i * 2 + 1) as f64;
        i += 1;
    }
    (i, rnd10(result * 4.0))
}

// task ends here

fn testing(epsilon: f64, exp: (i32, f64)) -> () {
    assert_eq!(iter_pi(epsilon), exp)
}

#[test]
fn run_tests() {
    testing(0.1, (10, 3.0418396189));
    testing(0.01,  (100, 3.1315929036));
    testing(0.001,  (1000, 3.1405926538));
    testing(0.0001,  (10000, 3.1414926536));
    testing(0.00001, (100001, 3.1416026535));
    testing(0.000001,  (1000001, 3.1415936536));
    testing(0.05,  (20, 3.0916238067));
}
