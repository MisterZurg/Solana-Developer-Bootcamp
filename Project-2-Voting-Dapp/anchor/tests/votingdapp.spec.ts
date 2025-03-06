import * as anchor from "@coral-xyz/anchor";

import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN, Program } from "@coral-xyz/anchor";
import { Votingdapp } from "../target/types/votingdapp";

// Helper for calling smart contracts — kinda ABI
const IDL = require("../target/idl/votingdapp.json");

const votingAddress = new PublicKey(
  "coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF",
);

/// anchor test --skip-local-validator --skip-deploy

describe("votingdapp", () => {
  let context, provider;
  let votingProgram: Program<Votingdapp>;

  // Prepare test data before each IT
  beforeAll(async () => {
    // Anchor.toml
    // [programs.localnet]
    // votingdapp = ...
    context = await startAnchor(
      "",
      [{ name: "votingdapp", programId: votingAddress }],
      [],
    );

    provider = new BankrunProvider(context);
    votingProgram = new Program<Votingdapp>(IDL, provider);
  });

  test("Initialize Votingdapp Pool", async () => {
    await votingProgram.methods
      .initializePool(
        new anchor.BN(1),
        "What color is your Bugatti? - Andrew Tate",
        new anchor.BN(0),
        new anchor.BN(1759508293),
      )
      .rpc();

    const [poolAddress] = PublicKey.findProgramAddressSync(
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
      votingAddress,
    );

    const pool = await votingProgram.account.pool.fetch(poolAddress);

    console.log("--> 🚧 DEBUG", pool);

    // region -- Actually testing
    expect(pool.poolId.toNumber()).toEqual(1);
    expect(pool.description.toString()).toEqual(
      "What color is your Bugatti? - Andrew Tate",
    );
    expect(pool.poolStart.toNumber()).toBeLessThan(pool.poolEnd.toNumber());
  });

  // it — integration test
  test("Initialize Candidate", async () => {
    await votingProgram.methods
      .initializeCandidate(
        // Same arg order
        "Zeliboba",
        new anchor.BN(1),
      )
      .rpc();

    await votingProgram.methods
      .initializeCandidate("Andrew Tate", new anchor.BN(1))
      .rpc();

    const [zelibobaAddress] = PublicKey.findProgramAddressSync(
      // Our account seeds from struct pool_id, name
      // seeds = [pool_id.to_le_bytes().as_ref(), name.as_bytes()],
      // pub candidate: Account<'info, Candidate>,
      [new anchor.BN(1).toArrayLike(Buffer, "le", 8), Buffer.from("Zeliboba")],
      votingAddress,
    );

    console.log("--> 🚧 DEBUG", zelibobaAddress);

    const [andrewTateAddress] = PublicKey.findProgramAddressSync(
      [
        new anchor.BN(1).toArrayLike(Buffer, "le", 8),
        Buffer.from("Andrew Tate"),
      ],
      votingAddress,
    );

    console.log("--> 🚧 DEBUG", andrewTateAddress);

    const zelibobaCandidate =
      await votingProgram.account.candidate.fetch(zelibobaAddress);

    const andrewCandidate =
      await votingProgram.account.candidate.fetch(andrewTateAddress);

    expect(zelibobaCandidate.votes.toNumber()).toEqual(0);
    expect(andrewCandidate.votes.toNumber()).toEqual(0);
  });

  test("Vote Test", async () => {});
});
