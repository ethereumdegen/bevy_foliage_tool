
### TODO 


- be able to 'load' a foliage folder like default_foliage 
- when loaded, this spawns all the 16 chunks (configged from .ron )
- when those chunks spawn, they will spawn child entities for the foliage based on the density maps or whatever - use a grayscale bitmap (16 bit) ! The bit being there -> that foliage there 



- Allow the user to 'paint'  layers of grayscale (like heightmap) , one layer per TYPE of foliage .  They will indeed need to paint  DENSITY and HEIGHT 

- Use the noise crate to multiply their density painting by some 2d perlin noise .  Use this result and the heightmap  to deterministically spawn 'foliage' entities such as bushes / trees ... whatever that particular LAYER calls for. 



## interesting notes 


- this crate will NOT be responsible for actually spawning in the GLTFs for the foliage , it only spawns in BLANK ENTITIES with components based on perlin noise so then YOU can attach the gltfs to them yourself 