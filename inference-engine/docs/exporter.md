# Module `exporter` — Inference Engine

Convertit une `KnowledgeBase` en texte Prolog-like, directement compatible
avec le `parser` existant du projet.

## Emplacement

| Fichier | Chemin |
|---|---|
| Module | `inference-engine/src/exporter.rs` |
| Tests  | `inference-engine/tests/exporter_tests.rs` |

## Build & tests

```bash
cargo build
cargo test exporter
cargo test demo_exporter -- --nocapture
```

## API publique

```rust
use inference_engine::exporter::{self, ExportFormat};

// Export compact (parse-ready, format par défaut)
let text = exporter::export_knowledge_base(&kb);

// Export avec format choisi
let text = exporter::export_knowledge_base_fmt(&kb, ExportFormat::Pretty);
let text = exporter::export_knowledge_base_fmt(&kb, ExportFormat::Prolog);

// Export vers fichier
exporter::export_to_file(&kb, "ruleset.pl", ExportFormat::Pretty)?;

// Export unitaire
let s = exporter::export_term(&term);
let s = exporter::export_fact(&fact);
let s = exporter::export_rule(&rule);
```

## Formats disponibles

| Format | Description | Usage |
|---|---|---|
| `Compact` | Une clause par ligne, sans commentaires | Re-parseable par `parser::parse()` |
| `Pretty` | Sections annotées, header avec stats | Affichage dans l'interface |
| `Prolog` | Header module Prolog standard | Compatibilité SWI-Prolog |

## Aller-retour garanti

Le format `Compact` est conçu pour que :
```
export_knowledge_base(&kb) → String → parser::parse(String) == kb
```
Ce cycle est testé dans `test_roundtrip_export_then_parse`.

## Intégration interface

Pour brancher l'exportateur dans l'interface graphique :

```rust
// Récupère le texte export pour l'afficher dans un widget
let preview: String = exporter::export_knowledge_base_fmt(&kb, ExportFormat::Pretty);

// Ou compact pour un champ de saisie ré-éditable
let editable: String = exporter::export_knowledge_base(&kb);

// Sauvegarde fichier sur action utilisateur
exporter::export_to_file(&kb, &chosen_path, ExportFormat::Compact)?;
```
