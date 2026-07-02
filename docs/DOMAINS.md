# CID Knowledge Domains - Greek Letter System

## 12 Specialized Knowledge Domains

| # | Letter | Domain Name | Focus Area | Example Facts |
|---|--------|-------------|------------|---------------|
| 1 | **Alpha (Α, α)** | Mathematics & Logic | Pure math, proofs, logic, algorithms | pi, e, sqrt2, theorem validation |
| 2 | **Beta (Β, β)** | Physics & Chemistry | Physical laws, elements, reactions | c, g, G, h, atomic masses |
| 3 | **Gamma (Γ, γ)** | Astronomy & Cosmology | Stars, planets, universe, space | au, light_year, parsec, galaxy data |
| 4 | **Delta (Δ, δ)** | Earth & Environment | Geography, climate, geology | earth_mass, ocean_area, river lengths |
| 5 | **Epsilon (Ε, ε)** | Biology & Medicine | Life sciences, health, anatomy | human_brain_neurons, DNA, species |
| 6 | **Zeta (Ζ, z)** | Computer Science & AI | Computing, ML, algorithms | GPU specs, model parameters, benchmarks |
| 7 | **Eta (Η, η)** | Engineering & Tech | Applied science, inventions | building heights, canal lengths |
| 8 | **Theta (Θ, θ)** | Economics & Finance | Markets, money, trade | GDP, stock markets, currencies |
| 9 | **Iota (Ι, ι)** | History & Anthropology | Civilizations, evolution, culture | empire dates, human evolution |
| 10 | **Kappa (Κ, κ)** | Language & Linguistics | Words, grammar, communication | language families, etymology |
| 11 | **Lambda (Λ, λ)** | Philosophy & Ethics | Thought, morality, logic | philosophical concepts, ethical systems |
| 12 | **Mu (Μ, μ)** | Psychology & Neuroscience | Mind, behavior, cognition | brain studies, cognitive biases |

## Domain Boundaries

### Alpha (Mathematics & Logic)
- **Includes**: Arithmetic, algebra, geometry, calculus, number theory, logic, proofs
- **Excludes**: Applied math in other domains (e.g., physics equations → Beta)
- **Validation**: Mathematical expressions, theorem statements, proof structures

### Beta (Physics & Chemistry)
- **Includes**: Mechanics, electromagnetism, thermodynamics, quantum mechanics, chemistry
- **Excludes**: Astronomy (→ Gamma), Earth science (→ Delta)
- **Validation**: Physical constants, unit conversions, chemical formulas

### Gamma (Astronomy & Cosmology)
- **Includes**: Stars, planets, galaxies,宇宙 phenomena, space missions
- **Excludes**: Physics laws (→ Beta), Earth (→ Delta)
- **Validation**: Astronomical distances, celestial body properties

### Delta (Earth & Environment)
- **Includes**: Geography, geology, climate, oceans, weather, environment
- **Excludes**: Biology (→ Epsilon), human populations (→ Iota)
- **Validation**: Geographic measurements, climate data

### Epsilon (Biology & Medicine)
- **Includes**: Genetics, anatomy, physiology, medicine, species, evolution
- **Excludes**: Human evolution history (→ Iota), neuroscience (→ Mu)
- **Validation**: Biological facts, medical data, species information

### Zeta (Computer Science & AI)
- **Includes**: Hardware specs, algorithms, ML models, software, networking
- **Excludes**: Math foundations (→ Alpha), engineering applications (→ Eta)
- **Validation**: Technical specifications, benchmark scores

### Eta (Engineering & Technology)
- **Includes**: Civil engineering, architecture, inventions, infrastructure
- **Excludes**: Computer tech (→ Zeta), basic physics (→ Beta)
- **Validation**: Structural measurements, engineering data

### Theta (Economics & Finance)
- **Includes**: GDP, markets, trade, currencies, financial instruments
- **Excludes**: Company revenues (→ Zeta for tech companies), history (→ Iota)
- **Validation**: Economic indicators, financial data

### Iota (History & Anthropology)
- **Includes**: Human evolution, civilizations, empires, cultural universals
- **Excludes**: Biology (→ Epsilon), philosophy (→ Lambda)
- **Validation**: Historical dates, anthropological facts

### Kappa (Language & Linguistics)
- **Includes**: Language families, grammar, translation, communication
- **Excludes**: Philosophy of language (→ Lambda), history of language (→ Iota)
- **Validation**: Linguistic facts, language data

### Lambda (Philosophy & Ethics)
- **Includes**: Logic philosophy, ethics, metaphysics, epistemology
- **Excludes**: Math logic (→ Alpha), psychology (→ Mu)
- **Validation**: Philosophical concepts, ethical principles

### Mu (Psychology & Neuroscience)
- **Includes**: Cognitive science, behavior, mental processes, brain studies
- **Excludes**: Biology (→ Epsilon), philosophy of mind (→ Lambda)
- **Validation**: Psychological facts, neuroscience data

## Cross-Domain Rules

1. **Primary Domain**: Each fact belongs to ONE primary domain
2. **Secondary Tags**: Facts can have secondary domain tags for cross-reference
3. **Validation Priority**: When validating, check primary domain first
4. **Conflict Resolution**: If fact appears in multiple domains, use primary domain value
5. **Domain Inheritance**: Child domains inherit parent domain validation rules

## Implementation Notes

- Domain stored as enum variant in Fact struct
- Index by domain for fast domain-specific queries
- Domain-specific validation rules in separate modules
- MCP tools can filter by domain
- CLI supports `--domain` flag for filtering
