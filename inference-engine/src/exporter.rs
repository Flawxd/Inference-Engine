//! # Ruleset Exporter
//!
//! Convertit une `KnowledgeBase` en texte Prolog-like,
//! directement compatible avec le parser existant du projet.
//!
//! ## Utilisation rapide
//! ```rust
//! use inference_engine::exporter;
//! let output = exporter::export_knowledge_base(&kb);
//! ```
//!
//! ## Formats de sortie disponibles
//! - `ExportFormat::Compact`  → tout sur une ligne (parse-ready)
//! - `ExportFormat::Pretty`   → sections annotées, lisible humain
//! - `ExportFormat::Prolog`   → syntaxe Prolog standard (compatible SWI-Prolog)

use crate::types::{Fact, KnowledgeBase, Rule, Term};

// ─── Format d'export ─────────────────────────────────────────────────────────

/// Les trois formats d'export disponibles.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ExportFormat {
    /// Une ligne par clause, sans commentaires.
    /// → Directement parseable par `parser::parse()`.
    #[default]
    Compact,

    /// Sections commentées, faits et règles séparés, lisible.
    /// → Idéal pour affichage dans l'interface.
    Pretty,

    /// Syntaxe Prolog standard avec `:-` et `.`
    /// → Compatible SWI-Prolog / autres moteurs.
    Prolog,
}

// ─── Conversion des Terms ─────────────────────────────────────────────────────

/// Convertit un `Term` en chaîne Prolog.
///
/// - `Atom("chat")`             → `"chat"`
/// - `Variable("X")`           → `"X"`
/// - `Compound("parent", ...)`  → `"parent(alice, bob)"`
pub fn export_term(term: &Term) -> String {
    match term {
        Term::Atom(name)     => name.clone(),
        Term::Variable(name) => name.clone(),
        Term::Compound { functor, args } => {
            let args_str = args.iter()
                .map(export_term)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", functor, args_str)
        }
    }
}

/// Convertit un `Fact` en clause Prolog : `functor(args).`
pub fn export_fact(fact: &Fact) -> String {
    format!("{}.", export_term(&fact.term))
}

/// Convertit une `Rule` en clause Prolog : `head :- cond1, cond2.`
pub fn export_rule(rule: &Rule) -> String {
    let head = export_term(&rule.head);
    let body = rule.body.iter()
        .map(export_term)
        .collect::<Vec<_>>()
        .join(", ");
    format!("{} :- {}.", head, body)
}

// ─── Export principal ─────────────────────────────────────────────────────────

/// Exporte une `KnowledgeBase` dans le format spécifié.
///
/// # Exemple
/// ```rust
/// let text = export_knowledge_base_fmt(&kb, ExportFormat::Pretty);
/// ```
pub fn export_knowledge_base_fmt(kb: &KnowledgeBase, format: ExportFormat) -> String {
    match format {
        ExportFormat::Compact => export_compact(kb),
        ExportFormat::Pretty  => export_pretty(kb),
        ExportFormat::Prolog  => export_prolog(kb),
    }
}

/// Raccourci : export `Compact` (format par défaut, parse-ready).
///
/// Équivalent à `export_knowledge_base_fmt(kb, ExportFormat::Compact)`.
pub fn export_knowledge_base(kb: &KnowledgeBase) -> String {
    export_compact(kb)
}

// ─── Implémentations des formats ─────────────────────────────────────────────

/// Format compact : une clause par ligne, aucun commentaire.
/// Compatible direct avec `parser::parse()`.
fn export_compact(kb: &KnowledgeBase) -> String {
    let mut lines: Vec<String> = Vec::new();

    for fact in &kb.facts {
        lines.push(export_fact(fact));
    }
    for rule in &kb.rules {
        lines.push(export_rule(rule));
    }

    lines.join("\n")
}

/// Format lisible : sections annotées avec stats.
fn export_pretty(kb: &KnowledgeBase) -> String {
    let mut out = String::new();

    // En-tête
    out.push_str(&format!(
        "% ── Knowledge Base ────────────────────────────────────\n\
         % Facts : {}  |  Rules : {}  |  Total : {}\n\
         % ──────────────────────────────────────────────────────\n\n",
        kb.facts.len(),
        kb.rules.len(),
        kb.facts.len() + kb.rules.len(),
    ));

    // Section Faits
    if !kb.facts.is_empty() {
        out.push_str("% --- Facts ---\n");
        for fact in &kb.facts {
            out.push_str(&export_fact(fact));
            out.push('\n');
        }
        out.push('\n');
    }

    // Section Règles
    if !kb.rules.is_empty() {
        out.push_str("% --- Rules ---\n");
        for rule in &kb.rules {
            out.push_str(&export_rule(rule));
            out.push('\n');
        }
    }

    out
}

/// Format Prolog : même syntaxe que Compact mais avec header Prolog standard.
fn export_prolog(kb: &KnowledgeBase) -> String {
    let mut out = String::new();

    out.push_str(":- module(knowledge_base, []).\n\n");

    if !kb.facts.is_empty() {
        for fact in &kb.facts {
            out.push_str(&export_fact(fact));
            out.push('\n');
        }
        out.push('\n');
    }

    if !kb.rules.is_empty() {
        for rule in &kb.rules {
            out.push_str(&export_rule(rule));
            out.push('\n');
        }
    }

    out
}

// ─── Export vers fichier ──────────────────────────────────────────────────────

/// Écrit l'export dans un fichier.
///
/// # Exemple
/// ```rust
/// exporter::export_to_file(&kb, "ruleset.pl", ExportFormat::Pretty)?;
/// ```
pub fn export_to_file(
    kb: &KnowledgeBase,
    path: &str,
    format: ExportFormat,
) -> std::io::Result<()> {
    use std::fs;
    let content = export_knowledge_base_fmt(kb, format);
    fs::write(path, content)
}
