// ProjectE native mod runtime implementation for Horizon

pub struct ItemDef {
    pub id: &'static str,
    pub template: &'static str,
    pub max_stack_size: Option<u8>,
    pub rarity: Option<&'static str>,
    pub crafting_remainder: Option<&'static str>,
}

pub const PROJECTE_ITEMS: &[ItemDef] = &[
    // 1. philosophers_stone: max stack 1, leaves itself in crafting grid as remainder
    ItemDef {
        id: "projecte:philosophers_stone",
        template: "minecraft:diamond",
        max_stack_size: Some(1),
        rarity: None,
        crafting_remainder: Some("projecte:philosophers_stone"),
    },
    // 2. low_covalence_dust
    ItemDef {
        id: "projecte:low_covalence_dust",
        template: "minecraft:redstone",
        max_stack_size: None,
        rarity: None,
        crafting_remainder: None,
    },
    // 3. medium_covalence_dust
    ItemDef {
        id: "projecte:medium_covalence_dust",
        template: "minecraft:redstone",
        max_stack_size: None,
        rarity: None,
        crafting_remainder: None,
    },
    // 4. high_covalence_dust
    ItemDef {
        id: "projecte:high_covalence_dust",
        template: "minecraft:redstone",
        max_stack_size: None,
        rarity: None,
        crafting_remainder: None,
    },
    // 5. alchemical_coal
    ItemDef {
        id: "projecte:alchemical_coal",
        template: "minecraft:coal",
        max_stack_size: None,
        rarity: None,
        crafting_remainder: None,
    },
    // 6. mobius_fuel
    ItemDef {
        id: "projecte:mobius_fuel",
        template: "minecraft:coal",
        max_stack_size: None,
        rarity: None,
        crafting_remainder: None,
    },
    // 7. aeternalis_fuel: Rare rarity
    ItemDef {
        id: "projecte:aeternalis_fuel",
        template: "minecraft:coal",
        max_stack_size: None,
        rarity: Some("rare"),
        crafting_remainder: None,
    },
    // 8. dark_matter: Rare rarity, fire immune (template netherite_ingot)
    ItemDef {
        id: "projecte:dark_matter",
        template: "minecraft:netherite_ingot",
        max_stack_size: None,
        rarity: Some("rare"),
        crafting_remainder: None,
    },
    // 9. red_matter: Epic rarity, fire immune (template netherite_ingot)
    ItemDef {
        id: "projecte:red_matter",
        template: "minecraft:netherite_ingot",
        max_stack_size: None,
        rarity: Some("epic"),
        crafting_remainder: None,
    },
];

pub const PHILOSOPHERS_STONE_JSON: &str = r#"{
  "type": "minecraft:crafting_shaped",
  "category": "equipment",
  "group": "projecte:philosophers_stone",
  "key": {
    "D": { "item": "minecraft:diamond" },
    "G": { "item": "minecraft:glowstone_dust" },
    "R": { "item": "minecraft:redstone" }
  },
  "pattern": [
    "RGR",
    "GDG",
    "RGR"
  ],
  "result": {
    "count": 1,
    "id": "projecte:philosophers_stone"
  }
}"#;

pub const PHILOSOPHERS_STONE_ALT_JSON: &str = r#"{
  "type": "minecraft:crafting_shaped",
  "category": "equipment",
  "group": "projecte:philosophers_stone",
  "key": {
    "D": { "item": "minecraft:diamond" },
    "G": { "item": "minecraft:glowstone_dust" },
    "R": { "item": "minecraft:redstone" }
  },
  "pattern": [
    "GRG",
    "RDR",
    "GRG"
  ],
  "result": {
    "count": 1,
    "id": "projecte:philosophers_stone"
  }
}"#;

pub const LOW_COVALENCE_DUST_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "minecraft:charcoal" },
    { "item": "minecraft:cobblestone" },
    { "item": "minecraft:cobblestone" },
    { "item": "minecraft:cobblestone" },
    { "item": "minecraft:cobblestone" },
    { "item": "minecraft:cobblestone" },
    { "item": "minecraft:cobblestone" },
    { "item": "minecraft:cobblestone" },
    { "item": "minecraft:cobblestone" }
  ],
  "result": {
    "count": 40,
    "id": "projecte:low_covalence_dust"
  }
}"#;

pub const MEDIUM_COVALENCE_DUST_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "minecraft:iron_ingot" },
    { "item": "minecraft:redstone" }
  ],
  "result": {
    "count": 40,
    "id": "projecte:medium_covalence_dust"
  }
}"#;

pub const HIGH_COVALENCE_DUST_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "minecraft:diamond" },
    { "item": "minecraft:coal" }
  ],
  "result": {
    "count": 40,
    "id": "projecte:high_covalence_dust"
  }
}"#;

pub const ALCHEMICAL_COAL_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "projecte:philosophers_stone" },
    { "item": "minecraft:coal" },
    { "item": "minecraft:coal" },
    { "item": "minecraft:coal" },
    { "item": "minecraft:coal" }
  ],
  "result": {
    "count": 1,
    "id": "projecte:alchemical_coal"
  }
}"#;

