use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub calculator1);
lalrpop_mod!(pub calculator2);
lalrpop_mod!(pub calculator3);
lalrpop_mod!(pub calculator4);
lalrpop_mod!(pub calculator5);
lalrpop_mod!(pub calculator6);
lalrpop_mod!(pub calculator7);

pub mod ast;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Calculator6Error {
    InputTooBig,
    OddNumber,
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn calculator1() {
    assert!(calculator1::TermParser::new().parse("22").is_ok());
    assert!(calculator1::TermParser::new().parse("(22)").is_ok());
    assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator1::TermParser::new().parse("((22)").is_err());
}

#[test]
fn calculator2() {
    assert!(calculator2::TermParser::new().parse("22").is_ok());
    assert!(calculator2::TermParser::new().parse("(22)").is_ok());
    assert!(calculator2::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator2::TermParser::new().parse("((22)").is_err());
}

#[test]
fn calculator3() {
    assert!(calculator3::ExprParser::new().parse("22").is_ok());
    assert!(calculator3::ExprParser::new().parse("22 + 33").is_ok());
    assert!(calculator3::ExprParser::new().parse("22 + 33 * 44").is_ok());
}

#[test]
fn calculator4() {
    let expr = calculator4::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}

#[test]
fn calculator5() {
    let expr = calculator5::ExprsParser::new().parse("").unwrap();
    assert_eq!(&format!("{:?}", expr), "[]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66,")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3,")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3)]");

    let expr = calculator5::ExprsParser::new()
        .parse("22 * 44 + 66, 13*3, 1+1")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (13 * 3), (1 + 1)]");
}

#[test]
fn calculator6() {
    use lalrpop_util::ParseError;

    let expr = calculator6::ExprsParser::new().parse("2147483648");
    assert!(expr.is_err());
    assert_eq!(expr.unwrap_err(), ParseError::User { error: Calculator6Error::InputTooBig });

    let expr = calculator6::ExprsParser::new().parse("3");
    assert!(expr.is_err());
    assert_eq!(expr.unwrap_err(), ParseError::User { error: Calculator6Error::OddNumber });
}

#[test]
fn calculator7() {
    let mut errors = Vec::new();

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "22 * + 3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * error) + 3)]");

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "22 * 44 + 66, *3")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[((22 * 44) + 66), (error * 3)]");

    let expr = calculator7::ExprsParser::new()
        .parse(&mut errors, "*")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "[(error * error)]");

    assert_eq!(errors.len(), 4);
}

