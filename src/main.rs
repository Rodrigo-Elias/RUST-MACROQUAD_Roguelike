use macroquad::prelude::*;
use std::process::exit; // Importar a função exit

// --- CONST ---
// --- setup macroquad---
const SCREEN_WIDTH: f32 = 1024.0;
const SCREEN_HEIGHT: f32 = 768.0;
const BACKGROUND_COLOR: Color = DARKGRAY;
// --- splash ---
const SPLASH_DURATION: f32 = 4.0;
const FADE_DURATION: f32 = 0.8;  //Talvez em outras partes? ->FADE
const SPLASH_COUNT: usize = 5;
// --- game room ---
const TILE_SIZE: f32 = 128.0;
const MAP_WIDTH: usize = 16; 
const MAP_HEIGHT: usize = 5;
// Definição da sala
// x = muro
// o = chão
// s = spawn
const MAP_DATA: [&str; MAP_HEIGHT] = [
    "xxxxx     xxxxxx",
    "xoooxxxxxxxoooox",
    "xsooooooooooooox",
    "xoooxxxxxxxooosx",
    "xxxxx     xxxxxx",
];
// --- anim ---
const MOVE_DURATION: f32 = 0.15; // Duração da animação de movimento (em segundos)

// --- const menu ---
const MENU_OPTIONS_COUNT: i32 = 3;
// Posições X, Y para o arrow.png
const MENU_POSITIONS: [(f32, f32); MENU_OPTIONS_COUNT as usize] = [
    (640.0, 495.0), // New
    (640.0, 575.0), // Load
    (640.0, 655.0), // Exit
];

// --- States ---
#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Splash, //splash
    MainScreen, //Mainscreen
    InGame, // No dungeon
    Load,   // Tela de load 
}

// --- Player ---
struct Player {
    // Posição x/y
    x: f32, 
    y: f32, 

    start_x: f32, // Posição X de onde o movimento começou
    start_y: f32, // Posição Y de onde o movimento começou
    target_x: f32, // Posição X para onde o jogador está indo
    target_y: f32, // Posição Y para onde o jogador está indo

    move_timer: f32, // Contador de tempo para a animação de movimento (restante)
}

// --- ASSETS ---
struct GameAssets {
    splash_textures: [Texture2D; SPLASH_COUNT],
    main_screen_texture: Texture2D,
    tileset_texture: Texture2D,     // tileset
    selector_texture: Texture2D,    // arrow.png
    load_screen_texture: Texture2D, // BG loadscreen
    player_texture: Texture2D,      // Player.png
}

impl GameAssets {
    async fn load() -> Result<Self, macroquad::Error> {

        // 1. Pra carregar splashs
        let mut splash_textures: Vec<Texture2D> = Vec::new();
        for i in 1..=SPLASH_COUNT {
            let filename = format!("BG/splash{:02}.png", i);
            let texture = load_texture(&filename).await?;
            texture.set_filter(FilterMode::Linear); 
            splash_textures.push(texture);
        }

        // 2. Carrega mainscreen
        let main_screen_texture = load_texture("BG/main.png").await?;
        main_screen_texture.set_filter(FilterMode::Linear);

        // 3. Carrega tileset
        let tileset_texture = load_texture("dng/map1.png").await?;
        tileset_texture.set_filter(FilterMode::Nearest);

        // 4. Carrega o seletor do menu
        let selector_texture = load_texture("arrow.png").await?;
        selector_texture.set_filter(FilterMode::Nearest);
        
        // 5. Carrega o background de load
        let load_screen_texture = load_texture("BG/load.png").await?;
        load_screen_texture.set_filter(FilterMode::Linear);

        // 6. Carrega o sprite do jogador
        let player_texture = load_texture("sprites/Player.png").await?;
        player_texture.set_filter(FilterMode::Nearest); // Nearest para pixel art


        // Converte o Vec para Array
        let splash_textures_array: [Texture2D; SPLASH_COUNT] = splash_textures
            .try_into()
            .unwrap_or_else(|_| panic!("Erro interno ao converter Vec<Texture2D> para Array."));

        Ok(GameAssets {
            splash_textures: splash_textures_array,
            main_screen_texture,
            tileset_texture, 
            selector_texture,   
            load_screen_texture,
            player_texture, // Adicionado aqui
        })
    }
}

