use crate::othello::{Board, Color, PointVec};
use rand::rng;
use rand::seq::IndexedRandom;

#[derive(Clone)]
struct MCTSNode {
    state: Board,
    player: Color,
    parent: Option<*mut MCTSNode>,
    children: Vec<Box<MCTSNode>>,
    action: Option<PointVec>,
    visits: u32,
    wins: f64,
    untried_actions: Vec<PointVec>,
}

impl MCTSNode {
    fn new(state: Board, player: Color, parent: Option<*mut MCTSNode>, action: Option<PointVec>) -> Self {
        let untried_actions = state.legal_moves(player);
        Self {
            state,
            player,
            parent,
            children: Vec::new(),
            action,
            visits: 0,
            wins: 0.0,
            untried_actions,
        }
    }

    fn is_terminal(&self) -> bool {
        self.state.game_over()
    }

    fn is_fully_expanded(&self) -> bool {
        self.untried_actions.is_empty()
    }

    fn expand(&mut self) -> Option<&mut MCTSNode> {
        if let Some(action) = self.untried_actions.pop() {
            let mut new_state = self.state.clone();
            let _ = new_state.play(self.player, action); // safe because action is legal
            let next_player = match self.player {
                Color::WHITE => Color::BLACK,
                Color::BLACK => Color::WHITE,
            };
            let child = Box::new(MCTSNode::new(new_state, next_player, Some(self as *mut _), Some(action)));
            self.children.push(child);
            return self.children.last_mut().map(|c| c.as_mut());
        }
        None
    }

    fn best_child(&self, c: f64) -> Option<&MCTSNode> {
        if self.children.is_empty() {
            return None;
        }
        self.children
            .iter()
            .max_by(|a, b| {
                let ucb1_a = (a.wins / a.visits as f64)
                    + c * ((self.visits as f64).ln() / a.visits as f64).sqrt();
                let ucb1_b = (b.wins / b.visits as f64)
                    + c * ((self.visits as f64).ln() / b.visits as f64).sqrt();
                ucb1_a.partial_cmp(&ucb1_b).unwrap()
            })
            .map(|boxed| boxed.as_ref())
    }

    fn rollout(&self) -> f64 {
        let mut rng = rng();
        let mut state = self.state.clone();
        let mut player = self.player;

        while !state.game_over() {
            let moves = state.legal_moves(player);
            if moves.is_empty() {
                // skip turn
                player = match player {
                    Color::WHITE => Color::BLACK,
                    Color::BLACK => Color::WHITE,
                };
                continue;
            }
            let action = *moves.choose(&mut rng).unwrap();
            let _ = state.play(player, action);
            player = match player {
                Color::WHITE => Color::BLACK,
                Color::BLACK => Color::WHITE,
            };
        }

        let (white, black) = state.score();
        match self.player {
            Color::WHITE => {
                if white > black {
                    1.0
                } else if white == black {
                    0.5
                } else {
                    0.0
                }
            }
            Color::BLACK => {
                if black > white {
                    1.0
                } else if black == white {
                    0.5
                } else {
                    0.0
                }
            }
        }
    }

    fn backpropagate(&mut self, result: f64) {
        self.visits += 1;
        self.wins += result;
        if let Some(parent_ptr) = self.parent {
            unsafe {
                (*parent_ptr).backpropagate(1.0 - result);
            }
        }
    }
}

/// Run MCTS search for a given board state and player
pub(crate) fn mcts_search(root_state: Board, player: Color, iterations: u32) -> Option<PointVec> {
    let mut root = MCTSNode::new(root_state.clone(), player, None, None);

    if root.untried_actions.is_empty() {
        return None;
    }

    for _ in 0..iterations {
        let mut node: *mut MCTSNode = &mut root;
        unsafe {
            while !(*node).is_terminal() && (*node).is_fully_expanded() {
                if let Some(best) = (*node).best_child(std::f64::consts::SQRT_2) {
                    node = best as *const _ as *mut _;
                } else {
                    break;
                }
            }

            if !(*node).is_terminal() {
                if let Some(child) = (*node).expand() {
                    node = child;
                }
            }

            let result = (*node).rollout();

            (*node).backpropagate(result);
        }
    }

    root.best_child(0.0).and_then(|n| n.action)
}