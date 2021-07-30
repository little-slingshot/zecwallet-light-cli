use crate::compact_formats::CompactBlock;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use prost::Message;
use std::convert::TryFrom;
use std::io::{self, Read, Write};
use std::usize;
use zcash_primitives::memo::MemoBytes;

use crate::blaze::fixed_size_buffer::FixedSizeBuffer;
use zcash_primitives::{consensus::BlockHeight, zip32::ExtendedSpendingKey};
use zcash_primitives::{
    memo::Memo,
    merkle_tree::{CommitmentTree, IncrementalWitness},
    primitives::{Diversifier, Note, Nullifier, Rseed},
    sapling::Node,
    serialize::{Optional, Vector},
    transaction::{components::OutPoint, TxId},
    zip32::ExtendedFullViewingKey,
};

#[derive(Clone)]
pub struct CCompactTx {
    pub hash: [u8; 32],
    pub nfs: Vec<[u8; 32]>,
    pub cmus: Vec<[u8; 32]>,
}

impl CCompactTx {
    pub fn serialized_version() -> u64 {
        return 21;
    }

    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let _version = reader.read_u64::<LittleEndian>()?;

        let mut hash = [0u8; 32];
        reader.read_exact(&mut hash)?;

        let nfs = Vector::read(&mut reader, |r| {
            let mut nf = [0u8; 32];
            r.read_exact(&mut nf)?;

            Ok(nf)
        })?;

        let cmus = Vector::read(&mut reader, |r| {
            let mut cmu = [0u8; 32];
            r.read_exact(&mut cmu)?;

            Ok(cmu)
        })?;

        Ok(Self { hash, nfs, cmus })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u64::<LittleEndian>(Self::serialized_version())?;

        writer.write_all(&self.hash)?;

        Vector::write(&mut writer, &self.nfs, |w, nf| w.write_all(nf))?;
        Vector::write(&mut writer, &self.cmus, |w, cmu| w.write_all(cmu))?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct BlockData {
    pub hash: [u8; 32],
    pub prev_hash: [u8; 32],
    pub height: BlockHeight,
    pub time: u32,
    pub txns: Vec<CCompactTx>,
}

impl BlockData {
    pub fn serialized_version() -> u64 {
        return 21;
    }

    pub(crate) fn new_with(height: u64, hash_str: &str) -> Self {
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hex::decode(hash_str).unwrap().into_iter().rev().collect::<Vec<_>>()[..]);

        Self {
            hash,
            prev_hash: [0u8; 32],
            height: BlockHeight::from_u32(height as u32),
            time: 0,
            txns: vec![],
        }
    }

    pub(crate) fn new(cb: CompactBlock) -> Self {
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&cb.hash[..]);

        let mut prev_hash = [0u8; 32];
        prev_hash.copy_from_slice(&cb.prev_hash[..]);

        let mut txns = vec![];

        for ctx in &cb.vtx {
            let mut txhash = [0u8; 32];
            let mut nfs = vec![];
            let mut cmus = vec![];

            txhash.copy_from_slice(&ctx.hash[..]);
            for co in &ctx.outputs {
                let mut cmu = [0u8; 32];
                cmu.copy_from_slice(&co.cmu[..]);
                cmus.push(cmu);
            }

            for cs in &ctx.spends {
                let mut nf = [0u8; 32];
                nf.copy_from_slice(&cs.nf[..]);
                nfs.push(nf);
            }

            txns.push(CCompactTx {
                hash: txhash,
                nfs,
                cmus,
            });
        }

