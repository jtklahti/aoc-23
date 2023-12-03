#[cfg(test)]
mod parse_line {
    use crate::elf::parse_line;

    #[test]
    fn parse_line_zero() {
        assert_eq!(parse_line("fdsaadfs".to_string()), 0)
    }

    #[test]
    fn parse_line_simple() {
        assert_eq!(parse_line("20".to_string()), 20);
        assert_eq!(parse_line("2".to_string()), 22);
        assert_eq!(parse_line("52".to_string()), 52);
    }

    #[test]
    fn parse_line_complex() {
        assert_eq!(parse_line("aa2xxx".to_string()), 22);
        assert_eq!(parse_line("aa8d2d1sd4xyx".to_string()), 84);
    }

    #[test]
    fn parse_line_examples() {
        let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let results = [12, 38, 15, 77];
        for i in 0..results.len() {
            assert_eq!(parse_line(inputs[i].to_string()), results[i]); //results[i])
        }
    }
}

mod parse_lines {
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
        assert_eq!(parse_content("12".to_string()), 12)
    }

    #[test]
    fn parse_content_multi_2() {
        let str = r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#.to_string();
        assert_eq!(parse_content(str), 142)
    }
}
