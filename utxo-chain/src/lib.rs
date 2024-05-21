use ring::digest;
use std::fmt::Display;
use std::io::Read;

fn double_sha256(raw: &[u8]) -> Hash {
    let first_sha = digest::digest(&digest::SHA256, raw);

    digest::digest(&digest::SHA256, first_sha.as_ref())
        .as_ref()
        .try_into()
        .unwrap()
}

trait Extract {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub version: u32,
    pub previous_hash: Hash,
    pub merkle_hash: Hash,
    pub time: u32,
    pub bits: u32,
    pub nonce: u32,
}

impl Extract for Header {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        let version = Extract::extract(blockchain, raw)?;
        let previous_hash = Extract::extract(blockchain, raw)?;
        let merkle_hash = Extract::extract(blockchain, raw)?;
        let time = Extract::extract(blockchain, raw)?;
        let bits = Extract::extract(blockchain, raw)?;
        let nonce = Extract::extract(blockchain, raw)?;

        Some(Self {
            version,
            previous_hash,
            merkle_hash,
            time,
            bits,
            nonce,
        })
    }
}

pub fn hash_str(hash: &[u8]) -> String {
    hash.iter()
        .map(|elem| format!("{elem:02X}"))
        .fold(String::new(), |acc, s| acc + &s)
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version = self.version;
        let previous_hash = hash_str(&self.previous_hash); // hash str
        let merkle_hash = hash_str(&self.merkle_hash); // hash str
        let time = self.time;
        let bits = self.bits;
        let nonce = self.nonce;

        writeln!(
            f,
            "Version:        {version} 
Previous Hash:  {previous_hash}
Merkle Root:    {merkle_hash}
Time:           {time}
Difficulty:     {bits}
Nonce:          {nonce}"
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxInput {
    pub prev_hash: Hash,
    pub out_id: u32,
    pub script_sig: Vec<u8>,
    pub seq_no: u32,
}

impl Extract for TxInput {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        let prev_hash = Extract::extract(blockchain, raw)?;
        let out_id = Extract::extract(blockchain, raw)?;
        let script_len: u8 = Extract::extract(blockchain, raw)?;
        let script_sig = extract_bytes_vec(blockchain, raw, script_len as usize)?;
        let seq_no = Extract::extract(blockchain, raw)?;

        Some(Self {
            prev_hash,
            out_id,
            script_sig,
            seq_no,
        })
    }
}

impl Display for TxInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prev_hash = hash_str(&self.prev_hash);
        let out_id = self.out_id;
        let script_sig = hash_str(&self.script_sig);
        let script_len = script_sig.len();
        let seq_no = self.seq_no;

        writeln!(
            f,
            "    Previous hash:  {prev_hash}
    Tx Out Index:   {out_id:x}
    Script Length:  {script_len}
    Script Sig:     {script_sig}
    Sequence:       {seq_no:x}"
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TxOutput {
    pub value: u64,
    pub pubkey: Vec<u8>,
}

impl Extract for TxOutput {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        let value = Extract::extract(blockchain, raw)?;
        let script_len: u8 = Extract::extract(blockchain, raw)?;
        let pubkey = extract_bytes_vec(blockchain, raw, script_len as usize)?;

        Some(Self { value, pubkey })
    }
}

impl Display for TxOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.value;
        let script_len = self.pubkey.len();
        let pubkey = hash_str(&self.pubkey);

        writeln!(
            f,
            "    Value:      {value}
    Script Len: {script_len}
    Pubkey:     {pubkey}"
        )
    }
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: Hash,
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub lock_time: u32,
}

impl Extract for Transaction {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        raw.truncate(0);

        let version = Extract::extract(blockchain, raw)?;

        let in_count: u8 = Extract::extract(blockchain, raw)?;

        let inputs = (0..in_count)
            .map(|_| <TxInput as Extract>::extract(blockchain, raw))
            .collect::<Option<_>>()?;

        let out_count: u8 = Extract::extract(blockchain, raw)?;
        let outputs = (0..out_count)
            .map(|_| <TxOutput as Extract>::extract(blockchain, raw))
            .collect::<Option<_>>()?;

        let lock_time = Extract::extract(blockchain, raw)?;

        let mut id = double_sha256(raw);
        id.reverse();

        Some(Self {
            id,
            version,
            inputs,
            outputs,
            lock_time,
        })
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version = self.version;
        let id = hash_str(&self.id);
        writeln!(
            f,
            "========== New Transaction ==========
Tx Version: {version}
Tx Hash:    {id}"
        )?;

        let in_count = self.inputs.len();
        let inputs = &self.inputs;

        writeln!(f, "Inputs: {in_count}")?;
        for input in inputs {
            writeln!(f, "{input}")?;
        }

        let out_count = self.outputs.len();
        let outputs = &self.outputs;

        writeln!(f, "Outputs: {out_count}")?;
        for output in outputs {
            writeln!(f, "{output}")?;
        }

        let lock_time = self.lock_time;
        writeln!(f, "Lock Time: {lock_time}")
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    pub magic_num: u32,
    pub size: u32,
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let magic_num = self.magic_num;
        let size = self.size;
        let header = self.header;
        let transactions = self.transactions.clone();
        let tx_count = transactions.len();

        writeln!(
            f,
            "Magick num: {magic_num:x}
Block size: {size:5}

########## Block header ##########
{header}

Transaction count: {tx_count:5}"
        )?;

        for transaction in transactions {
            writeln!(f, "{transaction}")?;
        }

        Ok(())
    }
}

