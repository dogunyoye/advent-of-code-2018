//! `cargo run --bin day_24`

use std::cmp::Ordering;
use std::f64;

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
            let mut attacker = fighting_groups.get(i).unwrap();
            let fighter_dmg = attacker.effective_power;

            let mut default = FightingGroup {
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
        //println!("HELP {:?}", fighting_groups);

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

    let immune0 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 89,
        hit_points: 11269,
        weaknesses: vec![AttackType::FIRE, AttackType::RADIATION],
        immunities: vec![],
        attack_type: AttackType::SLASHING,
        attack_points: 1018,
        effective_power: 0,
        initiative: 7
    };

    let immune1 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 371,
        hit_points: 8033,
        weaknesses: vec![],
        immunities: vec![],
        attack_type: AttackType::BLUDGEONING,
        attack_points: 204,
        effective_power: 0,
        initiative: 15
    };

    let immune2 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 86,
        hit_points: 12112,
        weaknesses: vec![AttackType::COLD],
        immunities: vec![AttackType::SLASHING, AttackType::BLUDGEONING],
        attack_type: AttackType::SLASHING,
        attack_points: 1110,
        effective_power: 0,
        initiative: 18
    };

    let immune3 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 4137,
        hit_points: 10451,
        weaknesses: vec![AttackType::SLASHING],
        immunities: vec![AttackType::RADIATION],
        attack_type: AttackType::SLASHING,
        attack_points: 20,
        effective_power: 0,
        initiative: 11
    };

    let immune4 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 3374,
        hit_points: 6277,
        weaknesses: vec![AttackType::SLASHING, AttackType::COLD],
        immunities: vec![],
        attack_type: AttackType::COLD,
        attack_points: 13,
        effective_power: 0,
        initiative: 10
    };

    let immune5 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 1907,
        hit_points: 1530,
        weaknesses: vec![AttackType::RADIATION],
        immunities: vec![AttackType::FIRE, AttackType::BLUDGEONING],
        attack_type: AttackType::FIRE,
        attack_points: 7,
        effective_power: 0,
        initiative: 9
    };

    let immune6 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 1179,
        hit_points: 6638,
        weaknesses: vec![AttackType::SLASHING, AttackType::BLUDGEONING],
        immunities: vec![AttackType::RADIATION],
        attack_type: AttackType::FIRE,
        attack_points: 49,
        effective_power: 0,
        initiative: 20
    };

    let immune7 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 4091,
        hit_points: 7627,
        weaknesses: vec![],
        immunities: vec![],
        attack_type: AttackType::BLUDGEONING,
        attack_points: 17,
        effective_power: 0,
        initiative: 17
    };

    let immune8 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 6318,
        hit_points: 7076,
        weaknesses: vec![],
        immunities: vec![],
        attack_type: AttackType::BLUDGEONING,
        attack_points: 8,
        effective_power: 0,
        initiative: 2
    };

    let immune9 = FightingGroup {
        group_type: GroupType::IMMUNE,
        num_of_units: 742,
        hit_points: 1702,
        weaknesses: vec![AttackType::RADIATION],
        immunities: vec![AttackType::SLASHING],
        attack_type: AttackType::RADIATION,
        attack_points: 22,
        effective_power: 0,
        initiative: 13
    };

    let infection0 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 3401,
        hit_points: 31843,
        weaknesses: vec![AttackType::COLD, AttackType::FIRE],
        immunities: vec![],
        attack_type: AttackType::SLASHING,
        attack_points: 16,
        effective_power: 0,
        initiative: 19
    };

    let infection1 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 1257,
        hit_points: 10190,
        weaknesses: vec![],
        immunities: vec![],
        attack_type: AttackType::COLD,
        attack_points: 16,
        effective_power: 0,
        initiative: 8
    };

    let infection2 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 2546,
        hit_points: 49009,
        weaknesses: vec![AttackType::BLUDGEONING, AttackType::RADIATION],
        immunities: vec![AttackType::COLD],
        attack_type: AttackType::BLUDGEONING,
        attack_points: 38,
        effective_power: 0,
        initiative: 6
    };

    let infection3 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 2593,
        hit_points: 12475,
        weaknesses: vec![],
        immunities: vec![],
        attack_type: AttackType::COLD,
        attack_points: 9,
        effective_power: 0,
        initiative: 1
    };

    let infection4 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 2194,
        hit_points: 25164,
        weaknesses: vec![AttackType::BLUDGEONING],
        immunities: vec![AttackType::COLD],
        attack_type: AttackType::BLUDGEONING,
        attack_points: 18,
        effective_power: 0,
        initiative: 14
    };

    let infection5 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 8250,
        hit_points: 40519,
        weaknesses: vec![AttackType::BLUDGEONING, AttackType::RADIATION],
        immunities: vec![AttackType::SLASHING],
        attack_type: AttackType::BLUDGEONING,
        attack_points: 8,
        effective_power: 0,
        initiative: 16
    };

    let infection6 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 1793,
        hit_points: 51817,
        weaknesses: vec![],
        immunities: vec![AttackType::BLUDGEONING],
        attack_type: AttackType::RADIATION,
        attack_points: 46,
        effective_power: 0,
        initiative: 3
    };

    let infection7 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 288,
        hit_points: 52213,
        weaknesses: vec![],
        immunities: vec![AttackType::BLUDGEONING],
        attack_type: AttackType::FIRE,
        attack_points: 339,
        effective_power: 0,
        initiative: 4
    };

    let infection8 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 22,
        hit_points: 38750,
        weaknesses: vec![AttackType::FIRE],
        immunities: vec![],
        attack_type: AttackType::SLASHING,
        attack_points: 3338,
        effective_power: 0,
        initiative: 5
    };

    let infection9 = FightingGroup {
        group_type: GroupType::INFECTION,
        num_of_units: 2365,
        hit_points: 25468,
        weaknesses: vec![AttackType::RADIATION, AttackType::COLD],
        immunities: vec![],
        attack_type: AttackType::FIRE,
        attack_points: 20,
        effective_power: 0,
        initiative: 12
    };

    let mut fighting_groups: Vec<FightingGroup> = Vec::new();
    fighting_groups.push(immune0.clone());
    fighting_groups.push(immune1.clone());
    fighting_groups.push(immune2.clone());
    fighting_groups.push(immune3.clone());
    fighting_groups.push(immune4.clone());
    fighting_groups.push(immune5.clone());
    fighting_groups.push(immune6.clone());
    fighting_groups.push(immune7.clone());
    fighting_groups.push(immune8.clone());
    fighting_groups.push(immune9.clone());

    fighting_groups.push(infection0.clone());
    fighting_groups.push(infection1.clone());
    fighting_groups.push(infection2.clone());
    fighting_groups.push(infection3.clone());
    fighting_groups.push(infection4.clone());
    fighting_groups.push(infection5.clone());
    fighting_groups.push(infection6.clone());
    fighting_groups.push(infection7.clone());
    fighting_groups.push(infection8.clone());
    fighting_groups.push(infection9.clone());

    for x in 0..fighting_groups.len() {
        fighting_groups[x].calculate_effective_power();
    }

    let fighting_groups_part2 = fighting_groups.clone();

    println!("Part one: {:?}", battle(fighting_groups));

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
            println!("Part two: {:?}", result);
            break;
        }

        boost += 1;
    }

}