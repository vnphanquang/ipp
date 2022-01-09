use std::process::Command;

pub fn system_add_printer(name: &str, port: u16) {
    if cfg!(target_os = "macos") {
        Command::new("lpadmin")
            .args(["-p", &name.replace(" ", "_")])
            .args(["-D", name])
            .args(["-v", &format!("ipp://localhost:{}", port)])
            .args(["-P", "/System/Library/Frameworks/ApplicationServices.framework/Versions/A/Frameworks/PrintCore.framework/Versions/A/Resources/Generic.ppd"])
            .args(["-o", "printer-is-shared=false", "-E"])
            .output()
            .expect("failed to auto-add printer to system with lpadmin");
    } else if cfg!(target_os = "windows") {
        let info = os_info::get();
        let version = info.version();

        let mut m = "\"MS Publisher Color Printer\"";
        if let os_info::Version::Semantic(major, _, _) = version {
            if *major >= 10 {
                m = "\"Microsoft Print To PDF\""
            }
        }

        // rundll32.exe printui.dll PrintUIEntry /y /b "Rust_IPP_Printer" /if /f "%windir%\inf\ntprint.inf" /r "http://localhost:6363" /m "Microsoft Print To PDF" /z

        let win_dir = match std::env::var("WINDIR") {
            Ok(val) => val,
            Err(_) => String::from("C:\\Windows"),
        };

        Command::new("rundll32.exe")
            .arg("printui.dll PrintUIEntry")
            .arg("/y")
            .args(["/b", &name.replace(" ", "_")])
            .arg("/if")
            .args(["/f", &format!("{}\\inf\\ntprint.inf", win_dir)])
            .args(["/r", &format!("http://localhost:{}", port)])
            .args(["/m", m])
            .arg("/z")
            .output()
            .expect("failed to auto-add printer to system with rundll32.exe");
    } else {
        // no support yet
    }
}

pub fn system_remove_printer(name: &str) {
    if cfg!(target_os = "macos") {
        Command::new("lpadmin")
            .args(["-x", &name.replace(" ", "_")])
            .output()
            .expect("failed to auto-remove printer to system with lpadmin");
    } else if cfg!(target_os = "windows") {
        Command::new("rundll32.exe")
            .arg("/dl")
            .args(["/n", &name.replace(" ", "_")])
            .output()
            .expect("failed to auto-remove printer to system with rundll32.exe");
    } else {
        // no support yet
    }
}

pub fn ps_to_pdf(ps_path: &str, pdf_path: &str) {
    let mut gs_path = "gs";
    if cfg!(target_os = "windows") {
        // FIXME: gs_path for windows should be passed dynamic
        gs_path = "C:\\gs\\gs9.23\\bin\\gswin32.exe"
    }
    Command::new(gs_path)
        .args([
            "-q",
            "-sPAPERSIZE=a4",
            "-dSAFER",
            "-dBATCH",
            "-dNOPAUSE",
            "-sDEVICE=pdfimage8",
            "-r600",
            "-dDownScaleFactor=3",
        ])
        .arg(format!("-sOutputFile={}", pdf_path))
        .args(["-c", "save", "pop"])
        .arg("-f")
        .arg(ps_path)
        .output()
        .expect("failed to execute gs command for ps-pdf conversion");
}