// --- FUNÇÕES AUXILIARES --
// Nova função para desenhar o mapa
fn draw_map(tileset: &Texture2D) {
    for (row_idx, row_str) in MAP_DATA.iter().enumerate() {
        for (col_idx, tile_char) in row_str.chars().enumerate() {
            // Define de qual parte do tileset vamos cortar o tile
            let source_rect = match tile_char {
                // 'x' (muro): (0, 0)
                'x' => Some(Rect::new(0.0, 0.0, TILE_SIZE, TILE_SIZE)),
                // 'o' (chão): (128, 0) do spritesheet
                'o' => Some(Rect::new(TILE_SIZE, 0.0, TILE_SIZE, TILE_SIZE)),
                // 's' (chão): idêntico ao tile de chão básico, serve como marcador de spawn.
                's' => Some(Rect::new(TILE_SIZE, 0.0, TILE_SIZE, TILE_SIZE)),
                // Qualquer outro caractere: não desenha nada
                _ => None,
            };

            if let Some(rect) = source_rect {
                draw_texture_ex(
                    tileset,
                    col_idx as f32 * TILE_SIZE, // Posição X na tela
                    row_idx as f32 * TILE_SIZE, // Posição Y na tela
                    WHITE,
                    DrawTextureParams {
                        source: Some(rect), // A mágica acontece aqui!
                        ..Default::default()
                    },
                );
            }
        }
    }
}

// Obtém o caractere do tile em uma posição de pixel (x, y) do mundo.
// Retorna ' ' se estiver fora dos limites.
fn get_tile_char(x: f32, y: f32) -> char {
    // Converte a posição em pixels para coordenadas de tile (coluna/linha)
    // Usamos floor para mapear o pixel (centro do jogador) para o tile correto
    // Ajustamos para TILE_SIZE para garantir que pegamos o tile de destino
    let col = (x / TILE_SIZE).floor() as isize;
    let row = (y / TILE_SIZE).floor() as isize;

    if row >= 0 && row < MAP_HEIGHT as isize && col >= 0 && col < MAP_WIDTH as isize {
        // Pega o caractere do mapa. A indexação é segura por causa do if
        MAP_DATA[row as usize].chars().nth(col as usize).unwrap_or(' ')
    } else {
        ' ' // Fora do limite
    }
}

// --- CONFIGURAÇÃO DE JANELA ---

