# Intro
Notes

# To do
- [ ] migrate structures to new infrastructure
- [ ] design energy infrastructure for a station
- [ ] storage and production mechanics
- [ ] create consumables - from NPCs and in production
- [ ] component can_be_produced - link to recipe
- [ ] migrate recipes to new infrastructure and design recipes
- [ ] recipe design and usage
- [ ] find open database where production chains are defined
- [ ] orbital mechanics
- [ ] save/load World
- [ ] usage of prefabs?
- [ ] Network communication of object updates
- [ ] Client or web interface
- [ ] integrate asteroid database with mining information and potential to use up a object because of mining

# Design Decisions
Player 1=>n Station 1=>n modules 1=>n plugins

- objects in space - have position in space
  - Asteroids (comets are like asteroids with high eccentricity) - can land on it and extract resources - 3d grid with material which will be mined
    - stations on asteroids(later) - grid off asteroid replaced with station information's - parent of station then will be asteroid
    - Planets are like asteroids, but you can't land on them and extract resources but send material there.
  - Structures
    - stations (has no propulsion, only correction engines later on) - has modules
    - ships - has propulsion, has modules
      - Mining robots?
