import json

with open('tauri.conf.json', 'w') as f:
    json.dump({
        'productName': {'name': 'SoundSwaper'},
        'version': '1.0.0',
        'identifier': 'com.soundswaper.windows',
        'build': {'beforeBuildCommand': '', 'beforeDevCommand': ''},
        'bundle': {'active': True, 'targets': ['msi']},
        'tauri': {
            'allowlist': {'all': False, 'clipboard': {'write': True}},
            'security': {'csp': "default-src 'self'"},
            'windows': [{'title': 'SoundSwaper', 'width': 600, 'height': 450, 'resizable': True, 'minimizable': True, 'startMinimized': False}],
            'splashScreen': {'imageSrc': '/assets/banners/sounswap-logo.png', 'transparent': True, 'fullScreen': True, 'time': 5000},
            'updater': {'active': True, 'endpoints': ['https://github.com/Stevehunn/soundswaper/releases/latest'], 'followRedirects': True, 'framework': 'default', 'installPath': 'bundle'}
        }
    }, f, indent=2)
