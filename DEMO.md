# 🐧 Shell Desktop - WASM Demo

Une démo complète d'un shell desktop moderne écrit en **Slint + Rust**, compilé en **WASM+JS** pour prototypage rapide sur web.

## 🎯 Architecture

```
┌─────────────────────────────────────────────────┐
│  AppWindow (Slint Component - inherits Window)  │
├─────────────────────────────────────────────────┤
│  ┌──────────────────────────────────────────┐   │
│  │ TopBar (🐧 Shell Desktop | Status)       │   │
│  ├──────────────────────────────────────────┤   │
│  │  ┌─────────────┐  ┌──────────────────┐   │   │
│  │  │ Workspaces  │  │ Desktop Area     │   │   │
│  │  │ [1][2][3][4]│  │ ┌──────────────┐ │   │   |
│  │  └─────────────┘  │ │ VirtualWindow│ │   │   |
│  │                   │ │  (Terminal)  │ │   │   |
│  │                   │ ├──────────────┤ │   │   |
│  │                   │ │ VirtualWindow│ │   │   |
│  │                   │ │  (Files)     │ │   │   |
│  │                   │ └──────────────┘ │   │   |
│  │                   │ [AppLauncher:    │   │   |
│  │                   │  toggle-able]    │   │   |
│  │                   └──────────────────┘   │   │
│  ├──────────────────────────────────────────┤   │
│  │ TaskBar (🖥️ Terminal | 📁 Files |...)    │   │
│  └──────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
```

## 📦 Stack Technologique

- **Frontend UI**: Slint 1.14+ (moderne, performant, réactif)
- **Logique**: Rust (sécurité mémoire, performance)
- **Compilation Web**: wasm-pack → WASM + JavaScript
- **Couleurs**: Catppuccin Mocha (beau théme sombre)

## 🎨 Composants Slint

### `TopBar`
- Affiche le titre du shell
- Indicateurs système: heure, volume, batterie

### `TaskBar`
- Liste des fenêtres ouvertes
- Boutons rapides pour basculer entre apps

### `VirtualWindow`
- Fenêtres simulées avec barre de titre
- Support focus/blur avec changement de couleur
- Bouton fermer (×)

### `AppLauncher`
- Grille d'applications
- Affichable/masquable
- 8 apps de démo avec émojis

### `WorkspaceButton` / WorkspaceIndicator
- Sélection multi-workspace (4 espaces de travail)
- Affichage du workspace actuel

## 🚀 Lancer la Démo

### 1. Build WASM
```bash
cd /workspaces/chatty
wasm-pack build --release --target web
```

### 2. Servir statiquement
```bash
python3 -m http.server 8000
# ou
npx http-server
```

### 3. Ouvrir le navigateur
```
http://localhost:8000
```

## 🤝 Interactions Disponibles

| Élément | Action | Résultat |
|---------|--------|---------|
| Workspace Buttons (1-4) | Click | Change workspace actif |
| TaskBar Apps | Click | (Dèmo visuelle pour futur) |
| AppLauncher Apps | Toggle via affichage | Show/hide launcher |
| Windows | Click titre | Focus visual update |
| Window Close (×) | Click | (Hook pour futur) |

## 🔄 Transition Native (Roadmap)

### Phase 1: Prototype Web ✅
- ✅ UI au pixel près en Slint
- ✅ Compilation WASM fonctionnelle
- ✅ Interactions de base

### Phase 2: Backend Wayland (Smithay)
- Port de la logique Rust → serveur Smithay
- RemoteView WASM → Smithay native IPC
- Multi-monitor support

### Phase 3: Full Desktop
- Compositing/rendering natif
- Gestion complète des fenêtres
- Workspaces, tiling layouts, etc.

## 📊 Structure du Projet

```
chatty/
├── src/
│   └── lib.rs          # Logique Rust/WASM
├── ui/
│   └── app-window.slint # UI complète
├── pkg/               # Output WASM (généré)
│   ├── slint_rust_template.wasm
│   ├── slint_rust_template.js
│   └── ...
├── index.html         # Entry point web
├── Cargo.toml        # Dépendances Rust
└── build.rs          # Slint compilation
```

## 🎯 Prochaines Étapes

1. **Améliorer l'UI**:
   - Animations de fenêtres
   - Drag & drop
   - Fullscreen mode

2. **Logique avancée**:
   - Gestion d'état réelle des fenêtres
   - Persistance workspace
   - Shortcuts clavier

3. **Intégration Smithay**:
   - Wayland protocol support
   - Native window managing
   - Tiling layouts

## 💡 Avantages de cette Approche

✅ **Prototypage rapide**: Web = feedback instant  
✅ **Pixel-perfect UI**: Slint permet précision totale  
✅ **Pas de redémarrage**: Hot reload possible  
✅ **Performance UI**: Rust + hardware acceleration  
✅ **Facile migration**: Code logic reste identique en native  

## 📝 Notes

- Le fichier `lib.rs` gère les callbacks Slint
- Le `app-window.slint` contient toute la structure visuelle
- WASM package (5MB) peut être optimisé avec  `wasm-opt`
- Catppuccin colors pour cohérence design

---

**Status**: 🟢 WASM Build Successful | Ready for Web Prototyping
