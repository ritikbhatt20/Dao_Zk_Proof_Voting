import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DaoVoting } from "../target/types/dao_voting";
import { expect } from 'chai';
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";

describe("dao_voting", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.DaoVoting as Program<DaoVoting>;

  let election: Keypair;
  let userStatePda: PublicKey;
  let electionPda: PublicKey;
  let changableTokenAccountPda: PublicKey;
  let electionBump: number;
  let userBump: number;
  let changableTokenAccountBump: number;

  before(async () => {
    election = Keypair.generate();

    [userStatePda, userBump] = await PublicKey.findProgramAddress(
      [Buffer.from("user"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    [electionPda, electionBump] = await PublicKey.findProgramAddress(
      [Buffer.from("election"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    [changableTokenAccountPda, changableTokenAccountBump] = await PublicKey.findProgramAddress(
      [Buffer.from("changabletoken"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Creates a new polling", async () => {
    try {
      const token = new PublicKey("YourTokenPublicKey"); // Replace with actual token public key
      const proposalVoting = "newSymbol";
      const value = "NEW";
      const additionalValue = "Additional Value";

      const tx = await program.methods
        .newPolling(token, proposalVoting, value, additionalValue)
        .accounts({
          election: electionPda,
          authority: provider.wallet.publicKey,
          tokenAccount: provider.wallet.publicKey, // Replace with actual token account
          changableTokenAccount: changableTokenAccountPda, // Replace with actual changable token account
          verifyingKey: Keypair.generate().publicKey, // Replace with actual verifying key account
          systemProgram: SystemProgram.programId,
        })
        .signers([election])
        .rpc();

      console.log("New polling transaction signature", tx);

      const electionAccount = await program.account.election.fetch(electionPda);
      expect(electionAccount.creator.toString()).to.equal(provider.wallet.publicKey.toString());
      expect(electionAccount.proposalVoting).to.equal(proposalVoting);
      expect(electionAccount.voteActive).to.be.true;

      console.log("Election State:", electionAccount);
    } catch (error) {
      console.error("Error during new polling creation:", error);
      throw error;
    }
  });

  it("Casts a vote", async () => {
    try {
      const vote = true;

      const tx = await program.methods
        .vote(vote, [], "") // zk_proof and public_input can be passed accordingly
        .accounts({
          election: electionPda,
          authority: provider.wallet.publicKey,
          tokenAccount: provider.wallet.publicKey, // Replace with actual token account
          changableTokenAccount: changableTokenAccountPda, // Replace with actual changable token account
          user: userStatePda,
          verifyingKey: Keypair.generate().publicKey, // Replace with actual verifying key account
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("Vote transaction signature", tx);

      const electionAccount = await program.account.election.fetch(electionPda);
      expect(electionAccount.voters).to.include(provider.wallet.publicKey);
      expect(electionAccount.numberOfVotes.toNumber()).to.equal(1);

      const userStateAccount = await program.account.user.fetch(userStatePda);
      expect(userStateAccount.rewardPoints.toNumber()).to.equal(1);
    } catch (error) {
      console.error("Error during vote casting:", error);
      throw error;
    }
  });

  it("Summarizes the results", async () => {
    try {
      const tx = await program.methods
        .toSumUp()
        .accounts({
          election: electionPda,
          authority: provider.wallet.publicKey,
          tokenAccount: provider.wallet.publicKey, // Replace with actual token account
          changableTokenAccount: changableTokenAccountPda, // Replace with actual changable token account
        })
        .rpc();

      console.log("Summarize transaction signature", tx);

      const electionAccount = await program.account.election.fetch(electionPda);
      expect(electionAccount.voteActive).to.be.false;

      console.log("Election State after summarizing:", electionAccount);
    } catch (error) {
      console.error("Error during summarizing:", error);
      throw error;
    }
  });

  it("Gets results", async () => {
    try {
      const tx = await program.methods
        .getResults()
        .accounts({
          election: electionPda,
        })
        .rpc();

      console.log("Get results transaction signature", tx);
    } catch (error) {
      console.error("Error during getting results:", error);
      throw error;
    }
  });

  it("Closes the election", async () => {
    try {
      const tx = await program.methods
        .closeElection()
        .accounts({
          election: electionPda,
          authority: provider.wallet.publicKey,
          changableTokenAccount: changableTokenAccountPda, // Replace with actual changable token account
        })
        .rpc();

      console.log("Close election transaction signature", tx);
    } catch (error) {
      console.error("Error during election closure:", error);
      throw error;
    }
  });
});
