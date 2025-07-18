mod bayes;
use bayes::NaiveBayes;

fn main() {
    let path = "bclass.json";

    let _training_data = vec![
        ("spam".to_string(), vec!["buy", "cheap", "meds"]),
        ("spam".to_string(), vec!["cheap", "pills", "offer"]),
        ("ham".to_string(), vec!["let's", "meet", "for", "lunch"]),
        ("ham".to_string(), vec!["see", "you", "at", "the", "party"]),
        ("social".to_string(), vec!["fun", "party", "invitation", "tonight"]),
    ];

    println!("Bayes Classifier");

    //let mut classifier = NaiveBayes::new();
    let mut classifier = NaiveBayes::load_from_file(path).unwrap_or(NaiveBayes::new());

    classifier.bulk_train(_training_data);
    
    let test = vec!["meet", "at", "party"];
    let t = classifier.classification_table(&test);
    println!("{t:?}");
    
    match classifier.classify(&test) {
        Some(label) => println!("Predicted class: {}", label),
        None => println!("Could not classify input."),
    }

    match classifier.save_to_file(path, true) {
        Ok(_) => println!("Saved."),
        Err(e) => eprintln!("Error: {}", e),
    }
}
