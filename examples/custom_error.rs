use winnow::error::ErrorKind;
use winnow::error::ParseError;
use winnow::Err::Error;
use winnow::IResult;

#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
  MyError,
  Nom(I, ErrorKind),
}

impl<I> ParseError<I> for CustomError<I> {
  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    CustomError::Nom(input, kind)
  }

  fn append(self, _: I, _: ErrorKind) -> Self {
    self
  }
}

pub fn parse(_input: &str) -> IResult<&str, &str, CustomError<&str>> {
  Err(Error(CustomError::MyError))
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::parse;
  use super::CustomError;
  use winnow::Err::Error;

  #[test]
  fn it_works() {
    let err = parse("").unwrap_err();
    match err {
      Error(e) => assert_eq!(e, CustomError::MyError),
      _ => panic!("Unexpected error: {:?}", err),
    }
  }
}
