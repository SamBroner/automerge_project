use automerge::transaction::CommitOptions;
use automerge::transaction::Transactable;
use automerge::AutomergeError;
use automerge::ObjType;
use automerge::{Automerge, ReadDoc, ROOT};
use automerge_test::pretty_print;

// Based on https://automerge.github.io/docs/quickstart
fn main() {
    let mut doc1 = Automerge::new();
    let (cards, card1, card2) = doc1
        .transact_with::<_, _, AutomergeError, _>(
            |_| CommitOptions::default().with_message("Add two cards".to_owned()),
            |tx| {
                // Create a new list of cards
                let cards = tx.put_object(ROOT, "cards", ObjType::List).unwrap();
                // Create a new card
                let card1 = tx.insert_object(&cards, 0, ObjType::Map)?;
                // Add stuff to the card
                tx.put(&card1, "title", "Rewrite everything in Clojure")?;
                tx.put(&card1, "done", false)?;
                // Create a second card
                let card2 = tx.insert_object(&cards, 0, ObjType::Map)?;
                // Add stuff to the second card
                tx.put(&card2, "title", "Rewrite everything in Haskell")?;
                tx.put(&card2, "done", false)?;
                Ok((cards, card1, card2))
            },
        )
        .unwrap()
        .result;


    
    print_document(&doc1);
    
    let mut doc2 = Automerge::new();
    doc2.merge(&mut doc1).unwrap();

    let binary = doc1.save();
    let mut doc2 = Automerge::load(&binary).unwrap();

    doc1.transact_with::<_, _, AutomergeError, _>(
        |_| CommitOptions::default().with_message("Mark card as done".to_owned()),
        |tx| {
            tx.put(&card1, "done", true)?;
            Ok(())
        },
    )
    .unwrap();

    doc2.transact_with::<_, _, AutomergeError, _>(
        |_| CommitOptions::default().with_message("Delete card".to_owned()),
        |tx| {
            tx.delete(&cards, 0)?;
            Ok(())
        },
    )
    .unwrap();

    doc1.merge(&mut doc2).unwrap();

    
    for change in doc1.get_changes(&[]) {
        let length = doc1.length_at(&cards, &[change.hash()]);
        println!("Change: {} length: {}", change.message().unwrap(), length);
    }
    print_document(&doc1);

    // Get a snapshot of the document
    let snapshot = doc1.save();
    // println!("Snapshot of the document (binary): {:?}", snapshot);

    // Optionally, load the snapshot into a new document to verify
    let snapshot_doc = Automerge::load(&snapshot).unwrap();

    println!("Snapshot document contents:");
    print_document(&snapshot_doc);

    print_changes(&snapshot_doc);

}

fn print_document(doc: &Automerge) {
    println!("Document contents:");
    pretty_print(doc);
}

fn print_changes(doc: &Automerge) {
    println!("Changes:");
    let cards = doc.get(ROOT, "cards").unwrap().unwrap().1;
    for change in doc.get_changes(&[]) {
        let length = doc.length_at(&cards, &[change.hash()]);
        println!("Change: {} length: {}", change.message().unwrap(), length);
    }
}