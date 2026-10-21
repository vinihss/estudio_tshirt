use std::fs::File;
use std::io::Write;
use image::{GenericImageView, Pixel};
use crate::app::EstudioApp;

pub fn exportar_arte_html(app: &EstudioApp, caminho_saida: &str) -> std::io::Result<()> {
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
                } else {
                    let tag = format!(
                        "<span style='color:#{:02x}{:02x}{:02x};'>{}</span>", 
                        p_cor.0[0], p_cor.0[1], p_cor.0[2], char_b
                    );
                    linha.push_str(&tag);
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
                    linha.push_str("&nbsp;");
                } else {
                    let tag = format!(
                        "<span style='color:#{:02x}{:02x}{:02x};'>{}</span>", 
                        rgb.0[0], rgb.0[1], rgb.0[2], char_ascii
                    );
                    linha.push_str(&tag);
                }
            }
            writeln!(file, "{}", linha)?;
        }
    }

    writeln!(file, "</pre></body></html>")?;
    Ok(())
}