        Self {
            height: BlockHeight::from_u32(cb.height as u32),
            time: cb.time,
            hash,
            prev_hash,
            txns,
        }
    }
    pub fn height(&self) -> u64 {
        u64::from(self.height)
    }

    pub(crate) fn hash(&self) -> String {
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&self.hash);
        hash_bytes.reverse();

        hex::encode(hash_bytes)
    }

    #[cfg(test)]
    pub fn to_cb(&self) -> CompactBlock {
        use crate::compact_formats::{CompactOutput, CompactSpend, CompactTx};

        let mut vtx = vec![];

        for tx in &self.txns {
            let mut spends = vec![];
            let mut outputs = vec![];

            for nf in &tx.nfs {
                let mut cs = CompactSpend::default();
                cs.nf = nf.to_vec();
                spends.push(cs);
            }

            for cmu in &tx.cmus {
                let mut co = CompactOutput::default();
                co.cmu = cmu.to_vec();
                outputs.push(co);
            }

            let mut ctx = CompactTx::default();
            ctx.hash = tx.hash.to_vec();
            ctx.spends = spends;
            ctx.outputs = outputs;
            vtx.push(ctx);
        }

        let mut cb = CompactBlock::default();
        cb.hash = self.hash.to_vec();
        cb.prev_hash = self.prev_hash.to_vec();
        cb.time = self.time;
        cb.height = self.height();
        cb.vtx = vtx;

        return cb;
    }

    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let height = reader.read_i32::<LittleEndian>()? as u32;

        let mut hash = [0; 32];
        reader.read_exact(&mut hash)?;

        // We don't need this, but because of a quirk, the version is stored later, so we can't actually
        // detect the version here. So we write an empty tree and read it back here
        let tree = CommitmentTree::<Node>::read(&mut reader)?;
        let _tree = if tree.size() == 0 { None } else { Some(tree) };

        let version = reader.read_u64::<LittleEndian>()?;

        let (time, prev_hash, txns) = if version <= 20 {
            if version <= 11 {
                (0, [0u8; 32], vec![])
            } else {
                let ecb_bytes = Vector::read(&mut reader, |r| r.read_u8())?;
                match Message::decode(&ecb_bytes[..]) {
                    Ok(cb) => {
                        let b = Self::new(cb);
                        (b.time, b.prev_hash, b.txns)
                    }
                    Err(_) => (0, [0u8; 32], vec![]),
                }
            }
        } else {
            let mut prev_hash = [0; 32];
            reader.read_exact(&mut prev_hash)?;

            let time = reader.read_u32::<LittleEndian>()?;
            let txns = Vector::read(reader, |r| CCompactTx::read(r))?;

            (time, prev_hash, txns)
        };

        Ok(Self {
            height: BlockHeight::from_u32(height),
            hash,
            prev_hash,
            time,
            txns,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        let height: u32 = self.height.into();
        writer.write_i32::<LittleEndian>(height as i32)?;
        writer.write_all(&self.hash)?;

        CommitmentTree::<Node>::empty().write(&mut writer)?;
        writer.write_u64::<LittleEndian>(Self::serialized_version())?;

        // prev hash
        writer.write_all(&self.prev_hash)?;

        // time
        writer.write_u32::<LittleEndian>(self.time)?;

        Vector::write(writer, &self.txns, |w, ctx| ctx.write(w))?;

        Ok(())
    }
}

#[derive(Clone)]
pub(crate) struct WitnessCache {
    witnesses: Vec<IncrementalWitness<Node>>,
    pub(crate) top_height: u64,
}

impl WitnessCache {
    pub fn new(witnesses: Vec<IncrementalWitness<Node>>, top_height: u64) -> Self {
        Self { witnesses, top_height }
    }

    pub fn empty() -> Self {
        Self {
            witnesses: vec![],
            top_height: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.witnesses.len()
    }

    pub fn is_empty(&self) -> bool {
        self.witnesses.is_empty()
    }

    pub fn clear(&mut self) {
        self.witnesses.clear();
    }

    pub fn get(&self, i: usize) -> Option<&IncrementalWitness<Node>> {
        self.witnesses.get(i)
    }

    #[cfg(test)]
    pub fn get_from_last(&self, i: usize) -> Option<&IncrementalWitness<Node>> {
        self.witnesses.get(self.len() - i - 1)
    }

    pub fn last(&self) -> Option<&IncrementalWitness<Node>> {
        self.witnesses.last()
    }

    pub fn into_fsb(self, fsb: &mut FixedSizeBuffer<IncrementalWitness<Node>>) {
        self.witnesses.into_iter().for_each(|w| fsb.push(w));
    }

    pub fn pop(&mut self, at_height: u64) {
        while !self.witnesses.is_empty() && self.top_height >= at_height {
            self.witnesses.pop();
            self.top_height -= 1;
        }
    }

    // pub fn get_as_string(&self, i: usize) -> String {
    //     if i >= self.witnesses.len() {
    //         return "".to_string();
    //     }

    //     let mut buf = vec![];
    //     self.get(i).unwrap().write(&mut buf).unwrap();
    //     return hex::encode(buf);
    // }
}

pub struct SaplingNoteData {
    // Technically, this should be recoverable from the account number,
    // but we're going to refactor this in the future, so I'll write it again here.
    pub(super) extfvk: ExtendedFullViewingKey,

