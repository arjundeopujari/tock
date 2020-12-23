//! All interfaces between capsules for `DALS`
/*
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
    fn return_buffer(&self, data_buffer: &'static mut [u8]);
}

/// Implemented by main `AppLoader` module
pub trait AppLoader<'a>: VerifierClient<'a> + ValidatorClient<'a> + kernel::hil::nonvolatile_storage::NonvolatileStorageClient<'a> {
    fn set_client(&self, client: &'a dyn AppLoaderClient);
    fn start_loading(&self, app_size: usize) -> Result<(),LoadingError>;
    fn next_buffer(&self, data_buffer: &'static mut [u8], length: usize, completed: bool);
}

/// Implemented by any module which provides an algorithm to decompress the app data buffer-by-buffer
pub trait Decompressor<'a> {
    fn set_client(&self, client: &'a dyn AppLoader<'a>);
    fn decompress_buffer(
        &self,
        buffer: &'static mut [u8],
        length: usize,
    ) -> (&'static mut [u8], usize, &'static mut [u8], Option<DecompressionError>);
    fn return_buffer(&self, decompressed_buffer: &'static [u8]);
}


/// Implemented by any module which provides an algorithm to verify the loaded app binary for security
/// purposes (ex: SHA, Checksum, MD5 implementations)
pub trait Verifier<'a> {
    fn set_client(&self, client: &'a dyn VerifierClient);
    fn verify_data(&self, app_flash: usize);
}

/// Implemented by main `AppLoader` module
pub trait VerifierClient<'a> {
    fn verification_complete(&self, error: Option<VerificationError>);
}

/// Implemented by any module which aims to validate the loaded app according to user specifications
pub trait Validator<'a> {
    fn set_client(&self, client: &'a dyn ValidatorClient);
    fn validate_data(&self, app_flash: usize);
}

/// Implemented by main `AppLoader` module
pub trait ValidatorClient<'a> {
    fn validation_complete(&self, error: Option<ValidationError>);
}
*/