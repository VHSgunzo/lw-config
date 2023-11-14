use std::{
    fs,
    env,
    path::Path,
    process::exit,
    process::Stdio, rc::Rc,
    // process::Child
};

use execute::{
    shell, 
    Execute
};
use slint::{
    ModelRc, 
    VecModel, 
    SharedString
};

slint::include_modules!();

fn get_env(var: &str) -> String {
    if let Ok(ret) = env::var(var)
        { ret } else { "".to_string() }
}

fn is_file_exist(path: String) -> bool {
    let file = Path::new(&path);
    if file.exists() && file.is_file() {
        return true
    }
    return false
}

fn is_dir_exist(path: String) -> bool {
    let file = Path::new(&path);
    if file.exists() && file.is_dir() {
        return true
    }
    return false
}

// fn shellspawn(command: &str) -> Child {
//     shell(command).spawn()
//         .expect("Shell command failed to start!")
// }

fn shellexec(command: &str) -> String {
    String::from_utf8(
        shell(command)
            .stdout(Stdio::piped())
            .execute_output().unwrap().stdout
    ).unwrap()
}

fn updcfg(var: &str, value: &str) {
    let lwcfg = get_env("LW_CFG");
    if ! lwcfg.is_empty() {
        let var_is_empty = shellexec(&format!("grep '^{}=' '{}'", var, lwcfg)).is_empty();
        if var_is_empty {
            shellexec(&format!("echo '{}=\"{}\"' >> '{}'", var, value, lwcfg));
        } else {
            shellexec(&format!("sed -i 's|^{0}=.*|{0}=\"{1}\"|g' '{2}'", var, value, lwcfg));
        }
    }
}

fn dir_list_model(path: String) -> Rc<VecModel<SharedString>> {
    let mut list = vec![];
    if let Ok(entries) = fs::read_dir(path) {
        for item in entries {
            list.push(item.unwrap().file_name().to_str().unwrap().to_string().into())
        }
    }
    Rc::new(VecModel::from(list))
}

// fn dir_list_modelrc(path: String) -> ModelRc<SharedString> {
//     let mut list = vec![];
//     if let Ok(entries) = fs::read_dir(path) {
//         for item in entries {
//             list.push(item.unwrap().file_name().to_str().unwrap().to_string().into())
//         }
//     }
//     ModelRc::from(Rc::new(VecModel::from(list)))
// }

pub fn main() {  
    let mut lu_exe = false;
    let mut run_exe = false;
    let mut first_run = false;
    let mut lu_exe_nofirstrun = false;
    if ! get_env("LU_EXE").is_empty() { lu_exe = true }
    if ! get_env("RUN_EXE").is_empty() { run_exe = true }
    if ! get_env("FIRST_RUN").is_empty() { first_run = true }
    if lu_exe && ! first_run { lu_exe_nofirstrun = true }

    slint::init_translations!(concat!(env!("CARGO_MANIFEST_DIR"), "/locale/"));

    let ui = LuxWineSettings::new().unwrap();

    // let handle = ui.as_weak();
    // let handle_clone = handle.clone();
    // ui.on_wine_dxgi_on(move || {
    //     let ui = handle_clone.unwrap();
    //     ui.set_wine_dxgi(true);
    // });

    let settwidth = get_env("SETTWIDTH");
    if ! settwidth.is_empty() && settwidth.parse::<f32>().is_ok() {
        ui.set_settwidth(settwidth.parse::<f32>().unwrap())
    }
    let settheight = get_env("SETTHEIGHT");
    if ! settheight.is_empty() && settheight.parse::<f32>().is_ok() {
        ui.set_settheight(settheight.parse::<f32>().unwrap())
    }

    if first_run {
        ui.set_reset_btn_visible(false);
        ui.set_tools_tab_btns_enabled(false)
    }
    if lu_exe_nofirstrun {
        if ! run_exe { ui.set_run_btn_visible(true) }
        ui.set_exe_full_name(format!("({})", get_env("EXE_FULL_NAME")).into())
    }

    let lw_wine_dir = get_env("LW_WINE_DIR");
    if is_dir_exist(lw_wine_dir.clone()) {
        let model = dir_list_model(lw_wine_dir.clone());
        if ! get_env("SYS_WINE").is_empty() { model.push("System".into()) }
        if lu_exe_nofirstrun { model.push("Default".into()) }
        ui.set_wine_list(ModelRc::from(model))
    }

    ui.set_wine_version(get_env("WINE_VERSION").into());
    ui.on_wine_version_cb(move |value| {
        updcfg("WINE_VERSION", &format!("{value}").as_str())
    });

    if get_env("DISABLE_NVAPI") == "1" { ui.set_disable_nvapi(true) }
    ui.on_disable_nvapi_on(move  || { updcfg("DISABLE_NVAPI", "1") });
    ui.on_disable_nvapi_off(move || { updcfg("DISABLE_NVAPI", "0") });

    if get_env("D3D_EXTRAS") == "1" { ui.set_d3d_extras(true) };
    ui.on_d3d_extras_on(move  || { updcfg("D3D_EXTRAS", "1") });
    ui.on_d3d_extras_off(move || { updcfg("D3D_EXTRAS", "0") });

    if get_env("WINE_DXGI") == "1" { ui.set_wine_dxgi(true) };
    ui.on_wine_dxgi_on(move  || { updcfg("WINE_DXGI", "1") });
    ui.on_wine_dxgi_off(move || { updcfg("WINE_DXGI", "0") });

    if get_env("DGVOODOO2") == "1" { ui.set_dgvoodoo2(true) };
    ui.on_dgvoodoo2_on(move  || { updcfg("DGVOODOO2", "1") });
    ui.on_dgvoodoo2_off(move || { updcfg("DGVOODOO2", "0") });
    
    let wtrx_log = get_env("WTRX_LOG");
    if ! wtrx_log.is_empty() {
        if is_file_exist(wtrx_log.clone()) {
            ui.set_winetricks_log(
                fs::read_to_string(
                    wtrx_log.clone()
                ).unwrap().into()
            )
        }
        ui.on_winetricks_log_te(move |log| {
            fs::write(wtrx_log.clone(), log).unwrap() 
        });
    }

    let lw_cenv = get_env("LW_CENV");
    if ! lw_cenv.is_empty() {
        if is_file_exist(lw_cenv.clone()) {
            ui.set_custom_env(
                fs::read_to_string(
                    lw_cenv.clone()
                ).unwrap().into()
            )
        }
        ui.on_custom_env_te(move |cenv| {
            fs::write(lw_cenv.clone(), cenv).unwrap() 
        });
    }

    // let _thread = std::thread::spawn(move || {
    //     let dir = dir_picker("".to_string());
    //     let _ = handle.upgrade_in_event_loop(move |ui|
    //         ui.set_wine_pfx(dir.to_str().unwrap().into())
    //     );
    // });

    ui.on_init_clicked(move || {
        shellexec("lwrap -init");
    });

    ui.on_reset_clicked(move || { exit(3) });
    ui.on_run_clicked(move || { exit(4) });
    ui.on_ok_clicked(move || { exit(0) });

    ui.run().unwrap();
}
