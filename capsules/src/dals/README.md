Dynamic App-Loading System (DALS)
=================================

## Description

This document introduces the design for a novel system named **DALS** which allows 
the Kernel to dynamically (at runtime) load and run apps.  The data for a new app 
to be loaded and run by Tock is transmitted to a Tock-compatible board through any 
standard  communication protocol for embedded systems (BLE, Zigbee, USB, etc.) which 
provides "acks" or a similar service to lower data rates.  This data is then pushed 
segment-by-segment through **DALS** which places it in an empty space in flash memory and
then sets up the process-specific elements for the newly-loaded app.    

## Design

The design of **DALS** consists of a series of capsules with one capsule, the **AppLoader**, considered
to be trusted.  This main trusted capsule is the heart of **DALS** and contains the code which collects
the raw app data from another capsule, moves it through a pipeline of steps, places the
resulting app binary in an empty slot in flash space, and, finally, sets up its process-memory region
so the scheduler can run it at the appropriate time.

Basic Pipeline at the core of **DALS**.


Collect raw data from peripheral capsule (BLE, Zigbee, etc.) -----> Decompress Data  ----->  Find empty slot in flash --- 

---> Verify Data ------> Validate Data ----> Set up process-specific structures (stack, heap, etc.)


The trusted `AppLoader` capsule interfaces with 4 differennt peripheral capsules. These
peripheral capsules consist of the **AppLoaderClient**, **Decompressor**, **Verifier**, and 
**Validator** capsules.  Each capsule is named after the trait or interface it should implement.
These interfaces are defined in `interfaces.rs`.

The way these capsules are interconnected is diagrammed below.


                                            AppLoaderClient
                                                 |
                                                 |
                                                 |
                                                 |
                         Validator ---------  AppLoader  ----------  Decompressor
                                                 |
                                                 |
                                                 |
                                                 |
                                              Verifier
                                        

There exists a circular reference between the **AppLoader** capsule and each of the peripheral
capsules.  This enables the **AppLoader** to call functions implemented by a peripheral capsule and 
vice versa.

The peripheral modules are detailed below.  Please see `interfaces.rs` for more information on functions and such.


## AppLoaderClient

This capsule implements `pub trait AppLoaderClient<'a>`.  This capsule can be any capsule
which receives data via standard communication protocols for embedded systems such as BLE, 
Zigbee, USB.  This capsule's main function is to send this raw received data buffer-by-buffer
to the **AppLoader** capsule.  In order to prevent bottleneck issues, this capsule must send an ack 
only after its buffer is returned to it from the **AppLoader**.

This capsule must also include a buffer the size of one data segment (different for different protocols) which
it sends to the **AppLoader** to send one segment of the received data.

## Decompressor

**DALS** support data decompression methods which "operate on-the-fly".  That is, the received app data
is decompressed segment-by-segment. This capsule must implement `pub trait Decompressor<'a>` and contain 
a decompression algorithm which conforms to the above constraints.

This capsule must include a buffer in which to store the resulting decompressed data which is passed
back to the **AppLoader**.


## Verifier

This capsule implements `pub trait Verifier<'a>`.  This capsule contains an algorithm which can
perform data verification on the app data loaded into flash.  Such an algorithm could be used to 
verify that no bit-errors occured during wireless transmission or that the loaded app is not malware.
Examples of this include checksums, digital signatures or more complex cryptographic computations.

This capsule must contain an immutable reference to the app binary data in flash space.

## Validator

This capsule implements `pub trait Validator<'a>`.  This capsule contains an algorithm which 
performs validation on the app data in flash according to a user specification. This allows the
kernel to check if it actually wants to run the newly loaded app. The validator can check the 
app's length, version, or header info for any salient features.

This capsule must contain an immutable reference to the app binary data in flash space.

## AppLoader

This capsule implements `pub trait AppLoader<'a>`, `pub trait DecompressorClient<'a>`, 
`pub trait VerifierClient<'a>`, and `pub trait ValidatorClient<'a>`.    

