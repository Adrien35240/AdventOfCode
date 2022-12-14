use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // On ouvre le fichier en lecture
    let file = File::open("data.txt").unwrap();

    // On crée un BufReader pour lire les lignes du fichier
    let reader = BufReader::new(file);

    let mut results: Vec<i32> = Vec::new();
        results.push(1);
    // On parcourt chaque ligne du fichier
    for line in reader.lines() {
        let line = line.unwrap();
        // Si la ligne n'est pas vide, on additionne le nombre à la dernière valeur du tableau
        if !line.is_empty() {
            *results.last_mut().unwrap() += line.parse::<i32>().unwrap();
        } else {
            // Si la ligne est vide, on ajoute une nouvelle valeur vide au tableau
            results.push(0);
        }
    }
    // on affiche le vec de tous les resultats 
    println!("results : {:?}",results);
    // On récupère l'index de la valeur la plus élevée du tableau
    let max_index = results.iter().enumerate().max_by_key(|(_, val)| *val).unwrap().0;
    println!("resultat : {}",results[max_index]);
}

