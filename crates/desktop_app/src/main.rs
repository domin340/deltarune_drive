use macroquad::{
    color,
    text::{draw_text, load_ttf_font, set_default_font},
    window::{Conf as WindowConf, clear_background, next_frame},
};

fn window_conf() -> WindowConf {
    WindowConf {
        window_title: "Deltarune Drive".into(),
        sample_count: 1,
        high_dpi: true,
        window_width: 840,
        window_height: 540,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    set_default_font(
        load_ttf_font("./assets/monogram/monogram.ttf")
            .await
            .expect("couldn't load monogram font!"),
    );

    loop {
        clear_background(color::WHITE);
        draw_text("hello world", 25.0, 25.0, 20.0, color::BLACK);
        next_frame().await;
    }
}
