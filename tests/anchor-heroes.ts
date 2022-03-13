import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorHeroes } from '../target/types/anchor_heroes';
import assert from "assert";

const {SystemProgram, Keypair} = anchor.web3
describe('anchor-heroes', () => {

  const provider = anchor.Provider.env()
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorHeroes as Program<AnchorHeroes>;
  const heroAccount = Keypair.generate();

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({
      accounts: {
        heroAccount: heroAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId
      },
      signers: [heroAccount]
    });
    console.log("Your transaction signature", tx);
    const account = await program.account.heroAccount.fetch(heroAccount.publicKey);

    assert.ok(account.level.toString() == '1')
  });

  it('Is leveled up!', async () => {
    const tx = await program.rpc.levelUp({
      accounts: {
        heroAccount: heroAccount.publicKey,
      }
    });
    console.log("Your transaction signature", tx);
    const account = await program.account.heroAccount.fetch(heroAccount.publicKey);

    assert.ok(account.level.toString() == '2')
  });
});


