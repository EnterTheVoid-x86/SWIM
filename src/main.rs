#[macro_use]
// SWIM Window Manager
// Main part of the workspace.
extern crate penrose;
mod styles;
mod hooks;
use styles::{ PROFONT, colors, dimensions };
use simplelog::{ LevelFilter, SimpleLogger };
use std::time::{SystemTime, UNIX_EPOCH};
use penrose::{
    core::{
        layout::{
            LayoutConf,
            side_stack,
        },
        Layout, 
        ring::Direction::{
            Forward,
            Backward,
        }, 
        data_types::Change::{
            More,
            Less,
        },
        helpers::index_selectors, 
        hooks::Hooks,
    }, 
    draw::{
        TextStyle,
        Color,
        dwm_bar,
    },
    xcb::XcbDraw,
    Config, 
    Selector, 
    XcbConnection, new_xcb_backed_window_manager, logging_error_handler, 
};
// Define your terminal, app launcher, and path to your startup script here.
pub const TERMINAL: &str = "st";
pub const LAUNCHER: &str = "dmenu_run";
pub const PATH_TO_START_SCRIPT: &str = "bash /etc/swim/startup";

fn main() -> penrose::Result<()> {
    println!("Welcome to SWIM v1.1.5.");
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e);
    };
    // Define core parts of the window manager
    let gap_size = 10;
    let side_stack_layout = Layout::new("SWIM v1.1.5 [[]=]", LayoutConf::default(), side_stack, 1, 0.6);
    let config = Config::default()
        .builder()
	    .gap_px(gap_size)
        .show_bar(true)
        .top_bar(true)
        .layouts(vec![side_stack_layout])
        .focused_border(colors::BLUE)?
        .build()
        .expect("Unable to build configuration");

    let style = TextStyle {
        font: PROFONT.to_string(),
        point_size: 11,
        fg: Color::try_from(colors::WHITE)?,
        bg: Some(Color::try_from(colors::GREY)?),
        padding: (2.0, 2.0),
    };

    let empty_ws = Color::try_from(colors::WHITE)?;
    let draw = XcbDraw::new()?;

    let bar = dwm_bar(
        draw,
        dimensions::HEIGHT,
        &style,
        Color::try_from(colors::BLUE)?,
        empty_ws,
        config.workspaces().clone(),
    )?;

    let key_bindings = gen_keybindings! {
        // Program launchers
        "M-d" => run_external!(LAUNCHER);
        "M-Return" => run_external!(TERMINAL);

        // Exit SWIM (important to remember this one!)
        "M-S-c" => run_internal!(exit);

        // client management
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-c" => run_internal!(kill_client);

        // workspace management
        "M-Tab" => run_internal!(toggle_workspace);
        "M-A-period" => run_internal!(cycle_workspace, Forward);
        "M-A-comma" => run_internal!(cycle_workspace, Backward);

        // Layout management
        "M-A-j" => run_internal!(cycle_layout, Forward);
        "M-A-k" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-l" => run_internal!(update_main_ratio, More);
        "M-h" => run_internal!(update_main_ratio, Less);
        "F10" => run_external!("pavucontrol-qt");
        "F1" => run_external!("swimkeybinds");
        // Workspace mapping
        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9", "0" } to index_selectors(10) => {
             "M-{}" => focus_workspace (REF);
             "M-S-{}" => client_to_workspace (REF);
         };
    };

    let hooks: Hooks<XcbConnection> = vec![
        Box::new(bar),
        Box::new(hooks::StartupScript::new(PATH_TO_START_SCRIPT)),
    ];

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, map!{})
}

