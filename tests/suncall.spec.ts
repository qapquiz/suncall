import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Suncall } from "../target/types/suncall";

describe("suncall", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Suncall as Program<Suncall>;
  const provider = program.provider as anchor.AnchorProvider;
  
  const yi = {
    underlyingMint: new anchor.web3.PublicKey("5fjG31cbSszE6FodW37UJnNzgVTyqg5WHWGCmL3ayAvA"),
    mint: new anchor.web3.PublicKey("6XyygxFmUeemaTvA9E9mhH9FvgpynZqARVyG3gUdCMt7"),
  };
  
  it("Is initialized!", async () => {
    const [lottoPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("lotto"),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId,
    );
    const [lottoAuthorityPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        lottoPda.toBuffer(),
        Buffer.from("authority"),
      ],
      program.programId,
    );
    const lottoYiUnderlyingAta = await anchor.utils.token.associatedAddress({
      mint: yi.underlyingMint,
      owner: lottoAuthorityPda,
    });
    const lottoYiAta = await anchor.utils.token.associatedAddress({
      mint: yi.mint,
      owner: lottoAuthorityPda,
    });
    const tx = await program.methods.initialize()
      .accounts({
        lottoYiUnderlyingAta: lottoYiUnderlyingAta,
        lottoYiAta: lottoYiAta,
        yiUnderlyingMint: yi.underlyingMint,
        yiMint: yi.mint,
        owner: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
