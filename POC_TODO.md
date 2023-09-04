# Prototype todo


BUGS
  Hover isn't removed on leaving tilemap
  Focus is removed on cursor move
  *Both* Probably related to them all setting stroke
    specifically, ladder_tile_unhighlight_system setting it back to black
    We should have only one system control any one output

Probably want the ability to have a below, above and local component for highlight/focus





One system should be responsible for setting things like style updates for tiles
  Multiple cause issues, fe: stroke/fill

MouseTilePosition, Cursor, Hover work
  - [X] Add component types
  - [X] Add spawn function
  - [-] Use in current mouse input system?
    - [X] Tried
    - [X] Bad idea, move to own system
        - [X] ... Could I just use the TilePosition component I'm calcing and adding to the TM?
        - [X] Maybe change TileCursorPosition to hold that value, and use changes in it to move the cursor
  - [X] Create if tile_cursor is on tilemap, and doesn't currently exist
    - [X] parent to tilemap for now
    - [X] create reference, or ref creation system
  - [X] If does exist, move
  - [X] If off tilemap, and does exist
    - [X] remove cursor
    - [ ] In-front layer and behind layer?

- [X] Better hover indicator
- Focus
  - on click
- Show extra controls while focused?
- Set contact type with keys while focused
- Set address by typing
  - Needs modal?
    - Or maybe leave it as invalid till fixed?


NamedRefMap?
  Component that has hashmap of string->entity references?
  Not sure if this, individual refs, or query of children with the marker components
  If too broad, adds too many queries to the system


Always ask, should it be a component?

Need effort in keeping display/action systems separate, yet synced properly

Current Tilemap mouse editing functionality should be based on tilemap+some other markers
  Want differing systems for a ladder editor view and a toolbar, which both may use tilemap

Decouple Tilemap graphical/positional data components from the data/logic
  So that we can have tilemaps that aren't for display at all
  Need a way to convert, or attach
    Maybe a function/system that generates and adds position/graphical info,
      and another to remove?
    OR - Conversion/Serialization?

Vec<Vec<Entity>> -> Img<Tile>?
  https://github.com/kornelski/imgref
  Might be useful for grid, probably less so with a more dynamic rung based approach



- [X] Define InputModule Component
  - [X] OutputModule Component
  - [X] PlcModule Component with static function to set outputs to inputs
  - [X] add a bit of logic for testing

- [X] Add TileLabel component
  Used with Text component, and translation
  - [X] Use this to make queries easier

- [ ] TileStyleSystem
    Updates tile graphics in response to system state
    Runs after other systems
  - [o] Mouseover Highlight
      - [ ] Background color change
      - [X] Style Color change
  - [ ] Focus
      Box around?
  - [ ] Text entry?
      Cursor?
  - [ ] Based on IO state
      Color of stroke

- [X] Add Parent/Child components to tilemap/tile queryseseieses
- [ ] Add a temporary address mapping hash thing
- [ ] When building a tile, use parent tilemap for
  - [ ] styling data
  - [ ] checking address mappings
- [ ] If mapping exists,
  - [ ] Check state of digital value
  - [ ]   style according to state
- [ ] Else, style accordingly
  - [ ] Possibly by colorizing both the tile, and the label
    - [ ] potentially with syntax like path highlighting
    - [ ] showing the unmatched erroneous portion

Try to generalize ladder display
  Take mouse events and produce set_tile, etc trait interface commands
    To/from il
    To/from graph
  Tile based editor
  Node based editor


Change to using graph structure for rung/ladder?
  Graph lib, w/ entities
    Bevy Parent/Child
      Reconstructed as needed for graphics, and events
  *Problem* Would be difficult for floating, temporarily disconnected portions
    would still need to store some positional info while editing anyway


Address Word - String
  Lazy binding?
    Attempt from string
  Once used for binding, keep entity?
    With maybe update w/ OnChange?
    Updates will always be known - ladder edits
  To
    Memory (through IO?)
    IO
    Execution environment
  Allow for multiple same word?
    Offset?
    Size?


InputModule Component
  Address?
  DigitalInputs
  AnalogInputs
  MediaInputs?
  UserInputs
  Parent?
OutputModule Component
  Address? Could this be it's own component?
  DigitalOutputs
  AnalogOutputs
  MediaOutputs
  UserOutputs
  Parent?
PlcModule Component
  InputModules
  OutputModules
  Program
  State

Choose address assignments for IO modules?
  Might be messy for sensor/actuators with low IO numbers?
    Maybe not, if those belong to their accompanying scopes?