    pub diversifier: Diversifier,
    pub note: Note,

    // Witnesses for the last 100 blocks. witnesses.last() is the latest witness
    pub(crate) witnesses: WitnessCache,
    pub(super) nullifier: Nullifier,
    pub spent: Option<(TxId, u32)>, // If this note was confirmed spent

    // If this note was spent in a send, but has not yet been confirmed.
    // Contains the txid and height at which it was broadcast
    pub unconfirmed_spent: Option<(TxId, u32)>,
    pub memo: Option<Memo>,
    pub is_change: bool,

    // If the spending key is available in the wallet (i.e., whether to keep witness up-to-date)
    pub have_spending_key: bool,
}

// Reading a note also needs the corresponding address to read from.
fn read_rseed<R: Read>(mut reader: R) -> io::Result<Rseed> {
    let note_type = reader.read_u8()?;

    let mut r_bytes: [u8; 32] = [0; 32];
    reader.read_exact(&mut r_bytes)?;

    let r = match note_type {
        1 => Rseed::BeforeZip212(jubjub::Fr::from_bytes(&r_bytes).unwrap()),
        2 => Rseed::AfterZip212(r_bytes),
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Bad note type")),
    };

    Ok(r)
}

fn write_rseed<W: Write>(mut writer: W, rseed: &Rseed) -> io::Result<()> {
    let note_type = match rseed {
        Rseed::BeforeZip212(_) => 1,
        Rseed::AfterZip212(_) => 2,
    };
    writer.write_u8(note_type)?;

    match rseed {
        Rseed::BeforeZip212(fr) => writer.write_all(&fr.to_bytes()),
        Rseed::AfterZip212(b) => writer.write_all(b),
    }
}

impl SaplingNoteData {
    fn serialized_version() -> u64 {
        20
    }

    // Reading a note also needs the corresponding address to read from.
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let version = reader.read_u64::<LittleEndian>()?;

        let _account = if version <= 5 {
            reader.read_u64::<LittleEndian>()?
        } else {
            0
        };

        let extfvk = ExtendedFullViewingKey::read(&mut reader)?;

        let mut diversifier_bytes = [0u8; 11];
        reader.read_exact(&mut diversifier_bytes)?;
        let diversifier = Diversifier { 0: diversifier_bytes };

        // To recover the note, read the value and r, and then use the payment address
        // to recreate the note
        let (value, rseed) = if version <= 3 {
            let value = reader.read_u64::<LittleEndian>()?;

            let mut r_bytes: [u8; 32] = [0; 32];
            reader.read_exact(&mut r_bytes)?;

            let r = jubjub::Fr::from_bytes(&r_bytes).unwrap();

            (value, Rseed::BeforeZip212(r))
        } else {
            let value = reader.read_u64::<LittleEndian>()?;
            let rseed = read_rseed(&mut reader)?;

            (value, rseed)
        };

        let maybe_note = extfvk
            .fvk
            .vk
            .to_payment_address(diversifier)
            .unwrap()
            .create_note(value, rseed);

