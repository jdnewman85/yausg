## PLC Interface

How do tiles know their surroundings?
  Store their position within? Ick...

How do tiles know their position?
How do we run different actions depending?
  Components can differentiate between a tile for a ladder and one for a tool?

Scroll Wheel - Change Tool

Tools
  Contacts
  Coils
  Wires

Syntax Highlighting
  Color by
    instruction
    data type
    data source
    value

Constant, possibly smaller left and right power rungs
Contact area
Right most usable column is coils

Placement of vertical and horizontal connections can infer the more complex wires
  Nope! Not entirely
  Can wires be inferred largely from contacts?

Fanuc style verticals that exist between tiles?
  With horizontals still being a regular tile

For now, even tiles are possible verticals?

Maybe work on single rung editing?

How should the user choose labels?
  Require them to be predefined in memory tables
  Only show correct type


Contact area is only contacts
  Contacts (2 simple)
    NO
    NC
  Wires (10)
    - |    _ _
    |_ _| |   |
    |- -| T _|_

More advanced PLC features
  Sequencers
  Timers
  Integer Ops
  Float Ops
  Vector Ops?

Try line drawing
May use bevy_prototype_lyon and svgs later


### Next Steps
- Add images for needed ladder symbols?
- Plan out more detailed layout
- Cursor?
- Grid lines?

Redefine UI to have our new line system?
Labels
IO

## Prototype todo
- [ ] Editor
  - [ ] Grid
    - [ ] Drawing
    - [ ] Click detection
  - [ ] Contacts
  - [ ] Coils
  - [ ] Connections/Wires
  - [ ] Real-time status display
  - [ ] Palette
  - [ ] Net generation
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
