# 🐧 waylestiaDemo - Modern Desktop Shell

> Un shell desktop alternative à KDE Plasma & Hyprland, basé sur **Slint + Rust**, prototypé en WASM pour itération rapide, avec transition prévue vers un composeur Wayland natif.

## 🎯 Vision

Créer un **shell desktop léger, performant et hautement personnalisable** qui:
- ✨ Combine la beauté de Slint (UI moderne) avec la puissance de Rust
- 🚀 Prototype rapidement en WASM pour valider le design
- 🔄 Transition sans friction vers un vrai composeur Wayland (via Smithay)
- 🎮 Support du tiling + floating layouts
- ⌨️ Highly hackable avec scripting/plugins

## 📁 Structure du Projet

```
chatty/
├── ui/
│   └── app-window.slint      # Interface utilisateur Slint
├── src/
│   └── lib.rs               # Logique Rust (WASM entry point)
├── pkg/                     # WASM output (généré par wasm-pack)
├── index.html               # Web demo entry point
├── Cargo.toml              # Dépendances Rust
├── build.rs                # Build script Slint
├── DEMO.md                 # Architecture & usage WASM
├── SMITHAY_ROADMAP.md      # Plan migration native
└── README.md               # Ce fichier
```

## 🚀 Quick Start

### 1. Build WASM
```bash
cd /workspaces/chatty
wasm-pack build --release --target web
```

### 2. Lancer le serveur
```bash
# Python
python3 -m http.server 8080

# Ou Node
npx http-server -p 8080

# Ou deno
deno run --allow-net --allow-read https://deno.land/std/http/file_server.ts
```

### 3. Ouvrir dans le navigateur
```
http://localhost:8080
```

## 🎮 Interactions de Démo

| Élément | Action |
|---------|--------|
| **Workspace Buttons** (1-4) | Basculer workspaces |
| **Window Titles** | Visual focus feedback |
| **App Grid** | Layout demo (future func) |
| **TaskBar** | Window list (future func) |

## 🛠️ Développement

### Modifier l'UI

Éditer `ui/app-window.slint` et rebuild:
```bash
cargo build --lib --target wasm32-unknown-unknown
wasm-pack build --release --target web
```

### Hot Reload en Développement

Pour plus rapide feedback, utiliser le mode debug:
```bash
wasm-pack build --target web --dev
```

Puis utiliser un file watcher:
```bash
cargo watch -s 'wasm-pack build --target web --dev'
```

### Structure de Code Slint

```slint
// Composants système
component TopBar inherits Rectangle { /* ... */ }
component TaskBar inherits Rectangle { /* ... */ }
component AppLauncher inherits Rectangle { /* ... */ }
component VirtualWindow inherits Rectangle { /* ... */ }

// Composant principal
export component AppWindow inherits Window {
    in-out property <bool> show-launcher;
    in-out property <int> active-ws;
    
    // Layout...
}
```

### Ajouter des Fonctionnalités

**Exemple: Ajouter un bouton de settings**

1. Dans `ui/app-window.slint`:
```slint
TaskBar {
    Button { text: "⚙️ Settings"; }
}
```

2. Dans `src/lib.rs`:
```rust
ui.on_settings_clicked(|| {
    // Handle settings
});
```

3. Rebuild & test!

## 📊 Performance

### WASM Size
- **Release**: ~5MB (.wasm + .js)
- **Gzip**: ~1.5MB
- Load time: <2s on 4G

### Rendering FPS
- WASM: 60 FPS (canvas)
- Target (native): 144+ FPS (direct GPU)

### Memory
- WASM runtime: ~50MB
- Native target: <100MB

## 🎨 Theming

Tous les colors utilisent la palette **Catppuccin Mocha**:

```slint
// Background colors
#11111b - Base
#1a1a2e - Darker
#1e1e2e - Dark
#313244 - Even darker

// Text colors
#cdd6f4 - Text
#a6adc8 - Subtext
#89b4fa - Accent (blue)
#f38ba8 - Error (red)
```

Pour changer le thème, éditer les hex colors dans `app-window.slint`.

## 🐛 Debugging

### Console Browser
```javascript
// Dans le browser console:
cliquer droit → Inspecter → Console
```

### Rust Panic Logs
```bash
RUST_BACKTRACE=1 wasm-pack build --target web
```

### Slint Debugging
```rust
// Dans lib.rs
slint::debug_log!("Variable: {:?}", value);
```

## 🔄 Transition vers Native

Voir [SMITHAY_ROADMAP.md](./SMITHAY_ROADMAP.md) pour le plan complet.

Quick summary:
1. ✅ **Phase 1**: Valider UI en WASM (current)
2. 🔵 **Phase 2**: Créer backend Smithay
3. 🟣 **Phase 3**: Intégrer et optimiser

## 📦 Dépendances Clés

```toml
[dependencies]
slint = "1.14"        # UI framework
wasm-bindgen = "0.2"  # WASM bindings

[build-dependencies]
slint-build = "1.14"  # Compile Slint
```

