pub trait ChessPlayer {
    fn play(&self, _: &chess::Game) -> chess::ChessMove;
}

#[derive(Debug)]
pub struct ChessGame<P1: ChessPlayer, P2: ChessPlayer> {
    players: (P1, P2),
    game: chess::Game
}

impl<P1: ChessPlayer, P2: ChessPlayer> ChessGame<P1, P2> {
    pub fn set_p1(mut self, player: P1) {
        self.players.0 = player;
    }
    pub fn set_p2(mut self, player: P2) {
        self.players.1 = player;
    }
    pub fn new(p1: P1, p2: P2) -> Self {
        Self {
            players: (p1, p2),
            game: chess::Game::new()
        }
    }
    pub fn board(self) -> chess::Board { return self.game.current_position() }
    pub fn step(&mut self) -> Option<chess::GameResult> {
        match self.game.result() {
            Some(r) => {
                // println!("Game Over!");
                Some(r)
            },
            Option::None => {
                let selected_move = match self.game.side_to_move() {
                    chess::Color::White => self.players.0.play(&self.game),
                    chess::Color::Black => self.players.1.play(&self.game),
                };
                self.game.make_move(selected_move);
                None
            }
        }
    }
    pub fn run(mut self) -> (Vec<chess::Board>, chess::GameResult) {
        let mut history = vec!(self.game.current_position());
        loop {
            let r = self.step();
            history.push(self.game.current_position());
            if self.game.can_declare_draw() {
                self.game.declare_draw();
            }
            match r {
                Option::None => (),
                Some(r) => {
                    return (history, r)
                }
            }
        }
    }
}

impl<P1: Default + ChessPlayer, P2: Default + ChessPlayer> Default for ChessGame<P1, P2> {
    fn default() -> ChessGame<P1, P2> {
        Self {
            players: (Default::default(), Default::default()),
            game: chess::Game::new()
        }
    }
}