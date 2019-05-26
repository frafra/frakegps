mod nmea;

extern crate web_view;
use web_view::*;

fn main() -> WVResult {
    let html = format!(include_str!("dist/map.html"),
        styles = inline_style(include_str!("dist/map.css")),
        scripts = inline_script(include_str!("dist/map.js")),
    );
    let webview = web_view::builder()
        .title("FrakeGPS")
        .content(Content::Html(html))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, arg| {
            let latlon: Vec<&str> = arg.split(",").collect();
            let lat: f64 = latlon[0].parse().unwrap();
            let lon: f64 = latlon[1].parse().unwrap();
            let alt: f64 = 0.0;
            println!("{}", nmea::gga(lat, lon, alt));
            Ok(())
        })
        .build()?;
    webview.run()
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
