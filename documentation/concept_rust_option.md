En Rust, si vous avez une structure avec beaucoup de champs optionnels et que vous trouvez fastidieux de devoir gérer les `Option` à chaque fois, vous pouvez envisager plusieurs approches pour simplifier l'utilisation de votre structure.

### Utilisation de `unwrap` ou `expect` 

Si vous êtes sûr que les champs ne seront pas `None` lorsque vous les utilisez, vous pouvez utiliser `unwrap` ou `expect` pour obtenir directement la valeur. Cependant, cela peut entraîner des paniques si les champs sont `None`.

```rust
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct(ma_struct: MaStruct) {
    let valeur1 = ma_struct.champ1.unwrap(); // Panique si champ1 est None
    // utiliser valeur1...
}
```

### Utilisation de `Default`

Vous pouvez implémenter le trait `Default` pour votre structure afin de fournir des valeurs par défaut pour les champs optionnels.

```rust
#[derive(Default)]
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct(ma_struct: MaStruct) {
    let valeur1 = ma_struct.champ1.unwrap_or_default(); // Utilise la valeur par défaut si champ1 est None
    // utiliser valeur1...
}
```

### Utilisation de `unwrap_or` ou `unwrap_or_else`

Vous pouvez fournir une valeur par défaut ou une closure pour obtenir une valeur par défaut si le champ est `None`.

```rust
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct(ma_struct: MaStruct) {
    let valeur1 = ma_struct.champ1.unwrap_or(Type1::default()); // Utilise Type1::default() si champ1 est None
    // utiliser valeur1...
}
```

### Utilisation de `match` ou `if let`

Vous pouvez utiliser `match` ou `if let` pour gérer explicitement les cas où les champs sont `None`.

```rust
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct(ma_struct: MaStruct) {
    if let Some(valeur1) = ma_struct.champ1 {
        // utiliser valeur1...
    } else {
        // gérer le cas où champ1 est None...
    }
}
```

### Utilisation de `map` et `and_then`

Vous pouvez utiliser `map` et `and_then` pour transformer les valeurs optionnelles.

```rust
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct(ma_struct: MaStruct) {
    let valeur1 = ma_struct.champ1.map(|v| v.some_method()).unwrap_or_default();
    // utiliser valeur1...
}
```

### Utilisation de crates externes

Il existe des crates comme `derive_builder` qui peuvent vous aider à créer des builders pour vos structures, simplifiant ainsi la gestion des champs optionnels.

```toml
[dependencies]
derive_builder = "0.10"
```

```rust
#[derive(Builder)]
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn main() {
    let ma_struct = MaStructBuilder::default()
        .champ1(Some(Type1::new()))
        // autres champs...
        .build()
        .unwrap();

    // utiliser ma_struct...
}
```

En utilisant ces approches, vous pouvez simplifier la gestion des champs optionnels dans vos structures Rust.

En Rust, lorsque vous passez une structure par référence et que vous utilisez `unwrap` sur un champ optionnel, vous ne déplacez pas la valeur, mais vous obtenez une référence à la valeur contenue dans l'`Option`. Cependant, si vous voulez éviter de déplacer la valeur et que vous avez besoin d'une copie, vous pouvez utiliser `clone`.

Voici comment vous pouvez gérer cela :

### Sans `clone`

Si vous utilisez `unwrap` sur une référence, vous obtenez une référence à la valeur contenue dans l'`Option`. Cela ne déplace pas la valeur.

```rust
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct(ma_struct: &MaStruct) {
    let valeur1 = ma_struct.champ1.as_ref().unwrap(); // Obtient une référence à la valeur
    // utiliser valeur1...
}
```

### Avec `clone`

Si vous avez besoin d'une copie de la valeur, vous pouvez utiliser `clone`. Cela nécessite que le type `Type1` implémente le trait `Clone`.

```rust
pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct(ma_struct: &MaStruct) {
    let valeur1 = ma_struct.champ1.as_ref().cloned(); // Clone la valeur si elle existe
    // utiliser valeur1...
}
```

### Exemple complet

Voici un exemple complet illustrant les deux approches :

```rust
#[derive(Clone, Debug)]
pub struct Type1 {
    valeur: i32,
}

pub struct MaStruct {
    champ1: Option<Type1>,
    // autres champs...
}

fn utiliser_ma_struct_sans_clone(ma_struct: &MaStruct) {
    let valeur1 = ma_struct.champ1.as_ref().unwrap();
    println!("Valeur sans clone : {:?}", valeur1);
}

fn utiliser_ma_struct_avec_clone(ma_struct: &MaStruct) {
    let valeur1 = ma_struct.champ1.as_ref().cloned().unwrap();
    println!("Valeur avec clone : {:?}", valeur1);
}

fn main() {
    let ma_struct = MaStruct {
        champ1: Some(Type1 { valeur: 42 }),
        // autres champs...
    };

    utiliser_ma_struct_sans_clone(&ma_struct);
    utiliser_ma_struct_avec_clone(&ma_struct);
}
```

### Explications

- **Sans `clone`** : `as_ref()` est utilisé pour obtenir une référence à la valeur contenue dans l'`Option`. `unwrap()` est ensuite utilisé pour obtenir cette référence. Cela ne déplace pas la valeur.
- **Avec `clone`** : `as_ref()` est utilisé pour obtenir une référence à la valeur contenue dans l'`Option`, puis `cloned()` est utilisé pour cloner cette valeur. Cela crée une nouvelle copie de la valeur.

En utilisant ces approches, vous pouvez gérer les champs optionnels dans vos structures Rust sans déplacer les valeurs et en utilisant des références ou des clones selon vos besoins.