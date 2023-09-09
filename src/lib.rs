const MAX_HEALTH: u32 = 1000;

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
        }
    }

    pub fn attack(&self, target: Self, damage: u32, range: u32) -> Self {
        if range > self.attack_range {
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

    pub fn heal(&self, health: u32) -> Self {
        if self.alive {
            match self.health + health > MAX_HEALTH {
                true => Self::new(health, self.level, self.class),
                false => Self::new(self.health + health, self.level, self.class),
            }
        } else {
            Self::new(0, self.level, self.class)
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
    }

    #[test]
    fn test_character_attack() {
        let attacker = Character::default();
        let defender = Character::default();
        let defender = attacker.attack(defender, 1000, 1);
        assert_eq!(defender.health, 0);
        assert!(!defender.alive);

        let attacker = Character::new(1000, 10, CharacterClass::Melee);
        let stronger_defender = Character::new(1000, 20, CharacterClass::Melee);
        let stronger_defender = attacker.attack(stronger_defender, 200, 1);
        assert_eq!(stronger_defender.health, 900);

        let weaker_defender = Character::new(1000, 1, CharacterClass::Melee);
        let weaker_defender = attacker.attack(weaker_defender, 200, 1);
        assert_eq!(weaker_defender.health, 700);
    }

    #[test]
    fn test_character_heal() {
        let healer = Character::new(100, 1, CharacterClass::Melee);
        let healer = healer.heal(100);
        assert_eq!(healer.health, 200);

        let healer = healer.heal(1000);
        assert_eq!(healer.health, 1000);

        let healer = Character::new(0, 1, CharacterClass::Melee);
        let healer = healer.heal(100);
        assert_eq!(healer.health, 0);
        assert!(!healer.alive);
    }

    #[test]
    fn test_attack_range() {
        let melee_fighter = Character::new(1000, 1, CharacterClass::Melee);
        let out_of_range_target = Character::default();
        let in_range_target = Character::default();
        let out_of_range_target = melee_fighter.attack(out_of_range_target, 1000, 5);
        let in_range_target = melee_fighter.attack(in_range_target, 1000, 1);
        assert_eq!(out_of_range_target.health, 1000);
        assert_eq!(in_range_target.health, 0);

        let ranged_fighter = Character::new(1000, 1, CharacterClass::Ranged);
        let out_of_range_target = Character::default();
        let in_range_target = Character::default();
        let out_of_range_target = ranged_fighter.attack(out_of_range_target, 1000, 25);
        let in_range_target = ranged_fighter.attack(in_range_target, 1000, 20);
        assert_eq!(out_of_range_target.health, 1000);
        assert_eq!(in_range_target.health, 0);
    }
}
