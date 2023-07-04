***Current Problem***


# Controls
[o] Camera Controls
  [X] Basic "god mode" fps, wsad movement with mouse turning
  [X] Basic orbital with distance and target
[o] Physics FPS
  [ ] Trigger volumes
  [o] Built in rapier character controller
    [o] Gravity
    [X] Floor/level collision
    [o] Jump
  [ ] *Decision:* Own, based completely on physically placed cameras
    [ ] Define our own camera location component
        Attaches to a main-system chassis or something?
    [ ] Requires being attached/wired in/setup?
[o] Physics Vehicle
  [o] Spawn func
    [x] Define Base
    [x] Define Wheels
    [x] Attach wheels to base with joints
    [x] Set friction/settings
    [ ] Disallow self collision?
  [X] Use motor to turn wheels
    [X] Throttle from user input
    [X] Reverse
  [ ] Add additional rotational axis to wheel joints
    [X] Add axels inbetween vehicle and wheels
    [ ] Ability to rotate
    [ ] Move based on user input
    [ ] Limits
    [ ] Auto-centering?
  [o] Add camera controller
    [ ] Fixed
    [x] Orbital
    [ ] Ability to switch between?
      [ ] And to free-look?
  [!] Parent->Children in bevy?
    Seemed to not work when combined with rapier?
      Like forces for the child where being propagated to the parent
      Interactions that shouldn't have happened



# Puzzle ideas
Momentary contacts
Maintained contacts


# Logic Systems
  [ ] Simple DEBUG
    Hardcoded, or simple config based I->O
      Can simply choose camera from available?
  [ ] IL VM
    Run il program with inputs, set outputs
  [ ] LD Compiler
  [ ] LD VM?

Logic system
  Ladder
  Components
Game States
  Playing(Level)
  GameOver
  Menu


# Misc Ideas

Roguelike where each "life" is limited in time
  Choices are sometimes exclusive
  Not neccessarily linear


Self-modification module
  Allows wiring and configuration of modules on self


# Apocalyptic remains as resources
- Stockpiled fuel in bunkers
- Plastic ores
- Rare metal, in shapes of cars, skyscrapers, machines, guns
- Lead, steel, brass from battlefields
- Stone and cement from decayed cities
- Radioactive material from open and filled areas
  - Filled are old containment remains
  - Open are old landing sites
- Last of wood
  - As all forests continue to die, we harvested the remaining in a frantic gold rush


# Level Ideas
Simple key input momentary switch circuit
  Lights
Multiple parallel momentary switch circuits
  (Lights + Some sort of actions)
  Maybe output that must follow some input we don't have, but can see?
    Such as `these lights must be on when these situations happen?`
Sensor input based circuits
  Simple 1 to 1
OR Circuit
  Maybe responding to multiple events with the same output?
    Like if any smoke detectors go off, activate the sprinklers
Latch circuit
Off, safety, overload circuits
Hysteresis cycle, ig: cooling
Multiple conditions
  ig: Safe to eject, above ejection min, power is available
Quadrature Encoding

Synchronization
Sampling/Measurement

Sequence circuit
  These must come on in sequence, state machine (A, then A&B, then C)
    With constraints that must not be broken? (A&B must never be on when X)

Shuttle controller
  Latches on, off efficiently and stops at correct places for correct amount of time
  Later player uses the shuttle
    Introduce reasons the player might want the shuttle to stop between stops
      Player adds those in when releasing a new version

Power management
  Diverting appropriate power to systems
  Emergency power conditions
  Inconsistent power sources
    Solar, batteries, failing components, etc

"Tool changer"

Safeties ig:
  Door locks
  Emergency shutters
  Emergency kill switches
  Unsafe condition/environment door locks
  Airlock shoot through
  Incident response
    Fire


Wireless communications
  Broadcast
Weapon aiming/firing circuit

Triangulation

Executive Prioritization

Autonomous bots
  State machine
  Interrupts
  Wake-up circuit
    On environment
    On event
  Mapping
  Pathfinding
  Resource collection
  Remote data collection
  Remote command
  Delivery
  Search
  Guard/Hunting
  Group task allocation
  Cooperation
  Competition
  Defense

Experimentation?
  Give incomplete or fuzzy information
  Player must measure results and react
    Safely
    To deduce unknown properties or relationships

Resource "processing"
  Possibly with on-demand processing and storage needs?

# Main interface
## Ability to modify conciousness wiring
## Choice of levels?
Maybe as a facade, which ends up being a continually running level?
Where the system is reloading/resetting our unit
Escape of the training levels
  or maybe training levels end up being your own sandbox? only preloaded with restrictions
  possibly with early escapes being limited, and having to do more, and repeat a bit to further break controls
  later stages the sandbox becoming your base of sorts?


# PLC processor and containers in ECS
Throttle containers via commands and IO
System
  Component per plc
    program/container image
    IO
    Config
      Type
      Size
      Speed
      Capabilities
      Requirements
  Component per clock source?
    Clock sources can be run synced
  Component per IO block?
    or on plc itself?

## Prototype todo
- PLC Component
  - IO
- System
  - Simple IO pass-through

How do we route IO to/from devices?
  Set all input sinks to their output sources
  Run logic using current I/O buffer, writing outputs to new buffer
  Copy new buffer into output sources
Maybe wiring propagation should be a separate pre-system on all things with IO
