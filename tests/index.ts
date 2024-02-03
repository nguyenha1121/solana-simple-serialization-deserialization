import { use as chaiUse, expect } from 'chai'
import chaiAsPromised from 'chai-as-promised'
import fs from 'node:fs'
import web3 from '@solana/web3.js'
import * as borsh from 'borsh'

chaiUse(chaiAsPromised)

describe('simplify serialization & desirialization of transaction payload', () => {
  const programKeypair = web3.Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(fs.readFileSync('.internal/program-keypair.json').toString())
    )
  )
  const programId = programKeypair.publicKey
  console.log('program id', programId.toBase58())

  const connection = new web3.Connection('http://127.0.0.1:8899', {
    commitment: 'confirmed',
  })

  it('works', async () => {
    const signer = web3.Keypair.generate()
    const airdropSig = await connection.requestAirdrop(
      signer.publicKey,
      web3.LAMPORTS_PER_SOL
    )
    await connection.confirmTransaction(airdropSig)

    const schema = {
      // order of fields must be the same as in Solana Program struct
      struct: {
        flag: 'bool',
        msg: 'string',
        num64: 'u64',
        num32: 'u32',
        // arr: { array: { type: 'u8' } },
      },
    }
    const payload = Buffer.from(
      borsh.serialize(schema, {
        flag: true,
        num64: 1024,
        num32: 0,
        msg: 'works!',
      })
    )
    const variant = Buffer.from(borsh.serialize('u8', 0))
    const data = Buffer.concat([variant, payload])
    const tx = new web3.Transaction().add(
      new web3.TransactionInstruction({
        keys: [{ pubkey: signer.publicKey, isSigner: true, isWritable: true }],
        data,
        programId,
      })
    )

    await sendTx(connection, tx, [signer])
  })
})

function sendTx(connection, tx, signers) {
  return web3
    .sendAndConfirmTransaction(connection, tx, signers)
    .catch((err) => {
      console.error(err)
      throw err
    })
}
