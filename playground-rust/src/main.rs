use std::collections::BTreeMap;

fn main() {
    // Création d'un BTreeMap vide
    let mut map = BTreeMap::new();

    // Insertion de quelques paires clé-valeur
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "df");

    // Récupération d'une valeur par sa clé
    if let Some(value) = map.get(&2) {
        println!("Value for key 2: {}", value);
    }

    // Itération sur les éléments du BTreeMap
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    // Suppression d'une clé du BTreeMap
    map.remove(&2);

    // Vérification de l'existence d'une clé
    if !map.contains_key(&2) {
        println!("Key 2 is not in the map");
    }
}
