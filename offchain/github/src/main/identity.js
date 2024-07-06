import { Secp256k1KeyIdentity } from '@dfinity/identity-secp256k1'

// Completely insecure seed phrase. Do not use for any purpose other than testing.
// Resolves to 'rwbxt-jvr66-qvpbz-2kbh3-u226q-w6djk-b45cp-66ewo-tpvng-thbkh-wae'
// When you are deploying your contract to ICP, you should change the seed out for something private.
const seed = 'test test test test test test test test test test test test'

export const identity = await Secp256k1KeyIdentity.fromSeedPhrase(seed)
