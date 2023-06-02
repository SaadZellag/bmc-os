use arrayvec::ArrayVec;
use core::iter::FromIterator;

use crate::{engine::EVALUATOR, search::position::Position, utils::tablesize::TableSize, Eval};

use cozy_chess::{Move, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryType {
    Exact,
    LowerBound,
    UpperBound,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TTEntry {
    pub hash: u64,
    pub flag: EntryType,
    pub depth: u8,
    pub eval: Eval,
    pub mv: Move,
}

impl TTEntry {
    const fn new() -> Self {
        Self {
            hash: 0,
            flag: EntryType::Invalid,
            depth: 0,
            eval: Eval::NEUTRAL,
            mv: Move {
                from: Square::A1,
                to: Square::A1,
                promotion: None,
            },
        }
    }
}

impl Default for TTEntry {
    fn default() -> Self {
        Self::new()
    }
}

const HASH_TABLE_SIZE: usize = 1 * 1024 * 1024 / core::mem::size_of::<TTEntry>();

static mut HASH_TABLE: [TTEntry; HASH_TABLE_SIZE] = [TTEntry::new(); HASH_TABLE_SIZE];

pub struct TranspositionTable {
    table: &'static mut [TTEntry; HASH_TABLE_SIZE],
    num_valid_entries: usize,
}

impl TranspositionTable {
    pub fn new(size: TableSize) -> Self {
        Self {
            table: unsafe { &mut HASH_TABLE },
            num_valid_entries: 0,
        }
    }

    pub fn get(&self, pos: &Position) -> Option<TTEntry> {
        let hash = pos.board().hash();
        let index = self.to_entry_hash(hash);
        let mut entry = self.table[index];

        (entry.flag != EntryType::Invalid && entry.hash == hash).then(|| {
            entry.eval = entry.eval.add_ply(pos.ply());
            entry
        })
    }

    pub fn set(&mut self, pos: &Position, mut entry: TTEntry) -> bool {
        let index = self.to_entry_hash(entry.hash);
        // self.insertions.entry(index).or_default().push(entry.hash);
        let old_entry = self.table[index];

        let mut replace = false;

        replace |= old_entry.hash == entry.hash && entry.flag == EntryType::Exact;

        replace |= entry.depth >= old_entry.depth;

        if replace {
            if old_entry.flag == EntryType::Invalid {
                self.num_valid_entries += 1;
            } else {
                // dbg!(old_entry);
                // dbg!(entry);
            }

            entry.eval = entry.eval.sub_ply(pos.ply());
            self.table[index] = entry;
        }

        replace
    }

    pub fn increment_age(&mut self) {
        self.table
            .iter_mut()
            .for_each(|e| e.depth = e.depth.saturating_sub(1))
    }

    // Knuth's method
    fn to_entry_hash(&self, original_hash: u64) -> usize {
        (original_hash as u128 * HASH_TABLE_SIZE as u128 >> 64) as usize
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new(TableSize::default())
    }
}

#[test]
fn test_table() {
    let mut table = TranspositionTable::default();

    let pos = Position::new(Board::default(), &EVALUATOR);

    let mut entry = TTEntry {
        hash: pos.board().get_hash(),
        flag: EntryType::Exact,
        depth: 7,
        eval: Eval::NEUTRAL,
        mv: Move::default(),
    };

    // Checking if insert works
    table.set(&pos, entry);
    assert!(table.get(&pos).is_some(), "Value inserted is not there");

    // Checking if invalid key should not be given back
    entry.flag = EntryType::Invalid;
    entry.depth += 1;
    table.set(&pos, entry);
    assert!(table.get(&pos).is_none(), "Got an invalid result");
}

// #[test]
// fn test_collisions() {
//     use crate::utils::positiongen::PositionGenerator;
//     use rand::{rngs::StdRng, Rng, SeedableRng};
//     // https://math.stackexchange.com/questions/4047136/how-to-find-the-expected-number-of-insertions-before-a-hash-table-is-full

//     let mut table = TranspositionTable::default();
//     let mut rng = StdRng::seed_from_u64(123);
//     let mut positions = PositionGenerator::sized(1_000_000);

//     let harmonic_number = (1..=table.size)
//         .map(|k| table.size as f64 / k as f64)
//         .sum::<f64>() as usize;

//     dbg!(table.size);

//     let mut total_inserts = 0;

//     while table.hashfull() < 1000 {
//         if total_inserts > harmonic_number * 10 {
//             break;
//         }
//         let _board = positions.next().unwrap();
//         let entry = TTEntry {
//             hash: rng.gen(),
//             flag: EntryType::Exact,
//             depth: 0,
//             eval: Eval::NEUTRAL,
//             mv: Move::default(),
//         };
//         let pos = Position::new(Board::default(), &EVALUATOR);
//         table.set(&pos, entry);
//         total_inserts += 1;
//     }

//     // dbg!(&table.table);
//     dbg!(total_inserts);
//     assert!(
//         total_inserts < harmonic_number,
//         "Table inserted {} times while the harmonic number is {}",
//         total_inserts,
//         harmonic_number
//     )
// }
