// tests/calculator.ts
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Calculator } from "../target/types/calculator";

describe("calculator", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Calculator as Program<Calculator>;

  it("Toplama testi", async () => {
    const calculator = anchor.web3.Keypair.generate();
    await program.rpc.add(new anchor.BN(5), new anchor.BN(3), {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [calculator],
    });
    const account = await program.account.calculatorAccount.fetch(calculator.publicKey);
    console.log("Sonuç:", account.result.toString()); // 8
  });
});
