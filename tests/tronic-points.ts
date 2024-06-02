import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TronicPoints } from "../target/types/tronic_points";
import { Token } from "@solana/spl-token";
import { assert } from "chai";

describe("tronic-points", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const tronicPointsProgram = anchor.workspace.TronicPoints as Program<TronicPoints>;

    // Generate keypairs for admin and state
    let admin = anchor.web3.Keypair.generate();
    let state = anchor.web3.Keypair.generate();
    let brand = anchor.web3.Keypair.generate();
    let userTokenAccount = anchor.web3.Keypair.generate(); // This account must be properly initialized as a token account

    // Log the public keys of the generated keypairs
    console.log("Admin Public Key:", admin.publicKey.toBase58());
    console.log("State Public Key:", state.publicKey.toBase58());
    console.log("Brand Public Key:", brand.publicKey.toBase58());
    console.log("User Token Account Public Key:", userTokenAccount.publicKey.toBase58());

    it("Initializes the Tronic Points Program", async () => {
        // Fund the admin account with SOL for transactions
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(admin.publicKey, 1000000000)
        );

        // Initialize the Tronic Points Program
        const tx = await tronicPointsProgram.methods
            .initialize(admin.publicKey)
            .accountsPartial({
                state: state.publicKey,
                admin: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin, state])
            .rpc();

        console.log("Initialized Tronic Points Program:", tx);

        // Fetch the state account to check if it was initialized correctly
        const stateAccount = await tronicPointsProgram.account.state.fetch(state.publicKey);
        assert.equal(stateAccount.admin.toString(), admin.publicKey.toString());
    });

    it("Creates a Brand", async () => {
        // Create a new brand
        const tx = await tronicPointsProgram.methods
            .createBrand("Test Brand")
            .accountsPartial({
                state: state.publicKey,
                brand: brand.publicKey,
                admin: admin.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([admin, brand])
            .rpc();

        console.log("Created Brand:", tx);

        // Fetch the brand account to check if it was created correctly
        const brandAccount = await tronicPointsProgram.account.brand.fetch(brand.publicKey);
        assert.equal(brandAccount.name, "Test Brand");
        assert.equal(brandAccount.owner.toString(), admin.publicKey.toString());

        // Initialize mint for the brand's points
        const mint = await Token.createMint(
            provider.connection,
            admin,
            admin.publicKey,
            null,
            0,
            TOKEN_PROGRAM_ID
        );
  
      // Create a token account for the user
      userTokenAccount = await mint.createAccount(admin.publicKey);
  
      console.log("Mint Public Key:", mint.toBase58());
      console.log("User Token Account Public Key:", userTokenAccount.toBase58());
    });

    it("Mints Points", async () => {
      const brandAccount = await tronicPointsProgram.account.brand.fetch(brand.publicKey);
        // Mint points to the user's token account
        const tx = await tronicPointsProgram.methods
            .mintPoints(new anchor.BN(100))
            .accounts({
                state: state.publicKey,
                brand: brand.publicKey,
                pointsMint: brandAccount.pointsMint, 
                userTokenAccount: userTokenAccount.publicKey,
                admin: admin.publicKey,
                pointsProgram: tronicPointsProgram.programId, // This should be the SPL Token program ID
            })
            .signers([admin])
            .rpc();

        console.log("Minted Points:", tx);

        // // Fetch the user's token account to check if points were minted correctly
        // // Ensure this account is initialized as a token account
        // const userTokenAccountInfo = await tronicPointsProgram.account.tokenAccount.fetch(userTokenAccount.publicKey);
        // assert.equal(userTokenAccountInfo.amount.toString(), "100");
    });
});
