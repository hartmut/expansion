# Notes

Version up until 0.5 will be developed in client mode and then split into server/client system in Version 1.

## To do

- [ ] usage of scenes for save/load of world
- [ ] modify config to be based on ron file - in progress
- [ ] create userinterface to create new modules etc.
- [ ] visualize stations and modules, transform
- [ ] migrate structures to new infrastructure - in progress
- [ ] usage of prefabs - in progress (bundles)
- [ ] orbital mechanics
- [ ] design energy infrastructure - in progress
- [ ] storage and production mechanics
- [ ] recipe design and usage, directory with one file per recipe, recipe database
- [ ] migrate recipes to new infrastructure and design recipes
- [ ] create consumables - from NPCs and in production
- [ ] find open database where production chains are defined
- [ ] Network communication of object updates
- [ ] Client or web interface
- [ ] integrate asteroid database with mining information and potential to use up a object because of mining

## Design Decisions

Player 1=>n Station 1=>n modules 1=>n plugins

- objects in space - have position in space
  - Asteroids (comets are like asteroids with high eccentricity) - can land on it and extract resources - 3d grid with material which will be mined
    - stations on asteroids(later) - grid off asteroid replaced with station information's - parent of station then will be asteroid
    - Planets are like asteroids, but you can't land on them and extract resources but send material there.
  - Structures
    - stations (has no propulsion, only correction engines later on) - has modules
    - ships - has propulsion, has modules
      - Mining robots?
