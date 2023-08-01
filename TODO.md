***Current Problem***
How should we represent plc memory?

What about non plc in-game components?
  Such as sensors, motors, lights, etc
  Need IO with each other and logic systems

All as components?

Game Structure
  Primary Game
    Levels (artificial)
    Stationary Logic Systems
    Mobiles/Bots
      PLCs/Logic Controllers
      Sensors
      Motors/Actuators
        User IO
        Cameras
        IO
        Memory

PLC Ladder Runner System
  Each Plc
    Sample inputs
    Compute
    Set outputs

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

## How do we route IO to/from devices? In game, and in code
### In game
  IO only available in logic handling devices
  Scopped, and with permissions and attributes
  Addressed

### In code
  Events?
    Check input components, generate change events as needed
    Build input buffer
    Run logic systems, generate change events as needed

### Memory address based connections?
  Permissions?
  Attributes
    Protected
    Hidden
  Scopes
    Device
    Bot
    Level
    World

# Controls TODO
[o] Camera Controls
  [X] Basic "god mode" fps, wsad movement with mouse turning
  [X] Basic orbital with distance and target
[o] Physics FPS
  [ ] Trigger volumes
  [o] Built in rapier character controller
    [o] Gravity
    [X] Floor/level collision
    [o] Jump
  [ ] *Decision:* Own camera system, based completely on physically placed cameras
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
    [X] Add axles in-between vehicle and wheels
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


# Level/Puzzle Ideas
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

# Misc Ideas
Players perception of time can jump back&forward via date manipulation
  Intended for player to think these are "flashbacks"
    and that the shown results are memories
    when they're actually real-time causal things?

EMPs, relay logic, simpler circuits, screw machines?
Day/Night & Solar?
Interrupts
Low voltage levels/portions

Guide player from stationary/mobile cameras to pre-programmed remote bot, then autonomous

Searches of player memory for contraban/sensitive data
  Requiring storage of these externally
  Or encoding/encryption
  "I don't want to see you brushing your pins. We'll do a quick scan"

If each component (plc, sensor, output device) has a >=1 cycle propagation delay
  Then does this solve our I/O throughput requirements?
    We still need to know the order, or we have undefined states until the pipeline is warm
      Can we order them according to their connections?

Do I need the ability for sensors to be connected directly to outputs?
  If not, do we shed complexity by having plc components doing all of the push/pull?
  Where would the player interface be and look like?
  These would either be small circuits
    or large circuits that are made of multiple discrete components
      This could be considered slower, with >=1 cycle propagation delays

Distance based physical scope?
  Could be cool for temporary connections, wifi-like
  How would they reference eachother's addresses?
  What would the ladder look like?
    Would need collection support and loops?
      I suppose this could be done in ladder over multiple itterations

First glimpse into main game sooner than later?

Difference between two runs/plays quantifiable and usable?
  Choices made, resources available, allocations and spending, etc, etc

Can I quantify difference between bots?
  with the vector thing from llms?

Parent->Children for sink/sources?
  I suppose not needed if no wiring?
  Might still be needed at a higher level, tracking what is in scope?

Writing circuits to control the viewports
  Remote stationary cameras
  Remote controllable cameras
  Bot cameras
  Multi cameras

Player writes programs via internal editor
  then uploads and registers them for use elsewhere
  possibly via the global API
    possibly also to more local networks

Might be fun to force the player to use immutable logic in certain places
  For puzzle design and constraints
  Repurpose logic?
    Simple logic
    Supplementing broken/inadequate logic externally

Restart/Reset of some "levels" could be logic in the main overall game, or the training

Player conciousness respawn/retrieval could be repurposing of the training level restart mechanism
  Maybe just a daemon that spawns you again on termination?
    Now your position being the initial value
      Which could be editable later

## Roguelike where each "life" is limited in time
  Choices are sometimes exclusive
  Not neccessarily linear

## Self-modification module
  Allows wiring and configuration of modules on self

## Apocalyptic remains as resources
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

# Links
deferred world mutations - https://bevyengine.org/news/bevy-0-10/#deferred-world-mutations
resource_scope - https://github.com/bevyengine/bevy/blob/ac661188c8679e767c4f2abebee22b4db3128e99/crates/bevy_ecs/src/world/mod.rs#L599-L615

events - https://github.com/bevyengine/bevy/blob/latest/examples/ecs/event.rs
system conditions - https://docs.rs/bevy/latest/bevy/ecs/schedule/common_conditions/index.html

# BUGS
fix-bad-first-joint - Branch contains example of problematic "first joint" bug

