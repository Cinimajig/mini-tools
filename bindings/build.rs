fn main() {
    windows::build!(
        Windows::UI::Notifications::*,
        Windows::Data::Xml::Dom::XmlDocument,
        Windows::Win32::WindowsProgramming::{REG_OPEN_CREATE_OPTIONS, REG_SAM_FLAGS, REG_VALUE_TYPE, RegOpenKeyExW, RegSetValueExA, RegSetValueExW, RegCloseKey, HKEY, RegCreateKeyExW},
        Windows::Win32::Shell::{IShellLinkW, ShellLink},
        Windows::Win32::Com::{CoCreateInstance, CoUninitialize, IPersistFile},
        Windows::Win32::FileSystem::GetFullPathNameW,
        Windows::Win32::SystemServices::{BOOL, LSTATUS, PSTR, PWSTR, GetStartupInfoW}
    );
}
