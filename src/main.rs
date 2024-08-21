use automerge::transaction::CommitOptions;
use automerge::transaction::Transactable;
use automerge::AutomergeError;
use automerge::ObjType;
use automerge::{Automerge, ReadDoc, ROOT};
use automerge_test::pretty_print;
use std::time::Instant;
use malloc_size_of::{MallocSizeOf, MallocSizeOfOps};

// Based on https://automerge.github.io/docs/quickstart
use malloc_size_of::{MallocSizeOf, MallocSizeOfOps};

impl MallocSizeOf for Automerge {
    fn malloc_size_of(&self, ops: &mut MallocSizeOfOps) -> usize {
        // Implement the logic to measure the heap-allocated memory of Automerge
        // This is a placeholder implementation and needs to be replaced with actual logic
        0
    }
}

fn main() {

    generate_and_change_cards(100000)

}
fn generate_and_change_cards(num_cards: usize) {

    let mut doc1 = Automerge::new();
    let cards = doc1
        .transact_with::<_, _, AutomergeError, _>(
            |_| CommitOptions::default().with_message("Create cards list".to_owned()),
            |tx| {
                // Create a new list of cards
                let cards = tx.put_object(ROOT, "cards", ObjType::List).unwrap();
                Ok(cards)
            },
        )
        .unwrap()
        .result;

    for i in 0..num_cards {
        doc1.transact_with::<_, _, AutomergeError, _>(
            |_| CommitOptions::default().with_message(format!("Add card {}", i)),
            |tx| {
                let card = tx.insert_object(&cards, i, ObjType::Map)?;
                tx.put(&card, "title", format!("Card {}", i))?;
                tx.put(&card, "done", false)?;
                Ok(())
            },
        )
        .unwrap();
    }

    for i in 0..num_cards {
        let card = doc1.get(&cards, i).unwrap().unwrap().1;
        doc1.transact_with::<_, _, AutomergeError, _>(
            |_| CommitOptions::default().with_message(format!("Mark card {} as done", i)),
            |tx| {
                tx.put(&card, "done", true)?;
                Ok(())
            },
        )
        .unwrap();
    }

    // Delete half of the cards
    for i in 0..(num_cards / 2) {
        doc1.transact_with::<_, _, AutomergeError, _>(
            |_| CommitOptions::default().with_message(format!("Delete card {}", i)),
            |tx| {
                tx.delete(&cards, i)?;
                Ok(())
            },
        )
        .unwrap();
    }

    let mut doc2 = Automerge::new();

    let start_merge = Instant::now();
    doc2.merge(&mut doc1).unwrap();
    let duration_merge = start_merge.elapsed();

    // Timing the save method
    let start = Instant::now();
    let snapshot = doc1.save();
    let duration = start.elapsed();

    let start_load = Instant::now();
    let _doc3 = Automerge::load(&snapshot).unwrap();
    let duration_load = start_load.elapsed();

    // Get the size in bytes of doc1, doc2, and doc3 without saving
    let mut ops = MallocSizeOfOps::default();
    let size_doc1 = doc1.malloc_size_of(&mut ops);
    let size_doc2 = doc2.malloc_size_of(&mut ops);
    let size_doc3 = _doc3.malloc_size_of(&mut ops);

    println!("Size of doc1: {} bytes", size_doc1);
    println!("Size of doc2: {} bytes", size_doc2);
    println!("Size of doc3: {} bytes", size_doc3);

    println!("Time taken to save the document with {} cards: {:?}", num_cards, duration);
    println!("Save: {:?}", duration);
    println!("Merge: {:?}", duration_merge);
    println!("Load from Save: {:?}", duration_load);

}

pub fn print_document(doc: &Automerge) {
    println!("Document contents:");
    pretty_print(doc);
}

pub fn print_changes(doc: &Automerge) {
    println!("Changes:");
    let cards = doc.get(ROOT, "cards").unwrap().unwrap().1;
    for change in doc.get_changes(&[]) {
        let length = doc.length_at(&cards, &[change.hash()]);
        println!("Change: {} length: {}", change.message().unwrap(), length);
    }
}