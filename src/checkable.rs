use crate::error::Result;

pub(crate) trait Checkable {
    fn check(&mut self, http: &ureq::Agent, url: &str) -> Result<bool>;
}
