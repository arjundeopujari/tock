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
    app_write_address: Cell<usize>,
    app_flash: OptionalCell<&'static [u8]>,
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
            nonvolatile_write_address: Cell::new(0),
            state: Cell::new(State::Idle),
        }
    }

    /// TODO: Main FSM for `DALS`
    pub fn process_state(&self, optional_buffer: Option<&'a mut [u8]>, optional_length: Option<usize>, optional_flag: Option<bool>) {

        let buffer = optional_buffer.unwrap();
        let length = optional_length.unwrap();
        let flag = optional_flag.unwrap();

        match self.state.get(){
            State::DecompressBuffer => {
                let (decomp_buf,decomp_buf_len,buffer,error) = self.decompressor.decompress_buffer(buffer,length);

                let client = self.app_loader_client.take().unwrap();
                client.return_buffer(buffer);
                self.app_loader_client.insert(Some(client));

                if flag {
                    self.state.set(State::WriteFlashFinal);
                } else {
                    self.state.set(State::WriteFlash);
                }
                let write_address = self.app_address.get();
                self.app_address.set(write_address + decomp_buf_len);
                self.nonvolatile_storage.write(decomp_buf, write_address, decomp_buf_len);
                
            }
            State::WriteFlash => {
                self.decompressor.return_buffer(buffer);
                self.state.set(State::Idle);
            }
            State::WriteFlashFinal => {
                self.decompressor.return_buffer(buffer);
                self.state.set(State::Verify);
                self.verifier.verify_data();
            }
            State::
            _ => {}
        }
    }
}

impl<'a> interfaces::AppLoader<'a> for AppLoader<'a> {
    fn set_client(&self, client: &'a dyn interfaces::AppLoaderClient) {}
    fn start_loading(&self, app_size: usize) -> Result<(), DalsError> {
        Ok(())
    }
    fn next_buffer(&self, data_buffer: &'a mut [u8], length: usize, completed: bool) {
        self.state.set(State::DecompressBuffer);
        self.process_state(Some(data_buffer),Some(length),Some(completed));
    }
}

impl<'a> interfaces::VerifierClient<'a> for AppLoader<'a> {
    fn verification_complete(&self, error: Option<DalsError>) {}
}

impl<'a> hil::nonvolatile_storage::NonvolatileStorageClient<'a> for AppLoader<'a> {
    fn read_done(&self, buffer: &'a mut [u8], length: usize) {}
    fn write_done(&self, buffer: &'a mut [u8], length: usize) {
        self.process_state(Some(buffer), Some(length) , None);
    }
}
