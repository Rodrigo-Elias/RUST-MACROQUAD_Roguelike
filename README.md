Projeto Rust/Macroquad: Dungeon Crawler (Estados e Movimento)Este é um protótipo inicial de um jogo de RPG estilo Dungeon Crawler, construído em Rust utilizando a biblioteca de jogos Macroquad. O projeto foca na implementação de um sistema robusto de estados de jogo, gerenciamento de assets e mecânica de movimento baseada em tiles com colisão.🌟 Funcionalidades ImplementadasO projeto inclui as seguintes funcionalidades principais:Sistema de Estados de Jogo (GameState):Splash: Exibição de múltiplas telas de abertura com efeito de fade in/out.MainScreen: Menu principal navegável.Load: Tela de carregamento (placeholder).InGame: Onde a ação acontece.Menu Principal Funcional:Navegação entre opções ("Novo Jogo", "Carregar", "Sair") usando setas (↑/↓).Confirmação de seleção usando Enter ou Z.A opção "Sair" encerra o programa.Movimento Clássico Baseado em Tiles:O jogador se move exatamente um tile por vez ao pressionar as setas do teclado.Animação de movimento suave com duração de 0.15 segundos (MOVE_DURATION).A câmera acompanha o jogador em tempo real.Verificação de Colisão com o Mapa:O jogador não pode se mover para tiles marcados como muro ('x') ou para fora dos limites do mapa.Configuração de Câmera (Macroquad):Implementação de uma Camera2D para seguir o jogador, com a correção necessária para inverter o eixo Y, garantindo que o mundo seja renderizado na orientação matemática correta (Y cresce para cima).🗺️ Estrutura do MapaO mapa é definido em tempo de compilação através do array de strings MAP_DATA:const MAP_DATA: [&str; MAP_HEIGHT] = [
    "xxxxx     xxxxxx",
    "xoooxxxxxxxoooox",
    "xsooooooooooooox",
    "xoooxxxxxxxooosx",
    "xxxxx     xxxxxx",
];
CaractereSignificadoTipo de Tile'x'MuroColisível'o'ChãoAndável's'SpawnAndável/Início do Jogador' 'Vazio (Espaço)Fora do Limite do Mapa (Colisível)⚙️ Como Rodar o ProjetoPré-requisitosRust: Você precisa ter o ambiente de desenvolvimento Rust instalado (incluindo cargo).Instale ou atualize: rustup updateAssets: O projeto depende de arquivos de imagem que devem ser colocados na pasta assets no diretório raiz do projeto.Estrutura de Assets NecessáriaCrie a seguinte estrutura de pastas e coloque os arquivos (se tiver) nos locais indicados:.
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
Nota: Se você não possui os assets, o jogo irá falhar ao carregar. Certifique-se de ter todos os arquivos referenciados em GameAssets::load().Compilação e ExecuçãoNo terminal, dentro do diretório do projeto:cargo run
