use eframe::egui;
use image::{DynamicImage, GenericImageView, Pixel};
use std::path::Path;
use crate::export::exportar_arte_html;

pub struct EstudioApp {
    pub imagem_original: Option<DynamicImage>,
    pub caminho_origem: String,
    pub largura: u32,
    pub contraste: f32,
    pub brilho: i32,
    pub estilo: usize,
    pub fundo_branco: bool,
    pub corte_braille: u8,
    textura_preview: Option<egui::TextureHandle>,
}

impl EstudioApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, caminho: String) -> Self {
        let img = if Path::new(&caminho).exists() {
            image::open(&caminho).ok()
        } else {
            None
        };

        if img.is_none() {
            println!("Aviso: Nenhuma imagem encontrada em '{}'.", caminho);
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

    pub fn pixel_para_braille(&self, bloco: &[[bool; 2]; 4]) -> char {
        let mut codigo = 0x2800;
        let valores_pontos = [[0x01, 0x08], [0x02, 0x10], [0x04, 0x20], [0x40, 0x80]];
        for y in 0..4 {
            for x in 0..2 {
                if bloco[y][x] { codigo |= valores_pontos[y][x]; }
            }
        }
        std::char::from_u32(codigo).unwrap_or(' ')
    }

        fn gerar_preview_pixels(&self) -> Option<egui::ColorImage> {
        let img = self.imagem_original.as_ref()?;
        let (largura_orig, altura_orig) = img.dimensions();
        let proporcao = altura_orig as f32 / largura_orig as f32;
        
        let l_preview = self.largura;
        // Ajusta a proporção vertical para simular a altura dos caracteres ASCII/Blocks
        let a_preview = if self.estilo == 2 {
            (l_preview as f32 * proporcao * 0.5) as u32
        } else {
            (l_preview as f32 * proporcao * 0.5) as u32
        };
        
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

                // --- NOVA LÓGICA DE SIMULAÇÃO DE ESTILO AO VIVO ---
                if self.estilo == 2 {
                    // Simulação para o modo Braille
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
                } else if self.estilo == 1 {
                    // Simulação para o modo Blocks (Simula o aspeto de Pixel Art geométrico)
                    if adj > 50 {
                        pixels_saida[(y * l_preview + x) as usize] = egui::Color32::from_rgb(rgb.0[0], rgb.0[1], rgb.0[2]);
                    } else {
                        pixels_saida[(y * l_preview + x) as usize] = if self.fundo_branco { egui::Color32::WHITE } else { egui::Color32::BLACK };
                    }
                } else {
                    // Simulação para o modo ASCII Clássico (Puxa os contornos por densidade)
                    if adj > 30 && adj < 230 {
                        pixels_saida[(y * l_preview + x) as usize] = egui::Color32::from_rgb(rgb.0[0], rgb.0[1], rgb.0[2]);
                    } else {
                        pixels_saida[(y * l_preview + x) as usize] = if self.fundo_branco { egui::Color32::WHITE } else { egui::Color32::BLACK };
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

            ui.horizontal(|ui| {
                ui.label("Estilo Visual:");
                if ui.radio_value(&mut self.estilo, 0, "ASCII").changed() ||
                   ui.radio_value(&mut self.estilo, 1, "Blocks").changed() ||
                   ui.radio_value(&mut self.estilo, 2, "Braille").changed() {
                       self.textura_preview = None;
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

            if mudou || self.textura_preview.is_none() {
                if let Some(cor_imagem) = self.gerar_preview_pixels() {
                    self.textura_preview = Some(ctx.load_texture(
                        "preview_live",
                        cor_imagem,
                        egui::TextureOptions::NEAREST,
                    ));
                }
            }

            if let Some(ref textura) = self.textura_preview {
                let largura_janela = 500.0;
                let proporcao_textura = textura.size()[1] as f32 / textura.size()[0] as f32;
                ui.image(egui::load::SizedTexture::new(
                    textura.id(), 
                    [largura_janela, largura_janela * proporcao_textura]
                ));
            }
        });
    }
}
