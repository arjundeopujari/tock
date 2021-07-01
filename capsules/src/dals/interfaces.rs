use kernel::hil::nonvolatile_storage;

/// Errors thrown by DALS.  These can be errors involved in loading, decompressing,
/// verifying, or authorizing the app data.
pub enum DalsError {
    NonVolatileStorageWrite,
}

/// Implemented by any capsule which provides the raw app data to load.
/// Ex: Capsules which provide interfaces to UART, BLE, IEEE 802.11, etc.
pub trait AppLoaderClient<'a> {
    fn return_buffer(&self, data_buffer: &'static mut [u8]);
    fn return_error(&self, error: DalsError);
}

/// Implemented by main `AppLoader` capsule
pub trait AppLoader<'a>:
    VerifierClient + nonvolatile_storage::NonvolatileStorageClient<'a>
{
    fn start_loading(&self, app_size: usize) -> Result<(), DalsError>;
    fn next_buffer(&self, data_buffer: &'static mut [u8], length: usize, completed: bool);
}

/// Implemented by any capsule which decompresses the raw app data buffer-by-buffer
pub trait Decompressor<'a> {
    fn set_client(&self, client: &'a dyn AppLoader<'a>);
    fn decompress_buffer(
        &self,
        buffer: &'static mut [u8],
        length: usize,
    ) -> (
        &'static mut [u8],
        usize,
        &'static mut [u8],
        Result<(),DalsError>,
    );
    fn return_buffer(&self, decompressed_buffer: &'static [u8]);
}

/// Implemented by any capsule which provides a software routine to authenticate the app data
/// Ex: Capsules which compute a checksum or SHA digest
pub trait Verifier<'a>: nonvolatile_storage::NonvolatileStorageClient<'a>{
    fn set_client(&self, client: &'a dyn VerifierClient);
    fn verify_data(&self, app_flash: usize);
    fn send_nonvolstorage_ref(&self, nonvol_storage: &'a dyn nonvolatile_storage::NonvolatileStorage<'a>);
}

/// Implemented by main `AppLoader` module
pub trait VerifierClient {
    fn verification_complete(&self, error: Option<DalsError>);
}

/// Implemented by any capsule which decides, based on some defined policy, whether to run the newly-loaded app.
/// This allocates the process's memory in SRAM, sets up process data structures in the kernel, and configures the MPU.
pub trait Authorizer<'a> {
    fn set_client(&self, client: &'a dyn AppLoader<'a>);
    fn authorize_data(&self, app_flash: usize) -> Result<(),DalsError>;
}
