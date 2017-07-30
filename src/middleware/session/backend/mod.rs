pub(super) mod memory;

use std::io;

use base64;
use rand::{self, Rng};
use futures::Future;

use middleware::session::{SessionError, SessionIdentifier};

/// Creates new `Backend` values.
pub trait NewBackend: Sync {
    /// The type of `Backend` created by the implementor.
    type Instance: Backend + Send + 'static;

    /// Create and return a new `Backend` value.
    fn new_backend(&self) -> io::Result<Self::Instance>;
}

/// Type alias for the trait objects returned by `Backend`.
pub type SessionFuture = Future<Item = Option<Vec<u8>>, Error = SessionError> + Send;

/// A `Backend` receives session data and stores it, and recalls the session data subsequently.
///
/// All session data is serialized into a `Vec<u8>` which is treated as opaque by the backend. The
/// serialization format is subject to change and must not be relied upon by the `Backend`.
pub trait Backend: Send {
    /// Creates a new random identifier for a session being created.
    fn random_identifier(&self) -> SessionIdentifier {
        let bytes: Vec<u8> = match rand::OsRng::new() {
            Ok(mut rng) => rng.gen_iter().take(64).collect(),
            Err(e) => {
                error!("Backend::random_identifier failed at rand::OsRng::new(), \
                        is the system RNG missing? {:?}",
                       e);
                unreachable!("no rng available, this should never happen");
            }
        };

        SessionIdentifier { value: base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD) }
    }

    /// Persists a session, either creating a new session or updating an existing session.
    fn persist_session(&self,
                       identifier: SessionIdentifier,
                       content: &[u8])
                       -> Result<(), SessionError>;

    /// Retrieves a session from the underlying storage.
    ///
    /// The returned future will resolve to an `Option<Vec<u8>>` on success, where a value of
    /// `None` indicates that the session is not available for use and a new session should be
    /// established.
    fn read_session(&self, identifier: SessionIdentifier) -> Box<SessionFuture>;

    /// Drops a session from the underlying storage.
    fn drop_session(&self, identifier: SessionIdentifier) -> Result<(), SessionError>;
}
