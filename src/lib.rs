const MAX_HEALTH: u32 = 1000;

pub struct Character {
    pub health: u32,
    pub level: u32,
    pub alive: bool,
}

impl Character {
    pub fn new(health: u32, level: u32) -> Self {
        let alive = health > 0;
        Self {
            health,
            level,
            alive,
        }
    }

    pub fn attack(&self, target: Self, damage: u32) -> Self {
        let level_diff = self.level as i32 - target.level as i32;
        let new_damage = match level_diff {
            5.. => damage + (damage / 2),
            ..=-5 => damage - (damage / 2),
            _ => damage,
        };
        let new_health = target.health - new_damage;
        Self::new(new_health, target.level)
    }

    pub fn heal(&self, health: u32) -> Self {
        if self.alive {
            match self.health + health > MAX_HEALTH {
                true => Self::new(health, self.level),
                false => Self::new(self.health + health, self.level),
            }
        } else {
            Self::new(0, self.level)
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self::new(MAX_HEALTH, 1)
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
        let defender = attacker.attack(defender, 1000);
        assert_eq!(defender.health, 0);
        assert!(!defender.alive);

        let attacker = Character::new(1000, 10);
        let stronger_defender = Character::new(1000, 20);
        let stronger_defender = attacker.attack(stronger_defender, 200);
        assert_eq!(stronger_defender.health, 900);

        let weaker_defender = Character::new(1000, 1);
        let weaker_defender = attacker.attack(weaker_defender, 200);
        assert_eq!(weaker_defender.health, 700);
    }

    #[test]
    fn test_character_heal() {
        let healer = Character::new(100, 1);
        let healer = healer.heal(100);
        assert_eq!(healer.health, 200);

        let healer = healer.heal(1000);
        assert_eq!(healer.health, 1000);

        let healer = Character::new(0, 1);
        let healer = healer.heal(100);
        assert_eq!(healer.health, 0);
        assert!(!healer.alive);
    }
}
