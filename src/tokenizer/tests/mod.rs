use super::*;

#[test]
fn it_make_tokens() {
    let clause = "len(list(1,list(2,list())),X).";
    let mut tokenizer = Tokenizer::from_str(clause);
    let r = tokenizer.parse().unwrap();
    assert_eq!(19, r.len());
    print!("{:?}", r);
}

#[test]
fn it_does_not_make_tokens() {
    let clause = "len(list(1,$list(2,list())),X).";
    let mut tokenizer = Tokenizer::from_str(clause);
    let r = tokenizer.parse();
    match r {
        Ok(_) => panic!("Should error"),
        Err(e) => {
            assert_eq!(
                e,
                TokenizerError::BaseError {
                    position: 11,
                    msg: String::from("Unrecognised character")
                }
            );
        }
    };
}
