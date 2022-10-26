## Taproot Output with Tapret Commitment

### ONLY FOR DEVELOPMENT and TESTING. These tools may not be suitable for production deployments.

### Notes
- The main goal for this repository is to test the fix to the taproot script after adding tapret commitment.
- This repo contains two tests:
    - the `verify_rgb_tapret_output` reproduce the output address generate after `rgb-cli transfer finalize` command.
    - the `create_spend_tx_for_taproot_tapret_output` create and sign spend tx using tx generated after `btc-cold finalize --publish` command.
- More details and motivation [here](https://github.com/BP-WG/bp-core/pull/20)

### Prerequisites
- Bitcoin Core Node
- Demo RGB instructions [here](https://github.com/LNP-BP/nodes/blob/master/contrib/demo-rgb.sh) 
- Create a folder `test-tapret`
- Clone `descriptor-wallet` https://github.com/crisdut/descriptor-wallet inside `test-tapret` folder and compiling the `exp\tapret` branch
- Clone `bp-code` https://github.com/crisdut/bp-core inside `test-tapret` folder and compiling the `exp\tapret` branch
- Clone `rgb-node` https://github.com/crisdut/rgb-node inside `test-tapret` folder and compiling the `exp\tapret` branch
    - cargo build --all-features --locked

### Instructions
1. Follows the instructions [here](https://github.com/LNP-BP/nodes/blob/master/contrib/demo-rgb.sh#L118) to create the PSBT file.
    - Please, change the official binaries to custom build generated in *Prerequisites* section.
    - Copy the following information printed in your terminal: 
        - _Change Script_
        - _Desc Internal Key_
        - _Desc Only PubKey_. 
2. Fill the variables and run the follow script:
3. Follows the instructions [here](https://github.com/LNP-BP/nodes/blob/master/contrib/demo-rgb.sh#L145) to finalize PSBT and Consginment File.
    - Please, change the official binaries to custom build generated in *Prerequisites* section.
    - Copy the following the information printed in your terminal: 
        - _Internal Key_ 
        - _Taptweak_
        - _Checksig Script_
        - _Commitment Script_
        - _Final Script_
        - _Control Block_
4. Follows the instructions [here](https://github.com/LNP-BP/nodes/blob/master/contrib/demo-rgb.sh#L155) to sign and publish transaction.
    - Copy the following information printed in your terminal:
        - The transaction hexadecimal
        - The transaction ID 
5. Go to `verify_rgb_tapret_output` method. Update variables and run the test. Check the output information
6. Go to `create_spend_tx_for_taproot_tapret_output` method. Update variables and run the test. Copy the spending transaction hexadecimal.
7. Run `bitcoin-cli sendrawtransaction SPEND_TX_HEX`.
8. If everything works, the bitcoin node returns the Transaction ID. 

### Tips
_How discover the Private Key of my Internal Key?_
```lang=rust
    let password = "";
    let secp = Secp256k1::new();

    let file = fs::File::open(account_path)?; # <-- Use the file generate with 'btc-hot derive' command 
    let account = MemorySigningAccount::read(&secp, file, password.as_deref())?;

    println!("Signing with {}\n", account.to_account());

    let data = fs::read(psbt_path)?;
    let mut psbt = Psbt::deserialize(&data)?;

    let mut key_provider = MemoryKeyProvider::with(&secp, musig);
    key_provider.add_account(account);

    let fp = ""; # <-- Fingerprint
    let dv = "m/86h/1h/0h/1/0";
    let xpb = ""; # <-- Internal PubKey

    let fingerprint = Fingerprint::from_str(fp).unwrap();
    let derivation = DerivationPath::from_str(dv).unwrap();
    let pubkey = XOnlyPublicKey::from_str(xpb).unwrap();

    let kp = SecretProvider::key_pair(&key_provider, fingerprint, &derivation, pubkey).unwrap();
    println!("Pub: {}", kp.public_key());
    println!("Prv: {}", kp.display_secret()); 
```

### Conclusion
If you do have any questions, please open an issue.

Thanks for feedback!