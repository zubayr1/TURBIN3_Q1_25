import * as anchor from '@coral-xyz/anchor'
import {Program} from '@coral-xyz/anchor'
import {Keypair} from '@solana/web3.js'
import {OneCs} from '../target/types/one_cs'

describe('one_cs', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.OneCs as Program<OneCs>

  const one_csKeypair = Keypair.generate()

  it('Initialize OneCs', async () => {
    await program.methods
      .initialize()
      .accounts({
        one_cs: one_csKeypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([one_csKeypair])
      .rpc()

    const currentCount = await program.account.one_cs.fetch(one_csKeypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment OneCs', async () => {
    await program.methods.increment().accounts({ one_cs: one_csKeypair.publicKey }).rpc()

    const currentCount = await program.account.one_cs.fetch(one_csKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment OneCs Again', async () => {
    await program.methods.increment().accounts({ one_cs: one_csKeypair.publicKey }).rpc()

    const currentCount = await program.account.one_cs.fetch(one_csKeypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement OneCs', async () => {
    await program.methods.decrement().accounts({ one_cs: one_csKeypair.publicKey }).rpc()

    const currentCount = await program.account.one_cs.fetch(one_csKeypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set one_cs value', async () => {
    await program.methods.set(42).accounts({ one_cs: one_csKeypair.publicKey }).rpc()

    const currentCount = await program.account.one_cs.fetch(one_csKeypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the one_cs account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        one_cs: one_csKeypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.one_cs.fetchNullable(one_csKeypair.publicKey)
    expect(userAccount).toBeNull()
  })
})
