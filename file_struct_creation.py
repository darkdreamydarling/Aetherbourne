import os
    
files = {
    'Aetherbourne': {
        'src': {
            'main.rs': "",
            'app': {
                'mod.rs': "",
                'app.rs': "",
                'input.rs': "",
                'camera.rs': "",
                'time.rs': "",
                'selection.rs': "",
            },
            'simulation': {
                'mod.rs': "",
                'tick.rs': "",
                'pipeline.rs': "",
                'phases.rs': "",
                'sim_context.rs': "",
                'access.rs': "",
                'snapshot.rs': "",
                'debug.rs': "",
            },
            'world': {
                'mod.rs': "",
                'world.rs': "",
                'region.rs': "",
                'chunk.rs': "",
                'spatial_index.rs': "",
                'memory.rs': "",
                'queries.rs': "",
            },
            },
            'ecs': {
                'mod.rs': "",
                'world.rs': "",
                'components': {
                    'mod.rs': "",
                    'transform.rs': "",
                    'velocity.rs': "",
                    'energy.rs': "",
                    'metabolism.rs': "",
                    'health.rs': "",
                    'species.rs': "",
                    'reproduction.rs': "",
                    'needs.rs': "",
                    'memory.rs': "",
                    'perception.rs': "",
                    'brain.rs': "",
                },
                'systems': {
                    'mod.rs': "",
                    'spatial': {
                        'mod.rs': "",
                        'partition.rs': "",
                        'queries.rs': "",
                        'pathfinding.rs': "",
                    },
                    'environment': {
                        'mod.rs': "",
                        'weather.rs': "",
                        'climate.rs': "",
                        'biome.rs': "",
                    },
                    'geology': {
                        'mod.rs': "",
                        'minerals.rs': "",
                    },
                    'biology': {
                        'mod.rs': "",
                        'flora': {
                            'mod.rs': "",
                            'growth.rs': "",
                            'decay.rs': "",
                            'spread.rs': "",
                        },
                        'fauna': {
                            'mod.rs': "",
                            'movement.rs': "",
                            'combat.rs': "",
                            'consumption.rs': "",
                            'reproduction.rs': "",
                            'interaction.rs': "",
                            'metabolism.rs': "",
                            'reproduction.rs': "",
                            'movement.rs': "",
                        },
                        'ecosystem.rs': "",
                        'evolution.rs': "",
                    },
                    'cognition': {
                        'mod.rs': "",
                        'perception.rs': "",
                        'memory.rs': "",
                        'needs.rs': "",
                        'decision.rs': "",
                        'planning.rs': "",
                    },
                    'action': {
                        'mod.rs': "",
                        'movement.rs': "",
                        'consumption.rs': "",
                        'interaction.rs': "",
                        'combat.rs': "",
                    },
                    'interaction': {
                        'mod.rs': "",
                        'affordances': {
                            'mod.rs': "",
                        },
                        'rules': {
                            'mod.rs': "",
                        },
                        'resolution': {
                            'mod.rs': "",
                        },
                    },
                    'civilization': {
                        'mod.rs': "",
                        'settlement.rs': "",
                        'economy.rs': "",
                        'expansion.rs': "",
                        'ai': {
                            'mod.rs': "",
                        },
                        'logistics': {
                            'mod.rs': "",
                        },
                    },
                    'events': {
                        'mod.rs': "",
                        'event.rs': "",
                        'queue.rs': "",
                        'dispatch.rs': "",
                        'handlers': {
                            'mod.rs': "",
                            'environment.rs': "",
                            'biology.rs': "",
                            'civilization.rs': "",
                        },
                    },
                    'meta': {
                        'mod.rs': "",
                        'statistics.rs': "",
                        'debug.rs': "",
                        'danger.rs': "",
                        'resources.rs': "",
                    },
                },
            'generation': {
                'mod.rs': "",
                'noise.rs': "",
                'world_gen.rs': "",
                'biome_gen.rs': "",
                'mineral_gen.rs': "",
                'flora_gen.rs': "",
                'fauna_gen.rs': "",
            },
            'data': {
                'mod.rs': "",
                'loaders.rs': "",
                'registry.rs': "",
            },
            'rendering': {
                'mod.rs': "",
                'renderer.rs': "",
                'camera_view.rs': "",
                'snapshot.rs': "",
                'math.rs': "",
                'color.rs': "",
                'timing.rs': "",
            },
        },
    },
}

def create_structure(base_path, structure):
    for name, content in structure.items():
        path = os.path.join(base_path, name)

        if isinstance(content, dict):
            # It's a folder
            os.makedirs(path, exist_ok=True)
            create_structure(path, content)
        else:
            # It's a file
            with open(path, "w") as f:
                f.write(content)

# Run it
if __name__ == "__main__":
    create_structure(".", files)
    print("Project structure created!")