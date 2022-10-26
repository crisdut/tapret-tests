#[cfg(test)]
#[allow(unused_imports)]
#[allow(non_snake_case)]
mod tests {
    use bitcoin::consensus::deserialize;
    use bitcoin::psbt::serialize::Serialize;
    use std::str::FromStr;

    use bitcoin::util::sighash::{Prevouts, ScriptPath};
    use bitcoin::util::taproot::{self, TaprootSpendInfo};
    use bitcoin::{
        blockdata::opcodes, util::address::WitnessVersion, Address, Script, XOnlyPublicKey,
    };
    use bitcoin::{
        KeyPair, OutPoint, PublicKey, SchnorrSighashType, Transaction, TxIn, TxOut, Witness,
    };
    use bitcoin_hashes::hex::{FromHex, ToHex};
    use bp::dbc::tapret;
    use commit_verify::{lnpbp4, CommitVerify};
    use secp256k1::{All, Message, Secp256k1};

    #[test]
    fn verify_rgb_tapret_output() {
        /*
        – <Root> (depth 0)
            – A (depth 1)
            – B (depth 1) <--- Tapret
         */

        // -------- CHANGE HERE ------------
        let YOUR_PRIVATE_KEY = "";
        let YOUR_COMMITMENT_SCRIPT_HEX = "";

        // TESTS ASSERT VARIABLES
        let INTERNAL_KEY_EXPECTED = "";
        let CHECKSIG_SCRIPT_HEX_EXPECTED = "";
        let OUTPUT_SCRIPT_HEX_EXPECTED = "";
        let CB_CHECKSIG_HEX_EXPECTED = "";

        let secp = Secp256k1::new();
        let keypair = KeyPair::from_seckey_str(&secp, YOUR_PRIVATE_KEY).unwrap();

        let internal_key = XOnlyPublicKey::from_keypair(&keypair);
        assert_eq!(internal_key.clone().to_hex(), INTERNAL_KEY_EXPECTED);

        let builder = bitcoin::blockdata::script::Builder::new();
        let script_a = builder
            .push_slice(&internal_key.serialize())
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();

        assert_eq!(script_a.to_hex(), CHECKSIG_SCRIPT_HEX_EXPECTED);

        let script_b_hex = YOUR_COMMITMENT_SCRIPT_HEX;
        let script_b = Script::from_hex(script_b_hex).unwrap();

        // add leaves in depth-first order
        let builder = taproot::TaprootBuilder::new();
        let builder = builder.add_leaf(1, script_a.clone()).unwrap();
        let builder = builder.add_leaf(1, script_b.clone()).unwrap();

        let secp = Secp256k1::verification_only();
        let tap_tree = builder.finalize(&secp, internal_key).unwrap();

        let spend_info =
            TaprootSpendInfo::new_key_spend(&secp, internal_key, tap_tree.merkle_root());

        let final_script =
            Script::new_witness_program(WitnessVersion::V1, &spend_info.output_key().serialize());

        assert_eq!(final_script.to_hex(), OUTPUT_SCRIPT_HEX_EXPECTED);

        let versioned_script_a = (script_a, taproot::LeafVersion::TapScript);
        let control_a = tap_tree
            .control_block(&versioned_script_a)
            .unwrap()
            .serialize()
            .to_hex();

        assert_eq!(control_a, CB_CHECKSIG_HEX_EXPECTED);

        let address = Address::from_script(&final_script, bitcoin::Network::Regtest).unwrap();
        println!("Address: {}", address.to_string());
    }

