#[cfg(test)]
mod tests {
    use assert_eq_float::assert_eq_float;

    use crate::{
        interpreter::{InterpreterError, InterpreterResults, openscad_interpret},
        parser::openscad_parse,
        tokenizer::openscad_tokenize,
    };

    fn interpret(expr: &str) -> InterpreterResults {
        let result = openscad_parse(openscad_tokenize(expr).unwrap());
        openscad_interpret(result.statements)
    }

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

    #[test]
    fn test_unary_expression_negation() {
        let result = interpret("echo(-20);");
        assert_eq!(result.output, "-20\n");
    }

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

    #[test]
    fn test_set_fa() {
        let result = interpret("$fa = 1;");
        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
    }

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

    #[test]
    fn test_rands() {
        let result = interpret("choose_mat = rands(0,1,1)[0];");
        assert_eq!(Vec::<InterpreterError>::new(), result.errors);
    }

    #[test]
    fn test_function() {
        let s = "
            function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));
            echo(distance([7, 4, 3], [17, 6, 2]));
        ";

        let result = interpret(s);
        assert_eq_float!(result.output.trim().parse().unwrap(), 10.246950765959598);
    }

    #[test]
    fn test_echo_string() {
        let s = r#"echo("ok\ntest");"#;

        let result = interpret(s);
        assert_eq!(result.output, "\"ok\\ntest\"\n");
    }

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
}
