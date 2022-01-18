import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { NewProgram } from '../target/types/sla_program';

describe('new-program', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SlaProgram as Program<SlaProgram>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