pub const MOBIUS_FUEL_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "projecte:philosophers_stone" },
    { "item": "projecte:alchemical_coal" },
    { "item": "projecte:alchemical_coal" },
    { "item": "projecte:alchemical_coal" },
    { "item": "projecte:alchemical_coal" }
  ],
  "result": {
    "count": 1,
    "id": "projecte:mobius_fuel"
  }
}"#;

pub const AETERNALIS_FUEL_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "projecte:philosophers_stone" },
    { "item": "projecte:mobius_fuel" },
    { "item": "projecte:mobius_fuel" },
    { "item": "projecte:mobius_fuel" },
    { "item": "projecte:mobius_fuel" }
  ],
  "result": {
    "count": 1,
    "id": "projecte:aeternalis_fuel"
  }
}"#;

pub const ALCHEMICAL_COAL_TO_COAL_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "projecte:philosophers_stone" },
    { "item": "projecte:alchemical_coal" }
  ],
  "result": {
    "count": 4,
    "id": "minecraft:coal"
  }
}"#;

pub const MOBIUS_FUEL_TO_ALCHEMICAL_COAL_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "projecte:philosophers_stone" },
    { "item": "projecte:mobius_fuel" }
  ],
  "result": {
    "count": 4,
    "id": "projecte:alchemical_coal"
  }
}"#;

pub const AETERNALIS_FUEL_TO_MOBIUS_FUEL_JSON: &str = r#"{
  "type": "minecraft:crafting_shapeless",
  "category": "misc",
  "ingredients": [
    { "item": "projecte:philosophers_stone" },
    { "item": "projecte:aeternalis_fuel" }
  ],
  "result": {
    "count": 4,
    "id": "projecte:mobius_fuel"
  }
}"#;

pub const DARK_MATTER_JSON: &str = r#"{
  "type": "minecraft:crafting_shaped",
  "category": "misc",
  "key": {
    "A": { "item": "projecte:aeternalis_fuel" },
    "D": { "item": "minecraft:diamond_block" }
  },
  "pattern": [
    "AAA",
    "ADA",
    "AAA"
  ],
  "result": {
    "count": 1,
    "id": "projecte:dark_matter"
  }
}"#;

pub const RED_MATTER_JSON: &str = r#"{
  "type": "minecraft:crafting_shaped",
  "category": "misc",
  "group": "projecte:red_matter",
  "key": {
    "A": { "item": "projecte:aeternalis_fuel" },
    "D": { "item": "projecte:dark_matter" }
  },
  "pattern": [
    "AAA",
    "DDD",
    "AAA"
  ],
  "result": {
    "count": 1,
    "id": "projecte:red_matter"
  }
}"#;

pub const RED_MATTER_ALT_JSON: &str = r#"{
  "type": "minecraft:crafting_shaped",
  "category": "misc",
  "group": "projecte:red_matter",
  "key": {
    "A": { "item": "projecte:aeternalis_fuel" },
    "D": { "item": "projecte:dark_matter" }
  },
  "pattern": [
    "ADA",
    "ADA",
    "ADA"
  ],
  "result": {
    "count": 1,
    "id": "projecte:red_matter"
  }
}"#;

pub const RECIPES: &[(&str, &str)] = &[
    ("projecte:philosophers_stone", PHILOSOPHERS_STONE_JSON),
    ("projecte:philosophers_stone_alt", PHILOSOPHERS_STONE_ALT_JSON),
    ("projecte:low_covalence_dust", LOW_COVALENCE_DUST_JSON),
    ("projecte:medium_covalence_dust", MEDIUM_COVALENCE_DUST_JSON),
    ("projecte:high_covalence_dust", HIGH_COVALENCE_DUST_JSON),
    ("projecte:alchemical_coal", ALCHEMICAL_COAL_JSON),
    ("projecte:mobius_fuel", MOBIUS_FUEL_JSON),
    ("projecte:aeternalis_fuel", AETERNALIS_FUEL_JSON),
    ("projecte:alchemical_coal_to_coal", ALCHEMICAL_COAL_TO_COAL_JSON),
    ("projecte:mobius_fuel_to_alchemical_coal", MOBIUS_FUEL_TO_ALCHEMICAL_COAL_JSON),
    ("projecte:aeternalis_fuel_to_mobius_fuel", AETERNALIS_FUEL_TO_MOBIUS_FUEL_JSON),
    ("projecte:dark_matter", DARK_MATTER_JSON),
    ("projecte:red_matter", RED_MATTER_JSON),
    ("projecte:red_matter_alt", RED_MATTER_ALT_JSON),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_definitions_count() {
        assert_eq!(PROJECTE_ITEMS.len(), 9);
        assert_eq!(PROJECTE_ITEMS[0].id, "projecte:philosophers_stone");
        assert_eq!(PROJECTE_ITEMS[0].max_stack_size, Some(1));
        assert_eq!(
            PROJECTE_ITEMS[0].crafting_remainder,
            Some("projecte:philosophers_stone")
        );
    }

    #[test]
    fn recipes_count() {
        assert_eq!(RECIPES.len(), 14);
    }
}
