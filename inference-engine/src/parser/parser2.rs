use crate::types::*;

fn strip_article_fr<'a>(words: &'a [&'a str]) -> &'a [&'a str] {
    match words.first() {
        Some(&"un")  | Some(&"une") | Some(&"des") |
        Some(&"de")  | Some(&"du")  | Some(&"le")  |
        Some(&"la")  | Some(&"les") | Some(&"l")   |
        Some(&"tous") | Some(&"toutes") => {
            strip_article_fr(&words[1..])
        },
        _ => words,
    }
}

fn clean(word: &str) -> String {
    word.to_lowercase()
        .trim_end_matches(|c: char| !c.is_alphanumeric())
        .to_string()
}

fn join_words(words: &[&str]) -> String {
    words
        .iter()
        .map(|w| clean(w))
        .filter(|w| !w.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn compound1_var(functor: &str, var: &str) -> Term {
    Term::Compound {
        functor: functor.to_string(),
        args: vec![Term::Variable(var.to_string())],
    }
}

fn depluralize_fr(word: &str) -> String {
    match word {
        "animaux"    => return "animal".to_string(),
        "mammifères" => return "mammifère".to_string(),
        "oiseaux"    => return "oiseau".to_string(),
        "chevaux"    => return "cheval".to_string(),
        _ => {}
    }
    if word.ends_with("aux") {
        return format!("{}al", &word[..word.len() - 3]);
    }
    if word.ends_with('s') {
        return word[..word.len() - 1].to_string();
    }
    word.to_string()
}

fn parse_est_un(subject: &str, object: &str, kb: &mut KnowledgeBase) {
    kb.add_fact(Fact {
        term: Term::Compound {
            functor: object.to_string(),
            args: vec![Term::Atom(subject.to_string())],
        },
    });
}

fn parse_a(subject: &str, object: &str, kb: &mut KnowledgeBase) {
    kb.add_fact(Fact {
        term: Term::Compound {
            functor: object.to_string(),
            args: vec![Term::Atom(subject.to_string())],
        },
    });
}

fn parse_tous_sont(subject: &str, object: &str, kb: &mut KnowledgeBase) {
    parse_est_un(subject, object, kb);
    kb.add_rule(Rule {
        head: compound1_var(object, "X"),
        body: vec![compound1_var(subject, "X")],
    });
}

fn try_parse_relation_binaire(ws: &[&str], kb: &mut KnowledgeBase) -> bool {
    let Some(est_pos) = ws.iter().position(|w| *w == "est") else {
        return false;
    };
    let subject_words = strip_article_fr(&ws[..est_pos]);
    let after_est = strip_article_fr(&ws[est_pos + 1..]);

    let Some(de_pos) = after_est.iter().rposition(|w| *w == "de") else {
        return false;
    };
    let relation_words = &after_est[..de_pos];
    let object_words   = strip_article_fr(&after_est[de_pos + 1..]);

    if subject_words.is_empty() || relation_words.is_empty() || object_words.is_empty() {
        return false;
    }

    let subject  = join_words(subject_words);
    let relation = join_words(relation_words);
    let object   = join_words(object_words);

    kb.add_fact(Fact {
        term: Term::Compound {
            functor: relation,
            args: vec![Term::Atom(subject), Term::Atom(object)],
        },
    });
    true
}

fn parse_clause_condition<'a>(words: &[&'a str]) -> Option<Term> {
    if words.len() < 3 { return None; }
    let var = words[0].to_string();
    match words[1] {
        "est" => {
            let rest = strip_article_fr(&words[2..]);
            if rest.is_empty() { return None; }
            Some(Term::Compound {
                functor: join_words(rest),
                args: vec![Term::Variable(var)],
            })
        }
        "a" | "possède" => {
            let rest = strip_article_fr(&words[2..]);
            if rest.is_empty() { return None; }
            Some(Term::Compound {
                functor: join_words(rest),
                args: vec![Term::Variable(var)],
            })
        }
        _ => None,
    }
}

fn parse_si_alors(sentence: &str, kb: &mut KnowledgeBase) -> Result<(), String> {
    let lower = sentence.to_lowercase();
    let after_si = lower.trim_start_matches("si ");
    let parts: Vec<&str> = after_si.splitn(2, " alors ").collect();
    if parts.len() != 2 {
        return Err(format!("Phrase si/alors mal formée : {:?}", sentence));
    }
    let condition_str  = parts[0].trim();
    let conclusion_str = parts[1].trim().trim_end_matches('.');

    let body: Vec<Term> = condition_str
        .split(" et ")
        .filter_map(|clause| {
            let words: Vec<&str> = clause.split_whitespace().collect();
            parse_clause_condition(&words)
        })
        .collect();

    if body.is_empty() {
        return Err(format!("Condition non reconnue dans : {:?}", sentence));
    }

    let conc_words: Vec<&str> = conclusion_str.split_whitespace().collect();
    match parse_clause_condition(&conc_words) {
        Some(head) => { kb.add_rule(Rule { head, body }); Ok(()) }
        None => Err(format!("Conclusion non reconnue dans : {:?}", sentence)),
    }
}

fn parse_sentence_fr(sentence: &str, kb: &mut KnowledgeBase) -> Result<(), String> {
    let trimmed = sentence.trim();
    if trimmed.is_empty() { return Ok(()); }
    let lower = trimmed.to_lowercase();
    if lower.starts_with("si ") {
        return parse_si_alors(trimmed, kb);
    }
    let raw_words: Vec<&str> = trimmed.split_whitespace().collect();
    let words: Vec<String>   = raw_words.iter().map(|w| clean(w)).collect();
    let ws: Vec<&str>        = words.iter().map(String::as_str).collect();
    let is_plural_trigger = matches!(ws.first(), Some(&"tous") | Some(&"toutes") | Some(&"les") | Some(&"le") | Some(&"la"));
    if is_plural_trigger {
        if let Some(sont_pos) = ws.iter().position(|w| *w == "sont") {
            let subject_words = strip_article_fr(&ws[..sont_pos]);
            let object_words  = strip_article_fr(&ws[sont_pos + 1..]);
            if !subject_words.is_empty() && !object_words.is_empty() {
                let subject = depluralize_fr(&join_words(subject_words));
                let object  = depluralize_fr(&join_words(object_words));
                parse_tous_sont(&subject, &object, kb);
                return Ok(());
            }
        }
    }
    if try_parse_relation_binaire(&ws, kb) {
        return Ok(());
    }
    if let Some(est_pos) = ws.iter().position(|w| *w == "est") {
        let subject_words = strip_article_fr(&ws[..est_pos]);
        let object_words  = strip_article_fr(&ws[est_pos + 1..]);
        let subject = join_words(subject_words);
        let object  = join_words(object_words);
        if !subject.is_empty() && !object.is_empty() {
            parse_est_un(&subject, &object, kb);
            return Ok(());
        }
    }
    if let Some(has_pos) = ws.iter().position(|w| *w == "a" || *w == "possède") {
        let subject_words = strip_article_fr(&ws[..has_pos]);
        let object_words  = strip_article_fr(&ws[has_pos + 1..]);
        let subject = join_words(subject_words);
        let object  = join_words(object_words);
        if !subject.is_empty() && !object.is_empty() {
            parse_a(&subject, &object, kb);
            return Ok(());
        }
    }
    Err(format!("Phrase non reconnue : {:?}", trimmed))
}

pub fn parse_naturel_fr(input: &str) -> Result<KnowledgeBase, String> {
    let mut kb = KnowledgeBase::new();
    let mut errors: Vec<String> = Vec::new();
    for raw in input.split('.') {
        let sentence = raw.trim();
        if !sentence.is_empty() {
            if let Err(e) = parse_sentence_fr(sentence, &mut kb) {
                errors.push(e);
            }
        }
    }
    if errors.is_empty() {
        Ok(kb)
    } else {
        Err(errors.join("\n"))
    }
}

fn term_to_string(term: &Term) -> String {
    match term {
        Term::Atom(s)     => s.clone(),
        Term::Variable(s) => s.clone(),
        Term::Compound { functor, args } => {
            let args_str: Vec<_> = args.iter().map(term_to_string).collect();
            format!("{}({})", functor, args_str.join(", "))
        }
    }
}

fn main() {
    let input = "
        Un chat est un animal.
        Un chien est un animal.
        Une baleine est un animal.
        Un chat a de la fourrure.
        tom est un grand parent de ann.
        Un chien possède de la fourrure.
        les mammifères sont des animaux.
        Toutes les baleines sont des mammifères.
        Si X est un chat et X a de la fourrure alors X est un mammifère.
        Si X est un chien et X possède de la fourrure alors X est un mammifère.
        Si X est un mammifère alors X est un animal.
    ";

    match parse_naturel_fr(input) {
        Ok(kb) => {
            println!("=== Base de Connaissances ===\n");
            println!("Faits ({}) :", kb.facts.len());
            for fact in &kb.facts {
                println!("  {}", term_to_string(&fact.term));
            }
            println!("\nRègles ({}) :", kb.rules.len());
            for rule in &kb.rules {
                let body_str: Vec<_> = rule.body.iter().map(term_to_string).collect();
                println!("  {} :- {}", term_to_string(&rule.head), body_str.join(", "));
            }
        }
        Err(e) => eprintln!("Erreurs de parsing :\n{}", e),
    }
}