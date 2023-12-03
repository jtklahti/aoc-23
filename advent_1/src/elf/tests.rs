#[cfg(test)]
mod parse_digit {
    use crate::elf::parse_digit;

    #[test]
    fn parse_digit_zero() {
        assert_eq!(parse_digit(&"a".to_string()), 0);
        assert_eq!(parse_digit(&"asdffsd".to_string()), 0);
        assert_eq!(parse_digit(&"".to_string()), 0);
    }

    #[test]
    fn parse_digit_str_simple() {
        assert_eq!(parse_digit(&"one".to_string()), 1);
        assert_eq!(parse_digit(&"two".to_string()), 2);
        assert_eq!(parse_digit(&"three".to_string()), 3);
        assert_eq!(parse_digit(&"four".to_string()), 4);
        assert_eq!(parse_digit(&"five".to_string()), 5);
        assert_eq!(parse_digit(&"six".to_string()), 6);
        assert_eq!(parse_digit(&"seven".to_string()), 7);
        assert_eq!(parse_digit(&"eight".to_string()), 8);
        assert_eq!(parse_digit(&"nine".to_string()), 9);
    }

    #[test]
    fn parse_digit_str_between() {
        assert_eq!(parse_digit(&"one332".to_string()), 1);
        assert_eq!(parse_digit(&"twosdfd".to_string()), 2);
        assert_eq!(parse_digit(&"threeXXc2".to_string()), 3);
    }

    #[test]
    fn parse_digit_num() {
        assert_eq!(parse_digit(&"2".to_string()), 2);
        assert_eq!(parse_digit(&"3".to_string()), 3);
        assert_eq!(parse_digit(&"2aazza".to_string()), 2);
        assert_eq!(parse_digit(&"3twox".to_string()), 3);
        assert_eq!(parse_digit(&"9xyzone".to_string()), 9);
    }
}

#[cfg(test)]
mod parse_line {
    use crate::elf::parse_line;

    #[test]
    fn parse_line_zero() {
        assert_eq!(parse_line("fdsaadfs".to_string()), 0)
    }

    #[test]
    fn parse_line_simple() {
        assert_eq!(parse_line("21".to_string()), 21);
        assert_eq!(parse_line("2".to_string()), 22);
        assert_eq!(parse_line("52".to_string()), 52);
    }

    #[test]
    fn parse_line_with_chars() {
        assert_eq!(parse_line("aa2xxx".to_string()), 22);
        assert_eq!(parse_line("aa8d2d1sd4xyx".to_string()), 84);
    }

    #[test]
    fn parse_line_examples1() {
        let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let results = [12, 38, 15, 77];
        for i in 0..results.len() {
            assert_eq!(parse_line(inputs[i].to_string()), results[i]);
        }
    }

    #[test]
    fn parse_line_examples2() {
        let inputs = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let results = [29, 83, 13, 24, 42, 14, 76];
        for i in 0..results.len() {
            assert_eq!(parse_line(inputs[i].to_string()), results[i]);
        }
    }
}

#[cfg(test)]
mod parse_content {
    use crate::elf::parse_content;

    #[test]
    fn parse_content_single() {
        assert_eq!(parse_content("12".to_string()), 12)
    }

    #[test]
    fn parse_content_multi_1() {
        let str = r#"
        1
        1
        "#;
        assert_eq!(parse_content(str.to_string()), 22)
    }

    #[test]
    fn parse_content_examples1() {
        let str = r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#
        .to_string();
        assert_eq!(parse_content(str), 142)
    }

    #[test]
    fn parse_content_examples2() {
        let str = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#.to_string();
        assert_eq!(parse_content(str), 281);
    }
}
