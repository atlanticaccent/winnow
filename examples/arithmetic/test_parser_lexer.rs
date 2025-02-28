use snapbox::assert_data_eq;
use snapbox::prelude::*;
use snapbox::str;
use winnow::prelude::*;

use crate::parser_lexer::*;

#[test]
fn lex_test() {
    let input = "3";
    let expected = str![[r#"
Ok(
    (
        "",
        [
            Value(
                3,
            ),
        ],
    ),
)

"#]];
    assert_data_eq!(lex.parse_peek(input).to_debug(), expected);

    let input = "  24     ";
    let expected = str![[r#"
Ok(
    (
        "",
        [
            Value(
                24,
            ),
        ],
    ),
)

"#]];
    assert_data_eq!(lex.parse_peek(input).to_debug(), expected);

    let input = " 12 *2 /  3";
    let expected = str![[r#"
Ok(
    (
        "",
        [
            Value(
                12,
            ),
            Oper(
                Mul,
            ),
            Value(
                2,
            ),
            Oper(
                Div,
            ),
            Value(
                3,
            ),
        ],
    ),
)

"#]];
    assert_data_eq!(lex.parse_peek(input).to_debug(), expected);

    let input = "  2*2 / ( 5 - 1) + 3";
    let expected = str![[r#"
Ok(
    (
        "",
        [
            Value(
                2,
            ),
            Oper(
                Mul,
            ),
            Value(
                2,
            ),
            Oper(
                Div,
            ),
            OpenParen,
            Value(
                5,
            ),
            Oper(
                Sub,
            ),
            Value(
                1,
            ),
            CloseParen,
            Oper(
                Add,
            ),
            Value(
                3,
            ),
        ],
    ),
)

"#]];
    assert_data_eq!(lex.parse_peek(input).to_debug(), expected);
}

#[test]
fn factor_test() {
    let input = "3";
    let expected = str![[r#"
Ok(
    Value(
        3,
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(factor.parse(&input).to_debug(), expected);

    let input = " 12";
    let expected = str![[r#"
Ok(
    Value(
        12,
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(factor.parse(&input).to_debug(), expected);

    let input = "537 ";
    let expected = str![[r#"
Ok(
    Value(
        537,
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(factor.parse(&input).to_debug(), expected);

    let input = "  24     ";
    let expected = str![[r#"
Ok(
    Value(
        24,
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(factor.parse(&input).to_debug(), expected);
}

#[test]
fn term_test() {
    let input = " 12 *2 /  3";
    let expected = str![[r#"
Ok(
    Div(
        Mul(
            Value(
                12,
            ),
            Value(
                2,
            ),
        ),
        Value(
            3,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(term.parse(&input).to_debug(), expected);

    let input = " 12 *2 /  3";
    let expected = str![[r#"
Ok(
    Div(
        Mul(
            Value(
                12,
            ),
            Value(
                2,
            ),
        ),
        Value(
            3,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(term.parse(&input).to_debug(), expected);

    let input = " 2* 3  *2 *2 /  3";
    let expected = str![[r#"
Ok(
    Div(
        Mul(
            Mul(
                Mul(
                    Value(
                        2,
                    ),
                    Value(
                        3,
                    ),
                ),
                Value(
                    2,
                ),
            ),
            Value(
                2,
            ),
        ),
        Value(
            3,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(term.parse(&input).to_debug(), expected);

    let input = " 48 /  3/2";
    let expected = str![[r#"
Ok(
    Div(
        Div(
            Value(
                48,
            ),
            Value(
                3,
            ),
        ),
        Value(
            2,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(term.parse(&input).to_debug(), expected);
}

#[test]
fn expr_test() {
    let input = " 1 +  2 ";
    let expected = str![[r#"
Ok(
    Add(
        Value(
            1,
        ),
        Value(
            2,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(expr.parse(&input).to_debug(), expected);

    let input = " 12 + 6 - 4+  3";
    let expected = str![[r#"
Ok(
    Add(
        Sub(
            Add(
                Value(
                    12,
                ),
                Value(
                    6,
                ),
            ),
            Value(
                4,
            ),
        ),
        Value(
            3,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(expr.parse(&input).to_debug(), expected);

    let input = " 1 + 2*3 + 4";
    let expected = str![[r#"
Ok(
    Add(
        Add(
            Value(
                1,
            ),
            Mul(
                Value(
                    2,
                ),
                Value(
                    3,
                ),
            ),
        ),
        Value(
            4,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(expr.parse(&input).to_debug(), expected);
}

#[test]
fn parens_test() {
    let input = " (  2 )";
    let expected = str![[r#"
Ok(
    Paren(
        Value(
            2,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(expr.parse(&input).to_debug(), expected);

    let input = " 2* (  3 + 4 ) ";
    let expected = str![[r#"
Ok(
    Mul(
        Value(
            2,
        ),
        Paren(
            Add(
                Value(
                    3,
                ),
                Value(
                    4,
                ),
            ),
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(expr.parse(&input).to_debug(), expected);

    let input = "  2*2 / ( 5 - 1) + 3";
    let expected = str![[r#"
Ok(
    Add(
        Div(
            Mul(
                Value(
                    2,
                ),
                Value(
                    2,
                ),
            ),
            Paren(
                Sub(
                    Value(
                        5,
                    ),
                    Value(
                        1,
                    ),
                ),
            ),
        ),
        Value(
            3,
        ),
    ),
)

"#]];
    let input = lex.parse(input).unwrap();
    assert_data_eq!(expr.parse(&input).to_debug(), expected);
}
