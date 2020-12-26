//! Code for the "AppLoader".  This capsule is the heart of the "DALS" and is
//! in charge of coordinating the whole application-loading process which includes
//! sending/receiving data to/from peripheral capsules in "DALS

use crate::dals::interfaces;
use crate::dals::interfaces::DalsError;
use core::cell::Cell;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::hil;

// States
#[derive(Clone, Copy, PartialEq)]
enum State {
    Idle,
    DecompressBuffer,
    WriteFlash,
    WriteFlashFinal,
    Verify,
    Authorize,
    Done,
}

pub struct AppLoader<'a> {
    app_loader_client: OptionalCell<&'a dyn interfaces::AppLoaderClient<'a>>,
    decompressor: &'a dyn interfaces::Decompressor<'a>,
    verifier: &'a dyn interfaces::Verifier<'a>,
    authorizer: &'a dyn interfaces::Authorizer<'a>,
    nonvolatile_storage: &'a dyn hil::nonvolatile_storage::NonvolatileStorage<'a>,
    app_address: Cell<usize>,
    state: Cell<State>,
}

impl<'a> AppLoader<'a> {
    pub fn new(
        decompressor: &'a dyn interfaces::Decompressor<'a>,
        verifier: &'a dyn interfaces::Verifier<'a>,
        authorizer: &'a dyn interfaces::Authorizer<'a>,
        nonvolatile_storage: &'a dyn hil::nonvolatile_storage::NonvolatileStorage<'a>,
    ) -> AppLoader<'a> {
        AppLoader {
            app_loader_client: OptionalCell::empty(),
            decompressor: decompressor,
            verifier: verifier,
            authorizer: authorizer,
            nonvolatile_storage: nonvolatile_storage,
            app_address: Cell::new(0),
            state: Cell::new(State::Idle),
        }
    }

    /// TODO: Implementation should follow `app_flash` to app data in flash and check header length and version
    pub fn check_app_header(&self) {}

    /// TODO: Main FSM for `DALS`
    pub fn process_state(
        &self,
        buffer: Option<&'a mut [u8]>,
        length: Option<usize>,
        flag: Option<bool>,
    ) {
    }
}

impl<'a> interfaces::AppLoader<'a> for AppLoader<'a> {
    fn set_client(&self, client: &'a dyn interfaces::AppLoaderClient) {}
    fn start_loading(&self, app_size: usize) -> Result<(), DalsError> {
        Ok(())
    }
    fn next_buffer(&self, data_buffer: &'a mut [u8], length: usize, completed: bool) {}
}

impl<'a> interfaces::VerifierClient<'a> for AppLoader<'a> {
    fn verification_complete(&self, error: Option<DalsError>) {}
}

impl<'a> interfaces::AuthorizerClient<'a> for AppLoader<'a> {
    fn authorization_complete(&self, error: Option<DalsError>) {}
}

impl<'a> hil::nonvolatile_storage::NonvolatileStorageClient<'a> for AppLoader<'a> {
    fn read_done(&self, buffer: &'a mut [u8], length: usize) {}
    fn write_done(&self, buffer: &'a mut [u8], length: usize) {}
}
