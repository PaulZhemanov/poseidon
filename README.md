1) Install Noir
```
curl -L https://raw.githubusercontent.com/noir-lang/noirup/refs/heads/main/install | bash
noirup
```

2) Install Proving Backend:
```
curl -L https://raw.githubusercontent.com/AztecProtocol/aztec-packages/refs/heads/master/barretenberg/bbup/install | bash
bbup
```

3) Create new repo:
```
mkdir zk
cd zk
```

4) Create new Nargo project:
```
nargo new deposit
```

5) Create Deposit proof from /deposit/Prover.toml
```
cd circuits/deposit
nargo compile
nargo execute
bb prove -b ./target/deposit.json -w ./target/deposit.gz -o ./target  
bb write_vk -b ./target/deposit.json -o ./target
```

6) Create new Cargo project:
```
cargo new cli
```

7) Run deposit_proof_convert.rs:
```
cargo run
```

8) Create Withdraw proof from /withdraw/Prover.toml
```
cd circuits/withdraw
nargo compile
nargo execute
bb prove -b ./target/withdraw.json -w ./target/withdraw.gz -o ./target  
bb write_vk -b ./target/withdraw.json -o ./target
```


