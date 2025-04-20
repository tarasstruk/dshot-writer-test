## DShot writer test

This binary crate is part of a test framework for DShot protocol communication.

It is a complementary side to [dshot-reader-test](https://github.com/tarasstruk/dshot-reader-test)
and implements its publisher counterpart.

The values are being read from UART in four-digit ASCII format (followed by newline)
and pushed forward as DShot messages.

### Hardware
 
A RP2040 and a UART USB adapter
are required to run the program.

### Connection

- connect RP2040 GPIO 5 pin to TX pin on the UART adapter;
- connect RP2040 signal ground to the ground of UART adapter;
- connect RP2040 GPIO 0 to a consumer of DShot signals;
- connect RP2040 signal ground to the ground of consumer.

### Installation

The compiled binary could be installed to PR2040 when it's
connected to the host computer as USB mass storage.
Then run `cargo r`.