impl Block {
    pub fn read_from(blockchain: &mut impl Read) -> Option<Self> {
        let mut raw = vec![];

        let magic_num = Extract::extract(blockchain, &mut raw)?;
        let size = Extract::extract(blockchain, &mut raw)?;
        let header = Extract::extract(blockchain, &mut raw)?;

        let tx_count: u8 = Extract::extract(blockchain, &mut raw)?;

        let mut transactions = Vec::with_capacity(tx_count as usize);

        for _ in 0..tx_count {
            let transaction = Extract::extract(blockchain, &mut raw)?;
            transactions.push(transaction);
        }

        Some(Self {
            magic_num,
            size,
            header,
            transactions,
        })
    }
}

fn extract_bytes<const N: usize>(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<[u8; N]> {
    let mut buff = [0; N];
    blockchain.read_exact(&mut buff).ok()?;
    raw.extend(buff);
    Some(buff)
}

fn extract_bytes_vec(blockchain: &mut impl Read, raw: &mut Vec<u8>, n: usize) -> Option<Vec<u8>> {
    let mut buff = vec![0; n];
    blockchain.read_exact(&mut buff).ok()?;
    raw.extend(buff.clone());
    Some(buff)
}

impl Extract for u8 {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        extract_bytes::<1>(blockchain, raw).map(u8::from_be_bytes)
    }
}

impl Extract for u16 {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        extract_bytes::<2>(blockchain, raw).map(u16::from_le_bytes)
    }
}

impl Extract for u32 {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        extract_bytes::<4>(blockchain, raw).map(u32::from_le_bytes)
    }
}

impl Extract for u64 {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        extract_bytes::<8>(blockchain, raw).map(u64::from_le_bytes)
    }
}

pub type Hash = [u8; 32];

impl Extract for Hash {
    fn extract(blockchain: &mut impl Read, raw: &mut Vec<u8>) -> Option<Self> {
        let mut bytes = extract_bytes::<32>(blockchain, raw)?;
        bytes.reverse();
        Some(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_sha256() {
        let input = b"hello";

        assert_eq!(
            double_sha256(input),
            [
                0x95, 0x95, 0xc9, 0xdf, 0x90, 0x07, 0x51, 0x48, 0xeb, 0x06, 0x86, 0x03, 0x65, 0xdf,
                0x33, 0x58, 0x4b, 0x75, 0xbf, 0xf7, 0x82, 0xa5, 0x10, 0xc6, 0xcd, 0x48, 0x83, 0xa4,
                0x19, 0x83, 0x3d, 0x50
            ]
        );

        let input = b"world";

        assert_eq!(
            double_sha256(input),
            [
                0x63, 0xe5, 0xc1, 0x63, 0xc8, 0x1e, 0xe9, 0xa3, 0xed, 0x99, 0xd3, 0x65, 0xff, 0x96,
                0x3e, 0xce, 0xa3, 0x40, 0xcc, 0x45, 0x5d, 0xee, 0xac, 0x7c, 0x4b, 0x63, 0xac, 0x75,
                0xb9, 0xcf, 0x47, 0x06
            ]
        );

        let input = [
            0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x4D,
            0x04, 0xFF, 0xFF, 0x00, 0x1D, 0x01, 0x04, 0x45, 0x54, 0x68, 0x65, 0x20, 0x54, 0x69,
            0x6D, 0x65, 0x73, 0x20, 0x30, 0x33, 0x2F, 0x4A, 0x61, 0x6E, 0x2F, 0x32, 0x30, 0x30,
            0x39, 0x20, 0x43, 0x68, 0x61, 0x6E, 0x63, 0x65, 0x6C, 0x6C, 0x6F, 0x72, 0x20, 0x6F,
            0x6E, 0x20, 0x62, 0x72, 0x69, 0x6E, 0x6B, 0x20, 0x6F, 0x66, 0x20, 0x73, 0x65, 0x63,
            0x6F, 0x6E, 0x64, 0x20, 0x62, 0x61, 0x69, 0x6C, 0x6F, 0x75, 0x74, 0x20, 0x66, 0x6F,
            0x72, 0x20, 0x62, 0x61, 0x6E, 0x6B, 0x73, 0xFF, 0xFF, 0xFF, 0xFF, 0x01, 0x00, 0xF2,
            0x05, 0x2A, 0x01, 0x00, 0x00, 0x00, 0x43, 0x41, 0x04, 0x67, 0x8A, 0xFD, 0xB0, 0xFE,
            0x55, 0x48, 0x27, 0x19, 0x67, 0xF1, 0xA6, 0x71, 0x30, 0xB7, 0x10, 0x5C, 0xD6, 0xA8,
            0x28, 0xE0, 0x39, 0x09, 0xA6, 0x79, 0x62, 0xE0, 0xEA, 0x1F, 0x61, 0xDE, 0xB6, 0x49,
            0xF6, 0xBC, 0x3F, 0x4C, 0xEF, 0x38, 0xC4, 0xF3, 0x55, 0x04, 0xE5, 0x1E, 0xC1, 0x12,
            0xDE, 0x5C, 0x38, 0x4D, 0xF7, 0xBA, 0x0B, 0x8D, 0x57, 0x8A, 0x4C, 0x70, 0x2B, 0x6B,
            0xF1, 0x1D, 0x5F, 0xAC, 0x00, 0x00, 0x00, 0x00,
        ]; // genesis block

        assert_eq!(
            double_sha256(&input),
            [
                59, 163, 237, 253, 122, 123, 18, 178, 122, 199, 44, 62, 103, 118, 143, 97, 127,
                200, 27, 195, 136, 138, 81, 50, 58, 159, 184, 170, 75, 30, 94, 74
            ]
        )
    }
}
