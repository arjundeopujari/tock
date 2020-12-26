//! All interfaces between capsules for `DALS`

/// Errors thrown by DALS.  These can be errors involved in loading, decompressing,
/// verifying, or authorizing the app data
pub enum DalsError {}

/// Implemented by any capsule (client) which can collect raw application data from an external source.
/// Ex: UART, BLE, 802.11, etc.
pub trait AppLoaderClient<'a> {
    fn return_buffer(&self, data_buffer: &'static mut [u8]);
}

/// Implemented by main `AppLoader` capsule
pub trait AppLoader<'a>:
    VerifierClient<'a>
    + AuthorizerClient<'a>
    + kernel::hil::nonvolatile_storage::NonvolatileStorageClient<'a>
{
    fn set_client(&self, client: &'a dyn AppLoaderClient);
    fn start_loading(&self, app_size: usize) -> Result<(), DalsError>;
    fn next_buffer(&self, data_buffer: &'a mut [u8], length: usize, completed: bool);
}

/// Implemented by any capsule which provides an algorithm to decompress the app data buffer-by-buffer
pub trait Decompressor<'a> {
    fn set_client(&self, client: &'a dyn AppLoader<'a>);
    fn decompress_buffer(
        &self,
        buffer: &'a mut [u8],
        length: usize,
    ) -> (&'a mut [u8], usize, &'a mut [u8], Option<DalsError>);
    fn return_buffer(&self, decompressed_buffer: &'a [u8]);
}

/// Implemented by any capsule which provides an algorithm to verify the loaded app binary for security
/// purposes (ex: SHA, Checksum, MD5 implementations)
pub trait Verifier<'a> {
    fn set_client(&self, client: &'a dyn VerifierClient);
    fn verify_data(&self, app_flash: usize);
}

/// Implemented by main `AppLoader` module
pub trait VerifierClient<'a> {
    fn verification_complete(&self, error: Option<DalsError>);
}

/// Implemented by any capsule which delivers a final decision on whether to set up process-specific structures
/// such as stack and run the app.
pub trait Authorizer<'a> {
    fn set_client(&self, client: &'a dyn AuthorizerClient);
    fn authorize_data(&self, app_flash: usize);
}

/// Implemented by main `AppLoader` module
pub trait AuthorizerClient<'a> {
    fn authorization_complete(&self, error: Option<DalsError>);
}
