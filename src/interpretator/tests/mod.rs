use std::collections::HashMap;

use crate::{tokenizer::Tokenizer, parser::Parser};

// use super::ContextEvaluator;


#[test]
fn it_unifies_variables() {
  let t = "nat(X).";
  let c = "nat(s(Y)).";
  let mut tokenizer_t = Tokenizer::from_str(t);
  let tokens_t = tokenizer_t.parse().unwrap();
  let mut tokenizer_c = Tokenizer::from_str(c);
  let tokens_c = tokenizer_c.parse().unwrap();
  let clause_t = Parser::from_tokens(tokens_t).parse().unwrap();
  let clause_c = Parser::from_tokens(tokens_c).parse().unwrap();
  let mut variable_mapping = HashMap::new();
  let u = clause_t[0].unify(clause_c[0].as_ref(), &mut variable_mapping).unwrap();
  println!("{:?}", u);
}

#[test]
fn it_unifies_rules() {
  let t = "len(list(one,list()), one).";
  let c = "len(list(H,T), X):-len(T,Y),plus(Y,num,X).";
  let mut tokenizer_t = Tokenizer::from_str(t);
  let tokens_t = tokenizer_t.parse().unwrap();
  let mut tokenizer_c = Tokenizer::from_str(c);
  let tokens_c = tokenizer_c.parse().unwrap();
  let clause_t = Parser::from_tokens(tokens_t).parse().unwrap();
  let clause_c = Parser::from_tokens(tokens_c).parse().unwrap();
  let mut variable_mapping = HashMap::new();
  println!("{:?}", clause_c);
  let u = clause_c[0].unify(clause_t[0].as_ref(), &mut variable_mapping).unwrap();
  println!("{:?}", u);
}

#[test]
fn it_evaluates_facts() {
  let t = "cat(list(), list(one,list(two,list())), X).";
  let c = "cat(list(), L, L).";
  let mut tokenizer_t = Tokenizer::from_str(t);
  let tokens_t = tokenizer_t.parse().unwrap();
  let mut tokenizer_c = Tokenizer::from_str(c);
  let tokens_c = tokenizer_c.parse().unwrap();
  // let clause_t = Parser::from_tokens(tokens_t).parse().unwrap();
  // let clause_c = Parser::from_tokens(tokens_c).parse().unwrap();
  //let mut variable_mapping = HashMap::new();
  // let mut ctx = ContextEvaluator::new_goal(clause_t[0].copy(), &None, &variable_mapping);
  // ctx.evaluate(&clause_c);
}

#[test]
fn it_evaluates_rules() {
  let t = "append(list(), L, L). append(list(X,Y), L, list(X,R)):-append(Y,L,R).";
  let c = "append(list(a,list(b,list())), list(), Z).";
  let mut tokenizer_t = Tokenizer::from_str(t);
  let tokens_t = tokenizer_t.parse().unwrap();
  let mut tokenizer_c = Tokenizer::from_str(c);
  let tokens_c = tokenizer_c.parse().unwrap();
  let clause_t = Parser::from_tokens(tokens_t).parse().unwrap();
  let clause_c = Parser::from_tokens(tokens_c).parse().unwrap();
  //let mut variable_mapping = HashMap::new();
  //let mut ctx = ContextEvaluator::new_goal(clause_c[0].copy(), &None, &variable_mapping);
  //ctx.evaluate(&clause_t);
}
