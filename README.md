# PTWAR

Persistent War game

Multiplayer Hex Tile War Game whe the player controls 1 or more squads of soldiers to conquer the map.

Fight for recources, build structures, logistics, and conquer the enemy.

Image

![game_image.png](game_image.png)

### How to run

```bash
# Server mode
cargon run --bin ptwar-server

# Client mode
cargon run --bin ptwar-ui
```

### What I want to achieve

Foxhole and HOI4 had a baby.

Want to see numbers go up (idle mode), and see the map change over time.

### TODO

- [x] Create a simple game loop
- [x] Soldier/Player bonuses system
- [ ] Structure System, factory, mine, etc
- [ ] Resource System
- [ ] Procedural Map Generation
- [ ] Battle System
- [ ] Map System
- [ ] Player System
- [ ] Logistic System
- [ ] Score System
- [ ] Persistence
- [ ] Render map for debug purpose
- [ ] Websocket for real-time communication
- [ ] Admin Api
- [ ] Player Api
- [ ] Auth System
- [ ] Cluster to enable multiple servers
    - Each Node will serve N numbers of Regions
- [ ] Dockerize
- [ ] CI/CD
- [ ] More configurations