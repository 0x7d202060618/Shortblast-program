import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaProgram } from "../target/types/solana_program";
import keys from "../keys/users.json";
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
} from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const connection = new Connection("https://api.devnet.solana.com", {
  commitment: "confirmed",
});
const user = Keypair.fromSecretKey(new Uint8Array(keys));
const mintKeypair = Keypair.generate();

///////////////////////////////Constants////////////////////////////////
const CURVE_SEED = "CurveConfiguration";
const POOL_SEED = "liquidity_pool";
const POOL_SOL_VAULT = "liquidity_sol_vault";

describe("solana-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaProgram as Program<SolanaProgram>;

  // it("Airdrop to admin wallet", async () => {
  //   console.log(
  //     `Requesting airdrop to admin for 1SOL : ${user.publicKey.toBase58()}`
  //   );
  //   // 1 - Request Airdrop
  //   const signature = await connection.requestAirdrop(user.publicKey, 10 ** 9);
  //   // 2 - Fetch the latest blockhash
  //   const { blockhash, lastValidBlockHeight } =
  //     await connection.getLatestBlockhash();
  //   // 3 - Confirm transaction success
  //   await connection.confirmTransaction(
  //     {
  //       blockhash,
  //       lastValidBlockHeight,
  //       signature,
  //     },
  //     "finalized"
  //   );
  //   console.log(
  //     "admin wallet balance : ",
  //     (await connection.getBalance(user.publicKey)) / 10 ** 9,
  //     "SOL"
  //   );
  // });

  // it("Create and Mint Token!", async () => {
  //   // Add your test here.
  //   const tx = new Transaction();
  //   tx.add(
  //     await program.methods
  //       .createToken("meme coin", "$PEPE", "https://example.com")
  //       .accountsStrict({
  //         payer: user.publicKey,
  //         mintAccount: mintKeypair.publicKey,
  //         metadataAccount: PublicKey.findProgramAddressSync(
  //           [
  //             Buffer.from("metadata"),
  //             new PublicKey(
  //               "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  //             ).toBuffer(),
  //             mintKeypair.publicKey.toBuffer(),
  //           ],
  //           new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")
  //         )[0],
  //         tokenProgram: new PublicKey(
  //           "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
  //         ),
  //         tokenMetadataProgram: new PublicKey(
  //           "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  //         ),
  //         systemProgram: new PublicKey("11111111111111111111111111111111"),
  //         rent: new PublicKey("SysvarRent111111111111111111111111111111111"),
  //       })
  //       .instruction()
  //   );

  //   // Token Mint
  //   const associatedTokenAccountAddress = getAssociatedTokenAddressSync(
  //     mintKeypair.publicKey,
  //     user.publicKey!
  //   );

  //   tx.add(
  //     await program.methods
  //       .mintToken()
  //       .accounts({
  //         mintAuthority: user.publicKey,
  //         recipient: user.publicKey,
  //         mintAccount: mintKeypair.publicKey,
  //         associatedTokenAccount: associatedTokenAccountAddress,
  //       })
  //       .instruction()
  //   );
  //   tx.feePayer = user.publicKey
  //   tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //   const sig = await sendAndConfirmTransaction(connection, tx, [user, mintKeypair], { skipPreflight: true })
  //   console.log("Your transaction signature", sig);
  // });

  // it("Initialize Pool", async () => {
  //   const [curveConfig] = PublicKey.findProgramAddressSync(
  //     [Buffer.from(CURVE_SEED)],
  //     program.programId
  //   );
  //   const tx = new Transaction();
  //   tx.add(
  //     await program.methods
  //       .initializeCurve(1)
  //       .accounts({
  //         dexConfigurationAccount: curveConfig,
  //         admin: user.publicKey,
  //         rent: SYSVAR_RENT_PUBKEY,
  //         systemProgram: SystemProgram.programId,
  //       })
  //       .instruction()
  //   );
  //   tx.feePayer = user.publicKey
  //   tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
  //   const sig = await sendAndConfirmTransaction(connection, tx, [user], { skipPreflight: true })
  //   console.log("Your transaction signature", sig);
  // });

  it("Create Pool", async () => {
    // mint: 8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi
    // Token Address: AimxJYEEwM2vkfgThs7aAzfKxVEYkZTEeXW1N2pmSBZU

    const mint_pkey = new PublicKey("8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi");
    const pool = PublicKey.findProgramAddressSync(
                [
                  Buffer.from(POOL_SEED),
                  new PublicKey(
                    "8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi"
                  ).toBuffer(),
                ],
                program.programId
              )[0];
    const pool_reg_KP = Keypair.generate();
    const pool_token_account = getAssociatedTokenAddressSync(mint_pkey, pool, true)
    const tx = new Transaction();
    const pool_sol_vault = PublicKey.findProgramAddressSync(
      [
        Buffer.from(POOL_SOL_VAULT),
        new PublicKey(
          "8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi"
        ).toBuffer(),
      ],
      program.programId
    )[0];
    tx.add( SystemProgram.createAccount({
      fromPubkey: user.publicKey,
      newAccountPubkey: pool_reg_KP.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(
        8+128
      ),
      space: 8+128,
      programId: program.programId,
    }));
    tx.add(await program.methods.createPool().accounts({
      payer: user.publicKey,
      tokenMint: new PublicKey("8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi"),
      poolRegistry: pool_reg_KP.publicKey,
      pool: pool,
      userTokenAccount: new PublicKey("AimxJYEEwM2vkfgThs7aAzfKxVEYkZTEeXW1N2pmSBZU"),
      poolTokenAccount: pool_token_account,
      poolSolVault : pool_sol_vault,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
      systemProgram: SystemProgram.programId
    }).instruction());
    tx.feePayer = user.publicKey
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
    const sig = await sendAndConfirmTransaction(connection, tx, [user, pool_reg_KP], { skipPreflight: true })
    console.log("Your transaction signature", sig);
  });

  // it("Get pool info", async () => {
  //   //pool registry: 4jNncG1wPxznKrY6reVfL4LAc5ANu3ijFpykn61s2TG9
  //   const pools = await program.account.liquidityPool.all();
  //   console.log("Pools", pools);

  //   const pool_registry = await program.account.poolRegistry.fetch("2rQ4WiVhwb2kthUqHr3X9HWzZf9DozV5kbpeoascbbxM")
  //   console.log("Pool Registry", pool_registry)
  // })

//   it("Buy Token", async () => {
//     const [curveConfig] = PublicKey.findProgramAddressSync(
//       [Buffer.from(CURVE_SEED)],
//       program.programId
//     );
//     const mint_pkey = new PublicKey("8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi");
//     const pool = PublicKey.findProgramAddressSync(
//                 [
//                   Buffer.from(POOL_SEED),
//                   new PublicKey(
//                     "8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi"
//                   ).toBuffer(),
//                 ],
//                 program.programId
//               )[0];
//     const pool_token_account = getAssociatedTokenAddressSync(mint_pkey, pool, true)
//     const tx = new Transaction();
//     const pool_sol_vault = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from(POOL_SOL_VAULT),
//         new PublicKey(
//           "8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi"
//         ).toBuffer(),
//       ],
//       program.programId
//     )[0];
//     tx.add(await program.methods.buy(new anchor.BN(10 ** 8)).accounts({
//       dexConfigurationAccount: curveConfig,
//       pool: pool,
//       user: user.publicKey,
//       tokenMint: new PublicKey("8CbAizCWGpNCwQ5oQJ6Ydv1ng4RwCyC5NCbANZfNiCJi"),
//       poolTokenAccount: pool_token_account,
//       poolSolVault : pool_sol_vault,
//       userTokenAccount: new PublicKey("AimxJYEEwM2vkfgThs7aAzfKxVEYkZTEeXW1N2pmSBZU"),
//       tokenProgram: TOKEN_PROGRAM_ID,
//       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
//       rent: SYSVAR_RENT_PUBKEY,
//       systemProgram: SystemProgram.programId
//     }).instruction());
//     tx.feePayer = user.publicKey
//     tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash
//     const sig = await sendAndConfirmTransaction(connection, tx, [user], { skipPreflight: true })
//     console.log("Your transaction signature", sig);
//   })
});
