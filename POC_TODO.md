## Prototype todo

LadderTileMap
  Atlas
  Parent
  SpriteBundle
    Transform
    Texture
LadderTile

click_system notes
  Only the tilemaps need to check
  Per-tile collision check
    (ClickPosition - TilePosition) < TileSize


- [ ] Editor
  - [ ] Grid
    - [ ] Drawing
    - [ ] Click detection
  - [ ] Contacts
  - [ ] Coils
  - [ ] Connections
  - [ ] Realtime status display
- [ ] Ld2Il Compiler
  - [ ] Generate IL
- [ ] IL Executor
  - [ ] IO
  - [ ] Simple static function
  - [ ] Simple Contact, Coil, and Connections
- [ ] Machine
  - [ ] Predefined
  - [ ] Output actuators
  - [ ] Input sensors




- [ ] Simple momentary circuit for lights
- [ ] Actuator puzzle should be impossible without the light, but easy with

- [ ] Force latching
- [ ] Add sensors that can be used

- [ ] Force completion with help of sensors
  - [ ] Possible reasons
    - [ ] Sensor senses more than camera
    - [ ] Reaction time
    - [ ] User input limit, such as needing their outputs for something else
- [ ] Force completion autonomously




Editor - Tile editor
  Manages grid of contacts, coils, and connections
  Hide/Show in some way
  Detects clicks to set tiles
