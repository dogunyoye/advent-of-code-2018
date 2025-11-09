//! `cargo run --bin day_24`

extern crate regex;

use std::cmp::Ordering;
use std::f64;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum AttackType {
    UNKNOWN,
    RADIATION,
    BLUDGEONING,
    FIRE,
    SLASHING,
    COLD
}

#[derive(Debug, Clone, PartialEq)]
enum GroupType {
    UNKNOWN,
    IMMUNE,
    INFECTION
}

#[derive(Debug, Clone, PartialEq)]
struct FightingGroup {
    group_type: GroupType,
    num_of_units: u32,
    hit_points: u32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
    attack_type: AttackType,
    attack_points: u32,
    effective_power: u32,
    initiative: u32
}

impl FightingGroup {
    fn calculate_effective_power(&mut self) {
        self.effective_power = self.num_of_units * self.attack_points;
    }
}

fn convert_to_attack_type(attack_str: &str) -> AttackType {
    match attack_str.trim() {
        "radiation" => return AttackType::RADIATION,
        "bludgeoning" => return AttackType::BLUDGEONING,
        "fire" => return AttackType::FIRE,
        "slashing" => return AttackType::SLASHING,
        "cold" => return AttackType::COLD,
        _ => return AttackType::UNKNOWN
    }
}

fn parse_fighting_group(line: &str, group_type: &GroupType) -> Option<FightingGroup> {

    let spec: Regex = Regex::new(
        r"(?x)
        (?P<units>\d+)\s+units\s+each\s+with\s+
        (?P<hp>\d+)\s+hit\s+points
        (?:\s+\((?P<attrs>[^)]+)\))?
        \s+with\s+an\s+attack\s+that\s+does\s+
        (?P<damage>\d+)\s+
        (?P<damage_type>\w+)\s+damage\s+
        at\s+initiative\s+(?P<initiative>\d+)"
    ).unwrap();

    if let Some(caps) = spec.captures(line) {
        let mut weaknesses: Vec<AttackType> = Vec::new();
        let mut immunities: Vec<AttackType> = Vec::new();

        if let Some(attrs) = caps.name("attrs") {
            for part in attrs.as_str().split(';') {
                let part = part.trim();
                if let Some(rest) = part.strip_prefix("weak to ") {
                    weaknesses.extend(rest.split(',').map(convert_to_attack_type));
                } else if let Some(rest) = part.strip_prefix("immune to ") {
                    immunities.extend(rest.split(',').map(convert_to_attack_type));
                }
            }
        }

        return Option::Some(FightingGroup {
            group_type: group_type.clone(),
            num_of_units: caps["units"].parse().unwrap(),
            hit_points: caps["hp"].parse().unwrap(),
            weaknesses: weaknesses,
            immunities: immunities,
            attack_type: convert_to_attack_type(&caps["damage_type"].to_string()),
            attack_points: caps["damage"].parse().unwrap(),
            effective_power: 0,
            initiative: caps["initiative"].parse().unwrap(),
        });
    }

    return Option::None
}

