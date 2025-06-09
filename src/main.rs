use std::{
    ptr,
    fs::File,
    io::Read,
    env::args,
    thread::sleep,
    time::Duration,
    mem::MaybeUninit,
    ffi::{CString, CStr, c_char, c_ushort, c_uint, c_ulong},
};
use x11::xlib;
use chrono::Local;

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
    fn XkbRF_GetNamesProp(
        _3: *mut xlib::Display,
        _2: *const c_char,
        _1: XkbRF_VarDefsPtr
    ) -> bool;
}

struct Bar {
    display: *mut xlib::Display,
    window: c_ulong,
    looped: bool,
}

impl Bar {
    pub fn new(looped: bool) -> Self {
        let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
        let window = unsafe { xlib::XRootWindow(display, xlib::XDefaultScreen(display)) };

        Self {
            display,
            window,
            looped
        }
    }

    pub fn run(&self) {
        if self.looped {
            loop {
                self.statusbar();
                sleep(ONE_SEC);
            };
        } else {
            self.statusbar();
        }

        self.close_display();
    }

    fn xsetroot<T: AsRef<str>>(&self, new_name: T) {
        let name = CString::new(new_name.as_ref()).unwrap();
        unsafe { xlib::XStoreName(self.display, self.window, name.as_ptr()) };
    }

    fn lang(&self) -> String {
        let (kl, s) = unsafe {
            let mut state: MaybeUninit<xlib::_XkbStateRec> = MaybeUninit::uninit();
            let _ = xlib::XkbGetState(self.display, XkbUseCoreKbd, state.as_mut_ptr());

            let mut vd: MaybeUninit<_XkbRF_VarDefs> = MaybeUninit::uninit();
            let _ = XkbRF_GetNamesProp(self.display, ptr::null(), vd.as_mut_ptr());

            (CStr::from_ptr(vd.assume_init().layout).to_str().unwrap(), state.assume_init().group)
        };

        kl.split(",")
            .collect::<Vec<&str>>()[s as usize]
            .to_string()
    }

    fn close_display(&self) {
        unsafe { xlib::XCloseDisplay(self.display) };
    }

    fn temp(&self) -> String {
        let mut temp = String::new();
        File::open("/sys/class/hwmon/hwmon0/temp1_input")
            .unwrap()
            .read_to_string(&mut temp)
            .unwrap();

        format!("+{:.2}.0°C", &temp[..2])
    }

    fn statusbar(&self) {
        let lang = self.lang().to_uppercase();

        let datetime = Local::now();
        let date = &datetime.format("%d.%m.%y");
        let time = &datetime.format("%H:%M:%S");

        let temp = self.temp();

        let bar = format!("    | {temp} | {lang} | {date} | {time} |   ");
        self.xsetroot(&bar);
    }
}

fn cli() -> bool {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => false,
        2 => {
            match args[1].as_ref() {
                "-l" | "--loop" => {
                    true
                }
                _ => panic!("Undifined flag"),
            }
        }
        _ => panic!("Undifined extra flag/s")
    }
}

fn main() {
    Bar::new(cli())
        .run();
}
