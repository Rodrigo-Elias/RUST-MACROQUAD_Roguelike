Projeto Rust/Macroquad: Dungeon Crawler (Estados e Movimento)

Este é um protótipo inicial de um jogo de RPG estilo Dungeon Crawler, construído em Rust utilizando a biblioteca de jogos Macroquad. O projeto foca na implementação de um sistema robusto de estados de jogo, gerenciamento de assets e mecânica de movimento baseada em tiles com colisão.

🌟 Funcionalidades Implementadas
🎮 Sistema de Estados de Jogo (GameState)

Splash: múltiplas telas de abertura com efeito fade in/out

MainScreen: menu principal navegável

Load: tela de carregamento (placeholder)

InGame: onde a ação acontece

🧭 Menu Principal Funcional

Navegação entre opções (“Novo Jogo”, “Carregar”, “Sair”) com as setas ↑/↓

Seleção confirmada com Enter ou Z

A opção “Sair” encerra o programa

🧱 Movimento Clássico Baseado em Tiles

O jogador se move um tile por vez ao pressionar as setas do teclado

Animação de movimento suave (duração: 0.15s, constante MOVE_DURATION)

Câmera segue o jogador em tempo real

🚧 Colisão com o Mapa

O jogador não pode atravessar paredes ('x')

Movimento bloqueado ao tentar sair dos limites do mapa

🎥 Câmera Configurada (Macroquad)

Implementação de uma Camera2D que acompanha o jogador

Correção aplicada para inverter o eixo Y, garantindo que o mundo seja renderizado de forma matemática correta (Y cresce para cima)

🗺️ Estrutura do Mapa

O mapa é definido em tempo de compilação via MAP_DATA:

const MAP_DATA: [&str; MAP_HEIGHT] = [
    "xxxxx     xxxxxx",
    "xoooxxxxxxxoooox",
    "xsooooooooooooox",
    "xoooxxxxxxxooosx",
    "xxxxx     xxxxxx",
];

Caractere	Significado	Tipo de Tile
x	Muro	Colisível
o	Chão	Andável
s	Spawn	Andável / Início do Jogador
(espaço)	Fora do limite do mapa	Colisível
⚙️ Como Rodar o Projeto
🧰 Pré-requisitos

Rust instalado (com cargo)

Instale ou atualize:

rustup update

🖼️ Estrutura de Assets Necessária

Crie a pasta assets/ na raiz do projeto e mantenha a seguinte estrutura:

.
├── Cargo.toml
└── assets/
    ├── arrow.png
    ├── BG/
    │   ├── load.png
    │   ├── main.png
    │   ├── splash01.png
    │   ├── splash02.png
    │   ├── splash03.png
    │   ├── splash04.png
    │   └── splash05.png
    ├── dng/
    │   └── map1.png
    └── sprites/
        └── Player.png


⚠️ Atenção: o jogo depende desses arquivos.
Caso algum esteja ausente, o carregamento falhará durante a inicialização (GameAssets::load()).

🚀 Compilação e Execução

No terminal, dentro do diretório do projeto:

cargo run

🧠 Observações

Este projeto serve como base para estudo de:

Organização de estados em jogos 2D

Gerenciamento de recursos (assets) em Rust

Movimentação baseada em tiles e colisão

Uso da Macroquad para renderização, entrada e lógica de jogo
