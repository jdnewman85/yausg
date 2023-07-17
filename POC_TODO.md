## Prototype todo


Selection Ideas
  Store in laddermap
  And add as component
  Add system that checks validity and removes if needed
  While also doing the highlighting itself maybe?

# *Problem* Atlas Bleeding
Can be helped with:
  .insert_resource(Msaa::Off)
  DefaultPlugins.set(ImagePlugin::default_nearest())
- However, this turns of msaa for our 3d scene also atm.
Other solutions involve a buffer pixel,
  but arean't ideal
## Current Decision
Switch to single image frames again for now
## Links
[Make Msaa a component not a resource.](https://github.com/bevyengine/bevy/pull/7215)
[Move Msaa to a component, instead of a resource](https://github.com/bevyengine/bevy/issues/7194)
[Pixel artifact between sprite, on specific camera position.](https://github.com/bevyengine/bevy/issues/4748)
[Shader to directly render to screen pixels](https://github.com/bevyengine/bevy/issues/1856)



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
