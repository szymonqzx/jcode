#!/usr/bin/env python3
with open('inline_interactive.rs', 'r') as f:
    content = f.read()

old = (
    '                    Some("cursor") => (\n'
    '                        "Cursor".to_string(),\n'
    '                        "cursor".to_string(),\n'
    '                        auth.cursor != crate::auth::AuthState::NotConfigured,\n'
    '                        String::new(),\n'
    '                    ),\n'
    '                    Some("openrouter") => ('
)

new = (
    '                    Some("cursor") => (\n'
    '                        "Cursor".to_string(),\n'
    '                        "cursor".to_string(),\n'
    '                        auth.cursor != crate::auth::AuthState::NotConfigured,\n'
    '                        String::new(),\n'
    '                    ),\n'
    '                    Some("windsurf") => (\n'
    '                        "Windsurf".to_string(),\n'
    '                        "windsurf".to_string(),\n'
    '                        auth.windsurf != crate::auth::AuthState::NotConfigured,\n'
    '                        String::new(),\n'
    '                    ),\n'
    '                    Some("openrouter") => ('
)

content = content.replace(old, new)

with open('inline_interactive.rs', 'w') as f:
    f.write(content)

print('Done')
