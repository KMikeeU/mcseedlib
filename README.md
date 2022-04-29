# mcseedlib
an (incomplete) implementation of Minecraft World Generation in rust. Will be refactored later™

The main.rs is just a test for now

This should be a library, but frankly, I don't care

## Working Features
---
The following things have been implemented in at least some capacity

- End Spikes (Dragon Fight Pillar) generation
- The End Biome Source
  - includes creation of map images
- Slime Chunks

## TODO
---
This library is nowhere near finished. The following parts will be implemented next:

### End Structures (Cities)
Biomes which can contain end cities:
- Highlands
- Midlands

Uses RandomSpread generation with the following parameters:
```java
new RandomSpreadStructurePlacement(20, 11, SpreadType.TRIANGULAR, 10387313));
```

That's
```yaml
spacing: 20
separation: 11
spreadType: TRIANGULAR
salt: 10387313
```

### Overworld Biome Generation
- todo

### Stronghold positions
- todo

### Other Overworld structures
- todo

### Nether
- todo

## Source annotations
---
My list of possibly useful annotations I made in the Minecraft source code for future reference.

### MultiNoiseBiomeSource
```--seedlib multi noise biome source```
responsible for overworld and nether biome sources (for End: TheEndBiomeSource)

### Default settings for structures
```--seedlib configured structures```
contains settings for all structures in the game. Includes generator method / settings

### Biome tags
```--seedlib biome tags```
tags for biomes. Contains structure info like ``MINESHAFT_HAS_STRUCTURE`` or ``SHIPWRECK_BEACHED_HAS_STRUCTURE`` as well as general biome info like ``IS_JUNGLE``

### Structure sets
``--seedlib structuresets``
structure generation algorithms with parameters

### Chunk gen features
``--seedlib idk``
ChunkGenerator randomSpread with params

### Locate structure
``--seedlib locate structure``
locate structure in world