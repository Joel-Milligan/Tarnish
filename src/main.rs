use std::env;

fn main() {
    let uefi = false;
    let path = if uefi {
        env!("UEFI_PATH")
    } else {
        env!("BIOS_PATH")
    };

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-drive").arg(format!("format=raw,file={path}"));

    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    }

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
