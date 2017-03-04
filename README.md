# GhostInTheCell
[Codingame](https://www.codingame.com/leaderboards/challenge/ghost-in-the-cell) Ghost in the Cell AI in Rust for an 1 week contest

## Rule based AI
- **Neutral first Strategy** : Target the neutral factory first, useful for the beginning of the game. Capture the factory the faster possible to gain production
- **Defend Strategy** : Search for allies factories that will be captured based on the troops and send reinforcement. Try to predict attack by looking at allies factories that are too close from the enemy.
- **Increase computing** : Simply check of remaining cyborg, and compute `INC` based on a threshold
- **Bomb computing** : Check for the highest production enemy factory, and send a little bomb on that target
- **Targeted Attack Strategy** : Search for the closest enemy factory from all allies factories, and send all the troops.
- **Max Strategy** : Find the ally factory that owns the max cyborg, and send them to the closest enemy.

## Random based AI

### Full random Strategy

- Random `MOVE` orders
- Simulation
- Evaluation

Can be upgraded later with a MCTS or a GA.
