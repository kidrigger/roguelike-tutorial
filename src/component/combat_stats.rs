#[derive(Debug, Clone, Copy)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub attack: i32,
}

impl CombatStats {
    pub fn new(max_hp: i32, defense: i32, attack: i32) -> Self {
        Self {
            max_hp,
            hp: max_hp,
            defense,
            attack,
        }
    }
}
