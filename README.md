# blue-scoundrel

this is an implementation of the solitaire board game called [scoundrel](http://stfj.net/art/2011/Scoundrel.pdf).


## building

this projects requires rust and cargo.

to build the project, simply clone it and run it.

```bash
git clone https://github.com/azure-blondon/blue-scoundrel.git
cd blue-scoundrel
cargo run
```


## how to play

for a detailled explanation of the rules, you can look at [this document](http://stfj.net/art/2011/Scoundrel.pdf).

### the prompt

the game prompt looks like this:
```
left:  39
room:  4♠  [J♣]  Q♠   7♦
20hp   5♦
```

the top row indicates the number of cards left in the deck.
the center row shows the current room.
the bottom row shows player hp and the equipped weapon.

use the arrow keys to move the cursor around.

### actions

in order to play, you can execute the following actions:

#### general actions (available anywhere)
- Arrow Keys - Navigate
- f - Fill room
- r - Run
- h - Help
- q - Quit

#### room actions (when a room card is selected)
- e - Equip weapon
- a - Attack without weapon
- w - Attack with weapon
- h - Heal (overwrites help)
- d - Discard

when any player card is selected pressing 'd' discards the current equipped weapon along with any enemy that was attached.

## notes

the max hp value is 255, hp starts at 20.

basically no rules are enforced:
- it is possible to run from multiple rooms in a row.
- it is possible to attack an enemy with a weapon that has a lower enemy attached.
- it is possible to do any action with any card: heal, equip, attack.
- it is possible to refill the room at any time, and it is possible to do 4 actions in a row without refilling.


## LICENSE

see [license](LICENSE).

