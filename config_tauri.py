import json

data = {
    'productName': 'SoundSwaper',
    'version': '1.0.0',
    'identifier': 'com.soundswaper.windows',
    'build': {'beforeBuildCommand': '', 'beforeDevCommand': ''},
    'bundle': {'active': True, 'targets': ['msi']},
    'windows': [{'title': 'SoundSwaper', 'width': 600, 'height': 450}],
    'splashScreen': {
        'imageSrc': '/assets/banners/Sounswap-logo.png',
        'transparent': True,
        'fullScreen': True,
        'time': 5000
    },
    'updater': {
        'active': True,
        'endpoints': ['https://github.com/Stevehunn/soundswaper/releases/latest'],
        'followRedirects': True,
        'framework': 'default',
        'installPath': 'bundle'
    }
}

with open('tauri.conf.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2)

print("✓ tauri.conf.json created successfully with SoundSwaper product name")
