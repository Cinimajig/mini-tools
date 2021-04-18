/// Helper struct, to help with CoInitialize and CoUnitialize of COM.
pub struct InitCom();

impl InitCom {
    #![allow(dead_code)]
    /// Single-Threaded Application.
    pub fn sta() -> windows::Result<Self> {
        if let Err(err) = windows::initialize_sta() {
            Err(err)
        } else {
            Ok(Self())
        }
    }

    /// Multi-Threaded Application.
    pub fn mta() -> windows::Result<Self> {
        if let Err(err) = windows::initialize_mta() {
            Err(err)
        } else {
            Ok(Self())
        }
    }
}

impl Drop for InitCom {
    fn drop(&mut self) {
        unsafe {
            bindings::Windows::Win32::Com::CoUninitialize();
        }
    }
}
