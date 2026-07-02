use super::facts::KnowledgeBase;

pub struct LookupResult {
    pub name: String,
    pub value: f64,
    pub unit: String,
    pub source: String,
    pub found: bool,
}

impl LookupResult {
    pub fn found(name: &str, value: f64, unit: &str, source: &str) -> Self {
        LookupResult {
            name: name.to_string(),
            value,
            unit: unit.to_string(),
            source: source.to_string(),
            found: true,
        }
    }

    pub fn not_found(name: &str) -> Self {
        LookupResult {
            name: name.to_string(),
            value: 0.0,
            unit: String::new(),
            source: String::new(),
            found: false,
        }
    }
}

pub fn lookup_fact(kb: &KnowledgeBase, name: &str) -> LookupResult {
    match kb.lookup(name) {
        Some(fact) => LookupResult::found(
            &fact.name,
            fact.value,
            &fact.unit,
            &fact.source,
        ),
        None => LookupResult::not_found(name),
    }
}

pub fn lookup_value(kb: &KnowledgeBase, name: &str) -> Option<f64> {
    kb.lookup_value(name)
}

pub fn fuzzy_lookup<'a>(kb: &'a KnowledgeBase, query: &str) -> Option<&'a super::facts::Fact> {
    let lower_query = query.to_lowercase();
    
    kb.facts.iter().find(|f| {
        f.name.to_lowercase().contains(&lower_query)
            || lower_query.contains(&f.name.to_lowercase())
    })
}

pub fn search_facts<'a>(kb: &'a KnowledgeBase, query: &str) -> Vec<&'a super::facts::Fact> {
    kb.search(query)
}
