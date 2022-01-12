use super::*;

#[test]
fn it_parses_fact() {
  let input = "fact(X, s(oke)).";
  let tokens = Tokenizer::from_str(input).parse().unwrap();
  let mut parser = Parser::from_tokens(tokens);
  let fact = parser.parse().unwrap();
  assert_eq!(fact.len(), 1);
}