import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SimpleAnchorApp } from "../target/types/simple_anchor_app";

describe("simple_anchor_app", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.simpleAnchorApp as Program<SimpleAnchorApp>;
  const provider = anchor.getProvider();

  const cpiTargetProgramId = anchor.workspace.cpiTarget.programId;

  it("Make a CPI", async () => {
    const tx = await program.methods
      .initialize(cpiTargetProgramId)
      .accounts({
        payer: provider.publicKey,
        cpiTargetProgram: cpiTargetProgramId,
      })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });
});
