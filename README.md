## requirements

- node 16+
- pnpm, preferably from node corepack

## setup

- run `pnpm install`
- generate keypair JSON array u8 into `.internal/program-keypair.json`, recommended way is to use `solana-keygen new` command
- to run/test locally run `solana-test-validator` in a separate terminal

## internal

`.internal` contains:
- keypair JSON of the program

## commands

        $ pnpm deploy

        $ pnpm test
