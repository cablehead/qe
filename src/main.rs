use std::io::{self, Read};

use base64::{engine::general_purpose, Engine as _};

pub fn b64decode(encoded: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(encoded).unwrap()
}

pub fn b64encode(s: &Vec<u8>) -> String {
    general_purpose::STANDARD.encode(s)
}

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;

fn main() -> wry::Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    // Read standard input
    let mut input = Vec::new();
    io::stdin()
        .read_to_end(&mut input)
        .expect("Failed to read from stdin");

    // Base64 encode the input
    let encoded_input = b64encode(&input);

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let builder = WebViewBuilder::new(&window);

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let builder = {
        use tao::platform::unix::WindowExtUnix;
        use wry::platform::unix::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        WebViewBuilder::new_gtk(vbox)
    };

    let webview = builder
        .with_devtools(true)
        .with_html(include_str!("../static/content.html"))?
        .with_ipc_handler(|message| {
            println!("Textarea content received: {}", message);
            std::process::exit(0);
        })
        .build()?;

    // Inject JavaScript to decode the base64 string and set the textarea value
    webview.evaluate_script(&format!(
        "document.getElementById('content').value = b64ToUtf8('{}');",
        encoded_input
    ))?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit
        }
    });
}
