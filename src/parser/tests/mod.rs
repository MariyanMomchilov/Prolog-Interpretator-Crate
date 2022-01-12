use super::*;

#[test]
fn it_parses_fact() {
  let input = "fact(X, s(oke)).";
  let tokens = Tokenizer::from_str(input).parse().unwrap();
  let mut parser = Parser::from_tokens(tokens);
  let fact = parser.parse().unwrap();
  assert_eq!(fact.len(), 1);
  println!("{:?}", &fact);
}

#[test]
fn it_parses_variable_fact() {
  let input = "Xvar.";
  let tokens = Tokenizer::from_str(input).parse().unwrap();
  let mut parser = Parser::from_tokens(tokens);
  let var = parser.parse().unwrap();
  assert_eq!(var.len(), 1);
  println!("{:?}", &var);
}

#[test]
fn it_parses_nested_fact() {
  let input = "fact(da(ne(X)), s(s(s(s(oke)))), something).";
  let tokens = Tokenizer::from_str(input).parse().unwrap();
  let mut parser = Parser::from_tokens(tokens);
  let fact = parser.parse().unwrap();
  assert_eq!(fact.len(), 1);
  println!("{:?}", &fact);
}

#[test]
fn it_parses_rule() {
  let input = "da(X):-ne(nex),da(ne(X)).";
  let tokens = Tokenizer::from_str(input).parse().unwrap();
  let mut parser = Parser::from_tokens(tokens);
  let rule = parser.parse().unwrap();
  assert_eq!(rule.len(), 1);
  println!("{:?}", &rule);
}

#[test]
fn it_parses_list_of_clauses() {
  let input = "
    nat(c).
    nat(s(X)):-nat(X).
  ";
  let tokens = Tokenizer::from_str(input).parse().unwrap();
  let mut parser = Parser::from_tokens(tokens);
  let clauses = parser.parse().unwrap();
  assert_eq!(clauses.len(), 2);
  println!("{:?}", &clauses);

}