import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Basic }   from "../target/types/basic";

describe("basic", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Basic as Program<Basic>;

  it("Is initialized!", async () => {
    const seeds = []
    const [storage, _bump] = anchor.web3.PublicKey.findProgramAddressSync(seeds, program.programId);
    console.log("the storage account address is", storage.toBase58());

    const tx = await program.methods
                  .initialize()
                    .accounts({storage: storage})
                      .rpc();
    console.log("Your transaction signature :", tx);
    console.log("https://solana.fm/tx/"+tx);

    const txRead  = await program.methods.read().accounts({storage: storage}).rpc();
    let data = await program.account.myStorage.fetch(storage);
    console.log("data =", data.data.toString());

    const txWrite = await program.methods.write(new anchor.BN(1337)).accounts({storage: storage}).rpc();

    const txRead2 = await program.methods.read().accounts({storage: storage}).rpc();
    data = await program.account.myStorage.fetch(storage);
    console.log("data =", data.data.toString());

    const all = await program.account.myStorage.all();
    console.log(all);
  });

});