        let note = match maybe_note {
            Some(n) => Ok(n),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Couldn't create the note for the address",
            )),
        }?;

        let witnesses_vec = Vector::read(&mut reader, |r| IncrementalWitness::<Node>::read(r))?;
        let top_height = if version < 20 {
            0
        } else {
            reader.read_u64::<LittleEndian>()?
        };
        let witnesses = WitnessCache::new(witnesses_vec, top_height);

        let mut nullifier = [0u8; 32];
        reader.read_exact(&mut nullifier)?;
        let nullifier = Nullifier(nullifier);

        // Note that this is only the spent field, we ignore the unconfirmed_spent field.
        // The reason is that unconfirmed spents are only in memory, and we need to get the actual value of spent
        // from the blockchain anyway.
        let spent = if version <= 5 {
            let spent = Optional::read(&mut reader, |r| {
                let mut txid_bytes = [0u8; 32];
                r.read_exact(&mut txid_bytes)?;
                Ok(TxId { 0: txid_bytes })
            })?;

            let spent_at_height = if version >= 2 {
                Optional::read(&mut reader, |r| r.read_i32::<LittleEndian>())?
            } else {
                None
            };

            if spent.is_some() && spent_at_height.is_some() {
                Some((spent.unwrap(), spent_at_height.unwrap() as u32))
            } else {
                None
            }
        } else {
            Optional::read(&mut reader, |r| {
                let mut txid_bytes = [0u8; 32];
                r.read_exact(&mut txid_bytes)?;
                let height = r.read_u32::<LittleEndian>()?;
                Ok((TxId { 0: txid_bytes }, height))
            })?
        };

        let unconfirmed_spent = if version <= 4 {
            None
        } else {
            Optional::read(&mut reader, |r| {
                let mut txbytes = [0u8; 32];
                r.read_exact(&mut txbytes)?;

                let height = r.read_u32::<LittleEndian>()?;
                Ok((TxId { 0: txbytes }, height))
            })?
        };

        let memo = Optional::read(&mut reader, |r| {
            let mut memo_bytes = [0u8; 512];
            r.read_exact(&mut memo_bytes)?;

            // Attempt to read memo, first as text, else as arbitrary 512 bytes
            match MemoBytes::from_bytes(&memo_bytes) {
                Ok(mb) => match Memo::try_from(mb.clone()) {
                    Ok(m) => Ok(m),
                    Err(_) => Ok(Memo::Future(mb)),
                },
                Err(e) => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Couldn't create memo: {}", e),
                )),
            }
        })?;

        let is_change: bool = reader.read_u8()? > 0;

        let have_spending_key = if version <= 2 {
            true // Will get populated in the lightwallet::read() method, for now assume true
        } else {
            reader.read_u8()? > 0
        };

        Ok(SaplingNoteData {
            extfvk,
            diversifier,
            note,
            witnesses,
            nullifier,
            spent,
            unconfirmed_spent,
            memo,
            is_change,
            have_spending_key,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        // Write a version number first, so we can later upgrade this if needed.
        writer.write_u64::<LittleEndian>(Self::serialized_version())?;

        self.extfvk.write(&mut writer)?;

        writer.write_all(&self.diversifier.0)?;

        // Writing the note means writing the note.value and note.r. The Note is recoverable
        // from these 2 values and the Payment address.
        writer.write_u64::<LittleEndian>(self.note.value)?;

        write_rseed(&mut writer, &self.note.rseed)?;

        Vector::write(&mut writer, &self.witnesses.witnesses, |wr, wi| wi.write(wr))?;
        writer.write_u64::<LittleEndian>(self.witnesses.top_height)?;

        writer.write_all(&self.nullifier.0)?;

        Optional::write(&mut writer, &self.spent, |w, (txid, h)| {
            w.write_all(&txid.0)?;
            w.write_u32::<LittleEndian>(*h)
        })?;

        Optional::write(&mut writer, &self.unconfirmed_spent, |w, (txid, height)| {
            w.write_all(&txid.0)?;
            w.write_u32::<LittleEndian>(*height)
        })?;

        Optional::write(&mut writer, &self.memo, |w, m| w.write_all(m.encode().as_array()))?;

        writer.write_u8(if self.is_change { 1 } else { 0 })?;

        writer.write_u8(if self.have_spending_key { 1 } else { 0 })?;

        // Note that we don't write the unconfirmed_spent field, because if the wallet is restarted,
        // we don't want to be beholden to any expired txns

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Utxo {
    pub address: String,
    pub txid: TxId,
    pub output_index: u64,
    pub script: Vec<u8>,
    pub value: u64,
    pub height: i32,

    pub spent_at_height: Option<i32>,
    pub spent: Option<TxId>, // If this utxo was confirmed spent

    // If this utxo was spent in a send, but has not yet been confirmed.
    // Contains the txid and height at which the Tx was broadcast
    pub unconfirmed_spent: Option<(TxId, u32)>,
}

impl Utxo {
    pub fn serialized_version() -> u64 {
        return 3;
    }

    pub fn to_outpoint(&self) -> OutPoint {
        OutPoint::new(self.txid.0, self.output_index as u32)
    }

    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let version = reader.read_u64::<LittleEndian>()?;

        let address_len = reader.read_i32::<LittleEndian>()?;
        let mut address_bytes = vec![0; address_len as usize];
        reader.read_exact(&mut address_bytes)?;
        let address = String::from_utf8(address_bytes).unwrap();
        assert_eq!(address.chars().take(1).collect::<Vec<char>>()[0], 't');

        let mut txid_bytes = [0; 32];
        reader.read_exact(&mut txid_bytes)?;
        let txid = TxId { 0: txid_bytes };

        let output_index = reader.read_u64::<LittleEndian>()?;
        let value = reader.read_u64::<LittleEndian>()?;
        let height = reader.read_i32::<LittleEndian>()?;

        let script = Vector::read(&mut reader, |r| {
            let mut byte = [0; 1];
            r.read_exact(&mut byte)?;
            Ok(byte[0])
        })?;

        let spent = Optional::read(&mut reader, |r| {
            let mut txbytes = [0u8; 32];
            r.read_exact(&mut txbytes)?;
            Ok(TxId { 0: txbytes })
        })?;

        let spent_at_height = if version <= 1 {
            None
        } else {
            Optional::read(&mut reader, |r| r.read_i32::<LittleEndian>())?
        };

        let unconfirmed_spent = if version <= 2 {
            None
        } else {
            Optional::read(&mut reader, |r| {
                let mut txbytes = [0u8; 32];
                r.read_exact(&mut txbytes)?;

                let height = r.read_u32::<LittleEndian>()?;
                Ok((TxId { 0: txbytes }, height))
            })?
        };

        Ok(Utxo {
            address,
            txid,
            output_index,
            script,
            value,
            height,
            spent_at_height,
            spent,
            unconfirmed_spent,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u64::<LittleEndian>(Self::serialized_version())?;

        writer.write_u32::<LittleEndian>(self.address.as_bytes().len() as u32)?;
        writer.write_all(self.address.as_bytes())?;

        writer.write_all(&self.txid.0)?;

        writer.write_u64::<LittleEndian>(self.output_index)?;
        writer.write_u64::<LittleEndian>(self.value)?;
        writer.write_i32::<LittleEndian>(self.height)?;

        Vector::write(&mut writer, &self.script, |w, b| w.write_all(&[*b]))?;

        Optional::write(&mut writer, &self.spent, |w, txid| w.write_all(&txid.0))?;

        Optional::write(&mut writer, &self.spent_at_height, |w, s| {
            w.write_i32::<LittleEndian>(*s)
        })?;

        Optional::write(&mut writer, &self.unconfirmed_spent, |w, (txid, height)| {
            w.write_all(&txid.0)?;
            w.write_u32::<LittleEndian>(*height)
        })?;

        Ok(())
    }
}

#[derive(PartialEq)]
pub struct OutgoingTxMetadata {
    pub address: String,
    pub value: u64,
    pub memo: Memo,
}

impl OutgoingTxMetadata {
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let address_len = reader.read_u64::<LittleEndian>()?;
        let mut address_bytes = vec![0; address_len as usize];
        reader.read_exact(&mut address_bytes)?;
        let address = String::from_utf8(address_bytes).unwrap();

        let value = reader.read_u64::<LittleEndian>()?;

        let mut memo_bytes = [0u8; 512];
        reader.read_exact(&mut memo_bytes)?;
        let memo = match MemoBytes::from_bytes(&memo_bytes) {
            Ok(mb) => match Memo::try_from(mb.clone()) {
                Ok(m) => Ok(m),
                Err(_) => Ok(Memo::Future(mb)),
            },
            Err(e) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Couldn't create memo: {}", e),
            )),
        }?;

        Ok(OutgoingTxMetadata { address, value, memo })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        // Strings are written as len + utf8
        writer.write_u64::<LittleEndian>(self.address.as_bytes().len() as u64)?;
        writer.write_all(self.address.as_bytes())?;

        writer.write_u64::<LittleEndian>(self.value)?;
        writer.write_all(self.memo.encode().as_array())
    }
}

