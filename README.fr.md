# SoundSwap - Lecteur audio Windows

**Changez vos périphériques audio Windows (sortie et entrée) depuis SoundSwaper.**

SoundSwaper est une réécriture Rust avec Tauri 2 de l'application Windows d'origine.

## État actuel du portage Rust

- **Application Tauri 2** : point d'entrée et chargement du frontend configurés.
- **Configuration** : lecture et sauvegarde JSON implémentées.
- **Mises à jour** : récupération des métadonnées GitHub et vérification SHA-256 implémentées côté Rust ; téléchargement et installation non intégrés.
- **Détection audio Windows** : implémentée via l'API Core Audio/MMDevice. Le service énumère les périphériques actifs de sortie et d'entrée, récupère leur nom et leur identifiant Windows, et lit les périphériques par défaut.
- **Changement des périphériques par défaut** : implémenté via l'interface COM `IPolicyConfig`. Cette interface n'est pas documentée par Microsoft ; sa compatibilité doit donc être vérifiée sur des machines Windows physiques et sur les versions de Windows visées. Par défaut, SoundSwaper applique le changement aux rôles Console, Multimedia et Communications.
- **Intégration de l'audio dans l'application** : les fonctions Rust ne sont pas encore exposées en commandes Tauri ni reliées à l'interface.
- **Raccourcis globaux et zone de notification** : non implémentés.

Les fonctionnalités de l'ancienne application .NET ne sont donc pas encore toutes disponibles dans cette réécriture.

## Utilisation de l'API audio

Les identifiants Windows (`hardware_id`) sont des identifiants opaques et persistants. Il faut les conserver dans les configurations et les transmettre à `set_default_output_device` ou `set_default_input_device`. Ne pas utiliser le nom convivial comme identifiant : plusieurs périphériques peuvent porter le même nom.

Dans le code de l'application, privilégier les fonctions `try_*` qui renvoient les erreurs. Les anciennes fonctions de convenance sont conservées pour compatibilité ; en cas d'erreur Windows, elles renvoient une liste vide ou un périphérique marqué comme indisponible.

## Compilation

Prérequis : Windows 10/11, Rust stable, Microsoft C++ Build Tools, WebView2 et Tauri CLI 2.

```powershell
cargo install tauri-cli --version "^2" --locked
cargo tauri dev
cargo test
cargo tauri build
```

## Licence

[MIT](LICENSE)
