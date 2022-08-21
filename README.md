# SLA Programs

This repository holds the Secret Llama Agency smart contract (Solana program).

The address of the program is: **GUSxqUfUdqchfErA3DrW1jNVJKGdMpxt71AeDkJJtG5R**.

The wallet currently owning the program is: **X1d8FSWgsM3QRjqLu9XboBRF2gSk3w6kP3tKGifWvxL**.


## Deployment

In order to be able to re-deploy (or update) this program, the [Anchor.toml](Anchor.toml) file must be updated with:

```
wallet = "<path_to_wallet_keypair_file>"
```

Once that is done, the on-chain program can be updated by running the script

```
./scripts/upgrade.sh
```

**NOTE**: this command is likely to fail a few times before succeeding. This is often due to Solana being congested. 