pub struct WalletTx {
    // Block in which this tx was included
    pub block: BlockHeight,

    // Is this Tx unconfirmed (i.e., not yet mined)
    pub unconfirmed: bool,

    // Timestamp of Tx. Added in v4
    pub datetime: u64,

    // Txid of this transaction. It's duplicated here (It is also the Key in the HashMap that points to this
    // WalletTx in LightWallet::txs)
    pub txid: TxId,

    // List of all nullifiers spent in this Tx. These nullifiers belong to the wallet.
    pub spent_nullifiers: Vec<Nullifier>,

    // List of all notes received in this tx. Some of these might be change notes.
    pub notes: Vec<SaplingNoteData>,

    // List of all Utxos received in this Tx. Some of these might be change notes
    pub utxos: Vec<Utxo>,

    // Total value of all the sapling nullifiers that were spent in this Tx
    pub total_sapling_value_spent: u64,

    // Total amount of transparent funds that belong to us that were spent in this Tx.
    pub total_transparent_value_spent: u64,

    // All outgoing sapling sends to addresses outside this wallet
    pub outgoing_metadata: Vec<OutgoingTxMetadata>,

    // Whether this TxID was downloaded from the server and scanned for Memos
    pub full_tx_scanned: bool,

    // Price of Zec when this Tx was created
    pub zec_price: Option<f64>,
}

