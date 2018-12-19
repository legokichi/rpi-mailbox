extern crate nix;

mod mailbox;
mod raspberrypi_firmware;

use mailbox::*;
use raspberrypi_firmware::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}