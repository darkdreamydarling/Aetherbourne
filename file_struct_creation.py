import os
    
f = {
    'src' : {
        'app', 'simulation', 'world', 'ecs': {
            'components', 'systems': {
                'spatial', 'environment', 'geology', 'biology': {
                    'flora', 'fauna'
                    }
                }, 'cognition', 'action', 'interaction': {
                    'affordances', 'rules', 'resolution'
                }, 'civilization': {
                    'ai', 'logistics'
                }, 'events': {
                    'handlers'
                }, 'meta'
            },
        'generation', 'data', 'rendering': {
            'interaction': {
                'rules', 'resolution' 'affordances'
            }, 'civilization': {
                'ai', 'logistics'
            }, 'events': {
                'handlers'
            }
        }
    }
}

 
def create_file_structure():
    # Create the directories
    for directory in directories:
        os.makedirs(directory, exist_ok=True)