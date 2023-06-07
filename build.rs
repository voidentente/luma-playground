fn main() {
    let Some(elf) = std::env::args().skip(1).next() else {
        // Currently in cargo environment.
        println!("cargo:rerun-if-changed=build.rs");
        let this = std::env::current_exe().unwrap();
        let mut root = build::cargo_manifest_dir();
        root.push(".cargo");
        root.push(this.file_name().unwrap());
        std::fs::copy(this, root).unwrap();
        return
    };

    // Currently in runner environment.
    let dolphin = match std::env::consts::FAMILY {
        "unix" => "dolphin-emu",
        "windows" => "Dolphin.exe",
        _ => unreachable!(),
    };

    let mut cmd = std::process::Command::new(dolphin);

    if std::env::consts::FAMILY == "unix" {
        cmd.env("QT_QPA_PLATFORM", "xcb");
    }

    cmd.args(["-C", "Dolphin.Display.Fullscreen=true"]);
    cmd.arg("-b");
    cmd.args(["-e", &elf]);

    cmd.spawn().expect("Could not find Dolphin.");
}
