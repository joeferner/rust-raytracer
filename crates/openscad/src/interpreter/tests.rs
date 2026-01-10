#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use assert_eq_float::assert_eq_float;
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

    // -- addition ----------------------------

    #[test]
    fn test_binary_expression_scalar_addition() {
        let result = interpret("echo(20 + 0.1);");
        assert_eq!(result.output, "20.1\n");
    }

    #[test]
    fn test_binary_expression_vector_addition() {
        let result = interpret("echo([5, 8, -12] + [3, -4, 18]);");
        assert_eq!(result.output, "[8, 4, 6]\n");
    }

    #[test]
    fn test_binary_expression_vector_addition_left_smaller() {
        let result = interpret("echo([5, 8] + [3, -4, 18]);");
        assert_eq!(result.output, "[8, 4]\n");
    }

    #[test]
    fn test_binary_expression_vector_addition_right_smaller() {
        let result = interpret("echo([5, 8, -12] + [3, -4]);");
        assert_eq!(result.output, "[8, 4]\n");
    }

    // -- subtraction ----------------------------

    #[test]
    fn test_binary_expression_scalar_subtraction() {
        let result = interpret("echo(20 - 0.1);");
        assert_eq!(result.output, "19.9\n");
    }

    #[test]
    fn test_binary_expression_vector_subtraction() {
        let result = interpret("echo([5, 8, -12] - [3, -4, 18]);");
        assert_eq!(result.output, "[2, 12, -30]\n");
    }

    #[test]
    fn test_binary_expression_vector_subtraction_left_smaller() {
        let result = interpret("echo([5, 8] - [3, -4, 18]);");
        assert_eq!(result.output, "[2, 12]\n");
    }

    #[test]
    fn test_binary_expression_vector_subtraction_right_smaller() {
        let result = interpret("echo([5, 8, -12] - [3, -4]);");
        assert_eq!(result.output, "[2, 12]\n");
    }

    // -- division ----------------------------

    #[test]
    fn test_binary_expression_scalar_division() {
        let result = interpret("echo(20 / 4);");
        assert_eq!(result.output, "5\n");
    }

    #[test]
    fn test_binary_expression_divide_vector_by_scaler() {
        let result = interpret("echo([5, 8, -12] / 4);");
        assert_eq!(result.output, "[1.25, 2, -3]\n");
    }

    #[test]
    fn test_binary_expression_divide_scaler_by_vector() {
        let result = interpret("echo(4 / [5, 8, -12]);");
        assert_eq!(result.output, "[0.8, 0.5, -0.333333]\n");
    }

    // -- multiplication ----------------------------

    #[test]
    fn test_binary_expression_scalar_multiplication() {
        let result = interpret("echo(20 * 4);");
        assert_eq!(result.output, "80\n");
    }

    #[test]
    fn test_binary_expression_multiply_vector_by_scaler() {
        let result = interpret("echo([5, 8, -12] * 4);");
        assert_eq!(result.output, "[20, 32, -48]\n");

        let result = interpret("echo(4 * [5, 8, -12]);");
        assert_eq!(result.output, "[20, 32, -48]\n");
    }

    // -- modulus ----------------------------

    #[test]
    fn test_binary_expression_scalar_modulus() {
        let result = interpret("echo(20 % 3);");
        assert_eq!(result.output, "2\n");
    }

    // -- exponentiation ----------------------

    #[test]
    fn test_binary_expression_scalar_exponentiation() {
        let result = interpret("echo(20 ^ 3);");
        assert_eq!(result.output, "8000\n");
    }

    // -- comparison ----------------------

    #[test]
    fn test_binary_expression_less_equal() {
        let result = interpret("echo(1 <= 2);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(2 <= 2);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(3 <= 2);");
        assert_eq!(result.output, "false\n");
    }

    #[test]
    fn test_binary_expression_less() {
        let result = interpret("echo(1 < 2);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(2 < 2);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(3 < 2);");
        assert_eq!(result.output, "false\n");
    }

    #[test]
    fn test_binary_expression_greater_equal() {
        let result = interpret("echo(1 >= 2);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(2 >= 2);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(3 >= 2);");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_binary_expression_greater() {
        let result = interpret("echo(1 > 2);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(2 > 2);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(3 > 2);");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_binary_expression_equals() {
        let result = interpret("echo(1 == 2);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(2 == 2);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo([1,2] == [2,2]);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo([1,2] == [1,2]);");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_binary_expression_not_equals() {
        let result = interpret("echo(1 != 2);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(2 != 2);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo([1,2] != [2,2]);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo([1,2] != [1,2]);");
        assert_eq!(result.output, "false\n");
    }

    #[test]
    fn test_binary_expression_and() {
        let result = interpret("echo(true && true);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(true && false);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(1 && true);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(true && 1);");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_binary_expression_or() {
        let result = interpret("echo(true || true);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(true || false);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(false || false);");
        assert_eq!(result.output, "false\n");
    }

    #[test]
    fn test_boolean_negation() {
        let result = interpret("echo(!true);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(!false);");
        assert_eq!(result.output, "true\n");

        let result = interpret("echo(!1);");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(!0);");
        assert_eq!(result.output, "true\n");
    }

    // -- type test --------------------------

    #[test]
    fn test_type_test_is_undef() {
        let result = interpret("echo(is_undef(1));");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(is_undef(undef));");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_type_test_is_bool() {
        let result = interpret("echo(is_bool(1));");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(is_bool(false));");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_type_test_is_num() {
        let result = interpret("echo(is_num(true));");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(is_num(4));");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_type_test_is_string() {
        let result = interpret("echo(is_string(1));");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(is_string(\"a\"));");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_type_test_is_list() {
        let result = interpret("echo(is_list(1));");
        assert_eq!(result.output, "false\n");

        let result = interpret("echo(is_list([1,2]));");
        assert_eq!(result.output, "true\n");
    }

    #[test]
    fn test_type_test_is_function() {
        let result = interpret("echo(is_function(1));");
        assert_eq!(result.output, "false\n");

        let result = interpret("function a() = 1; echo(is_function(a));");
        assert_eq!(result.output, "true\n");
    }

    // -- negation ----------------------------

    #[test]
    fn test_unary_expression_negation() {
        let result = interpret("echo(-20);");
        assert_eq!(result.output, "-20\n");
    }

    // -- order of operations ----------------------------

    #[test]
    fn test_order_of_operations_multiplication_first() {
        let result = interpret("echo(2 + 3 * -5);");
        assert_eq!(result.output, "-13\n");
    }

    #[test]
    fn test_order_of_operations_left_to_right() {
        let result = interpret("echo(2 * 3 + 5);");
        assert_eq!(result.output, "11\n");
    }

    #[test]
    fn test_order_of_operations_with_comparison() {
        let result = interpret("echo(2 + 3 * 5 < 15);");
        assert_eq!(result.output, "false\n");
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
        let result = interpret(
            "
                for(a = [-1 : 1])
                    for(b = [0 : 2])
                        echo(a,b);
            ",
        );
        assert_eq!(result.output, "-1, 0\n-1, 1\n0, 0\n0, 1\n");
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
        let s = "
            function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));
            echo(distance([7, 4, 3], [17, 6, 2]));
        ";

        let result = interpret(s);
        assert_eq_float!(result.output.trim().parse().unwrap(), 10.246951);
    }

    // -- echo ----------------------------

    #[test]
    fn test_echo_string() {
        let s = r#"echo("ok\ntest");"#;

        let result = interpret(s);
        assert_eq!(result.output, "\"ok\\ntest\"\n");
    }

    // -- if/else ----------------------------

    #[test]
    fn test_if_else() {
        let s = r#"
            if (1 > 2) {
              echo("false");
            } else if (5 > 2) {
              echo("ok");
            } else {
              echo("fail");
            }
        "#;

        let result = interpret(s);
        assert_eq!(result.output, "\"ok\"\n");
    }

    // -- ternary ----------------------------

    #[test]
    fn test_ternary() {
        let s = r#"echo(1 > 2 ? "false" : "ok");"#;
        let result = interpret(s);
        assert_eq!(result.output, "\"ok\"\n");
    }

    // -- constants ----------------------------

    #[test]
    fn test_pi() {
        let s = r#"echo(PI);"#;
        let result = interpret(s);
        assert_eq!(result.output, "3.141593\n");
    }

    #[test]
    fn test_undef() {
        let s = r#"echo(undef);"#;
        let result = interpret(s);
        assert_eq!(result.output, "undef\n");

        let s = r#"a = undef; echo(a);"#;
        let result = interpret(s);
        assert_eq!(result.output, "undef\n");
    }

    // -- strings ----------------------------

    #[test]
    fn test_index_string() {
        let s = r#"a = "123"; echo(a[0]);"#;
        let result = interpret(s);
        assert_eq!(result.output, "\"1\"\n");
    }

    #[test]
    fn test_index_out_of_range_string() {
        let s = r#"a = "123"; echo(a[3]);"#;
        let result = interpret(s);
        assert_eq!(result.output, "undef\n");

        let s = r#"a = "123"; echo(a[-1]);"#;
        let result = interpret(s);
        assert_eq!(result.output, "undef\n");
    }

    // -- lists ----------------------------

    #[test]
    fn test_index() {
        let s = r#"a = [1,2,3]; echo(a[0]);"#;
        let result = interpret(s);
        assert_eq!(result.output, "1\n");
    }

    #[test]
    fn test_index_xyz() {
        let s = r#"a = [1,2,3]; echo(a.x);"#;
        let result = interpret(s);
        assert_eq!(result.output, "1\n");

        let s = r#"a = [1,2,3]; echo(a.y);"#;
        let result = interpret(s);
        assert_eq!(result.output, "2\n");

        let s = r#"a = [1,2,3]; echo(a.z);"#;
        let result = interpret(s);
        assert_eq!(result.output, "3\n");
    }

    #[test]
    fn test_index_out_of_range() {
        let s = r#"a = [1,2,3]; echo(a[3]);"#;
        let result = interpret(s);
        assert_eq!(result.output, "undef\n");

        let s = r#"a = [1,2,3]; echo(a[-1]);"#;
        let result = interpret(s);
        assert_eq!(result.output, "undef\n");
    }
}
