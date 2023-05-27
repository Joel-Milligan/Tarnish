use std::env;

fn main() {
    let path = env!("UEFI_PATH");

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
    cmd.arg("-drive").arg(format!("format=raw,file={path}"));

    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
