use crate::dals::interfaces;
use crate::dals::interfaces::DalsError;
use core::cell::Cell;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::hil::nonvolatile_storage;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Idle,
    a,
    b,
    c,
    d,
    e,
    f,
    g,
}

pub struct AppLoader<'a> {
    app_loader_client: &'a dyn interfaces::AppLoaderClient<'a>,
    decompressor: &'a dyn interfaces::Decompressor<'a>,
    verifier: &'a dyn interfaces::Verifier<'a>,
    authorizer: &'a dyn interfaces::Authorizer<'a>,
    nonvol_storage: &'a dyn nonvolatile_storage::NonvolatileStorage<'a>,
    app_write_address: Cell<usize>,
    state: Cell<State>,
}

impl<'a> AppLoader<'a> {
    pub fn new(
        app_loader_client: &'a dyn interfaces::AppLoaderClient<'a>,
        decompressor: &'a dyn interfaces::Decompressor<'a>,
        verifier: &'a dyn interfaces::Verifier<'a>,
        authorizer: &'a dyn interfaces::Authorizer<'a>,
        nonvol_storage: &'a dyn nonvolatile_storage::NonvolatileStorage<'a>,
    ) -> AppLoader<'a> {
        AppLoader {
            app_loader_client: app_loader_client,
            decompressor: decompressor,
            verifier: verifier,
            authorizer: authorizer,
            nonvol_storage: nonvol_storage,
            app_write_address: Cell::new(0),
            state: Cell::new(State::Idle),
        }
    }

    /// Main FSM for DALS
    pub fn action_complete(&self, optional_buffer: Option<&'static mut [u8]>, optional_length: Option<usize>, optional_bool: Option<bool>) {
        match self.state.get(){
            State::a => {
                
                // Received buffer w/data from AL Client but it is not the last buffer.
                let buffer = optional_buffer.unwrap();
                let length = optional_length.unwrap();

                let (decompressed_buffer, dec_buf_length, buffer, status) = self.decompressor.decompress_buffer(buffer, length);

                match status {
                    Some(error) => {
                        self.app_loader_client.return_error(error);
                    }
                    None => {}
                }

                self.app_write_address.set(self.app_write_address.get() + dec_buf_length);

                self.app_loader_client.return_buffer(buffer);

                self.nonvol_storage.write(decompressed_buffer, self.app_write_address.get() , dec_buf_length);

            }
            State::b => {
                // Received buffer w/data from AL Client and it is the last buffer.
            }
            State::c => {

            }
            _ => {}
        }
    }

}

impl<'a> interfaces::AppLoader<'a> for AppLoader<'a> {
    fn start_loading(&self, app_size: usize) -> Result<(), DalsError> {
        Ok(())
    }
    fn next_buffer(&self, data_buffer: &'static mut [u8], length: usize, completed: bool){
        if completed {
            self.state.set(State::b);
        } else {
            self.state.set(State::a);
        }
        self.action_complete(Some(data_buffer), Some(length), Some(completed));
    }
}

impl<'a> interfaces::VerifierClient for AppLoader<'a> {
    fn verification_complete(&self, error: Option<DalsError>){

    }
}

impl<'a> nonvolatile_storage::NonvolatileStorageClient<'a> for AppLoader<'a> {
    fn read_done(&self, buffer: &'static mut [u8], length: usize){

    }
    fn write_done(&self, buffer: &'static mut [u8], length: usize){
        self.state.set(State::c);
        self.action_complete(Some(buffer), Some(length), None);
    }
}