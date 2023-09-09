const MAX_HEALTH: u32 = 1000;

pub struct Character {
    pub health: u32,
    pub level: u32,
    pub alive: bool,
}

impl Character {
    pub fn attack(&self, target: &mut Character, damage: u32) {
        if target.health <= damage {
            target.health = 0;
            target.alive = false;
        } else {
            target.health -= damage;
        }
    }

    pub fn heal(&self, target: &mut Character, health: u32) {
        if target.alive {
            match target.health + health > MAX_HEALTH {
                true => target.health = MAX_HEALTH,
                false => target.health += health,
            }
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            health: MAX_HEALTH,
            level: 1,
            alive: true,
        }
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
        let mut defender = Character::default();
        attacker.attack(&mut defender, 1000);
        assert_eq!(defender.health, 0);
        assert!(!defender.alive);
    }

    #[test]
    fn test_character_heal() {
        let healer = Character::default();
        let mut target = Character::default();
        healer.attack(&mut target, 200);
        healer.heal(&mut target, 100);
        assert_eq!(target.health, 900);
        healer.heal(&mut target, 200);
        assert_eq!(target.health, 1000);
        healer.attack(&mut target, 1200);
        healer.heal(&mut target, 100);
        assert_eq!(target.health, 0);
        assert!(!target.alive);
    }
}