fn window_conf() -> Conf {
    Conf {
        window_title: "Macroquad Game States".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

// --- LOOP PRINCIPAL DO JOGO ---

#[macroquad::main(window_conf)]
async fn main() {
    set_pc_assets_folder("assets");

    let assets = match GameAssets::load().await {
        Ok(a) => a,
        Err(e) => {
            panic!("Falha ao carregar assets. Verifique se os arquivos (splash01-05.png, main.png, dng/map1.png, arrow.png, BG/load.png, sprites/Player.png) estão na pasta 'assets'. Erro: {:?}", e);
        }
    };

    let mut game_state = GameState::Splash;
    let mut current_splash_index: usize = 0;
    let mut state_timer: f32 = 0.0;
    let mut menu_selection: i32 = 0; // 0: NewGame, 1: Load, 2: Exit

    // --- Ingame Variables ---
    let mut player: Option<Player> = None; // Jogador no momento opcional, pois só existe InGame

    // Tiles marcados como 's' servem de spawnpoint, apenas pra gerar algo meio aleatorio.
    let mut spawn_points: Vec<(f32, f32)> = Vec::new();
    for (row_idx, row_str) in MAP_DATA.iter().enumerate() {
        for (col_idx, tile_char) in row_str.chars().enumerate() {
            if tile_char == 's' {
                // Adiciona o *centro* do tile de spawn à lista... Uff...
                spawn_points.push((
                    (col_idx as f32 * TILE_SIZE) + (TILE_SIZE / 2.0), // Centro X
                    (row_idx as f32 * TILE_SIZE) + (TILE_SIZE / 2.0), // Centro Y
                ));
            }
        }
    }

    if spawn_points.is_empty() {
        // Se não houver 's' no mapa, o jogo não pode começar.
        panic!("O mapa (MAP_DATA) não contém nenhum ponto de spawn 's'.");
    }

    
    loop {
        let delta_time = get_frame_time(); 

        let skip_input = is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Z)
            || is_key_pressed(KeyCode::X)
            || is_key_pressed(KeyCode::C)
            || is_key_pressed(KeyCode::D);
            
        // Input de confirmação específico do menu
        let confirm_input = is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Z);

        // --- ATUALIZAÇÃO DE ESTADO ---

        match game_state {
            GameState::Splash => {
                state_timer += delta_time;

                if skip_input || state_timer >= SPLASH_DURATION {
                    if current_splash_index < SPLASH_COUNT - 1 && !skip_input {
                        current_splash_index += 1;
                        state_timer = 0.0; 
                    } else {
                        game_state = GameState::MainScreen;
                        println!("Splash finalizado. Entrando em MainScreen.");
                    }
                }
            },
            GameState::MainScreen => { //-----------------------------------------------------------MAINSCREEN
                // Sobe e desce basico do menu.
                if is_key_pressed(KeyCode::Down) {
                    menu_selection = (menu_selection + 1) % MENU_OPTIONS_COUNT;
                }
                if is_key_pressed(KeyCode::Up) {
                    menu_selection = (menu_selection - 1 + MENU_OPTIONS_COUNT) % MENU_OPTIONS_COUNT;
                }

                // Lógica de confirmação do menu. Eventualmente separa melhor essa parte.
                if confirm_input {
                    match menu_selection {
                        0 => { // Novo Jogo
                            game_state = GameState::InGame; // ---------------------------- Indo InGame
                            println!("Entrando no estado InGame.");
                            
                            // --- LÓGICA DE SPAWN DO JOGADOR ---
                            // Escolhe um ponto de spawn aleatório da lista
                            let spawn_index = rand::gen_range(0, spawn_points.len());
                            let (spawn_x, spawn_y) = spawn_points[spawn_index];
                            
                            // Cria o jogador nesse ponto, inicializando o movimento para a posição atual
                            player = Some(Player { 
                                x: spawn_x, 
                                y: spawn_y, 
                                start_x: spawn_x,
                                start_y: spawn_y,
                                target_x: spawn_x, 
                                target_y: spawn_y, 
                                move_timer: 0.0,   
                            });
                            println!("Jogador criado em: ({}, {})", spawn_x, spawn_y);
                        },
                        1 => { // ------------------------------------------------------------ Indo LOAD
                            game_state = GameState::Load;
                            println!("Entrando no estado Load.");
                        },
                        2 => { // Sair
                            println!("Fechando o jogo.");
                            exit(0);
                        },
                        _ => {} // Nunca deve acontecer
                    }
                }
            },
            GameState::Load => { //--------------------------------------------------------------LOAD
                // Lógica da tela de Load
                if is_key_pressed(KeyCode::X) {
                    game_state = GameState::MainScreen;
                    println!("Voltando para MainScreen.");
                }
            },
            GameState::InGame => { //------------------------------------------------------------INGAME
                let p = player.as_mut().expect("Player deve existir em InGame");
                
                // 1. Lógica de movimento/animação
                if p.move_timer > 0.0 {
                    // Está se movendo, diminui o timer
                    p.move_timer -= delta_time;
                    p.move_timer = p.move_timer.max(0.0);
                    
                    if p.move_timer <= 0.0 {
                        // Movimento finalizado. Trava a posição no alvo.
                        p.x = p.target_x;
                        p.y = p.target_y;
                    } else {
                        // Interpolação Linear (LERP)
                        let t = 1.0 - (p.move_timer / MOVE_DURATION); // Progresso de 0.0 a 1.0
                        
                        // Interpola a posição (x, y) entre start e target
                        p.x = p.start_x + (p.target_x - p.start_x) * t;
                        p.y = p.start_y + (p.target_y - p.start_y) * t;
                    }
                } else {
                    // 2. Não está se movendo. Checa por novo input.
                    
                    let mut target_col_offset: f32 = 0.0;
                    let mut target_row_offset: f32 = 0.0;
                    let mut moved = false;
                    
                    if is_key_pressed(KeyCode::Right) {
                        target_col_offset = 1.0;
                        moved = true;
                    } else if is_key_pressed(KeyCode::Left) {
                        target_col_offset = -1.0;
                        moved = true;
                    } else if is_key_pressed(KeyCode::Down) {
                        target_row_offset = 1.0;
                        moved = true;
                    } else if is_key_pressed(KeyCode::Up) {
                        target_row_offset = -1.0;
                        moved = true;
                    }
                    
                    if moved {
                        // Calcula a posição do centro do tile alvo (p.x/y atual é a posição do tile)
                        let new_x = p.x + target_col_offset * TILE_SIZE;
                        let new_y = p.y + target_row_offset * TILE_SIZE;
                        
                        // Checa colisão com o novo tile (apenas o centro do tile importa)
                        let tile_char = get_tile_char(new_x, new_y);
                        
                        // 'x' é muro. Espaço (' ') é fora do mapa. 'o' e 's' são chão.
                        if tile_char != 'x' && tile_char != ' ' {
                            // Movimento válido, inicia a animação
                            p.start_x = p.x; // Posição de partida é a atual (centro do tile)
                            p.start_y = p.y; // Posição de partida é a atual (centro do tile)
                            p.target_x = new_x; // Novo alvo (centro do próximo tile)
                            p.target_y = new_y; // Novo alvo (centro do próximo tile)
                            p.move_timer = MOVE_DURATION;
                        }
                    }
                }

                // Lógica para sair do jogo
                 if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::MainScreen;
                    player = None; // Remove o jogador ao sair para o menu
                    println!("Voltando para MainScreen.");
                }
            }
        }

        // --- DESENHO (DRAW) ---
        
        clear_background(BACKGROUND_COLOR);
        
        // Variável para armazenar as coordenadas do jogador para debug na UI
        let mut player_coords = String::new(); 

        match game_state {
            GameState::Splash => {
                let texture = &assets.splash_textures[current_splash_index];
                
                let alpha = {
                    if state_timer < FADE_DURATION {
                        state_timer / FADE_DURATION
                    } else if state_timer > SPLASH_DURATION - FADE_DURATION {
                        1.0 - (state_timer - (SPLASH_DURATION - FADE_DURATION)) / FADE_DURATION
                    } else {
                        1.0
                    }
                }.clamp(0.0, 1.0);
                
                draw_texture_ex(
                    texture,
                    0.0,
                    0.0,
                    Color::new(1.0, 1.0, 1.0, alpha), 
                    DrawTextureParams {
                        dest_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT)),
                        ..Default::default()
                    }
                );
                
                draw_text(&format!("Splash {}/{} | Tempo: {:.2}s", current_splash_index + 1, SPLASH_COUNT, state_timer), 10.0, 750.0, 20.0, WHITE);

            },
            GameState::MainScreen => {
                draw_texture_ex(
                    &assets.main_screen_texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT)),
                        ..Default::default()
                    }
                );
                
                // --- Desenha o Seletor ---
                // Pega a posição X,Y do array com base na seleção atual
                let (selector_x, selector_y) = MENU_POSITIONS[menu_selection as usize];
                
                draw_texture(&assets.selector_texture, selector_x, selector_y, WHITE);
            },
            GameState::Load => {
                // Desenha a tela de Load
                draw_texture_ex(
                    &assets.load_screen_texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT)),
                        ..Default::default()
                    }
                );
                
                draw_text("TELA DE LOAD", 50.0, 50.0, 30.0, YELLOW);
                draw_text("Pressione X para voltar", 50.0, 90.0, 24.0, WHITE);
            },
            GameState::InGame => {
                // --- CÂMERA SEGUINDO O JOGADOR ---
                if let Some(p) = &player {
                    // Preenche a variável de coordenadas para debug na UI
                    player_coords = format!("Posição: X={:.1}, Y={:.1}", p.x, p.y);
                    
                    // Define o ponto central da câmera (onde o jogador está)
                    let center_x = p.x;
                    let center_y = p.y;

                    // O retângulo da câmera define a porção do "mundo" visível.
                    let camera = Camera2D::from_display_rect(Rect::new(
                        center_x - SCREEN_WIDTH / 2.0,  // Canto esquerdo do mundo visível
                        center_y + SCREEN_HEIGHT / 2.0, // Canto superior do mundo visível (invertido)
                        SCREEN_WIDTH,                   // Largura do mundo visível
                        -SCREEN_HEIGHT,                 // Altura do mundo visível (negativa para inverter Y) <-estranho mas ok, bom saber q nao colocar negativo poe de cabeça pra baixo.
                    ));
                    set_camera(&camera);
                } else {
                    // Fallback
                    let map_pixel_width = MAP_WIDTH as f32 * TILE_SIZE;
                    let map_pixel_height = MAP_HEIGHT as f32 * TILE_SIZE;
                    let camera = Camera2D::from_display_rect(Rect::new(
                        -(SCREEN_WIDTH - map_pixel_width) / 2.0, 
                        -(SCREEN_HEIGHT - map_pixel_height) / 2.0, 
                        SCREEN_WIDTH, 
                        SCREEN_HEIGHT
                    ));
                    set_camera(&camera);
                }

                // --- TESTE DE DEBUG: QUADRADO AZUL NO (0,0) ---
                draw_rectangle(0.0, 0.0, TILE_SIZE, TILE_SIZE, BLUE);


                // Chama a nossa nova função de desenho (agora sob a câmera)
                draw_map(&assets.tileset_texture);

                // --- DESENHA O JOGADOR ---
                if let Some(p) = &player {
                    
                    // Desenha o sprite do jogador
                    let texture_width = assets.player_texture.width();
                    let texture_height = assets.player_texture.height();
                    
                    let draw_x = p.x - (texture_width / 2.0);
                    let draw_y = p.y - (texture_height / 2.0);

                    draw_texture(&assets.player_texture, draw_x, draw_y, WHITE);
                }

                
                // Reseta a câmera para desenhar o texto de UI
                set_default_camera();
                draw_text("MODO DE JOGO", 10.0, 30.0, 24.0, YELLOW);
                draw_text("Pressione ESC para voltar ao menu.", 10.0, 60.0, 24.0, WHITE);
                // Desenha as coordenadas do jogador na UI (fora da câmera do jogo)
                draw_text(&player_coords, 10.0, 90.0, 24.0, LIME);
            }
        }

        next_frame().await
    }
}
