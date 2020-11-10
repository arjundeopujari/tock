Dynamic App-Loading System (DALS)
=================================

When a microcontroller boots (or resets, or services an interrupt) it loads an

## Description

This document introduces the design for a novel system named `DALS` which allows 
the Kernel to dynamically (at runtime) load and run apps.  The data for a new app 
to be loaded and run by Tock is transmitted to a Tock-compatible board through any 
standard  communication protocol (BLE, Zigbee, USB, UART, etc.)  This data is then pushed 
segment-by-segment through `DALS` which places it in an empty space in flash space and
then sets up the process-specific elements for the newly-loaded app.

## Design

The design of `DALS` consists of a series of capsules with one capsule, the `AppLoader`, considered
to be trusted.  This main trusted capsule is the heart of `DALS` and contains the code which collects
the raw app data from another capsule, moves it through a pipeline of steps, places the
resulting app binary in an empty slot in flash space, and, finally, sets up its process-memory region
so the scheduler can run it at the appropriate time.


