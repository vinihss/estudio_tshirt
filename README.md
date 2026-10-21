# Estúdio de T-Shirts — Rust Edition

Aplicativo desktop em Rust que converte imagens em arte para serigrafia de camisetas, usando três estilos de representação baseados em caracteres: **ASCII**, **Blocks** e **Braille**.

O objetivo é gerar uma arte de alto contraste, otimizada para impressão em camisetas (brancas ou escuras), com exportação em HTML pronto para impressão.

## Funcionalidades

- **Três estilos visuais:**
  - **ASCII** — contornos por densidade de luminância.
  - **Blocks** — pixel art geométrico.
  - **Braille** — mapeamento de pixels para caracteres Braille ( grade 2×4 ).
- Ajustes em tempo real:
  - Largura da arte (40–250 colunas)
  - Contraste (0.5–3.0)
  - Brilho (-60 a 60)
  - Corte Braille (0–255)
  - Otimização para camiseta branca (fundo branco/preto)
- **Pré-visualização em tempo real** dentro da interface.
- **Exportação em HTML** com estilo de impressão (fonte monoespaçada, `@media print`).

## Requisitos

- [Rust](https://rustup.rs/) (toolchain estável)
- Sistema com suporte a GUI (as bibliotecas de janela do `eframe`/`egui`)

## Como executar

```bash
cargo run -- <caminho/para/imagem>
```

Se nenhum caminho for informado, o app tenta carregar `imagem_processada.png` no diretório atual.

```bash
# Exemplo
cargo run -- foto.png
```

## Exportar

Com a janela aberta, clique em **💾 EXPORTAR HTML PARA IMPRESSÃO**. Um arquivo `tshirt_rust_estilo_<n>.html` será gerado no diretório de trabalho, onde `<n>` corresponde ao estilo selecionado (0 = ASCII, 1 = Blocks, 2 = Braille).

## Estrutura do projeto

```
src/
├── main.rs    # Ponto de entrada da aplicação (eframe)
├── app.rs     # Estado da UI, controles e pré-visualização em tempo real
└── export.rs  # Geração do HTML de exportação
```

## Dependências

| Crate    | Versão | Uso                                    |
| -------- | ------ | -------------------------------------- |
| `eframe` | 0.27.2 | Framework de GUI nativa                |
| `image`  | 0.25.1 | Leitura, redimensionamento e píxeis    |

## Licença

Sem licença definida. Uso pessoal.
