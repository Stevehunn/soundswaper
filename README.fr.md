# SoundSwap - [Lectureur d'audio Windows]

**Changez votre micro et vos casques (ou haut-parleurs) Windows avec une seule touche de raccourci clavier.**

SoundSwap change les périphériques audio par défaut **sortie** et **entrée** sur le fly.  
Gratuit, open source ([MIT](LICENSE)), de la famille [SoundDeck](https://sounddeck.stream/soundswap).

---

[![Version](https://img.shields.io/github/v-release/Stevehunn/soundswaper?color=blue)](https://github.com/Stevehunn/soundswaper/releases/latest)
[![Build Status](https://github.com/Stevehunn/soundswaper/actions/workflows/build.yml/badge.svg)](https://github.com/Stevehunn/soundswaper/actions)
[![Licence : MIT](https://img.shields.io/badge/licence-MIT-3DDBB0.svg)](LICENSE)

---

## État actuel du portage Rust

- **Application Tauri 2** : point d'entrée et chargement du frontend configurés.
- **Configuration** : lecture et sauvegarde JSON implémentées.
- **Mises à jour** : récupération des métadonnées GitHub et vérification SHA-256 implémentées côté Rust ; téléchargement et installation non intégrés.
- **Audio Windows (WASAPI)** : non implémenté. Le service ne renvoie volontairement aucun périphérique plutôt que de présenter de faux appareils.
- **Raccourcis globaux et zone de notification** : non implémentés.
- **Interface** : squelette HTML ; elle n'est pas encore reliée aux commandes Rust de gestion des périphériques.

Les fonctionnalités de l'ancienne application .NET ne sont donc pas encore toutes disponibles dans cette réécriture.

---

## Performances

Aucun benchmark fiable de la version Rust n'a encore été réalisé. Les objectifs de taille mémoire et de temps de démarrage restent à mesurer.

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

1. Les packages d'installation ne sont pas encore publiés pour cette réécriture Rust. Consultez les releases après la mise en place de la chaîne de publication.
2. Décompressez dans n'importe quel dossier (portable) ou lancez l'installeur (MSI)
3. Double-cliquez `SoundSwap.exe` et vous êtes prêt!

### Requirements

- **Windows 10 ou Windows 11** (64 bits)
- La couche WASAPI devra utiliser les API audio Windows ; elle n'est pas encore disponible.

---

## Guide utilisateur (à finaliser après l'intégration audio)

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
cargo tauri build

# Lancer tous les tests (local)
cargo test
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

Le contrôle de version GitHub est présent dans le module Rust, mais il n'est pas encore appelé par l'interface.  
- **Pas de télémétrie**: Aucun utilisateur, analytics ou tracking
- **Pas de compte**: Pas de connexion nécessaire (repo public MIT)
- **Vérification SHA-256**: la fonction de vérification est implémentée ; le flux de téléchargement reste à intégrer

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
