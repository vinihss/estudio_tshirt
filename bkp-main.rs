use eframe::egui;
use image::{DynamicImage, GenericImageView, Pixel};
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), eframe::Error> {
    let mut opcoes = eframe::NativeOptions::default();
    opcoes.viewport = egui::ViewportBuilder::default()
        .with_title("Estúdio de T-Shirts - Rust Edition")
        .with_inner_size([700.0, 750.0]);

    eframe::run_native(
        "Estúdio de T-Shirts",
        opcoes,
        Box::new(|cc| {
            let caminho_imagem = std::env::args().nth(1).unwrap_or_else(|| "imagem_processada.png".to_string());
            Box::new(EstudioApp::new(cc, caminho_imagem))
        }),
    )
}

struct EstudioApp {
    imagem_original: Option<DynamicImage>,
    caminho_origem: String,
    largura: u32,
    contraste: f32,
    brilho: i32,
    estilo: usize, // 0: ASCII, 1: Blocks, 2: Braille
    fundo_branco: bool,
    corte_braille: u8,
    textura_preview: Option<egui::TextureHandle>,
}

impl EstudioApp {
    fn new(_cc: &eframe::CreationContext<'_>, caminho: String) -> Self {
        let img = if Path::new(&caminho).exists() {
            image::open(&caminho).ok()
        } else {
            None
        };

        if img.is_none() {
            println!("Aviso: Nenhuma imagem válida encontrada em '{}'.", caminho);
        }

        Self {
            imagem_original: img,
            caminho_origem: caminho,
            largura: 120,
            contraste: 1.3,
            brilho: 10,
            estilo: 2,
            fundo_branco: true,
            corte_braille: 128,
            textura_preview: None,
        }
    }

    fn pixel_para_braille(&self, bloco: &[[bool; 2]; 4]) -> char {
        let mut codigo = 0x2800;
        let valores_pontos = [
            [0x01, 0x08],
            [0x02, 0x10],
            [0x04, 0x20],
            [0x40, 0x80]
        ];
        for y in 0..4 {
            for x in 0..2 {
                if bloco[y][x] {
                    codigo |= valores_pontos[y][x];
                }
            }
        }
        std::char::from_u32(codigo).unwrap_or(' ')
    }

    // Função interna para gerar uma simulação visual rápida em píxeis para o painel do egui
    fn gerar_preview_pixels(&self) -> Option<egui::ColorImage> {
        let img = self.imagem_original.as_ref()?;
        let (largura_orig, altura_orig) = img.dimensions();
        let proporcao = altura_orig as f32 / largura_orig as f32;
        
        let l_preview = self.largura;
        let a_preview = (l_preview as f32 * proporcao * 0.5) as u32;
        if l_preview == 0 || a_preview == 0 { return None; }

        let img_red = img.resize_exact(l_preview, a_preview, image::imageops::FilterType::Nearest);
        let mut pixels_saida = vec![egui::Color32::BLACK; (l_preview * a_preview) as usize];

        for y in 0..a_preview {
            for x in 0..l_preview {
                let pixel = img_red.get_pixel(x, y);
                let rgb = pixel.to_rgb();
                
                let r = rgb.0[0] as f32;
                let g = rgb.0[1] as f32;
                let b = rgb.0[2] as f32;
                let b_pixel = (0.299 * r + 0.587 * g + 0.114 * b) as u8;

                let mut adj = (b_pixel as f32 * self.contraste) as i32 + self.brilho;
                if adj > 255 { adj = 255; }
                if adj < 0 { adj = 0; }

                // Simular cores baseado no modo de fundo selecionado
                if self.fundo_branco {
                    if adj < self.corte_braille as i32 {
                        pixels_saida[(y * l_preview + x) as usize] = egui::Color32::from_rgb(rgb.0[0], rgb.0[1], rgb.0[2]);
                    } else {
                        pixels_saida[(y * l_preview + x) as usize] = egui::Color32::WHITE;
                    }
                } else {
                    if adj > self.corte_braille as i32 {
                        pixels_saida[(y * l_preview + x) as usize] = egui::Color32::from_rgb(rgb.0[0], rgb.0[1], rgb.0[2]);
                    } else {
                        pixels_saida[(y * l_preview + x) as usize] = egui::Color32::BLACK;
                    }
                }
            }
        }

        Some(egui::ColorImage {
            size: [l_preview as usize, a_preview as usize],
            pixels: pixels_saida,
        })
    }
}

