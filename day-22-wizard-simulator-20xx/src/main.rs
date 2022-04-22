use itertools::Itertools;
use crate::Winner::{Nobody, Player, Villain};

fn main() {
    let part_1_solution = simulate_battle(false);
    println!("Part 1 solution: {}", part_1_solution);

    let part_2_solution = simulate_battle(true);
    println!("Part 2 solution: {}", part_2_solution);
}

#[derive(Clone, Copy)]
struct GameState {
    player_hp: i64,
    player_mana: i64,

    villain_hp: i64,
    villain_attack: i64,

    effects: [i64; 3],
    winner: Winner,
    hard_mode: bool,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Winner {
    Nobody,
    Player,
    Villain,
}

struct GameTreeNode {
    game_state: GameState,
    total_mana_used: i64,
}

fn simulate_battle(hard_mode: bool) -> i64 {
    let mut tree: Vec<GameTreeNode> = Vec::new();
    let mut minimum_mana = i64::MAX;

    let root = GameTreeNode {
        game_state: get_initial_game_state(hard_mode),
        total_mana_used: 0,
    };

    tree.push(root);

    while tree.len() > 0 {
        let (new_tree, new_minimum_mana) = expand_nodes(tree, minimum_mana);
        tree = new_tree;
        minimum_mana = new_minimum_mana;
    }

    minimum_mana
}

fn expand_nodes(tree: Vec<GameTreeNode>, current_minimum_mana: i64) -> (Vec<GameTreeNode>, i64) {
    let mut new_tree: Vec<GameTreeNode> = Vec::new();
    let mut minimum_mana = current_minimum_mana;

    for node in tree {
        if node.game_state.winner == Player {
            if node.total_mana_used < minimum_mana {
                minimum_mana = node.total_mana_used;
            }

            continue;
        }

        if node.game_state.winner == Villain {
            continue;
        }

        let spells = get_possible_spells(node.game_state);

        for spell in spells {
            if spell.mana_cost + node.total_mana_used < minimum_mana {
                let new_game_state = simulate_battle_round(node.game_state, spell);

                new_tree.push(GameTreeNode {
                    game_state: new_game_state,
                    total_mana_used: spell.mana_cost + node.total_mana_used,
                })
            }
        }
    }

    (new_tree, minimum_mana)
}

fn simulate_battle_round(game_state: GameState, chosen_spell: Spell) -> GameState {
    let mut new_game_state = chosen_spell.cast(game_state);

    if new_game_state.hard_mode {
        new_game_state.player_hp -= 1;
    }

    new_game_state = apply_effects(new_game_state);

    if new_game_state.villain_hp <= 0 {
        new_game_state.winner = Player;
        return new_game_state;
    }

    new_game_state = villain_attack(new_game_state);

    if new_game_state.player_hp <= 0 {
        new_game_state.winner = Villain;
        return new_game_state;
    }

    new_game_state = apply_effects(new_game_state);

    if new_game_state.villain_hp <= 0 {
        new_game_state.winner = Player;
    }

    new_game_state
}

fn get_possible_spells(game_state: GameState) -> Vec<Spell> {
    get_spell_list().iter().filter(|spell| spell.can_cast(game_state)).map(|spell_reference| *spell_reference).collect_vec()
}

const POISON: usize = 0;
const SHIELD: usize = 1;
const RECHARGE: usize = 2;

#[derive(Clone, Copy)]
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

fn get_spell_list() -> [Spell;5] {
    [
        Spell { mana_cost: 53, damage: 4, heal: 0, apply_effect: None },
        Spell { mana_cost: 73, damage: 2, heal: 2, apply_effect: None },
        Spell { mana_cost: 113, damage: 0, heal: 0, apply_effect: Some((SHIELD, 6)) },
        Spell { mana_cost: 173, damage: 0, heal: 0, apply_effect: Some((POISON, 6)) },
        Spell { mana_cost: 229, damage: 0, heal: 0, apply_effect: Some((RECHARGE, 5)) },
    ]
}

fn get_initial_game_state(hard_mode: bool) -> GameState {
    GameState {
        player_hp: 50,
        player_mana: 500,

        villain_hp: 71,
        villain_attack: 10,

        effects: [0, 0, 0],
        winner: Nobody,
        hard_mode,
    }
}

fn apply_effects(game_state: GameState) -> GameState {
    let mut new_game_state = game_state;

    if game_state.effects[POISON] > 0 {
        new_game_state.villain_hp -= 3;
    }

    if game_state.effects[RECHARGE] > 0 {
        new_game_state.player_mana += 101;
    }

    tick_effects(&mut new_game_state);

    new_game_state
}

fn tick_effects(game_state: &mut GameState) {
    for i in 0..game_state.effects.len() {
        if game_state.effects[i] > 0 {
            game_state.effects[i] -= 1;
        }
    }
}

fn villain_attack(game_state: GameState) -> GameState {
    let mut new_game_state = game_state;
    let mut dps = new_game_state.villain_attack;

    if new_game_state.effects[SHIELD] > 0 {
        dps -= 7;
    }

    if dps < 1 {
        dps = 1;
    }

    new_game_state.player_hp -= dps;

    new_game_state
}