use winrt::windows;
use wstring::WideString;

fn start() -> windows::Result<()> {
    println!("Hello, world!");

    Ok(())
}

fn main() {
    let result = start();

    if let Err(err) = result {
        eprintln!("{}", err);
    }
}
