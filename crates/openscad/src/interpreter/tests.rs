#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use caustic_core::random_new;

    use crate::{
        interpreter::{InterpreterError, InterpreterResults, openscad_interpret},
        parser::openscad_parse,
        source::StringSource,
        tokenizer::openscad_tokenize,
    };

    fn interpret(expr: &str) -> InterpreterResults {
        let source = Arc::new(StringSource::new(expr));
        let result = openscad_parse(openscad_tokenize(source).unwrap());
        let random = random_new();
        openscad_interpret(result.statements, random)
    }

    fn assert_output(expr: &str, expected: &str) {
        let result = interpret(expr);
        assert_eq!(result.output, expected);
    }

    fn assert_output_trim(expr: &str, expected: &str) {
        let result = interpret(expr);
        assert_eq!(result.output.trim(), expected);
    }

    // -- addition ----------------------------

    #[test]
    fn test_binary_expression_scalar_addition() {
        assert_output_trim("echo(20 + 0.1);", "20.1");
    }

    #[test]
    fn test_binary_expression_vector_addition() {
        assert_output_trim("echo([5, 8, -12] + [3, -4, 18]);", "[8, 4, 6]");
    }

    #[test]
    fn test_binary_expression_vector_addition_left_smaller() {
        assert_output_trim("echo([5, 8] + [3, -4, 18]);", "[8, 4]");
    }

    #[test]
    fn test_binary_expression_vector_addition_right_smaller() {
        assert_output_trim("echo([5, 8, -12] + [3, -4]);", "[8, 4]");
    }

    // -- subtraction ----------------------------

    #[test]
    fn test_binary_expression_scalar_subtraction() {
        assert_output_trim("echo(20 - 0.1);", "19.9");
    }

    #[test]
    fn test_binary_expression_vector_subtraction() {
        assert_output_trim("echo([5, 8, -12] - [3, -4, 18]);", "[2, 12, -30]");
    }

    #[test]
    fn test_binary_expression_vector_subtraction_left_smaller() {
        assert_output_trim("echo([5, 8] - [3, -4, 18]);", "[2, 12]");
    }

    #[test]
    fn test_binary_expression_vector_subtraction_right_smaller() {
        assert_output_trim("echo([5, 8, -12] - [3, -4]);", "[2, 12]");
    }

    // -- division ----------------------------

    #[test]
    fn test_binary_expression_scalar_division() {
        assert_output_trim("echo(20 / 4);", "5");
    }

    #[test]
    fn test_binary_expression_divide_vector_by_scaler() {
        assert_output_trim("echo([5, 8, -12] / 4);", "[1.25, 2, -3]");
    }

    #[test]
    fn test_binary_expression_divide_scaler_by_vector() {
        assert_output_trim("echo(4 / [5, 8, -12]);", "[0.8, 0.5, -0.333333]");
    }

    // -- multiplication ----------------------------

    #[test]
    fn test_binary_expression_scalar_multiplication() {
        assert_output_trim("echo(20 * 4);", "80");
    }

    #[test]
    fn test_binary_expression_multiply_vector_by_scaler() {
        assert_output_trim("echo([5, 8, -12] * 4);", "[20, 32, -48]");
        assert_output_trim("echo(4 * [5, 8, -12]);", "[20, 32, -48]");
    }

    // -- modulus ----------------------------

    #[test]
    fn test_binary_expression_scalar_modulus() {
        assert_output_trim("echo(20 % 3);", "2");
    }

    // -- exponentiation ----------------------

    #[test]
    fn test_binary_expression_scalar_exponentiation() {
        assert_output_trim("echo(20 ^ 3);", "8000");
    }

    // -- comparison ----------------------

    #[test]
    fn test_binary_expression_less_equal() {
        assert_output_trim("echo(1 <= 2);", "true");
        assert_output_trim("echo(2 <= 2);", "true");
        assert_output_trim("echo(3 <= 2);", "false");
    }

    #[test]
    fn test_binary_expression_less() {
        assert_output_trim("echo(1 < 2);", "true");
        assert_output_trim("echo(2 < 2);", "false");
        assert_output_trim("echo(3 < 2);", "false");
    }

    #[test]
    fn test_binary_expression_greater_equal() {
        assert_output_trim("echo(1 >= 2);", "false");
        assert_output_trim("echo(2 >= 2);", "true");
        assert_output_trim("echo(3 >= 2);", "true");
    }

    #[test]
    fn test_binary_expression_greater() {
        assert_output_trim("echo(1 > 2);", "false");
        assert_output_trim("echo(2 > 2);", "false");
        assert_output_trim("echo(3 > 2);", "true");
    }

    #[test]
    fn test_binary_expression_equals() {
        assert_output_trim("echo(1 == 2);", "false");
        assert_output_trim("echo(2 == 2);", "true");
        assert_output_trim("echo([1,2] == [2,2]);", "false");
        assert_output_trim("echo([1,2] == [1,2]);", "true");
    }

    #[test]
    fn test_binary_expression_not_equals() {
        assert_output_trim("echo(1 != 2);", "true");
        assert_output_trim("echo(2 != 2);", "false");
        assert_output_trim("echo([1,2] != [2,2]);", "true");
        assert_output_trim("echo([1,2] != [1,2]);", "false");
    }

    #[test]
    fn test_binary_expression_and() {
        assert_output_trim("echo(true && true);", "true");
        assert_output_trim("echo(true && false);", "false");
        assert_output_trim("echo(1 && true);", "true");
        assert_output_trim("echo(true && 1);", "true");
    }

    #[test]
    fn test_binary_expression_or() {
        assert_output_trim("echo(true || true);", "true");
        assert_output_trim("echo(true || false);", "true");
        assert_output_trim("echo(false || false);", "false");
    }

    #[test]
    fn test_boolean_negation() {
        assert_output_trim("echo(!true);", "false");
        assert_output_trim("echo(!false);", "true");
        assert_output_trim("echo(!1);", "false");
        assert_output_trim("echo(!0);", "true");
    }

    // -- type test --------------------------

    #[test]
    fn test_type_test_is_undef() {
        assert_output_trim("echo(is_undef(1));", "false");
        assert_output_trim("echo(is_undef(undef));", "true");
    }

    #[test]
    fn test_type_test_is_bool() {
        assert_output_trim("echo(is_bool(1));", "false");
        assert_output_trim("echo(is_bool(false));", "true");
    }

    #[test]
    fn test_type_test_is_num() {
        assert_output_trim("echo(is_num(true));", "false");
        assert_output_trim("echo(is_num(4));", "true");
    }

    #[test]
    fn test_type_test_is_string() {
        assert_output_trim("echo(is_string(1));", "false");
        assert_output_trim("echo(is_string(\"a\"));", "true");
    }

    #[test]
    fn test_type_test_is_list() {
        assert_output_trim("echo(is_list(1));", "false");
        assert_output_trim("echo(is_list([1,2]));", "true");
    }

    #[test]
    fn test_type_test_is_function() {
        assert_output_trim("echo(is_function(1));", "false");
        assert_output_trim("function a() = 1; echo(is_function(a));", "true");
    }

    // -- negation ----------------------------

    #[test]
    fn test_unary_expression_negation() {
        assert_output_trim("echo(-20);", "-20");
    }

    // -- order of operations ----------------------------

    #[test]
    fn test_order_of_operations_multiplication_first() {
        assert_output_trim("echo(2 + 3 * -5);", "-13");
    }

    #[test]
    fn test_order_of_operations_left_to_right() {
        assert_output_trim("echo(2 * 3 + 5);", "11");
    }

    #[test]
    fn test_order_of_operations_with_comparison() {
        assert_output_trim("echo(2 + 3 * 5 < 15);", "false");
    }

    // -- set variables ----------------------------

    #[test]
    fn test_set_fa() {
        let result = interpret("$fa = 1;");
        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
    }

    // -- for loop ----------------------------

    #[test]
    fn test_for_loop() {
        assert_output(
            "
                for(a = [-1 : 1])
                    for(b = [0 : 2])
                        echo(a,b);
            ",
            "-1, 0\n-1, 1\n0, 0\n0, 1\n",
        );
    }

    // -- rands ----------------------------

    #[test]
    fn test_rands() {
        let result = interpret("choose_mat = rands(0,1,1)[0];");
        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
    }

    // -- function ----------------------------

    #[test]
    fn test_function() {
        assert_output_trim("
            function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));
            echo(distance([7, 4, 3], [17, 6, 2]));
        ","10.246951");
    }

    // -- echo ----------------------------

    #[test]
    fn test_echo_string() {
        assert_output_trim(r#"echo("ok\ntest");"#, "\"ok\\ntest\"");
    }

    // -- if/else ----------------------------

    #[test]
    fn test_if_else() {
        assert_output_trim(
            r#"
                if (1 > 2) {
                echo("false");
                } else if (5 > 2) {
                echo("ok");
                } else {
                echo("fail");
                }
            "#,
            "\"ok\"",
        );
    }

    // -- ternary ----------------------------

    #[test]
    fn test_ternary() {
        assert_output_trim(r#"echo(1 > 2 ? "false" : "ok");"#, "\"ok\"");
    }

    // -- constants ----------------------------

    #[test]
    fn test_pi() {
        assert_output_trim(r#"echo(PI);"#, "3.141593");
    }

    #[test]
    fn test_undef() {
        assert_output_trim(r#"echo(undef);"#, "undef");
        assert_output_trim(r#"a = undef; echo(a);"#, "undef");
    }

    // -- strings ----------------------------

    #[test]
    fn test_index_string() {
        assert_output_trim(r#"a = "123"; echo(a[0]);"#, "\"1\"");
    }

    #[test]
    fn test_index_out_of_range_string() {
        assert_output_trim(r#"a = "123"; echo(a[3]);"#, "undef");
        assert_output_trim(r#"a = "123"; echo(a[-1]);"#, "undef");
    }

    // -- lists ----------------------------

    #[test]
    fn test_index() {
        assert_output_trim(r#"a = [1,2,3]; echo(a[0]);"#, "1");
    }

    #[test]
    fn test_index_xyz() {
        assert_output_trim(r#"a = [1,2,3]; echo(a.x);"#, "1");
        assert_output_trim(r#"a = [1,2,3]; echo(a.y);"#, "2");
        assert_output_trim(r#"a = [1,2,3]; echo(a.z);"#, "3");
    }

    #[test]
    fn test_index_out_of_range() {
        assert_output_trim(r#"a = [1,2,3]; echo(a[3]);"#, "undef");
        assert_output_trim(r#"a = [1,2,3]; echo(a[-1]);"#, "undef");
    }

    // -- functions ----------------------------

    #[test]
    fn test_concat() {
        assert_output_trim(
            r#"echo(concat("a","b","c","d","e","f"));"#,
            r#"["a", "b", "c", "d", "e", "f"]"#,
        );
        assert_output_trim(
            r#"echo(concat(["a","b","c"],["d","e","f"]));"#,
            r#"["a", "b", "c", "d", "e", "f"]"#,
        );
        assert_output_trim(r#"echo(concat(1,2,3,4,5,6));"#, r#"[1, 2, 3, 4, 5, 6]"#);
        assert_output_trim(
            r#"echo(concat([ [1],[2] ], [ [3] ]));"#,
            r#"[[1], [2], [3]]"#,
        );
        assert_output_trim(r#"echo(concat([1,2,3],[4,5,6]));"#, r#"[1, 2, 3, 4, 5, 6]"#);
        assert_output_trim(r#"echo(concat("abc","def"));"#, r#"["abc", "def"]"#);
    }

    // -- math ----------------------------

    #[test]
    fn test_abs() {
        assert_output_trim(r#"echo(abs(-1));"#, "1");
    }

    #[test]
    fn test_sign() {
        assert_output_trim(r#"echo(sign(x=-5));"#, "-1");
        assert_output_trim(r#"echo(sign(0));"#, "0");
        assert_output_trim(r#"echo(sign(8));"#, "1");
    }

    #[test]
    fn test_sin() {
        assert_output_trim(r#"echo(sin(degrees=35));"#, "0.573576");
    }

    #[test]
    fn test_cos() {
        assert_output_trim(r#"echo(cos(degrees=35));"#, "0.819152");
    }

    #[test]
    fn test_tan() {
        assert_output_trim(r#"echo(tan(degrees=35));"#, "0.700208");
    }

    #[test]
    fn test_asin() {
        assert_output_trim(r#"echo(asin(x=0.57357643635));"#, "35");
    }

    #[test]
    fn test_acos() {
        assert_output_trim(r#"echo(acos(x=0.81915204428));"#, "35");
    }

    #[test]
    fn test_atan() {
        assert_output_trim(r#"echo(atan(x=0.70020753821));"#, "35");
        assert_output_trim(r#"echo(atan(-1));"#, "-45");
    }

    #[test]
    fn test_atan2() {
        assert_output_trim(r#"echo(atan2(y=5.0,x=-5.0));"#, "135");
    }

    #[test]
    fn test_floor() {
        assert_output_trim(r#"echo(floor(1.9));"#, "1");
    }

    #[test]
    fn test_round() {
        assert_output_trim(r#"echo(round(1.1));"#, "1");
        assert_output_trim(r#"echo(round(1.5));"#, "2");
    }

    #[test]
    fn test_ceil() {
        assert_output_trim(r#"echo(ceil(1.2));"#, "2");
    }

    #[test]
    fn test_ln() {
        assert_output_trim(r#"echo(ln(105.3));"#, "4.656813");
    }

    #[test]
    fn test_log() {
        assert_output_trim(r#"echo(log(105.3));"#, "2.022428");
    }

    #[test]
    fn test_pow() {
        assert_output_trim(r#"echo(pow(base=2,exponent=3));"#, "8");
    }

    #[test]
    fn test_sqrt() {
        assert_output_trim(r#"echo(sqrt(9));"#, "3");
    }

    #[test]
    fn test_exp() {
        assert_output_trim(r#"echo(exp(4.2));"#, "66.686331");
    }

    #[test]
    fn test_min() {
        assert_output_trim(r#"echo(min(1));"#, "1");
        assert_output_trim(r#"echo(min(3,2));"#, "2");
        assert_output_trim(r#"echo(min([5,4,2]));"#, "2");
    }

    #[test]
    fn test_max() {
        assert_output_trim(r#"echo(max(1));"#, "1");
        assert_output_trim(r#"echo(max(3,2));"#, "3");
        assert_output_trim(r#"echo(max([5,4,2]));"#, "5");
    }

    #[test]
    fn test_norm() {
        assert_output_trim(r#"echo(norm([1,2,3]));"#, "3.741657");
        assert_output_trim(r#"echo(norm([1,2,3,4,5,6]));"#, "9.539392");
        assert_output_trim(r#"echo(norm([]));"#, "0");
    }

    #[test]
    fn test_cross() {
        assert_output_trim(r#"echo(cross([2, 3, 4], [5, 6, 7]));"#, "[-3, 6, -3]");
        assert_output_trim(r#"echo(cross([2, 1, -3], [0, 4, 5]));"#, "[17, -10, 8]");
        assert_output_trim(r#"echo(cross([2, 1], [0, 4]));"#, "8");
        assert_output_trim(r#"echo(cross([1, -3], [4, 5]));"#, "17");
        assert_output_trim(r#"echo(cross([2, 1, -3], [4, 5]));"#, "undef");
        assert_output_trim(r#"echo(cross([2, 3, 4], "5"));"#, "undef");
    }
}
