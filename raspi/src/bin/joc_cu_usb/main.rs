use udev::Enumerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut enumerator = Enumerator::new()?;
    enumerator.match_subsystem("usb")?;

    for device in enumerator.scan_devices()? {
        if let Some(devnode) = device.devnode() {
            println!("Device Node: {}", devnode.display());
        }

        for property in device.properties() {
            println!(
                "  {} = {}",
                property.name().to_string_lossy(),
                property.value().to_string_lossy()
            );
        }
    }

    Ok(())
}
