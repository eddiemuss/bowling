## Bowling

### What is this Repo all about?

This repo is all about playing around with Rust by builing a little bowling Game in rust.
The idea is to build a crate that can be used to calculate the points of a bowling game with one or multiple player.

The main purpose of this is to get familiar with rust and to see how a synchronise CLI game may look like.

### State machine

```mermaid
stateDiagram-v2
  direction LR

  state Gamer {
    %% States
    active: active(rounds, current_round)
    passive: passive(rounds, last_round)
    completed: completed(rounds)

    state role <<choice>>
    state has_open_rounds <<choice>>

    %% Transitions
    [*] --> active: init()
    active --> role: role
    role --> active: if Round is second
    role --> passive: if Round is completed
    passive --> has_open_rounds: Has open rounds?
    has_open_rounds --> active: if yes
    has_open_rounds --> completed: if no
  }

  state Round {
    %% States
    first
    second
    complete

    %% Transitions
    [*] --> first: new()
    first --> second: Hit
    first --> second: Miss
    first --> complete: Strike
    second --> complete: Hit
    second --> complete: Spare
    second --> complete: Miss
    complete --> [*]

    %% Notes
    note left of first
      First describes the state in
      when the first throw
      is about to happen.
    end note

    note left of second
      Second describes the state
      when the second throw is
      about to happen.
    end note
  }
```

### Vision

The vison is to have a CLI game that makes it possible that multiple player bowl sequentiually while the crate returns a displable score and game table.
