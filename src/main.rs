mod caster;
mod framebuffer;
mod maze;
mod player;
mod sfx;
mod game; // Asumiendo que tienes un archivo separado para Framebuffer

fn main() {
    let mut game = game::Game::new();

    while game.window.is_open() {
        game.render();

        // Verifica el estado del juego y sale si es necesario
        if let game::GameState::Exiting = game.state {
            break;
        }
    }
}
