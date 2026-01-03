// use crate::{
//     errors::parseError::ParseError,
//     lexing::token::Token,
//     parsing::ast::{BinaryOp, Literal, UnaryOp},
// };

// pub fn parse_binary_op(token: &Token) -> Result<BinaryOp, ParseError> {
//     match token {
//         Token::And(_) => Ok(BinaryOp::And),
//         Token::Or(_) => Ok(BinaryOp::Or),
//         Token::Plus(_) => Ok(BinaryOp::Plus),
//         Token::Minus(_) => Ok(BinaryOp::Minus),
//         Token::Star(_) => Ok(BinaryOp::Star),
//         Token::Slash(_) => Ok(BinaryOp::Slash),
//         Token::GreaterEqual(_) => Ok(BinaryOp::GreaterEqual),
//         Token::GreaterThan(_) => Ok(BinaryOp::GreaterThan),
//         Token::EqualEqual(_) => Ok(BinaryOp::EqualEqual),
//         Token::BangEqual(_) => Ok(BinaryOp::BangEqual),
//         Token::LessEqual(_) => Ok(BinaryOp::LessEqual),
//         Token::LessThan(_) => Ok(BinaryOp::LessThan),
//         Token::Assign(_) => Ok(BinaryOp::Equal), //note that assign should be equal
//         _ => Err(ParseError::InvalidConversion(
//             "could not convert to binary operator".to_string(),
//         )), //figure out formatting later to enter to display tokens
//     }
// }

// pub fn parse_unary_op(token: &Token) -> Result<UnaryOp, ParseError> {
//     match token {
//         Token::Bang(_) => Ok(UnaryOp::Bang),
//         _ => Err(ParseError::InvalidConversion(
//             "could not convert to unary operator".to_string(),
//         )),
//     }
// }

// pub fn parse_literal(token: &Token) -> Result<Literal, ParseError> {
//     match token {
//         Token::IntegerLiteral(i) => Ok(Literal::IntegerLiteral(*i)),
//         Token::StringLiteral(s) => Ok(Literal::StringLiteral(s.clone())),
//         Token::Boolean(b) => Ok(Literal::Boolean(*b)),
//         Token::Null(_) => Ok(Literal::Null),
//         _ => Err(ParseError::InvalidConversion(
//             "could not convert literal".to_string(),
//         )),
//     }
// }
