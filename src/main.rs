use chrono::Local;
use std::{
    env::args,
    ffi::{c_char, c_uint, c_ulong, c_ushort, CStr, CString},
    fs::File,
    io::Read,
    mem::MaybeUninit,
    ptr,
    thread::sleep,
    time::Duration,
};
use x11::xlib;

static ONE_SEC: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct _XkbRF_VarDefs {
    pub model: *mut c_char,
    pub layout: *mut c_char,
    pub variant: *mut c_char,
    pub options: *mut c_char,
    pub sz_extra: c_ushort,
    pub num_extra: c_ushort,
    pub extra_names: *mut c_char,
    pub extra_values: *mut c_char,
}

#[allow(non_camel_case_types)]
type XkbRF_VarDefsPtr = *mut _XkbRF_VarDefs;

#[allow(non_upper_case_globals)]
static XkbUseCoreKbd: c_uint = 0x0100;

extern "C" {
    fn XkbRF_GetNamesProp(_3: *mut xlib::Display, _2: *const c_char, _1: XkbRF_VarDefsPtr) -> bool;
}

struct X11Bar {
    display: *mut xlib::Display,
    window: c_ulong,
    refresh_rate: Duration,
    is_looped: bool,
}

impl Default for X11Bar {
    fn default() -> Self {
        let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        let window = unsafe { xlib::XRootWindow(display, xlib::XDefaultScreen(display)) };
        let refresh_rate = ONE_SEC;
        let is_looped = false;

        Self {
            display,
            window,
            refresh_rate,
            is_looped,
        }
    }
}

impl X11Bar {
    pub fn run(&self) {
        self.xsetroot(self.statusbar());
        while self.is_looped {
            sleep(self.refresh_rate);
            self.xsetroot(self.statusbar());
        }

        self.close_display();
    }

    fn xsetroot<T: AsRef<str>>(&self, new_name: T) {
        let name = CString::new(new_name.as_ref()).unwrap();
        unsafe { xlib::XStoreName(self.display, self.window, name.as_ptr()) };
    }

    fn close_display(&self) {
        unsafe { xlib::XCloseDisplay(self.display) };
    }
}

trait StatusBar {
    fn statusbar(&self) -> String;
}

impl StatusBar for X11Bar {
    fn statusbar(&self) -> String {
        let mut temp = String::new();
        let temp_file_path = "/sys/class/hwmon/hwmon0/temp1_input ";
        File::open(&temp_file_path)
            .unwrap_or_else(|_| panic!("Can not open file {}", temp_file_path))
            .read_to_string(&mut temp)
            .unwrap();
        temp = format!("+{:.2}.0°C", &temp[..2]);

        let (kl, s) = unsafe {
            let mut state: MaybeUninit<xlib::_XkbStateRec> = MaybeUninit::uninit();
            let _ = xlib::XkbGetState(self.display, XkbUseCoreKbd, state.as_mut_ptr());

            let mut vd: MaybeUninit<_XkbRF_VarDefs> = MaybeUninit::uninit();
            let _ = XkbRF_GetNamesProp(self.display, ptr::null(), vd.as_mut_ptr());

            (
                CStr::from_ptr(vd.assume_init().layout).to_str().unwrap(),
                state.assume_init().group,
            )
        };

        let lang = kl.split(",").collect::<Vec<&str>>()[s as usize]
            .to_string()
            .to_uppercase();

        let datetime = Local::now();
        let date = &datetime.format("%d.%m.%y");
        let time = &datetime.format("%H:%M:%S");

        format!("    | {temp} | {lang} | {date} | {time} |   ")
    }
}

fn cli() -> X11Bar {
    let args: Vec<String> = args().collect();
    let mut bar = X11Bar::default();

    if args.len() < 2 {
        return bar;
    }

    for arg in args.iter().enumerate() {
        match arg.1.as_ref() {
            "-r" | "--refresh-rate" => {
                bar.refresh_rate =
                    Duration::from_millis(args[arg.0 + 1].parse::<u64>().unwrap_or_else(|_| {
                        panic!(
                            ">>> {} <<<\nInvalid syntax: value of refresh rate must be an integer",
                            args[arg.0 + 1]
                        )
                    }))
            }
            "-l" | "--loop" => bar.is_looped = true,
            _ => (),
        }
    }

    bar
}

fn main() {
    cli().run();
}
