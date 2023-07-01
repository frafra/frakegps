mod nmea;

use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

fn main() -> wry::Result<()> {
    let html = format!(
        include_str!("dist/map.html"),
        styles = inline_style(include_str!("dist/map.css")),
        scripts = inline_script(include_str!("dist/map.js")),
    );
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("FrakeGPS")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_html(html)?
        .with_ipc_handler(move |_webview, arg| {
            let latlon: Vec<&str> = arg.split(",").collect();
            let lat: f64 = latlon[0].parse().unwrap();
            let lon: f64 = latlon[1].parse().unwrap();
            let alt: f64 = 0.0;
            println!("{}", nmea::gga(lat, lon, alt));
        })
        .build()?;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::NewEvents(StartCause::Init) => (),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
