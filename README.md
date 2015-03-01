# sneak_poc

## The Goal
A simple console application with the following features:
  The application simulates a room containing two entities from opposing factions.
  The player controls both entities separately, and may control them:
    Entities can move in any 8-way direction.
    Entities can look in any 8-way direction.
    Entities may crouch to half-height or stand to full-height.

  The room is populated with barriers that are either half or full height
  Barriers prevent movement and break line of sight
  When either entity can see the opposition, they are rendered red.

## The Stack
The engine is run on an Entity-Component System in Rust, and rendered as a console application using Curses.
