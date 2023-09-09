const MAX_HEALTH: u32 = 1000;

pub enum AttackTargetType {
    Character(Character),
    Prop(Prop),
}

impl AttackTargetType {
    pub fn health(&self) -> u32 {
        match self {
            AttackTargetType::Character(character) => character.health,
            AttackTargetType::Prop(prop) => prop.health,
        }
    }

    pub fn alive(&self) -> bool {
        match self {
            AttackTargetType::Character(character) => character.alive,
            AttackTargetType::Prop(prop) => !prop.destroyed,
        }
    }

    pub fn destroyed(&self) -> bool {
        match self {
            AttackTargetType::Character(character) => !character.alive,
            AttackTargetType::Prop(prop) => prop.destroyed,
        }
    }
}

impl From<Character> for AttackTargetType {
    fn from(character: Character) -> Self {
        Self::Character(character)
    }
}

impl From<Prop> for AttackTargetType {
    fn from(prop: Prop) -> Self {
        Self::Prop(prop)
    }
}

pub struct Prop {
    pub health: u32,
    pub destroyed: bool,
}

impl Prop {
    pub fn new(health: u32) -> Self {
        Self {
            health,
            destroyed: health == 0,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CharacterClass {
    Melee,
    Ranged,
}

pub struct Character {
    pub health: u32,
    pub level: u32,
    pub alive: bool,
    pub class: CharacterClass,
    pub attack_range: u32,
    pub factions: Option<Vec<&'static str>>,
}

impl Character {
    pub fn new(health: u32, level: u32, class: CharacterClass) -> Self {
        let alive = health > 0;
        let attack_range = match class {
            CharacterClass::Melee => 2,
            CharacterClass::Ranged => 20,
        };
        Self {
            health,
            level,
            alive,
            class,
            attack_range,
            factions: None,
        }
    }

    fn attack_prop(&self, target: Prop, damage: u32) -> Prop {
        let new_health = target.health - damage;
        Prop::new(new_health)
    }

    fn attack_character(&self, target: Self, damage: u32) -> Self {
        if self.is_allied_with(&target) {
            return target;
        }
        let level_diff = self.level as i32 - target.level as i32;
        let new_damage = match level_diff {
            5.. => damage + (damage / 2),
            ..=-5 => damage - (damage / 2),
            _ => damage,
        };
        let new_health = target.health - new_damage;
        Self::new(new_health, target.level, target.class)
    }

    pub fn attack(&self, target: AttackTargetType, damage: u32, range: u32) -> AttackTargetType {
        if range > self.attack_range {
            return target;
        }
        match target {
            AttackTargetType::Character(target) => {
                AttackTargetType::Character(self.attack_character(target, damage))
            }
            AttackTargetType::Prop(target) => {
                AttackTargetType::Prop(self.attack_prop(target, damage))
            }
        }
    }

    fn heal(&self, target: &Self, health: u32) -> Self {
        if target.alive {
            match target.health + health > MAX_HEALTH {
                true => Self::new(health, target.level, target.class),
                false => Self::new(target.health + health, target.level, target.class),
            }
        } else {
            Self::new(0, target.level, target.class)
        }
    }

    pub fn heal_self(&self, health: u32) -> Self {
        self.heal(self, health)
    }

    pub fn heal_others(&self, target: Self, health: u32) -> Self {
        if self.is_allied_with(&target) {
            return self.heal(&target, health);
        };
        target
    }

    pub fn join_faction(&self, faction: &'static str) -> Self {
        let mut factions = match self.factions.clone() {
            Some(factions) => factions,
            None => vec![],
        };
        factions.push(faction);
        Self {
            factions: Some(factions),
            ..*self
        }
    }

    pub fn leave_faction(&self, faction: &'static str) -> Self {
        let factions = match self.factions.clone() {
            Some(factions) => {
                let mut factions = factions;
                factions.retain(|f| f != &faction);
                if factions.is_empty() {
                    None
                } else {
                    Some(factions)
                }
            }
            None => None,
        };
        Self { factions, ..*self }
    }

    pub fn is_allied_with(&self, other_character: &Character) -> bool {
        let other_char_factions = match other_character.factions.clone() {
            Some(factions) => factions,
            None => return false,
        };
        match self.factions.clone() {
            Some(factions) => factions.iter().any(|f| other_char_factions.contains(f)),
            None => false,
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new(MAX_HEALTH, 1, CharacterClass::Melee)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_character() {
        let character = Character::default();
        assert_eq!(character.health, 1000);
        assert_eq!(character.level, 1);
        assert!(character.alive);
        assert_eq!(character.factions, None);
    }

    #[test]
    fn test_character_attack() {
        let attacker = Character::default();
        let defender = Character::default();
        let defender = attacker.attack(defender.into(), 1000, 1);
        assert_eq!(defender.health(), 0);
        assert!(!defender.alive());

        let attacker = Character::new(1000, 10, CharacterClass::Melee);
        let stronger_defender = Character::new(1000, 20, CharacterClass::Melee);
        let stronger_defender = attacker.attack(stronger_defender.into(), 200, 1);
        assert_eq!(stronger_defender.health(), 900);

        let weaker_defender = Character::new(1000, 1, CharacterClass::Melee);
        let weaker_defender = attacker.attack(weaker_defender.into(), 200, 1);
        assert_eq!(weaker_defender.health(), 700);
    }

    #[test]
    fn test_character_heal() {
        let healer = Character::new(100, 1, CharacterClass::Melee);
        let healer = healer.heal_self(100);
        assert_eq!(healer.health, 200);

        let healer = healer.heal_self(1000);
        assert_eq!(healer.health, 1000);

        let healer = Character::new(0, 1, CharacterClass::Melee);
        let healer = healer.heal_self(100);
        assert_eq!(healer.health, 0);
        assert!(!healer.alive);
    }

    #[test]
    fn test_attack_range() {
        let melee_fighter = Character::new(1000, 1, CharacterClass::Melee);
        let out_of_range_target = Character::default();
        let in_range_target = Character::default();
        let out_of_range_target = melee_fighter.attack(out_of_range_target.into(), 1000, 5);
        let in_range_target = melee_fighter.attack(in_range_target.into(), 1000, 1);
        assert_eq!(out_of_range_target.health(), 1000);
        assert_eq!(in_range_target.health(), 0);

        let ranged_fighter = Character::new(1000, 1, CharacterClass::Ranged);
        let out_of_range_target = Character::default();
        let in_range_target = Character::default();
        let out_of_range_target = ranged_fighter.attack(out_of_range_target.into(), 1000, 25);
        let in_range_target = ranged_fighter.attack(in_range_target.into(), 1000, 20);
        assert_eq!(out_of_range_target.health(), 1000);
        assert_eq!(in_range_target.health(), 0);
    }

    #[test]
    fn test_join_and_leave_one_or_more_factions() {
        let factions = vec!["Faction-A"];
        let character = Character::default().join_faction("Faction-A");
        assert_eq!(character.factions, Some(factions));

        let factions = vec!["Faction-A", "Faction-B"];
        let character = character.join_faction("Faction-B");
        assert_eq!(character.factions, Some(factions));

        let factions = vec!["Faction-A"];
        let character = character.leave_faction("Faction-B");
        assert_eq!(character.factions, Some(factions));

        let character = character.leave_faction("Faction-A");
        assert_eq!(character.factions, None);
    }

    #[test]
    fn test_same_faction_allies() {
        let faction_name = "Faction-A";
        let char_a = Character::default().join_faction(faction_name);
        let char_b = Character::default().join_faction(faction_name);
        let char_c = Character::default();
        assert!(char_a.is_allied_with(&char_b));
        assert!(!char_a.is_allied_with(&char_c));
    }

    #[test]
    fn test_can_not_damage_allies() {
        let faction_name = "Faction-A";
        let char_a = Character::default().join_faction(faction_name);
        let char_b = Character::default().join_faction(faction_name);
        let char_c = Character::default();
        let char_b = char_a.attack(char_b.into(), 1000, 1);
        let char_c = char_a.attack(char_c.into(), 1000, 1);
        assert_eq!(char_b.health(), 1000);
        assert_eq!(char_c.health(), 0);
    }

    #[test]
    fn test_can_heal_allies() {
        let faction_name = "Faction-A";
        let char_a = Character::default().join_faction(faction_name);
        let char_b = Character::new(500, 1, CharacterClass::Melee).join_faction(faction_name);
        let char_c = Character::new(500, 1, CharacterClass::Melee);
        let char_b = char_a.heal_others(char_b, 200);
        let char_c = char_a.heal_others(char_c, 200);
        assert_eq!(char_b.health, 700);
        assert_eq!(char_c.health, 500);
    }

    #[test]
    fn test_create_prop() {
        let prop = Prop::new(1000);
        assert_eq!(prop.health, 1000);
        assert!(!prop.destroyed);

        let prop = Prop::new(0);
        assert_eq!(prop.health, 0);
        assert!(prop.destroyed);
    }

    #[test]
    fn test_can_attack_prop() {
        let attacker = Character::default();
        let prop = Prop::new(1000);
        let prop = attacker.attack(prop.into(), 1000, 1);
        assert_eq!(prop.health(), 0);
        assert!(prop.destroyed());
    }
}