impl eframe::App for EstudioApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ajustes de Design para T-Shirt (Rust)");
            ui.separator();

            if self.imagem_original.is_none() {
                ui.colored_label(egui::Color32::RED, format!("Ficheiro não encontrado: {}", self.caminho_origem));
                return;
            }

            // --- MENU DE AJUSTES ---
            ui.horizontal(|ui| {
                ui.label("Estilo Visual:");
                if ui.radio_value(&mut self.estilo, 0, "ASCII").changed() ||
                   ui.radio_value(&mut self.estilo, 1, "Blocks").changed() ||
                   ui.radio_value(&mut self.estilo, 2, "Braille").changed() {
                       self.textura_preview = None; // Reset textura se mudar estilo
                }
            });

            let mut mudou = false;
            if ui.add(egui::Slider::new(&mut self.largura, 40..=250).text("Largura")).changed() { mudou = true; }
            if ui.add(egui::Slider::new(&mut self.contraste, 0.5..=3.0).text("Contraste")).changed() { mudou = true; }
            if ui.add(egui::Slider::new(&mut self.brilho, -60..=60).text("Brilho")).changed() { mudou = true; }

            if self.estilo == 2 {
                if ui.add(egui::Slider::new(&mut self.corte_braille, 0..=255).text("Corte Braille")).changed() { mudou = true; }
            }

            if ui.checkbox(&mut self.fundo_branco, "Otimizar para T-Shirt Branca").changed() { mudou = true; }

            ui.add_space(10.0);

            if ui.button("💾 EXPORTAR HTML PARA IMPRESSÃO").clicked() {
                let nome_saida = format!("tshirt_rust_estilo_{}.html", self.estilo);
                if exportar_arte_html(self, &nome_saida).is_ok() {
                    println!("--> Exportado com sucesso para: {}", nome_saida);
                }
            }

            ui.separator();
            ui.label("Pré-visualização em tempo real:");

            // --- ATUALIZAÇÃO DA TEXTURA E DESENHO NA TELA ---
            if mudou || self.textura_preview.is_none() {
                if let Some(cor_imagem) = self.gerar_preview_pixels() {
                    self.textura_preview = Some(ctx.load_texture(
                        "preview_live",
                        cor_imagem,
                        egui::TextureOptions::NEAREST, // Mantém o aspeto pixelizado/retro
                    ));
                }
            }

            if let Some(ref textura) = self.textura_preview {
                // Desenha a imagem centralizada redimensionando de forma bonita para a janela
                ui.image(egui::load::SizedTexture::new(textura.id(), [500.0, 500.0 * (textura.size()[1] as f32 / textura.size()[0] as f32)]));
            }
        });
    }
}

fn exportar_arte_html(app: &EstudioApp, caminho_saida: &str) -> std::io::Result<()> {
    let img = app.imagem_original.as_ref().unwrap();
    let mut file = File::create(caminho_saida)?;

    let cor_fundo = if app.fundo_branco { "#FFFFFF" } else { "#000000" };
    
    writeln!(file, "<!DOCTYPE html><html><head><meta charset='utf-8'>")?;
    writeln!(file, "<style>body {{ background-color: {}; text-align: center; }} pre {{ font-family: monospace; font-size: 8px; line-height: 1.0; display: inline-block; text-align: left; }} @media print {{ pre {{ font-size: 5px; }} }}</style></head><body><pre>", cor_fundo)?;

    let (largura_orig, altura_orig) = img.dimensions();
    let proporcao = altura_orig as f32 / largura_orig as f32;

    if app.estilo == 2 {
        let l_img = app.largura * 2;
        let mut a_img = (l_img as f32 * proporcao) as u32;
        a_img = (a_img / 4) * 4;

        let img_red = img.resize_exact(l_img, a_img, image::imageops::FilterType::Nearest);

        for y in (0..a_img).step_by(4) {
            let mut linha = String::new();
            for x in (0..l_img).step_by(2) {
                let mut bloco = [[false; 2]; 4];
                
                for by in 0..4 {
                    for bx in 0..2 {
                        if x + bx < l_img && y + by < a_img {
                            let pixel = img_red.get_pixel(x + bx, y + by);
                            let rgb = pixel.to_rgb();
                            
                            let r = rgb.0[0] as f32;
                            let g = rgb.0[1] as f32;
                            let b = rgb.0[2] as f32;
                            let brilho = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
                            
                            let mut pixel_ajustado = (brilho as f32 * app.contraste) as i32 + app.brilho;
                            if pixel_ajustado > 255 { pixel_ajustado = 255; }
                            if pixel_ajustado < 0 { pixel_ajustado = 0; }

                            if app.fundo_branco {
                                bloco[by as usize][bx as usize] = pixel_ajustado < app.corte_braille as i32;
                            } else {
                                bloco[by as usize][bx as usize] = pixel_ajustado > app.corte_braille as i32;
                            }
                        }
                    }
                }
                
                let char_b = app.pixel_para_braille(&bloco);
                let p_cor = img_red.get_pixel(x, y).to_rgb();
                if char_b == ' ' || char_b == '\u{2800}' {
                    linha.push_str("&nbsp;");
                } else {linha.push_str(&format!("{}", p_cor.0[0], p_cor.0[1], p_cor.0[2], char_b));
}
}
writeln!(file, "{}", linha)?;
}
} else {
let caracteres = if app.estilo == 1 { "█▓▒░ " } else { "@#W$9876543210?!abc;:+=-,._ " };
let num_chars = caracteres.chars().count();
let a_chars = (app.largura as f32 * proporcao * 0.5) as u32;
let img_red = img.resize_exact(app.largura, a_chars, image::imageops::FilterType::Nearest);
for y in 0..a_chars {
let mut linha = String::new();
for x in 0..app.largura {
let pixel = img_red.get_pixel(x, y);
let rgb = pixel.to_rgb();
let r = rgb.0[0] as f32;
let g = rgb.0[1] as f32;
let b = rgb.0[2] as f32;
let b_pixel = (0.299 * r + 0.587 * g + 0.114 * b) as u8;
let mut adj = (b_pixel as f32 * app.contraste) as i32 + app.brilho;
if adj > 255 { adj = 255; }
if adj < 0 { adj = 0; }
let idx = if app.fundo_branco {
((255 - adj) * (num_chars as i32 - 1) / 255) as usize
} else {
(adj * (num_chars as i32 - 1) / 255) as usize
};
let char_ascii = caracteres.chars().nth(idx).unwrap_or(' ');
if char_ascii == ' ' {
linha.push_str(" ");
} else {
linha.push_str(&format!("{}", rgb.0[0], rgb.0[1], rgb.0[2], char_ascii));
}
}
writeln!(file, "{}", linha)?;
}
}
writeln!(file, "")?;
Ok(())
}
