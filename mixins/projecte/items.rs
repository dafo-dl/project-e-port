fn projecte_items(items: &mut Vec<horizon_mods::content::Item>) {
    for def in crate::projecte::PROJECTE_ITEMS {
        let rarity = match def.rarity {
            Some("rare") => Some(horizon_mods::content::Rarity::Rare),
            Some("epic") => Some(horizon_mods::content::Rarity::Epic),
            _ => None,
        };
        items.push(horizon_mods::content::Item {
            id: def.id.to_string(),
            template: def.template.to_string(),
            max_stack_size: def.max_stack_size,
            rarity,
            components: std::collections::BTreeMap::new(),
            crafting_remainder: def.crafting_remainder.map(str::to_string),
            place_block: None,
        });
    }
}
