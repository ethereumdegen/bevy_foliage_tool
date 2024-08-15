
### TODO 



- Allow the user to 'paint'  layers of grayscale (like heightmap) , one layer per TYPE of foliage .  They will indeed need to paint  DENSITY and HEIGHT 

- Use the noise crate to multiply their density painting by some 2d perlin noise .  Use this result and the heightmap  to deterministically spawn 'foliage' entities such as bushes / trees ... whatever that particular LAYER calls for. 