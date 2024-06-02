import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BrandLoyaltyProgram } from "../target/types/brand_loyalty_program";
import { PointsProgram } from "../target/types/points_program";
import { assert } from "chai";

describe("brand-loyalty-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const brandLoyaltyProgram = anchor.workspace.BrandLoyaltyProgram as Program<BrandLoyaltyProgram>;
  const pointsProgram = anchor.workspace.PointsProgram as Program<PointsProgram>;

  let admin = anchor.web3.Keypair.generate();
  let state = anchor.web3.Keypair.generate(); // New state account for the program
  let brand = anchor.web3.Keypair.generate();
  let mint = anchor.web3.Keypair.generate();
  let userTokenAccount = anchor.web3.Keypair.generate();
  let anotherUserTokenAccount = anchor.web3.Keypair.generate();

  console.log("Admin Public Key:", admin.publicKey.toBase58());
  console.log("State Public Key:", state.publicKey.toBase58());
  console.log("Brand Public Key:", brand.publicKey.toBase58());
  console.log("Mint Public Key:", mint.publicKey.toBase58());
  console.log("User Token Account Public Key:", userTokenAccount.publicKey.toBase58());
  console.log("Another User Token Account Public Key:", anotherUserTokenAccount.publicKey.toBase58());

  it("Initializes the Brand Loyalty Program", async () => {
    // Fund the admin account
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey, 1000000000)
    );

    // Initialize the Brand Loyalty Program
    const tx = await brandLoyaltyProgram.methods
      .initialize(admin.publicKey)
      .accountsPartial({
        state: state.publicKey,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin, state])
      .rpc();

    console.log("Initialized Brand Loyalty Program:", tx);

    // Fetch the state account to check if initialized correctly
    const stateAccount = await brandLoyaltyProgram.account.state.fetch(state.publicKey);
    assert.equal(stateAccount.admin.toString(), admin.publicKey.toString());
  });

  it("Creates a Brand", async () => {
    // Create a brand
    const tx = await brandLoyaltyProgram.methods
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

    // Fetch the brand account to check if created correctly
    const brandAccount = await brandLoyaltyProgram.account.brand.fetch(brand.publicKey);
    assert.equal(brandAccount.name, "Test Brand");
    assert.equal(brandAccount.owner.toString(), admin.publicKey.toString());
  });

  it("Initializes the Points Program", async () => {
    // Initialize the Points Program
    const pointsState = anchor.web3.Keypair.generate(); // New state account for the points program

    console.log("Points State Public Key:", pointsState.publicKey.toBase58());

    const tx = await pointsProgram.methods
      .initialize(admin.publicKey)
      .accountsPartial({
        state: pointsState.publicKey,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin, pointsState])
      .rpc();

    console.log("Initialized Points Program:", tx);

    // Fetch the state account to check if initialized correctly
    const stateAccount = await pointsProgram.account.state.fetch(pointsState.publicKey);
    assert.equal(stateAccount.admin.toString(), admin.publicKey.toString());
  });

  it("Mints Points", async () => {
    // Fund the mint account
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(mint.publicKey, 1000000000)
    );

    // Mint points to the user
    const tx = await brandLoyaltyProgram.methods
      .mintPoints(new anchor.BN(100))
      .accountsPartial({
        state: state.publicKey,
        brand: brand.publicKey,
        pointsMint: mint.publicKey,
        userTokenAccount: userTokenAccount.publicKey,
        admin: admin.publicKey,
        pointsProgram: pointsProgram.programId,
      })
      .signers([admin])
      .rpc();

    console.log("Minted Points:", tx);

    // Fetch the mint account to check if points were minted correctly
    const mintAccount = await pointsProgram.account.mint.fetch(mint.publicKey);
    assert.equal(mintAccount.supply.toString(), "100");
  });

  it("Transfers Points", async () => {
    // Initialize user token accounts
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(userTokenAccount.publicKey, 1000000000)
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(anotherUserTokenAccount.publicKey, 1000000000)
    );

    // Transfer points from one user to another
    const pointsState = anchor.web3.Keypair.generate();

    console.log("Points State for Transfer Public Key:", pointsState.publicKey.toBase58());

    // Ensure the points state is initialized
    const initTx = await pointsProgram.methods
      .initialize(admin.publicKey)
      .accountsPartial({
        state: pointsState.publicKey,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin, pointsState])
      .rpc();

    console.log("Initialized Points State for Transfer:", initTx);

    const tx = await pointsProgram.methods
      .transferPoints(new anchor.BN(50))
      .accountsPartial({
        state: pointsState.publicKey,
        from: userTokenAccount.publicKey,
        to: anotherUserTokenAccount.publicKey,
        admin: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    console.log("Transferred Points:", tx);

    // Fetch the user token accounts to check if points were transferred correctly
    const fromAccount = await pointsProgram.account.tokenAccount.fetch(userTokenAccount.publicKey);
    const toAccount = await pointsProgram.account.tokenAccount.fetch(anotherUserTokenAccount.publicKey);

    assert.equal(fromAccount.balance.toString(), "50");
    assert.equal(toAccount.balance.toString(), "50");
  });
});
