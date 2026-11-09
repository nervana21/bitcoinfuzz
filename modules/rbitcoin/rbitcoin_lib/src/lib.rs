//! C ABI for bitcoinfuzz: rbitcoin-consensus script_eval / verify_script.
//!
//! Core's harness uses a fuzzed signature checker (CHECKSIG always passes).
//! This module runs real ECDSA verify, so inputs that hit CHECKSIG may
//! disagree with BITCOIN_CORE by design until a mock checker exists upstream.

use bitcoin::absolute::LockTime;
use bitcoin::hashes::Hash;
use bitcoin::script::Script;
use bitcoin::transaction::Version as TxVersion;
use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Witness};
use rbitcoin_consensus::block::{InputIndex, ScriptCheckJob, ScriptVerifyFlags};
use rbitcoin_consensus::script::interpreter::{eval_script, EvalContext, SigVersion};
use rbitcoin_consensus::script::verify_job_all_inputs;
use rbitcoin_consensus::TxScriptPrecompute;
use std::borrow::Cow;
use std::slice;

/// SCRIPT_VERIFY_NONE-shaped flags (all policy / buried forks off).
fn verify_none_flags() -> ScriptVerifyFlags {
    ScriptVerifyFlags {
        bip65_active: false,
        bip112_active: false,
        bip66_active: false,
        bip16_active: false,
        taproot_active: false,
        minimal_if: false,
        nullfail: false,
        low_s: false,
        strictenc: false,
        null_dummy: false,
        minimal_data: false,
        witness_pubkeytype: false,
        witness_active: false,
        discourage_upgradable_witness: false,
        const_scriptcode: false,
        cleanstack: false,
    }
}

fn dummy_tx_with_script_sig(script_sig: ScriptBuf) -> Transaction {
    Transaction {
        version: TxVersion::ONE,
        lock_time: LockTime::ZERO,
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: bitcoin::Txid::from_byte_array([1; 32]),
                vout: 0,
            },
            script_sig,
            sequence: Sequence::MAX,
            witness: Witness::new(),
        }],
        output: vec![TxOut {
            value: Amount::from_sat(49_0000_0000),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]),
        }],
    }
}

/// 1 = true, 0 = false, -1 = skip.
#[no_mangle]
pub unsafe extern "C" fn rbitcoin_script_eval(
    data: *const u8,
    len: usize,
    _flags: u32,
    version: usize,
) -> i8 {
    if data.is_null() || len == 0 {
        return -1;
    }
    let bytes = slice::from_raw_parts(data, len);
    let script = Script::from_bytes(bytes);
    let sig_version = if version == 0 {
        SigVersion::Base
    } else {
        SigVersion::WitnessV0
    };

    let tx = dummy_tx_with_script_sig(ScriptBuf::new());
    let Ok(input_index) = InputIndex::in_tx(&tx, 0) else {
        return 0;
    };
    let ctx = EvalContext::from_eval_parts(
        &tx,
        input_index,
        Amount::from_sat(50_0000_0000),
        script,
        sig_version,
        false,
        false,
        false,
        Cow::Owned(TxScriptPrecompute::from_tx(&tx)),
    );
    let mut stack: Vec<Vec<u8>> = Vec::new();
    match eval_script(script, &mut stack, &ctx) {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

/// 1 = true, 0 = false, -1 = skip.
#[no_mangle]
pub unsafe extern "C" fn rbitcoin_verify_script(
    script_sig: *const u8,
    script_sig_len: usize,
    script_pubkey: *const u8,
    script_pubkey_len: usize,
) -> i8 {
    if script_sig.is_null()
        || script_pubkey.is_null()
        || script_sig_len == 0
        || script_pubkey_len == 0
    {
        return -1;
    }
    let sig = slice::from_raw_parts(script_sig, script_sig_len);
    let pk = slice::from_raw_parts(script_pubkey, script_pubkey_len);

    let prevouts = vec![TxOut {
        value: Amount::from_sat(50_0000_0000),
        script_pubkey: ScriptBuf::from_bytes(pk.to_vec()),
    }];
    let tx = dummy_tx_with_script_sig(ScriptBuf::from_bytes(sig.to_vec()));
    let Ok(job) = ScriptCheckJob::try_new(prevouts, tx, verify_none_flags()) else {
        return 0;
    };
    match verify_job_all_inputs(&job) {
        Ok(()) => 1,
        Err(_) => 0,
    }
}
