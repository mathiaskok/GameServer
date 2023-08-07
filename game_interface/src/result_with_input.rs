#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResultWithInput<Result, Error>(Result, Option<Error>);

impl<Result, Error> ResultWithInput<Result, Error> {
  pub fn from(result: Result) -> Self {
    ResultWithInput(result, Option::None)
  }

  pub fn with(mut self, error: Error) -> Self {
    self.1 = Option::Some(error);
    self
  }

  pub fn result(&self) -> &Result {
    &self.0
  }

  pub fn into_result(self) -> Result {
    self.0
  }

  pub fn error(&self) -> &Option<Error> {
    &self.1
  }

  pub fn into_error(self) -> Option<Error> {
    self.1
  }

  pub fn map<B, F>(self, map: F) -> ResultWithInput<B, Error>
  where F: FnOnce(Result) -> B {
    ResultWithInput(map(self.0), self.1)
  }

  pub fn map_err<B, F>(self, map: F) -> ResultWithInput<Result, B>
  where F: FnOnce(Error) -> B {
    ResultWithInput(self.0, self.1.map(map))
  }
}