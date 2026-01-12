# 🦀 Rust Terminal Music Player

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Ratatui](https://img.shields.io/badge/Ratatui-%23E30B5C.svg?style=for-the-badge&logo=rust&logoColor=white)


> Um reprodutor de música leve e interativo via terminal (TUI), desenvolvido em Rust.

## 📖 Sobre o Projeto

Este projeto consiste em um music player de terminal focado em simplicidade e eficiência. Basta informar o caminho do diretório onde suas músicas estão localizadas para começar a ouvir.

A interface foi construída utilizando o framework **Ratatui**, oferecendo:
* **Playlist Dinâmica:** Carrega arquivos automaticamente da pasta informada.
* **Timeline em Tempo Real:** Visualização do progresso da música atual.
* **Controles de Mídia:** Pausa, play, próxima faixa, faixa anterior e controle de volume.
* **Metadados:** Exibição de Artista, Álbum e Título (via biblioteca Lofty).

# Rust Terminal Music Player demo

![Layout do Player](assets/demo.png)

> Um reprodutor de música leve e interativo...

## ⚙️ Pré-requisitos

Para compilar e rodar este projeto no Linux, é necessário ter o compilador Rust instalado. Além disso, devido à dependência da crate `rodio` (utilizada para o áudio), você precisa das bibliotecas de desenvolvimento do **ALSA**.

### Instalando dependências do ALSA:

**Ubuntu/Debian:**
```bash
sudo apt install libasound2-dev
```
Fedora:
```Bash
sudo dnf install alsa-lib-devel
```

Arch Linux:
```Bash
sudo pacman -S alsa-lib
```

🚀 Como Rodar

1. Clone o repositório:
```Bash
git clone [https://github.com/icaro-s16/SEU_REPOSITORIO.git](https://github.com/icaro-s16/SEU_REPOSITORIO.git)
```

2. Entre na pasta:
```Bash
cd SEU_REPOSITORIO
```

3. Execute o projeto:
```Bash
    cargo run --release
```

| Tecla | Ação |
| :--- | :--- |
| `TAB` | Alternar foco (Input / Playlist) |
| `Space` | Play / Pause |
| `Enter` | Carregar Playlist (após digitar o caminho) |
| `n` | Próxima Faixa (Next) |
| `p` | Faixa Anterior (Previous) |
| `↑` / `↓` | Ajustar Volume (ou navegar na lista) |
| `q` | Sair do Player |
