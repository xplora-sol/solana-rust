import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert, expect } from "chai";
import { SolanaProg } from "../target/types/solana_prog";

describe("Xplora Quest Program", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.solanaProg as Program<SolanaProg>;

  // Test accounts
  let authority: Keypair;
  let registryPDA: PublicKey;
  let registryBump: number;

  // Test data
  const testLocation1 = "Kathmandu, Nepal";
  const testLocation2 = "Pokhara, Nepal";
  let location1PDA: PublicKey;
  let location2PDA: PublicKey;

  const sampleQuest1 = {
    title: "Hidden Temple",
    description: "Find the ancient temple in Kathmandu",
    questType: { discovery: {} },
    difficulty: { medium: {} },
    timeToLiveHours: 48,
    verifiableLandmark: "Stone pillar with lotus",
    landmarkName: "Temple",
    latitude: 27.7172,
    longitude: 85.324,
    createdAt: new BN(0),
    reserved: [0, 0, 0, 0],
  };

  const sampleQuest2 = {
    title: "Peace Pagoda",
    description: "Trek to World Peace Pagoda",
    questType: { exploration: {} },
    difficulty: { hard: {} },
    timeToLiveHours: 72,
    verifiableLandmark: "White dome",
    landmarkName: "Pagoda",
    latitude: 28.2096,
    longitude: 83.9856,
    createdAt: new BN(0),
    reserved: [0, 0, 0, 0],
  };

  const sampleQuest3 = {
    title: "Durbar Square",
    description: "Visit ancient temples",
    questType: { challenge: {} },
    difficulty: { easy: {} },
    timeToLiveHours: 24,
    verifiableLandmark: "Basantapur Tower",
    landmarkName: "Durbar",
    latitude: 27.7045,
    longitude: 85.3077,
    createdAt: new BN(0),
    reserved: [0, 0, 0, 0],
  };

  before(async () => {
    // Generate authority keypair
    authority = Keypair.generate();

    // Airdrop SOL to authority
    const signature = await provider.connection.requestAirdrop(
      authority.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);

    // Derive PDAs
    [registryPDA, registryBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("quest_registry")],
      program.programId
    );

    [location1PDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("location_quests"), Buffer.from(testLocation1)],
      program.programId
    );

    [location2PDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("location_quests"), Buffer.from(testLocation2)],
      program.programId
    );

    console.log("\n🔑 Test Setup:");
    console.log("Authority:", authority.publicKey.toString());
    console.log("Registry PDA:", registryPDA.toString());
    console.log("Location 1 PDA:", location1PDA.toString());
    console.log("Location 2 PDA:", location2PDA.toString());
  });

  describe("1️⃣  Initialize Registry", () => {
    it("Should initialize the quest registry", async () => {
      const tx = await program.methods
        .initialize(authority.publicKey)
        .accounts({
          registry: registryPDA,
          payer: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("\n✅ Registry initialized. Tx:", tx);

      // Fetch and verify registry data
      const registry = await program.account.questRegistry.fetch(registryPDA);

      assert.equal(
        registry.authority.toString(),
        authority.publicKey.toString(),
        "Authority should match"
      );
      assert.equal(
        registry.totalLocations.toNumber(),
        0,
        "Initial location count should be 0"
      );
      // Note: version field exists in the program but isn't exposed in IDL

      console.log("Registry data:", {
        authority: registry.authority.toString(),
        totalLocations: registry.totalLocations.toNumber(),
      });
    });

    it("Should fail to initialize registry twice", async () => {
      try {
        await program.methods
          .initialize(authority.publicKey)
          .accounts({
            registry: registryPDA,
            payer: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([authority])
          .rpc();

        assert.fail("Should have failed to initialize twice");
      } catch (error) {
        console.log("✅ Correctly failed to initialize twice");
        expect(error.toString()).to.include("already in use");
      }
    });
  });

  describe("2️⃣  Create Location Quests", () => {
    it("Should create location quests with multiple quests", async () => {
      const tx = await program.methods
        .createLocationQuests(testLocation1, [sampleQuest1, sampleQuest3])
        .accounts({
          registry: registryPDA,
          locationQuests: location1PDA,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("\n✅ Location created. Tx:", tx);

      // Fetch and verify location data
      const locationQuests = await program.account.locationQuests.fetch(
        location1PDA
      );

      assert.equal(locationQuests.location, testLocation1);
      assert.equal(locationQuests.quests.length, 2);
      assert.equal(locationQuests.initialized, true);
      assert.equal(locationQuests.quests[0].title, sampleQuest1.title);
      assert.equal(locationQuests.quests[1].title, sampleQuest3.title);

      console.log("Location data:", {
        location: locationQuests.location,
        questCount: locationQuests.quests.length,
        initialized: locationQuests.initialized,
      });

      // Verify registry was updated
      const registry = await program.account.questRegistry.fetch(registryPDA);
      assert.equal(registry.totalLocations.toNumber(), 1);
    });

    it("Should create second location with different quests", async () => {
      const tx = await program.methods
        .createLocationQuests(testLocation2, [sampleQuest2])
        .accounts({
          registry: registryPDA,
          locationQuests: location2PDA,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      console.log("\n✅ Second location created. Tx:", tx);

      const locationQuests = await program.account.locationQuests.fetch(
        location2PDA
      );
      assert.equal(locationQuests.location, testLocation2);
      assert.equal(locationQuests.quests.length, 1);

      // Verify registry counter incremented
      const registry = await program.account.questRegistry.fetch(registryPDA);
      assert.equal(registry.totalLocations.toNumber(), 2);

      console.log("Total locations:", registry.totalLocations.toNumber());
    });

    it("Should fail with empty quest array", async () => {
      const testLocation3 = "Bhaktapur, Nepal";
      const [location3PDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("location_quests"), Buffer.from(testLocation3)],
        program.programId
      );

      try {
        await program.methods
          .createLocationQuests(testLocation3, [])
          .accounts({
            registry: registryPDA,
            locationQuests: location3PDA,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([authority])
          .rpc();

        assert.fail("Should have failed with empty quest array");
      } catch (error) {
        console.log("✅ Correctly failed with empty quest array");
        expect(error.toString()).to.include("EmptyQuestsArray");
      }
    });

    it("Should fail when unauthorized user tries to create location", async () => {
      const unauthorizedUser = Keypair.generate();

      // Airdrop to unauthorized user
      const sig = await provider.connection.requestAirdrop(
        unauthorizedUser.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);

      const testLocation4 = "Lalitpur, Nepal";
      const [location4PDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("location_quests"), Buffer.from(testLocation4)],
        program.programId
      );

      try {
        await program.methods
          .createLocationQuests(testLocation4, [sampleQuest1])
          .accounts({
            registry: registryPDA,
            locationQuests: location4PDA,
            authority: unauthorizedUser.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([unauthorizedUser])
          .rpc();

        assert.fail("Should have failed with unauthorized user");
      } catch (error) {
        console.log("✅ Correctly failed with unauthorized user");
        expect(error.toString()).to.include("Unauthorized");
      }
    });
  });

  describe("3️⃣  Add Quest to Location", () => {
    it("Should add a new quest to existing location", async () => {
      const newQuest = {
        title: "Swayambhunath Monkey Quest",
        description:
          "Climb the 365 steps to Swayambhunath Stupa and spot the resident monkeys.",
        questType: { exploration: {} },
        difficulty: { medium: {} },
        timeToLiveHours: 36,
        verifiableLandmark: "Golden spire at the top",
        landmarkName: "Swayambhunath Stupa",
        latitude: 27.7149,
        longitude: 85.2906,
        createdAt: new BN(0),
        reserved: [0, 0, 0, 0],
      };

      const beforeQuests = await program.account.locationQuests.fetch(
        location1PDA
      );
      const beforeCount = beforeQuests.quests.length;

      const tx = await program.methods
        .addQuestToLocation(newQuest)
        .accounts({
          registry: registryPDA,
          locationQuests: location1PDA,
          authority: authority.publicKey,
        })
        .signers([authority])
        .rpc();

      console.log("\n✅ Quest added. Tx:", tx);

      const afterQuests = await program.account.locationQuests.fetch(
        location1PDA
      );
      assert.equal(afterQuests.quests.length, beforeCount + 1);
      assert.equal(
        afterQuests.quests[afterQuests.quests.length - 1].title,
        newQuest.title
      );

      console.log(
        "Quest count increased from",
        beforeCount,
        "to",
        afterQuests.quests.length
      );
    });

    it("Should fail when adding quest with invalid coordinates", async () => {
      const invalidQuest = {
        ...sampleQuest1,
        title: "Invalid Quest",
        latitude: 50.0, // Outside Nepal bounds
        longitude: 85.0,
      };

      try {
        await program.methods
          .addQuestToLocation(invalidQuest)
          .accounts({
            registry: registryPDA,
            locationQuests: location1PDA,
            authority: authority.publicKey,
          })
          .signers([authority])
          .rpc();

        assert.fail("Should have failed with invalid coordinates");
      } catch (error) {
        console.log("✅ Correctly failed with invalid latitude");
        expect(error.toString()).to.include("InvalidLatitude");
      }
    });
  });

  describe("4️⃣  Update Quest", () => {
    it("Should update an existing quest", async () => {
      const updatedQuest = {
        ...sampleQuest1,
        title: "Updated: Find the Hidden Temple",
        description: "Updated description with more details about the quest.",
        difficulty: { hard: {} }, // Changed from medium to hard
      };

      const tx = await program.methods
        .updateQuest(0, updatedQuest)
        .accounts({
          registry: registryPDA,
          locationQuests: location1PDA,
          authority: authority.publicKey,
        })
        .signers([authority])
        .rpc();

      console.log("\n✅ Quest updated. Tx:", tx);

      const locationQuests = await program.account.locationQuests.fetch(
        location1PDA
      );
      assert.equal(locationQuests.quests[0].title, updatedQuest.title);
      assert.deepEqual(locationQuests.quests[0].difficulty, { hard: {} });

      console.log("Updated quest:", {
        title: locationQuests.quests[0].title,
        difficulty: locationQuests.quests[0].difficulty,
      });
    });

    it("Should fail to update quest with invalid index", async () => {
      try {
        await program.methods
          .updateQuest(99, sampleQuest1) // Invalid index
          .accounts({
            registry: registryPDA,
            locationQuests: location1PDA,
            authority: authority.publicKey,
          })
          .signers([authority])
          .rpc();

        assert.fail("Should have failed with invalid index");
      } catch (error) {
        console.log("✅ Correctly failed with invalid index");
        expect(error.toString()).to.include("InvalidQuestIndex");
      }
    });
  });

  describe("5️⃣  Delete Quest", () => {
    it("Should delete a quest from location", async () => {
      const beforeQuests = await program.account.locationQuests.fetch(
        location1PDA
      );
      const beforeCount = beforeQuests.quests.length;
      const questToDelete = beforeQuests.quests[1];

      const tx = await program.methods
        .deleteQuest(1) // Delete second quest
        .accounts({
          registry: registryPDA,
          locationQuests: location1PDA,
          authority: authority.publicKey,
        })
        .signers([authority])
        .rpc();

      console.log("\n✅ Quest deleted. Tx:", tx);

      const afterQuests = await program.account.locationQuests.fetch(
        location1PDA
      );
      assert.equal(afterQuests.quests.length, beforeCount - 1);

      // Verify the deleted quest is not in the array
      const stillExists = afterQuests.quests.some(
        (q) => q.title === questToDelete.title
      );
      assert.equal(stillExists, false);

      console.log(
        "Quest count decreased from",
        beforeCount,
        "to",
        afterQuests.quests.length
      );
    });

    it("Should fail to delete quest with invalid index", async () => {
      try {
        await program.methods
          .deleteQuest(99) // Invalid index
          .accounts({
            registry: registryPDA,
            locationQuests: location1PDA,
            authority: authority.publicKey,
          })
          .signers([authority])
          .rpc();

        assert.fail("Should have failed with invalid index");
      } catch (error) {
        console.log("✅ Correctly failed with invalid index");
        expect(error.toString()).to.include("InvalidQuestIndex");
      }
    });
  });

  describe("6️⃣  Quest Data Validation", () => {
    it("Should validate quest with all quest types", async () => {
      const questTypes = [
        { discovery: {} },
        { exploration: {} },
        { challenge: {} },
      ];

      for (let i = 0; i < questTypes.length; i++) {
        const quest = {
          ...sampleQuest1,
          title: `Quest Type Test ${i}`,
          questType: questTypes[i],
        };

        const tx = await program.methods
          .addQuestToLocation(quest)
          .accounts({
            registry: registryPDA,
            locationQuests: location1PDA,
            authority: authority.publicKey,
          })
          .signers([authority])
          .rpc();

        console.log(`✅ Quest type ${Object.keys(questTypes[i])[0]} validated`);
      }
    });

    it("Should validate quest with all difficulty levels", async () => {
      const difficulties = [{ easy: {} }, { medium: {} }, { hard: {} }];

      for (let i = 0; i < difficulties.length; i++) {
        const quest = {
          ...sampleQuest1,
          title: `Difficulty Test ${i}`,
          difficulty: difficulties[i],
        };

        const tx = await program.methods
          .addQuestToLocation(quest)
          .accounts({
            registry: registryPDA,
            locationQuests: location1PDA,
            authority: authority.publicKey,
          })
          .signers([authority])
          .rpc();

        console.log(
          `✅ Difficulty ${Object.keys(difficulties[i])[0]} validated`
        );
      }
    });
  });

  describe("7️⃣  Final State Verification", () => {
    it("Should have correct final state", async () => {
      console.log("\n📊 Final State Report:");

      // Check registry
      const registry = await program.account.questRegistry.fetch(registryPDA);
      console.log("\n🏛️  Registry:");
      console.log("  Total Locations:", registry.totalLocations.toNumber());
      console.log("  Authority:", registry.authority.toString());

      // Check location 1
      const location1 = await program.account.locationQuests.fetch(
        location1PDA
      );
      console.log("\n📍 Location 1:", location1.location);
      console.log("  Total Quests:", location1.quests.length);
      console.log("  Initialized:", location1.initialized);
      location1.quests.forEach((quest, idx) => {
        console.log(`  Quest ${idx + 1}:`, quest.title);
      });

      // Check location 2
      const location2 = await program.account.locationQuests.fetch(
        location2PDA
      );
      console.log("\n📍 Location 2:", location2.location);
      console.log("  Total Quests:", location2.quests.length);
      console.log("  Initialized:", location2.initialized);
      location2.quests.forEach((quest, idx) => {
        console.log(`  Quest ${idx + 1}:`, quest.title);
      });

      // Final assertions
      assert.equal(registry.totalLocations.toNumber(), 2);
      assert.isTrue(location1.quests.length > 0);
      assert.isTrue(location2.quests.length > 0);

      console.log("\n✅ All tests passed successfully!");
    });
  });
});
