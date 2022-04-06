import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftVerify } from "../target/types/nft_verify";

describe("nft_verify", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.NftVerify as Program<NftVerify>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
