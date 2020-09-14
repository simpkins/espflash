use std::fs::read;

use espflash::Flasher;
use main_error::MainError;
use pico_args::Arguments;
use serial::{BaudRate, SerialPort};

fn help() -> Result<(), MainError> {
    println!("Usage: espflash [--board-info] [--ram] <serial> [<elf image>]");
    Ok(())
}

fn main() -> Result<(), MainError> {
    let mut args = Arguments::from_env();

    if args.contains(["-h", "--help"]) {
        return help();
    }

    let ram = args.contains("--ram");
    let board_info = args.contains("--board-info");

    let serial: String = match args.free_from_str()? {
        Some(serial) => serial,
        _ => return help(),
    };

    let mut serial = serial::open(&serial)?;
    serial.reconfigure(&|settings| {
        settings.set_baud_rate(BaudRate::Baud115200)?;

        Ok(())
    })?;

    let mut flasher = Flasher::connect(serial)?;

    if board_info {
        println!("Chip type: {:?}", flasher.chip());
        println!("Flash size: {:?}", flasher.flash_size());

        return Ok(());
    }

    let input: String = match args.free_from_str()? {
        Some(input) => input,
        _ => return help(),
    };
    let input_bytes = read(&input)?;

    if ram {
        flasher.load_elf_to_ram(&input_bytes)?;
    } else {
        flasher.load_elf_to_flash(&input_bytes)?;
    }

    Ok(())
}
