import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Suncall } from "../target/types/suncall";

describe("suncall", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Suncall as Program<Suncall>;
  const provider = program.provider as anchor.AnchorProvider;
  
  const yi = {
    token: new anchor.web3.PublicKey("8yazwmgc66uKrDBy3TZpNCgLa8qUDcuH8PZCz9jy6dzd"),
    underlyingMint: new anchor.web3.PublicKey("5fjG31cbSszE6FodW37UJnNzgVTyqg5WHWGCmL3ayAvA"),
    mint: new anchor.web3.PublicKey("6XyygxFmUeemaTvA9E9mhH9FvgpynZqARVyG3gUdCMt7"),
    program: new anchor.web3.PublicKey("YiiTopEnX2vyoWdXuG45ovDFYZars4XZ4w6td6RVTFm"),
  };

  const poolName = "solust_pool";
  
  it("should initialize", async () => {
    const [lottoPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(poolName),
        Buffer.from("lotto"),
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
    const tx = await program.methods.initialize(
        poolName
      )
      .accounts({
        lottoYiUnderlyingAta: lottoYiUnderlyingAta,
        lottoYiAta: lottoYiAta,
        yiToken: yi.token,
        yiUnderlyingMint: yi.underlyingMint,
        yiMint: yi.mint,
        owner: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("should user ledger reference initialize", async () => {
    const [lottoPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(poolName),
        Buffer.from("lotto"),
      ],
      program.programId,
    );

    const tx = await program.methods.userLedgerReferenceInitialize()
      .accounts({
        lotto: lottoPda,
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("User ledger reference initialize", tx);
  });

  it("should deposit initialize", async () => {
    const [lottoPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(poolName),
        Buffer.from("lotto"),
      ],
      program.programId,
    );

    const tx = await program.methods.depositInitialize()
      .accounts({
        lotto: lottoPda,
        user: provider.wallet.publicKey,
      })
      .rpc();
    console.log("Deposit initialize", tx);
  });

  it("should deposit", async () => {
    const [lottoPda] = await anchor.web3.PublicKey.findProgramAddress(
      [
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(poolName),
        Buffer.from("lotto"),
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
    const userYiUnderlyingAta = await anchor.utils.token.associatedAddress({
      mint: yi.underlyingMint,
      owner: provider.wallet.publicKey,
    });
    const yiUnderlyingAta = await anchor.utils.token.associatedAddress({
      mint: yi.underlyingMint,
      owner: yi.token,
    });

    const tx = await program.methods.deposit(new anchor.BN(10^6))
      .accounts({
        lotto: lottoPda,
        yiUnderlyingMint: yi.underlyingMint,
        yiMint: yi.mint,
        yiToken: yi.token,
        lottoYiUnderlyingAta,
        lottoYiAta,
        userYiUnderlyingAta,
        yiUnderlyingAta,
        user: provider.wallet.publicKey,
        yiProgram: yi.program,
      })
      .rpc();
    console.log("Deposit", tx);
  });
});
