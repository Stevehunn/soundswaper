# Generate correct tauri.conf.json with SoundSwaper product name
import json

data = {
    'productName': 'SoundSwaper',
    'version': '1.0.0',
    'identifier': 'com.soundswaper.windows',
    'build': {'beforeBuildCommand': '', 'beforeDevCommand': ''},
    'bundle': {'active': True, 'targets': ['msi']},
    'windows': [{'title': 'SoundSwaper'}],
    'splashScreen': {
        'imageSrc': '/assets/banners/Sounswap-logo.jpg',
        'transparent': True,
        'fullScreen': True,
        'time': 5000
    },
    'updater': {'active': True}
}

with open('tauri.conf.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2)

print("✓ tauri.conf.json created successfully with SoundSwaper product name")
