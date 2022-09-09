use soloud::*;
extern crate system_shutdown;

use system_shutdown::reboot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sl = Soloud::default()?;

    let mut xeogenisis = audio::Wav::default();

    xeogenisis.load(&std::path::Path::new("/home/matees/outro.wav"))?;

    sl.play(&xeogenisis);
    let mut index: i8 = 10;

    while index != -1 {
        if index != 0 {
            std::thread::sleep(std::time::Duration::from_millis(1500));
            println!("Rebooting in {}...", index);
            index = index - 1;
            continue;
        }

        println!("See you next time 😎😎");
        std::thread::sleep(std::time::Duration::from_millis(1500));
        break;
    }

    match reboot() {
        Ok(_) => println!("Bye"),
        Err(error) => eprintln!("Failed to shut down: {}", error),
    }

    Ok(())
}
