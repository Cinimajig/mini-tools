fn main() {
    windows::build!(
        windows::ui::notifications::*,
        windows::data::xml::dom::XmlDocument,
        windows::win32::windows_programming::{RegOpenKeyExW, RegSetValueExA, RegSetValueExW, RegCloseKey, HKEY, RegCreateKeyExW},
        windows::win32::shell::{IShellLinkW, ShellLink},
        windows::win32::com::{CoCreateInstance, CoUninitialize, IPersistFile},
        windows::win32::file_system::GetFullPathNameW,
        windows::win32::system_services::{BOOL, LSTATUS, PSTR, PWSTR, GetStartupInfoW}
    );
}
