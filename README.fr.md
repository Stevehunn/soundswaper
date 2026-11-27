# SoundSwaper - Gestion des périphériques audio Windows

**Changez vos périphériques audio Windows (sortie et entrée) depuis SoundSwaper.**

SoundSwaper est une réécriture Rust avec Tauri 2 de l'application Windows d'origine.

## Prérequis de développement

- Windows 10 ou Windows 11
- Rust stable (rustup)
- Microsoft C++ Build Tools
- Runtime WebView2
- Tauri CLI 2

Installation du CLI :

```powershell
cargo install tauri-cli --version "^2" --locked
```

Lancer l'application depuis la racine du dépôt :

```powershell
cargo tauri dev
```

Compiler l'installateur :

```powershell
cargo tauri build
```

Lancer les tests :

```powershell
cargo test
```

## État actuel du portage Rust

- **Application Tauri 2** : point d'entrée et chargement du frontend configurés.
- **Configuration** : lecture et sauvegarde des paramètres JSON implémentées.
- **Mises à jour** : récupération des métadonnées GitHub et vérification SHA-256 implémentées côté Rust ; téléchargement et installation non intégrés.
- **Détection audio Windows** : énumération des périphériques actifs de sortie et d'entrée via Core Audio/MMDevice, avec nom, identifiant Windows et périphérique par défaut.
- **Changement des périphériques par défaut** : implémenté via l'interface COM `IPolicyConfig`, non documentée par Microsoft. Le changement s'applique aux rôles Console, Multimedia et Communications.
- **Interface Tauri** : les commandes audio sont reliées aux sélecteurs de périphériques d'entrée et de sortie.
- **Zone de notification** : menu permettant d'ouvrir SoundSwaper, d'actualiser les périphériques et de quitter l'application. Un clic gauche ouvre la fenêtre.
- **Profils audio persistants** : enregistrer, appliquer et supprimer des combinaisons de périphériques d'entrée et de sortie. Les profils sont stockés dans le répertoire de configuration utilisateur sous forme de JSON et réutilisent les identifiants Windows des périphériques.
- **Raccourcis clavier globaux** (lorsque l'application est lancée) :
  - `Ctrl + Alt + S` : afficher la fenêtre.
  - `Ctrl + Alt + O` : passer au périphérique de sortie actif suivant.
  - `Ctrl + Alt + I` : passer au périphérique d'entrée actif suivant.

## Identifiants des périphériques

Les identifiants Windows (`hardware_id`) sont opaques et persistants. Il faut les conserver dans les configurations et les transmettre à `set_default_output_device` ou `set_default_input_device`. Ne pas utiliser le nom convivial comme identifiant : plusieurs périphériques peuvent porter le même nom.

Dans le code de l'application, privilégier les fonctions `try_*` qui renvoient les erreurs. Les anciennes fonctions de convenance sont conservées pour compatibilité.

## Validation Windows

Le changement du périphérique par défaut utilise une interface Windows non documentée. Le comportement audio, les raccourcis et la zone de notification doivent donc encore être testés sur des machines Windows 10/11 physiques. La CI et les tests unitaires ne peuvent pas vérifier les interactions avec les pilotes audio et les autres applications.

## Licence

[MIT](LICENSE)
