import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  Commitment,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
} from "@solana/web3.js";
import { WbaOracle } from "../target/types/wba_oracle";
import { assert } from "chai";

const commitment: Commitment = "confirmed";

describe("oracle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.WbaOracle as Program<WbaOracle>;
  const connection: Connection = anchor.getProvider().connection;

  const owner = Keypair.generate();
  const operator1 = Keypair.generate();
  console.log("operator 1 : ", operator1.publicKey.toString());
  const operator2 = Keypair.generate();
  console.log("operator 2 : ", operator2.publicKey.toString());

  const oracle = PublicKey.findProgramAddressSync(
    // b"hack", protocol.key().as_ref(), amount.to_le_bytes().as_ref()
    [Buffer.from("oracle"), owner.publicKey.toBytes()],
    program.programId
  )[0];

  it("airdrop", async () => {
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        owner.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        operator1.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
    await anchor
      .getProvider()
      .connection.requestAirdrop(
        operator2.publicKey,
        100 * anchor.web3.LAMPORTS_PER_SOL
      )
      .then(confirmTx);
  });

  it("oracle create", async () => {
    await program.methods
      .oracleCreate()
      .accounts({
        owner: owner.publicKey,
        oracle,
        operator: operator1.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const pda = await program.account.oracle.fetch(oracle);
        console.log(pda);
        assert.equal(pda.owner.toString(), owner.publicKey.toString());
      });
  });

  it("oracle update", async () => {
    const value = new BN(1);
    await program.methods
      .oracleUpdate(value)
      .accounts({
        oracle,
        operator: operator1.publicKey,
      })
      .signers([operator1])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const pda = await program.account.oracle.fetch(oracle);
        console.log(pda);
        assert.equal(pda.operators[0].value.toNumber(), 1);
      });
  });

  it("operator add", async () => {
    await program.methods
      .operatorAdd()
      .accounts({
        owner: owner.publicKey,
        oracle,
        operator: operator2.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const pda = await program.account.oracle.fetch(oracle);
        assert.equal(pda.operators.length, 2);
        assert.notEqual(
          pda.operators[0].address.toString(),
          pda.operators[1].address.toString()
        );
        console.log(pda);
      });
  });

  it("operator remove", async () => {
    await program.methods
      .operatorRemove()
      .accounts({
        owner: owner.publicKey,
        oracle,
        operator: operator1.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx)
      .then(async () => {
        const pda = await program.account.oracle.fetch(oracle);
        console.log(pda);
        assert.equal(
          pda.operators[0].address.toString(),
          operator2.publicKey.toString()
        );
      });
  });

  it("oracle delete", async () => {
    await program.methods
      .oracleDelete()
      .accounts({
        owner: owner.publicKey,
        oracle,
      })
      .signers([owner])
      .rpc()
      .then(confirmTx);
    // .then(async () => {
    //   const pda = await program.account.oracle.fetch(oracle);
    //   console.log(pda);
    //   assert.fail("Account does not exist or has no data");
    // });
  });
});

const confirmTx = async (signature: string) => {
  const latestBlockhash = await anchor
    .getProvider()
    .connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    commitment
  );
};
