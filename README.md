# Episkos

## Dependencies for Rust recommendations
- serde (serde_toml)
- thiserror (errorhandling)
- sqlx (database)

##  Inkrement/Arbeitspaket Dokumente
- Inkrementübersicht
    - Wer macht was?
    - "Tracing"
- Arbeitspaketdokumente
    - Anforderungsbewertung
        - Bewertung von relevanten Requirements
    - Designpaper
        - Ziele aufstellen, wie werden diese umgesetzt?
    - Entwicklerdoku
- Inkrementreview

## Workflows
- Inkrement startet
    - Branches für Arbeitspakete "feat/..."
    - PullRequest Draft auf Alpha
- Commits auf Arbeitspaket branches
- Inkrement ende
    - Falls mit Code aus vergangenem Inkrement keine Fehler
    - alpha -> beta
- Review
    - Falls zufrieden: Merge mit alpha
    - Falls unzufrieden: Nächstes Inkrement 

- Wenn Basis Produkt steht: "Release" durch beta -> main

## GitHook for Conventional commits

execute ./init.sh to activate

## Documentation Links

### Svelte 5

https://svelte.dev/docs

### Tauri 2

https://tauri.app/start/

### shadcn-svelte

https://next.shadcn-svelte.com/
