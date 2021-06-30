mod hivegrid;
mod pieces;

/*

XXX Probably start by defining all of the pieces.
Then grid movement:
  - adjacent
    - occupied
    - open
  - breaking the hive
  - slide rule
  - 6 cardinal directions

Piece movement
  - Queen Bee
    - one space in every direction

  - Beetle
    - one space in every direction
    - may move on top of board
      - when climbing on/off top, slide rule can be ignored

  - Hopper
    - next open space in direction that is not empty.
    - ignores slide rule

  - Ant
    - anywhere on the current edge of the hive
    - check on forks

  - Spider
    - 3 moves without backtracking
    - be sure to check for forks

*/
