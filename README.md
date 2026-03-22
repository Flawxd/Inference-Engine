## Installation & Execution

Besoin d'avoir [Rust et Cargo](https://www.rust-lang.org/tools/install) installés sur votre système.

### Compiler le projet
```bash
cargo build
```

### Lancer le projet
```bash
cargo run
```

### Lancer les tests

Tous les tests :
```bash
cargo test
```

Tests d'un module spécifique :
```bash
cargo test parser
cargo test unification
cargo test forward
cargo test backward
```

### Demos visuelles

Afficher les résultats de tous les modules :
```bash
cargo test demo_ -- --nocapture --test-threads=1
```

Demo d'un seul module :
```bash
cargo test demo_parser -- --nocapture
cargo test demo_unification -- --nocapture
cargo test demo_forward -- --nocapture
cargo test demo_backward -- --nocapture
```