impl WalletTx {
    pub fn serialized_version() -> u64 {
        return 21;
    }

    pub fn new_txid(txid: &Vec<u8>) -> TxId {
        let mut txid_bytes = [0u8; 32];
        txid_bytes.copy_from_slice(txid);
        TxId { 0: txid_bytes }
    }

    pub fn get_price(datetime: u64, price: &WalletZecPriceInfo) -> Option<f64> {
        match price.zec_price {
            None => None,
            Some((t, p)) => {
                // If the price was fetched within 24 hours of this Tx, we use the "current" price
                // else, we mark it as None, for the historical price fetcher to get
                if (t as i64 - datetime as i64).abs() < 24 * 60 * 60 {
                    Some(p)
                } else {
                    None
                }
            }
        }
    }

    pub fn new(height: BlockHeight, datetime: u64, txid: &TxId, unconfirmed: bool) -> Self {
        WalletTx {
            block: height,
            unconfirmed,
            datetime,
            txid: txid.clone(),
            spent_nullifiers: vec![],
            notes: vec![],
            utxos: vec![],
            total_transparent_value_spent: 0,
            total_sapling_value_spent: 0,
            outgoing_metadata: vec![],
            full_tx_scanned: false,
            zec_price: None,
        }
    }

    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let version = reader.read_u64::<LittleEndian>()?;

        let block = BlockHeight::from_u32(reader.read_i32::<LittleEndian>()? as u32);

        let unconfirmed = if version <= 20 { false } else { reader.read_u8()? == 1 };

        let datetime = if version >= 4 {
            reader.read_u64::<LittleEndian>()?
        } else {
            0
        };

        let mut txid_bytes = [0u8; 32];
        reader.read_exact(&mut txid_bytes)?;

        let txid = TxId { 0: txid_bytes };

        let notes = Vector::read(&mut reader, |r| SaplingNoteData::read(r))?;
        let utxos = Vector::read(&mut reader, |r| Utxo::read(r))?;

        let total_sapling_value_spent = reader.read_u64::<LittleEndian>()?;
        let total_transparent_value_spent = reader.read_u64::<LittleEndian>()?;

        // Outgoing metadata was only added in version 2
        let outgoing_metadata = Vector::read(&mut reader, |r| OutgoingTxMetadata::read(r))?;

        let full_tx_scanned = reader.read_u8()? > 0;

        let zec_price = if version <= 4 {
            None
        } else {
            Optional::read(&mut reader, |r| r.read_f64::<LittleEndian>())?
        };

        let spent_nullifiers = if version <= 5 {
            vec![]
        } else {
            Vector::read(&mut reader, |r| {
                let mut n = [0u8; 32];
                r.read_exact(&mut n)?;
                Ok(Nullifier(n))
            })?
        };

