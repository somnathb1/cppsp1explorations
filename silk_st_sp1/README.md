
Install sp1 and its c-toolchain
see https://docs.succinct.xyz/docs/sp1/getting-started/install
```
curl -L https://sp1up.succinct.xyz | bash
sp1up --c-toolchain
```

Verify installation
```
cargo prove --version
```

You should point the g++ to the one provided by the installation of sp1up, for the rv32im-ilp32 arch/platform
```sh
export RUSTC_LINKER=/home/som/.sp1/riscv/riscv32im-linux-x86_64/bin/riscv32-unknown-elf-ld 
export CXX_riscv32im_succinct_zkvm_elf=/home/som/.sp1/riscv/riscv32im-linux-x86_64/bin/riscv32-unknown-elf-g++ 
```

To build the source in `program` folder
```
cd program
cargo prove build
```
This build should succeed

The program is automatically built through `script/build.rs` when the script is built.

```sh
cd script
cargo run --release -- --execute
```

This will execute the program and display the output.

### Generate an SP1 Core Proof

To generate an SP1 [core proof](https://docs.succinct.xyz/docs/sp1/generating-proofs/proof-types#core-default) for your program:

```sh
cd script
cargo run --release -- --prove
```

### Generate an EVM-Compatible Proof

> [!WARNING]
> You will need at least 16GB RAM to generate a Groth16 or PLONK proof. View the [SP1 docs](https://docs.succinct.xyz/docs/sp1/getting-started/hardware-requirements#local-proving) for more information.

Generating a proof that is cheap to verify on the EVM (e.g. Groth16 or PLONK) is more intensive than generating a core proof.

To generate a Groth16 proof:

```sh
cd script
cargo run --release --bin evm -- --system groth16
```

To generate a PLONK proof:

```sh
cargo run --release --bin evm -- --system plonk
```

These commands will also generate fixtures that can be used to test the verification of SP1 proofs
inside Solidity.

### Retrieve the Verification Key

To retrieve your `programVKey` for your on-chain contract, run the following command in `script`:

```sh
cargo run --release --bin vkey
```

## Using the Prover Network

We highly recommend using the [Succinct Prover Network](https://docs.succinct.xyz/docs/network/introduction) for any non-trivial programs or benchmarking purposes. For more information, see the [key setup guide](https://docs.succinct.xyz/docs/network/developers/key-setup) to get started.

To get started, copy the example environment file:

```sh
cp .env.example .env
```

Then, set the `SP1_PROVER` environment variable to `network` and set the `NETWORK_PRIVATE_KEY`
environment variable to your whitelisted private key.

For example, to generate an EVM-compatible proof using the prover network, run the following
command:

```sh
SP1_PROVER=network NETWORK_PRIVATE_KEY=... cargo run --release --bin evm
```


Look for build path at 
/home/som/Documents/code/cppsp1explorations/silk_st_sp1/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/build/silk-rust-state_transition-5a8ac9689182d625/out