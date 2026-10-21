mod app;
mod export;

use eframe::egui;
use app::EstudioApp;

fn main() -> Result<(), eframe::Error> {
    let mut opcoes = eframe::NativeOptions::default();
    opcoes.viewport = egui::ViewportBuilder::default()
        .with_title("Estúdio de T-Shirts - Rust Edition")
        .with_inner_size([750.0, 800.0]);

    eframe::run_native(
        "Estúdio de T-Shirts",
        opcoes,
        Box::new(|cc| {
            let caminho_imagem = std::env::args()
                .nth(1)
                .unwrap_or_else(|| "imagem_processada.png".to_string());
            Box::new(EstudioApp::new(cc, caminho_imagem))
        }),
    )
}