        Ok(Self {
            block,
            unconfirmed,
            datetime,
            txid,
            notes,
            utxos,
            spent_nullifiers,
            total_sapling_value_spent,
            total_transparent_value_spent,
            outgoing_metadata,
            full_tx_scanned,
            zec_price,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u64::<LittleEndian>(Self::serialized_version())?;

        let block: u32 = self.block.into();
        writer.write_i32::<LittleEndian>(block as i32)?;

        writer.write_u8(if self.unconfirmed { 1 } else { 0 })?;

        writer.write_u64::<LittleEndian>(self.datetime)?;

        writer.write_all(&self.txid.0)?;

        Vector::write(&mut writer, &self.notes, |w, nd| nd.write(w))?;
        Vector::write(&mut writer, &self.utxos, |w, u| u.write(w))?;

        writer.write_u64::<LittleEndian>(self.total_sapling_value_spent)?;
        writer.write_u64::<LittleEndian>(self.total_transparent_value_spent)?;

        // Write the outgoing metadata
        Vector::write(&mut writer, &self.outgoing_metadata, |w, om| om.write(w))?;

        writer.write_u8(if self.full_tx_scanned { 1 } else { 0 })?;

        Optional::write(&mut writer, &self.zec_price, |w, p| w.write_f64::<LittleEndian>(*p))?;

        Vector::write(&mut writer, &self.spent_nullifiers, |w, n| w.write_all(&n.0))?;

        Ok(())
    }
}

pub struct SpendableNote {
    pub txid: TxId,
    pub nullifier: Nullifier,
    pub diversifier: Diversifier,
    pub note: Note,
    pub witness: IncrementalWitness<Node>,
    pub extsk: ExtendedSpendingKey,
}

impl SpendableNote {
    pub fn from(
        txid: TxId,
        nd: &SaplingNoteData,
        anchor_offset: usize,
        extsk: &Option<ExtendedSpendingKey>,
    ) -> Option<Self> {
        // Include only notes that haven't been spent, or haven't been included in an unconfirmed spend yet.
        if nd.spent.is_none()
            && nd.unconfirmed_spent.is_none()
            && extsk.is_some()
            && nd.witnesses.len() >= (anchor_offset + 1)
        {
            let witness = nd.witnesses.get(nd.witnesses.len() - anchor_offset - 1);

            witness.map(|w| SpendableNote {
                txid,
                nullifier: nd.nullifier,
                diversifier: nd.diversifier,
                note: nd.note.clone(),
                witness: w.clone(),
                extsk: extsk.clone().unwrap(),
            })
        } else {
            None
        }
    }
}

// Struct that tracks the latest and historical price of ZEC in the wallet
#[derive(Clone, Debug)]
pub struct WalletZecPriceInfo {
    // Latest price of ZEC and when it was fetched
    pub zec_price: Option<(u64, f64)>,

    // Wallet's currency. All the prices are in this currency
    pub currency: String,

    // When the last time historical prices were fetched
    pub last_historical_prices_fetched_at: Option<u64>,

    // Historical prices retry count
    pub historical_prices_retry_count: u64,
}

impl WalletZecPriceInfo {
    pub fn new() -> Self {
        Self {
            zec_price: None,
            currency: "USD".to_string(), // Only USD is supported right now.
            last_historical_prices_fetched_at: None,
            historical_prices_retry_count: 0,
        }
    }

    pub fn serialized_version() -> u64 {
        return 20;
    }

    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let version = reader.read_u64::<LittleEndian>()?;
        if version > Self::serialized_version() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Can't read ZecPriceInfo because of incorrect version",
            ));
        }

        // The "current" zec price is not persisted, since it is almost certainly outdated
        let zec_price = None;

        // Currency is only USD for now
        let currency = "USD".to_string();

        let last_historical_prices_fetched_at = Optional::read(&mut reader, |r| r.read_u64::<LittleEndian>())?;
        let historical_prices_retry_count = reader.read_u64::<LittleEndian>()?;

        Ok(Self {
            zec_price,
            currency,
            last_historical_prices_fetched_at,
            historical_prices_retry_count,
        })
    }

    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_u64::<LittleEndian>(Self::serialized_version())?;

        // We don't write the currency zec price or the currency yet.
        Optional::write(&mut writer, &self.last_historical_prices_fetched_at, |w, t| {
            w.write_u64::<LittleEndian>(*t)
        })?;
        writer.write_u64::<LittleEndian>(self.historical_prices_retry_count)?;

        Ok(())
    }
}
