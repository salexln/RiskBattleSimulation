# Risk Battle Simulation in Rust

How to build (on Ubuntu 16.04):

 * Install rust
 * Clone github repo
  `git clone https://github.com/salexln/RiskBattleSimulation.git`
 * Go into the relevand dir: `cd RiskBattleSimulation/risk_battle_simulation`
 * Build the code: `cargo build`

How to run:

Once the code was build, you can run it using the following command line

`./target/debug/risk_battle_simulation --number-of-attackers 3 --number-of-defenders 3`

Expected output:

```
Attack rolls : [5, 5, 3]
Defend rolls : [2, 2]

**** Summery ****
Attacker won: 2
Defender won: 0
Defender lost 2 soldiers
```


Input params:

In order to see help, run

```./target/debug/risk_battle_simulation --help```

and you'll see:
```
--number-of-attackers - Number of soldies to attack
--number-of-defenders - Number of soldies to defend
```
