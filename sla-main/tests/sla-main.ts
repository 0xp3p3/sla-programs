import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SlaMain } from '../target/types/sla_main';

describe('sla-main', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SlaMain as Program<SlaMain>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
