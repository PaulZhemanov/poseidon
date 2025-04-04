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
nargo execute
bb prove -b ./target/zk.json -w ./target/zk.gz -o ./target  
bb write_vk -b ./target/zk.json -o ./target
```