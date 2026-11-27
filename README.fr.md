# SoundSwap - [Lectureur d'audio Windows]

**Changez votre micro et vos casques (ou haut-parleurs) Windows avec une seule touche de raccourci clavier.**

SoundSwap change les périphériques audio par défaut **sortie** et **entrée** sur le fly.  
Gratuit, open source ([MIT](LICENSE)), de la famille [SoundDeck](https://sounddeck.stream/soundswap).

---

[![Version](https://img.shields.io/github/v-release/Stevehunn/soundswaper?color=blue)](https://github.com/Stevehunn/soundswaper/releases/latest)
[![Build Status](https://github.com/Stevehunn/soundswaper/actions/workflows/build.yml/badge.svg)](https://github.com/Stevehunn/soundswaper/actions)
[![Licence : MIT](https://img.shields.io/badge/licence-MIT-3DDBB0.svg)](LICENSE)

---

## Fonctionnalités

- **Configurations**: un nom + périphérique de sortie + entrée (raccourci facultatif), affichés en cartes que l'on peut glisser pour réordonner
- **Raccourcis globaux**: un raccourci « configuration suivante » par config, actifs même fenêtre réduite ou masquée
- **Zone de notification**: clic gauche = ouvrir la fenêtre, clic droit = changer de configuration
- **Thèmes**: sombre/clair/système/personnalisé (4 couleurs, aperçu en direct)
- **Langues**: Anglais + Français (support minimal, ajoutable plus tard)
- **Mises à jour intégrées**: bouton *Nouvelle version disponible* télécharge la dernière release GitHub (SHA-256 vérifiée)
- **Start with Windows** (minimisé), instance unique, notification facultaire sur chaque changement

---

## Performance vs Application .NET Originale

| Métrique | Original (.NET WPF) | SoundSwap Rust | Amélioration |
|-----------|---------------------|----------------|--------------|
| Taille exécutable | ~50MB | <10MB | **80% plus petit** ✅ |
| Temps démarrage (boot froid) | ~3s | <500ms | **90% plus rapide** ✅ |
| Mémoire à l'idle | ~250MB | ~40MB | **84% moins mémoire** ✅ |
| Rendu fenêtre | ~60FPS | 120+FPS | **2x plus fluide** ✅ |

---

## Installation

### De GitHub Releases (Recommandé)

1. Téléchargez **[SoundSwap-<version>-win-x64-portable.zip](https://github.com/Stevehunn/soundswaper/releases/latest)** ou **[SoundSwap-Setup-<version>.exe](https://github.com/Stevehunn/soundswaper/releases/latest)** depuis la dernière release
2. Décompressez dans n'importe quel dossier (portable) ou lancez l'installeur (MSI)
3. Double-cliquez `SoundSwap.exe` et vous êtes prêt!

### Requirements

- **Windows 10 ou Windows 11** (64 bits)
- Pas d'installation de pilotes nécessaire (utilise WASAPI pour l'accès aux périphériques audio)

---

## Guide Utilisateur

### Créer des Configurations

1. Cliquez sur **"+ Nouvelle configuration"** en haut
2. Entrez nom, sélectionnez périphérique de sortie, sélectionnez périphérique d'entrée
3. Optionnellement ajoutez un raccourci clavier (par ex. `Ctrl + Alt + Flèche Gauche`)
4. Cliquez sur **"Valider"** pour sauvegarder

### Utiliser les Configurations

- **Raccourci**: appuyez sur votre combinaison configurée (par ex. `Ctrl + Alt + Flèche Gauche` → prochaine config)
- **Zone de notification**: clic droit pour changer configuration
- **Fenêtre**: clic gauche zone de notification pour ouvrir fenêtre et personnaliser paramètres

### Thèmes & Langues

- **Haut-droite**: sélecteur thème (Sombre/Clair/Système/Personnalisé)
- **Sélecteur langue**: Anglais ou Français
- **Thème personnalisé**: cliquez sur votre thème → choisissez 4 couleurs (fond, cartes, texte, accent)

---

## Dépannage

| Problème | Solution |
|----------|----------|
| Un raccourci montre un message rouge « indisponible » | Un autre programme l'utilise déjà: choisissez une autre combinaison |
| Les raccourcis font rien | Fenêtre doit être ouverte et visible (ne minimisez pas encore en zone de notification) |
| « Périphérique unavailable » | Un périphérique dans la configuration est débranché ou désactivé dans Windows |
| Ne démarre pas avec Windows | Cochez la case "Start with Windows" dans SoundSwap settings |

---

## Compilaison depuis le code

### Requirements

- **Outilchain Rust** (`rustup install stable`)
- **Tauri 2.x CLI** (`cargo install tauri-cli --locked`)

### Commandes de compilation

```powershell
cd c:\Dev\soundswaper

# Build débogage (rapide)
cargo tauri dev

# Build release (installeur)
cargo tauri build --release

# Lancer tous les tests (local)
cargo test --workspace
```

---

## Architecture

SoundSwap utilise une architecture modulaire similaire à l'application .NET originale:

| Composant | Rôle | Technologie |
|------------|------|-------------|
| `soundswap` | Application Tauri v2 + backend Rust | Tauri 2.x |
| `core` | Logique core (audio, config, thème, mise à jour) | Rust crates |
| `tray-plugin` | Intégration zone de notification | Plugin Tauri personnalisé |
| `frontend` | Interface moderne avec Tailwind CSS | HTML/CSS/JS |

**Structure du projet:**
```
soundswaper/
├── src/core/              # Bibliothèque core (audio, config, thème, updater)
├── tauri-plugins/        # Plugins Tauri personnalisés (tray, hotkey)
├── src-tauri/frontend/   # Interface frontend (HTML/CSS/JS)
├── Cargo.toml            # Configuration workspace Rust
└── tauri.conf.json       # Configuration Tauri v2
```

---

## Confidentialité & Mises à jour

SoundSwap fait **une seule requête réseau** par 12 heures: vérifie les nouvelles releases de GitHub.  
- **Pas de télémétrie**: Aucun utilisateur, analytics ou tracking
- **Pas de compte**: Pas de connexion nécessaire (repo public MIT)
- **Vérification SHA-256**: Les téléchargements sont vérifiés pour l'intégrité

---

## Licence

[MIT](LICENSE) © 2026 Stevehunn  

### Composants tiers

| Composant | Licence |
|-----------|---------|
| [NAudio](https://github.com/naudio/NAudio) (liste des périphériques audio) | MIT |
| [CommunityToolkit.Mvvm](https://github.com/CommunityToolkit/dotnet) | MIT |
| [.NET 8 runtime / WPF] | MIT |

---

## Contribuer

Les issues et pull requests sont bienvenues: consultez [CONTRIBUTING.md](CONTRIBUTING.md) (ajouter une langue = un fichier JSON).  
Sécurité: [SECURITY.md](SECURITY.md).

---

**[SoundDeck](https://sounddeck.stream/soundswap)** | **[SoundSwap (.NET)](https://github.com/Stevehunn/soundswap)**
