//! Helpers for HTTP related data

pub mod request_path;
pub mod query_string;

use url::percent_encoding::percent_decode;

/// Represents data that has been successfully percent decoded and is valid utf8
#[derive(Clone, PartialEq, Debug)]
pub struct PercentDecoded {
    val: String,
}

impl PercentDecoded {
    /// Attempt to decode data that has been provided in a perecent encoded format and ensure that
    /// the result is valid utf8.
    ///
    /// On success encapulate resultant data for use by components that expect this transformation
    /// has already occured.
    pub fn new(raw: &str) -> Option<Self> {
        match percent_decode(raw.as_bytes()).decode_utf8() {
            Ok(pd) => {
                trace!(" percent_decode: {}, src: {}", pd, raw);
                Some(PercentDecoded { val: pd.into_owned() })
            }
            Err(_) => {
                trace!(" percent_decode: error, src: {}", raw);
                None
            }
        }
    }

    /// Provide the decoded data this type encapsulates
    pub fn val(&self) -> &str {
        &self.val
    }
}

/// Represents data that has been successfully decoded from a form-urlencoded source and is
/// valid utf8
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct FormUrlDecoded {
    val: String,
}

impl FormUrlDecoded {
    /// Attempt to decode data that has been provided in www-form-urlencoded format and ensure that
    /// the result is valid utf8.
    ///
    /// On success encapulate resultant data for use by components that expect this transformation
    /// has already occured.
    pub fn new(raw: &str) -> Option<Self> {
        match percent_decode(raw.replace("+", " ").as_bytes()).decode_utf8() {
            Ok(pd) => {
                trace!(" form_url_decoded: {}, src: {}", pd, raw);
                Some(FormUrlDecoded { val: pd.into_owned() })
            }
            Err(_) => {
                trace!(" form_url_decoded: error, src: {}", raw);
                None
            }
        }
    }

    /// Provide the decoded data this type encapsulates
    pub fn val(&self) -> &str {
        &self.val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_valid_percent_decode() {
        let pd = PercentDecoded::new("%41+%42%2B%63%20%64").unwrap();
        assert_eq!("A+B+c d", pd.val());
    }

    #[test]
    fn ensure_valid_www_form_url_encoded_value() {
        let f = FormUrlDecoded::new("%41+%42%2B%63%20%64").unwrap();
        assert_eq!("A B+c d", f.val());
    }
}