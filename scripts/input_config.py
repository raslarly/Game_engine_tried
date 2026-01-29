"""
Phoenix Engine - Input Configuration Script

Configure input bindings using Python.
"""

def get_input_config():
    """Return input action and axis bindings."""
    
    return {
        "actions": {
            "jump": {
                "keys": [
                    {"key": "Space", "modifiers": {"ctrl": False, "shift": False, "alt": False}}
                ],
                "mouse_buttons": []
            },
            "attack": {
                "keys": [
                    {"key": "J", "modifiers": {"ctrl": False, "shift": False, "alt": False}}
                ],
                "mouse_buttons": [
                    {"button": "left"}
                ]
            },
            "interact": {
                "keys": [
                    {"key": "E", "modifiers": {"ctrl": False, "shift": False, "alt": False}}
                ],
                "mouse_buttons": []
            },
            "dash": {
                "keys": [
                    {"key": "K", "modifiers": {"ctrl": False, "shift": False, "alt": False}},
                    {"key": "Space", "modifiers": {"ctrl": False, "shift": True, "alt": False}}
                ],
                "mouse_buttons": []
            },
            "pause": {
                "keys": [
                    {"key": "Escape", "modifiers": {"ctrl": False, "shift": False, "alt": False}}
                ],
                "mouse_buttons": []
            },
            "inventory": {
                "keys": [
                    {"key": "I", "modifiers": {"ctrl": False, "shift": False, "alt": False}},
                    {"key": "Tab", "modifiers": {"ctrl": False, "shift": False, "alt": False}}
                ],
                "mouse_buttons": []
            },
            "use_item": {
                "keys": [
                    {"key": "Q", "modifiers": {"ctrl": False, "shift": False, "alt": False}}
                ],
                "mouse_buttons": [
                    {"button": "right"}
                ]
            }
        },
        
        "axes": {
            "horizontal": {
                "positive_keys": ["D", "ArrowRight"],
                "negative_keys": ["A", "ArrowLeft"],
                "sensitivity": 0.1,
                "gravity": 0.2
            },
            "vertical": {
                "positive_keys": ["W", "ArrowUp"],
                "negative_keys": ["S", "ArrowDown"],
                "sensitivity": 0.1,
                "gravity": 0.2
            }
        }
    }


if __name__ == "__main__":
    import json
    config = get_input_config()
    print(json.dumps(config, indent=2))
