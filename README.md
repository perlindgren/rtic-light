# RTIC-light

Modular reincarnation of the RTIC framework.

## Design goal

The implementation of the RTFM/RTIC framework builds on a monolithic design, modular in only the syntax crate being factored out (and possible to re-use for supporting other back-ends than the cortex-m architecture).

Supporting experimental features require both the syntax (`rtic-syntax`) and backend (`cortex-m-rtic`) to be forked, which complicates development. Furthermore, it makes it hard (or even impossible) to extend the framework with features available under another license (without maintaining a complete fork).

RTIC-light, aims at a modular architecture where an application is passed through a pipeline of passes, which for a correct application produces a syntactically (and semantically) valid input for the next pass. Errors in the application may be caught by the current pass, or by a later pass.

## Tentative set of passes

### `rtic-rt`

Models the lowest abstraction needed for implementing an embedded application with safe access to shared resources.

The `rtic-rt-cortex-m` is the last pass for a cortex-m application, which implements the `rtic-rt` model. 

Basic functionality:

- general platform startup/initialization code, setting up statics etc.
- a non returning main
- run to completion interrupt handlers with static priority 
- resource management, with direct access to unique resources and claim (lock) for resources that might be accessed by interrupts at different priority
  
### `rtic-spawn`

Already at this point RTIC becomes target agnostic. In this pass, we provide syntax and implementation for message passing. It produces an `rtic-rt` model.

### `rtic-spawn-after`

This pass allows for tasks to be spawned at future points in time. This pass produces an `rtic-spawn` model.

### `rtic-time`

This pass allows task to be annotated with additional information, e.g., deadlines, minimum inter-arrival time, periodicity etc. useful both to express the intended timing behavior and for analysis.

### `rtic-contract`

This pass, extents the RTIC model with contracts, pre and post conditions, resource invariants etc. useful for formal analysis.

## Composition

At top level an RTIC-light application will look something like:

``` rust
#[rtic(passes = [rtic-spawn, rtic-rt-cortex-m])]
mod rtic {
    ...
}
```

Or any other composition following the hierarchy (a lacking pass may yield incorrect syntax for the next pass).

``` rust
#[rtic(passes = [rtic-contract, ..., rtic-spawn, rtic-rt-cortex-m])]
mod rtic {
    ...
}
```

## Design and Implementation

This repository is a playground for experimenting with different approach to achieve the overall design goal. There are many design and implementation issues to be ironed out. How can we make the implementation of each pass to be as convenient as possible, what are the commonalities and possibilities for code re-use. How can we propagate additional information from one pass-to the next. 

Initial experiments indicate that the approach could potentially work, and original spans will be correctly propagated, thus error reporting will not suffer. Additional build artifacts can be generated in the `target` directory and/or under the `mod rtic {...}`.

The `passes` array will be consumed left to right.

One can think of using the `build.rs` in order to invoke other tools to accomplish additional functionality (like invoking external tools as part of the build process, producing `dot` graphs, reports, invoking test runners etc.



## License 

For now: All rights reserved.

Will be changed to some proper license later.