fn battle(mut fighting_groups: Vec<FightingGroup>) -> (GroupType, u32) {
    let mut stalemate = false;

    loop {
        fighting_groups.sort_by(|a, b| {
            match a.effective_power.cmp(&b.effective_power) {
                Ordering::Equal => a.initiative.cmp(&b.initiative),
                other => other,
            }
        });

        fighting_groups.reverse();

        let num_of_groups = fighting_groups.len();

        // dmg, attacker, attacker-index, defender, defender-index,
        let mut pending_attacks: Vec<(u32, FightingGroup, i32, FightingGroup, i32)> = Vec::new();

        for i in 0..num_of_groups {
            let attacker = fighting_groups.get(i).unwrap();
            let fighter_dmg = attacker.effective_power;

            let default = FightingGroup {
                group_type: GroupType::UNKNOWN,
                num_of_units: 0,
                hit_points: 0,
                weaknesses: vec![],
                immunities: vec![],
                attack_type: AttackType::UNKNOWN,
                attack_points: 0,
                effective_power: 0,
                initiative: 0
            };

            let mut to_attack = (0, attacker.clone(), -1, default.clone(), -1);

            let mut greatest_dmg = 0;

            for j in 0..num_of_groups {

                let defender: &FightingGroup = fighting_groups.get(j).unwrap();

                // check pending attacks to see if the target (defender) has already
                // been selected
                let mut already_selected = false;
                for x in 0..pending_attacks.len() {
                    if pending_attacks[x].3 == *defender {
                        already_selected = true;
                    }
                }

                if already_selected {
                    // skip iteration, find another target
                    // to attack.
                    continue;
                }

                if defender.group_type != attacker.group_type {

                    let mut dmg = fighter_dmg;
                    let is_immune = defender.immunities.contains(&attacker.attack_type);
                    let is_weak = defender.weaknesses.contains(&attacker.attack_type);

                    if is_weak {
                        dmg *= 2;
                    } else if is_immune {
                        // target is immune skip iteration
                        continue;

                    }

                    if dmg > greatest_dmg {
                        greatest_dmg = dmg;
                        to_attack = (dmg, attacker.clone(), i as i32, defender.clone(), j as i32);
                    } else if dmg == greatest_dmg {

                        if defender.effective_power > to_attack.3.effective_power {
                            to_attack = (dmg, attacker.clone(), i as i32, defender.clone(), j as i32);
                        } else if defender.effective_power == to_attack.3.effective_power {

                            if defender.initiative > to_attack.3.initiative {
                                to_attack = (dmg, attacker.clone(), i as i32, defender.clone(), j as i32);
                            } else if defender.initiative == to_attack.3.initiative {
                                // don't choose a target aka set back to default
                                to_attack = (0, attacker.clone(), -1, default.clone(), -1);
                            }
                        }
                    }
                }
            }

            if to_attack.2 != -1 {
                pending_attacks.push(to_attack);
            }
        }

        pending_attacks.sort_by(|a, b| { a.1.initiative.cmp(&b.1.initiative) });
        pending_attacks.reverse();

        let mut any_killed = false;

        for attack in pending_attacks {
            let mut dmg = 0;
            if let Some(attacker) = fighting_groups.get_mut(attack.2 as usize) {
                dmg = attacker.effective_power;
            }

            if dmg == 0 {
                continue;
            }

            let is_immune = attack.3.immunities.contains(&attack.1.attack_type);
            let is_weak = attack.3.weaknesses.contains(&attack.1.attack_type);

            if is_weak {
                dmg *= 2;
            } else if is_immune {
                dmg = 0;
            }

            let target: &mut FightingGroup = fighting_groups.get_mut(attack.4 as usize).unwrap();
            let units_removed: f64 = dmg as f64 / target.hit_points as f64;

            if units_removed >= (target.num_of_units as i32).into() {
                target.num_of_units = 0;
            } else {
                target.num_of_units -= units_removed as u32;
            }

            if units_removed as u32 > 0 {
                any_killed = true;
            }

            // re-evaluate target's effective power
            target.effective_power = target.num_of_units * target.attack_points;
        }

        if !any_killed {
            stalemate = true;
            break;
        }

        // get rid of the groups with no fighters
        fighting_groups.retain(|ref i|i.num_of_units > 0 );

        let mut all_same_type = false;
        for x in 0..fighting_groups.len() {
            if x + 1 == fighting_groups.len() {
                break;
            }

            if fighting_groups[x].group_type == fighting_groups[x+1].group_type {
                all_same_type = true;
            }
            else {
                all_same_type = false;
                break;
            }
        }

        // if all the fighting groups are the
        // same type, we have a winner.
        // Break loop.
        if all_same_type {
            break;
        }
    }

    let mut total = 0;
    let mut winning_type: GroupType = GroupType::UNKNOWN;

    if stalemate {
        return (GroupType::UNKNOWN, total);
    }

    for group in fighting_groups {
        winning_type = group.group_type;
        total += group.num_of_units;
    }

    (winning_type, total)
}

fn main() -> () {
    let mut fighting_groups: Vec<FightingGroup> = Vec::new();
    let mut group_type = GroupType::IMMUNE;
 
    for line in BufReader::new(File::open("src/data/day_24_input.txt").unwrap()).lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("Immune System:") {
            group_type = GroupType::IMMUNE;
            continue;
        } else if line.starts_with("Infection:") {
            group_type = GroupType::INFECTION;
            continue;
        }

        fighting_groups.push(parse_fighting_group(&line, &group_type).unwrap());
    }

    for fg in &mut fighting_groups {
        fg.calculate_effective_power();
    }

    let fighting_groups_part2 = fighting_groups.clone();

    println!("Part one: {:?}", battle(fighting_groups).1);

    let mut boost = 1;
    loop {
        let mut immune_boosted_fighters = fighting_groups_part2.clone();
        for i in 0..10 {
            let imm = immune_boosted_fighters.get_mut(i).unwrap();
            imm.attack_points += boost;
            imm.calculate_effective_power();
        }

        let result = battle(immune_boosted_fighters);
        if result.0 == GroupType::IMMUNE {
            println!("Part two: {:?}", result.1);
            break;
        }

        boost += 1;
    }

}