Pour native (futur):
```toml
smithay = "0.3"       # Wayland compositor
```

## 🤝 Contributing

### Avant de commiter
1. Tester les builds WASM: `wasm-pack build --release --target web`
2. Vérifier pas d'erreurs Slint/Rust
3. Documenter changements majeurs

### Workflow recommandé
```bash
# 1. Créer branche
git checkout -b feature/mon-feature

# 2. Modifier code
# 3. Test WASM
wasm-pack build --target web

# 4. Commit & push
git commit -m "feat: description"
```

## 📚 Ressources

### Apprendre Slint
- [Official Tutorial](https://slint.dev/docs)
- [Widget Gallery](https://slint.dev/demos/gallery)
- [GitHub Repo](https://github.com/slint-ui/slint)

### Apprendre Rust
- [The Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [Interactive Tour](https://tourofrust.com/)

### WASM Development
- [MDN Web Docs](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [wasm-pack Book](https://rustwasm.github.io/docs/wasm-pack/)

## 🎯 Roadmap

### Court terme (2-4 semaines)
- [ ] Améliorer animations UI
- [ ] Ajouter plus d'apps au launcher
- [ ] Drag & drop windows
- [ ] Keyboard shortcuts

### Moyen terme (1-2 mois)
- [ ] Créer backend Smithay
- [ ] Multi-monitor support
- [ ] Persistent workspace state
- [ ] Custom keybind config

### Long terme (3+ mois)
- [ ] Full tiling layouts
- [ ] Plugin system
- [ ] System integration (audio, network)
- [ ] Mobile/tablet support

## 📄 License

MIT (à définir)

## 👨‍💻 Author

**aHIPPOz** - 2026

---

## ⚡ TL;DR

```bash
# Setup (une fois)
wasm-pack build --release --target web
python3 -m http.server 8080

# Ouvrir browser
http://localhost:8080

# Modifier UI
# → Edit ui/app-window.slint
# → Rebuild: wasm-pack build --target web
# → Refresh browser

# Aller natif (bientôt)
# → Voir SMITHAY_ROADMAP.md
```
Fonctionnalité Description Avancement
Options avancées pour tout Tout ce qui existe (tiling, snap, etc.) aura ses options avancées (séparation, gap personnalisable, etc.) 🔜 Bientôt

🤝 Contributing

Avant de commiter

1. Tester les builds WASM: wasm-pack build --release --target web
2. Vérifier pas d'erreurs Slint/Rust
3. Documenter changements majeurs

Workflow recommandé

```bash
# 1. Créer branche
git checkout -b feature/mon-feature

# 2. Modifier code
# 3. Test WASM
wasm-pack build --target web

# 4. Commit & push
git commit -m "feat: description"
```

📚 Ressources

Apprendre Slint

· Official Tutorial
· Widget Gallery
· GitHub Repo

Apprendre Rust

· The Book
· Rust By Example
· Interactive Tour

WASM Development

· MDN Web Docs
· wasm-pack Book

🎯 Roadmap

Court terme (2-4 semaines)

· Améliorer animations UI
· Ajouter plus d'apps au launcher
· Drag & drop windows
· Keyboard shortcuts
· Commencer l'installateur graphique
· Implémenter le mode tiling simple

Moyen terme (1-2 mois)

· Créer backend Smithay
· Multi-monitor support
· Persistent workspace state
· Custom keybind config
· Intégration Wallpaper Engine
· Configurateur graphique (première version)
· Support Hyprland + Wayfire

Long terme (3+ mois)

· Full tiling layouts
· Plugin system
· System integration (audio, network)
· Mobile/tablet support
· Chat IA intégré
· Personnalisation infinie (IA)


👨‍💻 Author

aHIPPOz - 2026
A2ER7Y - 2026 
PYRROX - 2026

**Questions?** Consultez DEMO.md et SMITHAY_ROADMAP.md! 🚀

---

**Status**: 🟢 WASM Prototype Working | Ready for UI Iteration

## Usage

1. Install Rust by following its [getting-started guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your `PATH`.
2. Download and extract the [ZIP archive of this repository](https://github.com/slint-ui/slint-rust-template/archive/refs/heads/main.zip).
3. Rename the extracted directory and change into it:
    ```
    mv slint-rust-template-main my-project
    cd my-project    
    ```
4. Build with `cargo`:
    ```
    cargo build
    ```
5. Run the application binary:
    ```
    cargo run
    ```

We recommend using an IDE for development, along with our [LSP-based IDE integration for `.slint` files](https://github.com/slint-ui/slint/blob/master/tools/lsp/README.md). You can also load this project directly in [Visual Studio Code](https://code.visualstudio.com) and install our [Slint extension](https://marketplace.visualstudio.com/items?itemName=Slint.slint).

## Next Steps

We hope that this template helps you get started, and that you enjoy exploring making user interfaces with Slint. To learn more
about the Slint APIs and the `.slint` markup language, check out our [online documentation](https://slint.dev/docs).

Don't forget to edit this readme to replace it by yours, and edit the `name =` field in `Cargo.toml` to match the name of your
project.