Maybe name per object
  Scoped by parent->child relationship
    Maybe also still requiring some sort of mapping?
    Also handles conflicts?
OR just name per mapped object

and what addressing the address space within the object?
  maybe this is always index based?




Let's change to not creating tiles until needed?
  Per Rung?

Need selection and mouse over
Ability to enter address and type
Ability to draw connections

Must be able to be edited with no-valid info



How does the player input addresses and similar?
  Keyboard? Typing?
    Requires verification, typos
  Ability to select or drag drop from list or even the 3d scene?
    fe: drag from the sensor to create a contact?

Scoping


# Current Ideas
LadderMap will probably scroll, and so should probably also only display a portion at a time
  Decouple laddertile display tiles with components for such
    from list of tiles in a program
      Which is still in LD not IL!










## LadderTile buildout
  Display Info
    Path
    Size
    Connection points
    Labels
      Address
      Option<Tile func name>
  Need room for labels

  Address
  Label

    None,
    Element
      Contact,
        No
        Nc (Inverted?)
        Comparison
        Edges
      Coil,
        No
        Nc
        Set
        Reset
      Functions
    Connection/Line
      Horz,
      Vert,
      LeftDown,
      LeftUp,
      RightDown,
      RightUp,
      T000,
      T090,
      T180,
      T270,
      Cross,
    _Length,

Maybe the tilemap should have functions to define child arrangement?

## Moar

Selection Ideas
  Store in laddermap?
  And add as component?
  Add system that checks validity and removes if needed
  While also doing the highlighting itself maybe?


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



- [ ] Levels
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



# Hacks
## iter_many_mut
//HACK, iter_many_mut doesn't impl Iterator with mut, but instead FusedIter
//  Requiring fetch_next() when needing mut access, instead of next(), which
//  has a lifetime signature that isn't compatible with safety guarantees
//  See: https://github.com/bevyengine/bevy/blob/main/crates/bevy_ecs/src/query/iter.rs#L387
//
//  for mut label_text in label_query.iter_many_mut(children) {
let mut iter = label_query.iter_many_mut(children);
while let Some(mut label_text) = iter.fetch_next() {



# OLD
Should I be prefering bundles as return for spawn functions?
  *decision* atm, returning bundles makes children entities more difficult
    Staying with spawning functions for now
Think I should move away from childbuilder
  *did*
  Functions that return bundles
    *Ick* Children must be attatched at spawn time
  Functions that spawn and return ids
    *this* Since it makes children easier

Probably need my own label type and maybe sub types as components
  *yep, did*
  This would make addressing easier without having to crawl children
  Or alteast easier to select the correct child entity

Should I add a TilePosition component for position relative in the tilemap?
  *did*

Entity is versioned
  So let's store child entity references as needed
  *did*
  Much less overhead compared to parent->child
  May still be useful to use the parent->child relationship for transform/offset capability
  Maybe children that are simply groups?
    *haven't yet, still might*
    Would provide a group offset when useful
    With no offset, provides clean children grouping
      May be useful for things like collection of collision boxes

Components for easy querying of child stuff, such as labels?
  *yes*
  Label component would make use of the text component
  Queries would be for something with both label, text, and parent for needed data

Each tilemap should have it's own focus, highlight, etc?
  *NO! These are common components on the same entity!*
  If each keep their state, then simultaneous editing might be easier
  Matches with components better?

Should focus, and highlight be
  Component based
    *yes, this*
  Stored in tilemap/whatever, and calculated on change?
  Both somehow?

  If a component
    System can query these specifically
      For styling, and unstyling


## Editor - Tile editor
  Manages grid of contacts, coils, and connections
  Hide/Show in some way
  Detects clicks to set tiles

## *Problem* Atlas Bleeding
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


## Offset for SvgPathShape in bevy_prototype_lyon is hard coded as size/2
  Great opensource PR opportunity?
    https://github.com/Nilirad/bevy_prototype_lyon/blob/2d4de13724b4f8b465feaa7c322cbfc0c61f9913/src/shapes.rs#L335
    Adding anchor offsets similar to what is allowed in sprites
    https://github.com/bevyengine/bevy/blob/5b0e6a53214277db5fe3276b297172f3ecc5f812/crates/bevy_sprite/src/sprite.rs#L30
      Or just respect the Sprite::Anchor component
  *Solution?* Can adjust the path via lyon geom transform
    Use unit sized svg paths
    Scale/translate using lyon

## If we decide to have variable sized tiles,
  Either sparse maps are needed
  Or long/wide tiles that take more space
  Or long/wide tiles constant for a column

## To get good pixel svg results
  *Final solution* Unit scale, and scale via lyon dynamically
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

