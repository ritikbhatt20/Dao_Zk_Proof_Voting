import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { DaoZkProofContractUsingCurveDalek } from "../target/types/dao_zk_proof_contract_using_curve_dalek";

describe("Dao_Zk_Proof_Contract_Using_Curve_Dalek", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DaoZkProofContractUsingCurveDalek as Program<DaoZkProofContractUsingCurveDalek>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