    #[test]
    fn create_spend_tx_for_tapret_output() {
        // -------- CHANGE HERE ------------
        let YOUR_PRIVATE_KEY = "";

        let YOUR_SOURCE_TX_HEX = "";
        let YOUR_COMMITMENT_SCRIPT_HEX = "";

        let YOUR_OUTPUT_FEE = 0;
        let YOUR_OUTPUT_SPEND = 0;
        let YOUR_OUTPUT_PUBKEY = "";

        // TESTS ASSERT VARIABLES
        let INTERNAL_KEY_EXPECTED = "";
        let SOURCE_TX_ID_EXPECTED = "";

        let CHECKSIG_SCRIPT_HEX_EXPECTED = "";

        let secp = Secp256k1::new();
        let source_tx: Transaction =
            deserialize(&Vec::from_hex(YOUR_SOURCE_TX_HEX).unwrap()).unwrap();
        assert_eq!(source_tx.txid().to_hex(), SOURCE_TX_ID_EXPECTED);

        let keypair = KeyPair::from_seckey_str(&secp, YOUR_PRIVATE_KEY).unwrap();
        let internal_key = XOnlyPublicKey::from_keypair(&keypair);
        assert_eq!(internal_key.clone().to_hex(), INTERNAL_KEY_EXPECTED);

        let builder = bitcoin::blockdata::script::Builder::new();
        let script_a = builder
            .push_slice(&internal_key.serialize())
            .push_opcode(opcodes::all::OP_CHECKSIG)
            .into_script();

        assert_eq!(script_a.to_hex(), CHECKSIG_SCRIPT_HEX_EXPECTED);

        let script_b_hex = YOUR_COMMITMENT_SCRIPT_HEX;
        let script_b = Script::from_hex(script_b_hex).unwrap();

        // add leaves in depth-first order
        let builder = taproot::TaprootBuilder::new();
        let builder = builder.add_leaf(1, script_a.clone()).unwrap();
        let builder = builder.add_leaf(1, script_b.clone()).unwrap();

        let secp = Secp256k1::verification_only();
        let tap_tree = builder.finalize(&secp, internal_key).unwrap();

        let fee = YOUR_OUTPUT_FEE;
        let spend = YOUR_OUTPUT_SPEND;

        let pubkey_output = PublicKey::from_str(YOUR_OUTPUT_PUBKEY).unwrap();
        let prevout = source_tx
            .output
            .iter()
            .find(|out| out.value == spend)
            .unwrap();
        let prevout_index = source_tx.output.iter().position(|r| r == prevout).unwrap();

        let mut spending_tx = Transaction {
            version: source_tx.version,
            lock_time: 0,
            input: vec![TxIn {
                previous_output: OutPoint::new(source_tx.txid(), prevout_index.try_into().unwrap()),
                script_sig: Default::default(),
                sequence: 0,
                witness: Witness::default(),
            }],
            output: vec![TxOut {
                value: spend - fee,
                script_pubkey: Script::new_witness_program(
                    WitnessVersion::V1,
                    &pubkey_output.serialize(),
                ),
            }],
        };

        let prevouts = [prevout.clone()];
        let mut sighash_cache = bitcoin::util::sighash::SighashCache::new(&spending_tx);
        let signature_hash = sighash_cache
            .taproot_script_spend_signature_hash(
                0,
                &Prevouts::All(&prevouts),
                ScriptPath::with_defaults(&script_a),
                SchnorrSighashType::Default,
            )
            .unwrap();

        let message = Message::from_slice(&signature_hash.to_vec()).unwrap();

        let secp_sign = Secp256k1::new();
        let signature = secp_sign.sign_schnorr_no_aux_rand(&message, &keypair);
        assert_eq!(
            true,
            signature.verify(&message, &keypair.public_key()).is_ok()
        );

        let versioned_script_a = (script_a.clone(), taproot::LeafVersion::TapScript);
        let control_vec = tap_tree
            .control_block(&versioned_script_a)
            .unwrap()
            .serialize();

        let signature_vec = signature.as_ref().to_vec();

        let witness = vec![
            signature_vec.clone(),
            script_a.serialize().to_vec(),
            control_vec.clone(),
        ];

        spending_tx.input[0].witness = Witness::from_vec(witness);
        println!("TxId: {}", spending_tx.txid().to_hex());

        println!("TX: {}", spending_tx.serialize().to_hex());
    }
}
