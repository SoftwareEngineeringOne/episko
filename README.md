![backend_lints](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DefinitelyNotSimon13/6576287f91ca84ec0583a5ca2d5ec495/raw/lints_backend.json)
![backend_tests](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DefinitelyNotSimon13/6576287f91ca84ec0583a5ca2d5ec495/raw/tests_backend.json)
![backend_coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DefinitelyNotSimon13/6576287f91ca84ec0583a5ca2d5ec495/raw/coverage_backend.json)


![frontend_lints](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DefinitelyNotSimon13/6576287f91ca84ec0583a5ca2d5ec495/raw/lints_frontend.json)
![frontend_tests](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DefinitelyNotSimon13/6576287f91ca84ec0583a5ca2d5ec495/raw/tests_frontend.json)
![frontend_coverage](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/DefinitelyNotSimon13/6576287f91ca84ec0583a5ca2d5ec495/raw/coverage_frontend.json)

# Episko
>[!WARNING]
>TODO: Update README.md

## Dependencies for Rust recommendations

- serde (serde_toml)
- thiserror (errorhandling)
- sqlx (database)

## Inkrement/Arbeitspaket Dokumente

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
