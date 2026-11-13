fn projecte_recipes(sources: &mut Vec<(String, serde_json::Value)>) {
    for (id, raw) in crate::projecte::RECIPES {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(raw) {
            sources.push((id.to_string(), v));
        }
    }
}
