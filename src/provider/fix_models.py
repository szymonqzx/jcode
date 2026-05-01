#!/usr/bin/env python3
with open('models.rs', 'r') as f:
    content = f.read()

old = '    } else if cursor::is_known_model(model) {\n        Some("cursor")\n    } else {\n        None\n    }'
new = '    } else if cursor::is_known_model(model) {\n        Some("cursor")\n    } else if windsurf::is_known_model(model) {\n        Some("windsurf")\n    } else {\n        None\n    }'

content = content.replace(old, new)

with open('models.rs', 'w') as f:
    f.write(content)

print('Done')
