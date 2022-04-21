use std::collections::HashMap;

fn main() {
    println!("Hello World!");
}

#[derive(Clone, Copy)]
struct GameState {
    player_hp: i64,
    player_mana: i64,

    villain_hp: i64,
    villain_attack: i64,

    effects: [i64; 3],
}

const POISON: usize = 0;
const SHIELD: usize = 1;
const RECHARGE: usize = 2;

struct Spell {
    mana_cost: i64,
    damage: i64,
    heal: i64,
    apply_effect: Option<(usize, i64)>,
}

impl Spell {
    fn can_cast(&self, game_state: GameState) -> bool {
        if game_state.player_mana < self.mana_cost {
            return false;
        }

        match self.apply_effect {
            Some((index, _)) => game_state.effects[index] == 0,
            _ => true,
        }
    }

    fn cast(&self, game_state: GameState) -> GameState {
        let mut new_game_state = game_state;
        new_game_state.player_hp += self.heal;
        new_game_state.player_mana -= self.mana_cost;
        new_game_state.villain_hp -= self.damage;

        match self.apply_effect {
            Some((index, duration)) => new_game_state.effects[index] = duration,
            _ => (),
        }

        new_game_state
    }
}