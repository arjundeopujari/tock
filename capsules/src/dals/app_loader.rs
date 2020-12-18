//! Code for the "AppLoader".  This capsule is the heart of the "DALS" system and is
//! in charge of coordinating the whole application loading process which includes 
//! sending/receiving data to/from peripheral capsules in "DALS"hjkhjkhjkhjkhjssssdddssfffddd

use core::cell::Cell;
use kernel::common::cells::{OptionalCell, TakeCell};
use kernel::hil;
use crate::dals::interfaces;
use crate::dals::interfaces::{LoadingError, DecompressionError, VerificationError, ValidationError};

// States
#[derive(Clone, Copy, PartialEq)]
enum State {
    Idle,
    DecompressBuffer,
    WriteFlash,
    WriteFlashFinal,
    Verify,
    Validate,
    Done,
}

pub struct AppLoader<'a> {
    app_loader_client: OptionalCell<&'a dyn interfaces::AppLoaderClient<'a>>,
    decompressor: &'a dyn interfaces::Decompressor<'a>,
    verifier: &'a dyn interfaces::Verifier<'a>,
    validator: &'a dyn interfaces::Validator<'a>,
    nonvolatile_storage: &'a dyn hil::nonvolatile_storage::NonvolatileStorage<'a>,
    app_address: Cell<usize>,
    state: Cell<State>,
}

impl<'a> AppLoader<'a> {
    pub fn new(
        decompressor: &'a dyn interfaces::Decompressor<'a>,
        verifier: &'a dyn interfaces::Verifier<'a>,
        validator: &'a dyn interfaces::Validator<'a>,
        nonvolatile_storage: &'a dyn hil::nonvolatile_storage::NonvolatileStorage<'a>,
    ) -> AppLoader<'a> {
        AppLoader {
            app_loader_client: OptionalCell::empty(),
            decompressor: decompressor,
            verifier: verifier,
            validator: validator,
            nonvolatile_storage: nonvolatile_storage,
            app_address: Cell::new(0),
            state: Cell::new(State::Idle),
        }
    }

    /// TODO: Implementation should set `app_address` field to point to start of loaded app in flash 
    pub fn get_flash_ptr(&self){
        
    }
    
    /// TODO: Implementation should follow `app_flash` to app data in flash and check header length and version
    pub fn check_app_header(&self){
         
    }

    /// TODO: Main FSM for `DALS`
    pub fn process_state(&self, buffer: Option<&'a mut [u8]>, length: Option<usize>, flag: Option<bool>){
        match self.state.get() {
            State::DecompressBuffer => {
                let (decompr_buf, decompr_buf_len, data_buffer, error) = self.decompressor.decompress_buffer(buffer,length);
                if flag {
                    self.state.set(State::WriteFlashFinal(data_buffer));
                } else {
                    self.state.set(State::WriteFlash(data_buffer));
                }
                let write_address : usize = self.app_address.get();
                self.app_address.set(self.app_address.get() + decompr_buf_len);
                self.nonvolatile_storage.write(decompr_buf,write_address,decompr_buf_len);               
            }
            State::WriteFlash => {
                self.decompressor.return_buffer(buffer);
                self.state.set(State::Idle);
            }
            State::WriteFlashFinal => {
                self.decompressor.return_buffer(buffer);
                self.state.set(State::Verify);
                self.verifier.verify_data(self.app_address.get());
            }
            State::Validate => {
                self.validator.validate_data(self.app_address.get());
            }
            State::Done => {

            }
            
            _ => {}
        }
    } 
}

impl<'a> interfaces::AppLoader<'a> for AppLoader<'a> {
    fn set_client(&self, client: &'a dyn interfaces::AppLoaderClient){}
    fn start_loading(&self, app_size: usize) -> Result<(),LoadingError>{
        // Check if board meets conditions for loading an app (ex: sufficient flash, ram space) of this size
        // If not return an error
        Ok(())
    }
    fn next_buffer(&self, data_buffer: &'a mut [u8], length: usize, completed: bool){
        self.state.set(State::DecompressBuffer);
        self.process_state(Some(data_buffer),Some(length),Some(completed));
    }
}

impl<'a> interfaces::VerifierClient<'a> for AppLoader<'a> {
    fn verification_complete(&self, error: Option<VerificationError>){
        self.state.set(State::Validate);
        self.process_state(None, None, None);
    }
}

impl<'a> interfaces::ValidatorClient<'a> for AppLoader<'a> {
    fn validation_complete(&self, error: Option<ValidationError>){
        self.state.set(State::Done);
        self.process_state(None, None, None);
    }
}

impl<'a> hil::nonvolatile_storage::NonvolatileStorageClient<'a> for AppLoader<'a>{
    fn read_done(&self, buffer: &'a mut [u8], length: usize){

    }
    fn write_done(&self, buffer: &'a mut [u8], length: usize){
        self.process_state(Some(buffer),Some(length),None);
    }
}
