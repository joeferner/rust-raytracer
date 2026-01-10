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

    // -- addition ----------------------------

    #[test]
    fn test_binary_expression_scalar_addition() {
        assert_output("echo(20 + 0.1);", "20.1\n");
    }

    #[test]
    fn test_binary_expression_vector_addition() {
        assert_output("echo([5, 8, -12] + [3, -4, 18]);", "[8, 4, 6]\n");
    }

    #[test]
    fn test_binary_expression_vector_addition_left_smaller() {
        assert_output("echo([5, 8] + [3, -4, 18]);", "[8, 4]\n");
    }

    #[test]
    fn test_binary_expression_vector_addition_right_smaller() {
        assert_output("echo([5, 8, -12] + [3, -4]);", "[8, 4]\n");
    }

    // -- subtraction ----------------------------

    #[test]
    fn test_binary_expression_scalar_subtraction() {
        assert_output("echo(20 - 0.1);", "19.9\n");
    }

    #[test]
    fn test_binary_expression_vector_subtraction() {
        assert_output("echo([5, 8, -12] - [3, -4, 18]);", "[2, 12, -30]\n");
    }

    #[test]
    fn test_binary_expression_vector_subtraction_left_smaller() {
        assert_output("echo([5, 8] - [3, -4, 18]);", "[2, 12]\n");
    }

    #[test]
    fn test_binary_expression_vector_subtraction_right_smaller() {
        assert_output("echo([5, 8, -12] - [3, -4]);", "[2, 12]\n");
    }

    // -- division ----------------------------

    #[test]
    fn test_binary_expression_scalar_division() {
        assert_output("echo(20 / 4);", "5\n");
    }

    #[test]
    fn test_binary_expression_divide_vector_by_scaler() {
        assert_output("echo([5, 8, -12] / 4);", "[1.25, 2, -3]\n");
    }

    #[test]
    fn test_binary_expression_divide_scaler_by_vector() {
        assert_output("echo(4 / [5, 8, -12]);", "[0.8, 0.5, -0.333333]\n");
    }

    // -- multiplication ----------------------------

    #[test]
    fn test_binary_expression_scalar_multiplication() {
        assert_output("echo(20 * 4);", "80\n");
    }

    #[test]
    fn test_binary_expression_multiply_vector_by_scaler() {
        assert_output("echo([5, 8, -12] * 4);", "[20, 32, -48]\n");
        assert_output("echo(4 * [5, 8, -12]);", "[20, 32, -48]\n");
    }

    // -- modulus ----------------------------

    #[test]
    fn test_binary_expression_scalar_modulus() {
        assert_output("echo(20 % 3);", "2\n");
    }

    // -- exponentiation ----------------------

    #[test]
    fn test_binary_expression_scalar_exponentiation() {
        assert_output("echo(20 ^ 3);", "8000\n");
    }

    // -- comparison ----------------------

    #[test]
    fn test_binary_expression_less_equal() {
        assert_output("echo(1 <= 2);", "true\n");
        assert_output("echo(2 <= 2);", "true\n");
        assert_output("echo(3 <= 2);", "false\n");
    }

    #[test]
    fn test_binary_expression_less() {
        assert_output("echo(1 < 2);", "true\n");
        assert_output("echo(2 < 2);", "false\n");
        assert_output("echo(3 < 2);", "false\n");
    }

    #[test]
    fn test_binary_expression_greater_equal() {
        assert_output("echo(1 >= 2);", "false\n");
        assert_output("echo(2 >= 2);", "true\n");
        assert_output("echo(3 >= 2);", "true\n");
    }

    #[test]
    fn test_binary_expression_greater() {
        assert_output("echo(1 > 2);", "false\n");
        assert_output("echo(2 > 2);", "false\n");
        assert_output("echo(3 > 2);", "true\n");
    }

    #[test]
    fn test_binary_expression_equals() {
        assert_output("echo(1 == 2);", "false\n");
        assert_output("echo(2 == 2);", "true\n");
        assert_output("echo([1,2] == [2,2]);", "false\n");
        assert_output("echo([1,2] == [1,2]);", "true\n");
    }

    #[test]
    fn test_binary_expression_not_equals() {
        assert_output("echo(1 != 2);", "true\n");
        assert_output("echo(2 != 2);", "false\n");
        assert_output("echo([1,2] != [2,2]);", "true\n");
        assert_output("echo([1,2] != [1,2]);", "false\n");
    }

    #[test]
    fn test_binary_expression_and() {
        assert_output("echo(true && true);", "true\n");
        assert_output("echo(true && false);", "false\n");
        assert_output("echo(1 && true);", "true\n");
        assert_output("echo(true && 1);", "true\n");
    }

    #[test]
    fn test_binary_expression_or() {
        assert_output("echo(true || true);", "true\n");
        assert_output("echo(true || false);", "true\n");
        assert_output("echo(false || false);", "false\n");
    }

    #[test]
    fn test_boolean_negation() {
        assert_output("echo(!true);", "false\n");
        assert_output("echo(!false);", "true\n");
        assert_output("echo(!1);", "false\n");
        assert_output("echo(!0);", "true\n");
    }

    // -- type test --------------------------

    #[test]
    fn test_type_test_is_undef() {
        assert_output("echo(is_undef(1));", "false\n");
        assert_output("echo(is_undef(undef));", "true\n");
    }

    #[test]
    fn test_type_test_is_bool() {
        assert_output("echo(is_bool(1));", "false\n");
        assert_output("echo(is_bool(false));", "true\n");
    }

    #[test]
    fn test_type_test_is_num() {
        assert_output("echo(is_num(true));", "false\n");
        assert_output("echo(is_num(4));", "true\n");
    }

    #[test]
    fn test_type_test_is_string() {
        assert_output("echo(is_string(1));", "false\n");
        assert_output("echo(is_string(\"a\"));", "true\n");
    }

    #[test]
    fn test_type_test_is_list() {
        assert_output("echo(is_list(1));", "false\n");
        assert_output("echo(is_list([1,2]));", "true\n");
    }

    #[test]
    fn test_type_test_is_function() {
        assert_output("echo(is_function(1));", "false\n");
        assert_output("function a() = 1; echo(is_function(a));", "true\n");
    }

    // -- negation ----------------------------

    #[test]
    fn test_unary_expression_negation() {
        assert_output("echo(-20);", "-20\n");
    }

    // -- order of operations ----------------------------

    #[test]
    fn test_order_of_operations_multiplication_first() {
        assert_output("echo(2 + 3 * -5);", "-13\n");
    }

    #[test]
    fn test_order_of_operations_left_to_right() {
        assert_output("echo(2 * 3 + 5);", "11\n");
    }

    #[test]
    fn test_order_of_operations_with_comparison() {
        assert_output("echo(2 + 3 * 5 < 15);", "false\n");
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
        assert_output("
            function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));
            echo(distance([7, 4, 3], [17, 6, 2]));
        ","10.246951\n");
    }

    // -- echo ----------------------------

    #[test]
    fn test_echo_string() {
        assert_output(r#"echo("ok\ntest");"#, "\"ok\\ntest\"\n");
    }

    // -- if/else ----------------------------

    #[test]
    fn test_if_else() {
        assert_output(
            r#"
                if (1 > 2) {
                echo("false");
                } else if (5 > 2) {
                echo("ok");
                } else {
                echo("fail");
                }
            "#,
            "\"ok\"\n",
        );
    }

    // -- ternary ----------------------------

    #[test]
    fn test_ternary() {
        assert_output(r#"echo(1 > 2 ? "false" : "ok");"#, "\"ok\"\n");
    }

    // -- constants ----------------------------

    #[test]
    fn test_pi() {
        assert_output(r#"echo(PI);"#, "3.141593\n");
    }

    #[test]
    fn test_undef() {
        assert_output(r#"echo(undef);"#, "undef\n");
        assert_output(r#"a = undef; echo(a);"#, "undef\n");
    }

    // -- strings ----------------------------

    #[test]
    fn test_index_string() {
        assert_output(r#"a = "123"; echo(a[0]);"#, "\"1\"\n");
    }

    #[test]
    fn test_index_out_of_range_string() {
        assert_output(r#"a = "123"; echo(a[3]);"#, "undef\n");
        assert_output(r#"a = "123"; echo(a[-1]);"#, "undef\n");
    }

    // -- lists ----------------------------

    #[test]
    fn test_index() {
        assert_output(r#"a = [1,2,3]; echo(a[0]);"#, "1\n");
    }

    #[test]
    fn test_index_xyz() {
        assert_output(r#"a = [1,2,3]; echo(a.x);"#, "1\n");
        assert_output(r#"a = [1,2,3]; echo(a.y);"#, "2\n");
        assert_output(r#"a = [1,2,3]; echo(a.z);"#, "3\n");
    }

    #[test]
    fn test_index_out_of_range() {
        assert_output(r#"a = [1,2,3]; echo(a[3]);"#, "undef\n");
        assert_output(r#"a = [1,2,3]; echo(a[-1]);"#, "undef\n");
    }

    // -- math ----------------------------

    #[test]
    fn test_abs() {
        assert_output(r#"echo(abs(-1));"#, "1\n");
    }

    #[test]
    fn test_sign() {
        assert_output(r#"echo(sign(x=-5));"#, "-1\n");
        assert_output(r#"echo(sign(0));"#, "0\n");
        assert_output(r#"echo(sign(8));"#, "1\n");
    }

    #[test]
    fn test_sin() {
        assert_output(r#"echo(sin(degrees=35));"#, "0.573576\n");
    }

    #[test]
    fn test_cos() {
        assert_output(r#"echo(cos(degrees=35));"#, "0.819152\n");
    }

    #[test]
    fn test_tan() {
        assert_output(r#"echo(tan(degrees=35));"#, "0.700208\n");
    }

    #[test]
    fn test_pow() {
        assert_output(r#"echo(pow(base=2,exponent=3));"#, "8\n");
    }

    #[test]
    fn test_sqrt() {
        assert_output(r#"echo(sqrt(9));"#, "3\n");
    }
}
