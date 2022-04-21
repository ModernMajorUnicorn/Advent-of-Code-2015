use itertools::{Itertools};

fn main() {
    let all_loadouts = generate_loadouts();
    let villain = get_villain();

    let mut cheapest_winner = i64::MAX;
    let mut most_expensive_loser = 0;

    for (cost, player_character) in all_loadouts {
        let victory = simulate_battle(player_character, villain);

        if victory && cost < cheapest_winner {
            cheapest_winner = cost;
        } else if !victory && cost > most_expensive_loser {
            most_expensive_loser = cost;
        }
    }

    println!("Part 1 solution: {}", cheapest_winner);
    println!("Part 2 solution: {}", most_expensive_loser);
}

fn generate_loadouts() -> Vec<(i64, Character)> {
    let shop = get_shop();
    let ring_combos = (0..=2).flat_map(|k| shop.rings.iter().combinations(k)).collect_vec();
    let mut loadouts = Vec::new();

    for weapon in shop.weapons {
        for armor in shop.armors {
            for ring_combo in ring_combos.iter() {
                let mut loadout = Vec::new();
                loadout.push(weapon);
                loadout.push(armor);

                for ring in ring_combo {
                    loadout.push(**ring);
                }

                let cost = loadout.iter().map(|item| item.cost).sum();
                let character = Character::from_gear(loadout);
                loadouts.push((cost, character));
            }
        }

        for ring_combo in ring_combos.iter() {
            let mut loadout = Vec::new();
            loadout.push(weapon);

            for ring in ring_combo {
                loadout.push(**ring);
            }

            let cost = loadout.iter().map(|item| item.cost).sum();
            let character = Character::from_gear(loadout);
            loadouts.push((cost, character));
        }
    }

    loadouts
}

fn get_shop() -> Shop {
    Shop {
        weapons: [
            Item::weapon(8, 4),
            Item::weapon(10, 5),
            Item::weapon(25, 6),
            Item::weapon(40, 7),
            Item::weapon(74, 8),
        ],

        armors: [
            Item::armor(13, 1),
            Item::armor(31, 2),
            Item::armor(53, 3),
            Item::armor(75, 4),
            Item::armor(102, 5),
        ],

        rings: [
            Item::ring(25, 1, 0),
            Item::ring(50, 2, 0),
            Item::ring(100, 3, 0),
            Item::ring(20, 0, 1),
            Item::ring(40, 0, 2),
            Item::ring(80, 0, 3),
        ]
    }
}

fn get_villain() -> Character {
    Character {
        hit_points: 109,
        damage: 8,
        armor: 2
    }
}

fn simulate_battle(player_character: Character, villain: Character) -> bool {
    let mut player_hp = player_character.hit_points;
    let player_dps = calculate_dps(player_character, villain);

    let mut villain_hp = villain.hit_points;
    let villain_dps = calculate_dps(villain, player_character);

    loop {
        villain_hp -= player_dps;

        if villain_hp <= 0 {
            return true;
        }

        player_hp -= villain_dps;

        if player_hp <= 0 {
            return false;
        }
    }
}

fn calculate_dps(attacker: Character, defender: Character) -> i64 {
    if attacker.damage > defender.armor {
        attacker.damage - defender.armor
    } else {
        1
    }
}

#[derive(Clone, Copy)]
struct Item {
    cost: i64,
    damage: i64,
    armor: i64
}

impl Item {
    fn weapon(cost: i64, damage: i64) -> Item {
        Item {
            cost,
            damage,
            armor: 0
        }
    }

    fn armor(cost: i64, armor: i64) -> Item {
        Item {
            cost,
            damage: 0,
            armor
        }
    }

    fn ring(cost: i64, damage: i64, armor: i64) -> Item {
        Item {
            cost,
            damage,
            armor
        }
    }
}

struct Shop {
    weapons: [Item; 5],
    armors: [Item; 5],
    rings: [Item; 6]
}

#[derive(Clone, Copy)]
struct Character {
    hit_points: i64,
    damage: i64,
    armor: i64
}

impl Character {
    fn from_gear(gear: Vec<Item>) -> Character {
        Character {
            hit_points: 100,
            damage: gear.iter().map(|item| item.damage).sum(),
            armor: gear.iter().map(|item| item.armor).sum()
        }
    }
}