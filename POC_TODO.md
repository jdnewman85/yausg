## Prototype todo

Maybe the tilemap should have functions to define child arrangement?


Offset for SvgPathShape in bevy_prototype_lyon is hard coded as size/2
  Great opensource PR opportunity?
    https://github.com/Nilirad/bevy_prototype_lyon/blob/2d4de13724b4f8b465feaa7c322cbfc0c61f9913/src/shapes.rs#L335
    Adding anchor offsets similar to what is allowed in sprites
    https://github.com/bevyengine/bevy/blob/5b0e6a53214277db5fe3276b297172f3ecc5f812/crates/bevy_sprite/src/sprite.rs#L30
      Or just respect the Sprite::Anchor component


If we decide to have variable sized tiles,
  Either sparse maps are needed
  Or long/wide tiles that take more space
  Or long/wide tiles constant for a column

Need to build out the tile type
  Display Info
    Path
    Size
    Connection points
    Labels
      Address
      Option<Tile func name>


To get good pixel svg results
  qcad
    Inches
    Center on origin
    Scale by 1/96
    Save
  inkscape
    Setup XML output/export preferences
      Such as precision, and abs/rel
    Setup document
      Pixels
      1/1 Scale
      Page size if wanted
    Import dxf
    Combine paths from layers as wanted
    Delete any groups
    Set offset and size for paths
    Align centers if wanted
    Explore XML and copy out svg path





Need room for labels


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
