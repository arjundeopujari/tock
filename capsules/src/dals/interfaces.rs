//! All interfaces between capsules for `DALS`

/// Errors which prevent an app from being loaded into memory.
pub enum LoadingError {
    //TODO
}

/// Errors thrown by the `Decompressor` (errors which prevent an app from being decompressed)
pub enum DecompressionError {
    //TODO
}

/// Errors thrown by the `Verifier` if the loaded app did not pass the verification.
pub enum VerificationError {
    //TODO
}

/// Errors thrown by the `Validator` if the loaded app did not pass the validation.
pub enum ValidationError {
    //TODO
}

/// Implemented by any module (client) which seeks to load data buffer-by-buffer from a hardware resource
/// such as Wifi, Bluetooth, Zigbee, USB, etc.
pub trait AppLoaderClient<'a> {
    /// Called by `AppLoader` implementation when data buffer is ready to be returned to `AppLoaderClient`
    ///
    /// - `data_buffer`: Buffer which was passed to `AppLoader` via the `next_buffer()` call
    fn return_buffer(&self, data_buffer: &'static mut [u8]);
}

/// Implemented by main `AppLoader` module
pub trait AppLoader<'a> {
    fn set_client(&self, client: &'a dyn AppLoaderClient);
    /// Called by `AppLoaderClient` implementation when the app binary data is ready to be loaded into memory.
    ///
    /// - `app_length` : Size of app (in bytes)
    fn start_loading(&self, app_size: usize) -> Result<(), LoadingError>;
    /// Called by `AppLoaderClient` implementation when there exists a buffer of data for the app binary which is to be sent to the
    /// `AppLoader` implementation.
    ///
    /// - `data_buffer` : Buffer which contains the raw received data for the app
    /// - `len` : Size of data within the buffer
    /// - `completed` : Flag which is set when current buffer contains last of the data to be sent to the `AppLoader`
    fn next_buffer(&self, data_buffer: &'static mut [u8], len: u16, completed: bool);
}

/// Implemented by any module which provides an algorithm to decompress the app data buffer-by-buffer
pub trait Decompressor<'a> {
    fn set_client(&self, client: &'a dyn DecompressorClient);
    /// Called by `DecompressorClient` implementation
    ///
    /// - `buffer` : Buffer of data awaiting to be decompressed.
    /// - `len` : Size of data within buffer.
    fn decompress_buffer(
        &self,
        buffer: &'static mut [u8],
        len: u16,
    ) -> Result<(), DecompressionError>;
}

/// Implemented by main `AppLoader` module
pub trait DecompressorClient<'a> {
    /// Called by `Decompressor` implementation
    ///
    /// - `decompressed_data_buffer` : Buffer which contains the next sqeuence of decompressed app data.
    /// - `original_data_buffer` : Original buffer passed via `decompress_buffer().  This original buffer is
    /// passed back to `DecompressorClient` only when all of the data in it has been decompressed.
    /// In other words, this buffer being passed back indicates to the `DecompressorClient` that all of the data in it
    /// has been processed by the `Decompressor`
    fn decompressed_buffer(
        &self,
        decompressed_data_buffer: &'static [u8],
        len: u16,
        original_data_buffer: Option<&'static [u8]>,
    );
}

/// Implemented by any module which provides an algorithm to verify the loaded app binary for security
/// purposes (ex: SHA, Checksum, MD5 implementations)
pub trait Verifier<'a> {
    fn set_client(&self, client: &'a dyn VerifierClient);
    /// Called by `VerifierClient` implementation
    ///
    /// - `app_flash` : Immutable reference to app data stored in flash
    /// - `len` : Size of app in flash
    fn verify_data(&self, app_flash: &'static [u8], len: u32);
}

/// Implemented by main `AppLoader` module
pub trait VerifierClient<'a> {
    /// Called by `Verifier` implementation
    ///
    /// - `Option<VerificationError>` : Optional error passed back to indicate why verification failed.  No error returned if verification is successful.
    fn verification_complete(&self, error: Option<VerificationError>);
}

/// Implemented by any module which aims to validate the loaded app according to user specifications
pub trait Validator<'a> {
    fn set_client(&self, client: &'a dyn ValidatorClient);
    /// Called by `ValidatorClient`
    ///
    /// - `app_flash` : Immutable reference to app data stored in flash
    /// - `len` : Size of app in flash
    fn validate_data(&self, app_flash: &'static [u8], len: u32);
}

/// Implemented by main `AppLoader` module
pub trait ValidatorClient<'a> {
    /// Called by `Validator` implementation
    ///
    /// - `Option<ValidationError>` : Optional error passed back to indicate why validation failed.  No error returned if validation is successful.
    fn validation_complete(&self, error: Option<ValidationError>);
}
