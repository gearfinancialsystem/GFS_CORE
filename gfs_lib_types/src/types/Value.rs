use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Vstring(String),
    VhashMap(HashMap<String, Value>),
    VvecVal(Vec<Value>),
    Vf64(f64),
    Vbool(bool),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Vstring(s) => write!(f, "{}", s),
            Value::Vf64(n) => write!(f, "{}", n),
            Value::Vbool(b) => write!(f, "{}", b),
            Value::None => write!(f, "None"),
            Value::VvecVal(v) => {
                write!(f, "[")?;
                for (i, item) in v.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            },
            Value::VhashMap(m) => {
                write!(f, "{{")?;
                let mut entries = m.iter().collect::<Vec<_>>();
                entries.sort_by(|a, b| a.0.cmp(b.0)); // Trier les clés pour un affichage cohérent

                for (i, (key, value)) in entries.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, value)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl Value {
    // =========================
    // Méthodes de création (constructeurs)
    // =========================

    /// Crée une nouvelle Value::Vf64
    pub fn from_f64(n: f64) -> Self {
        Value::Vf64(n)
    }

    /// Crée une nouvelle Value::Vstring
    pub fn from_string(s: String) -> Self {
        Value::Vstring(s)
    }

    /// Crée une nouvelle Value::VvecVal
    pub fn from_vec(v: Vec<Value>) -> Self {
        Value::VvecVal(v)
    }

    /// Crée une nouvelle Value::VhashMap
    pub fn from_hashmap(m: HashMap<String, Value>) -> Self {
        Value::VhashMap(m)
    }

    /// Crée une nouvelle Value::Bool
    pub fn from_bool(m: bool) -> Self {
        Value::Vbool(m)
    }

    // =========================
    // Méthodes d'inspection (ne consomment pas la valeur)
    // =========================

    /// Retourne true si la valeur est None
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }

    /// Retourne true si la valeur est un Vf64
    pub fn is_f64(&self) -> bool {
        matches!(self, Value::Vf64(_))
    }

    /// Retourne true si la valeur est un bool
    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Vbool(_))
    }

    /// Retourne true si la valeur est un Vstring
    pub fn is_string(&self) -> bool {
        matches!(self, Value::Vstring(_))
    }

    /// Retourne true si la valeur est un VhashMap
    pub fn is_hashmap(&self) -> bool {
        matches!(self, Value::VhashMap(_))
    }

    /// Retourne true si la valeur est un VvecVal
    pub fn is_vec(&self) -> bool {
        matches!(self, Value::VvecVal(_))
    }

    // =========================
    // Méthodes de conversion par référence (ne consomment pas la valeur)
    // Préfixe: as_
    // =========================

    /// Retourne une référence à la valeur f64 si c'en est une
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Vbool(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    /// Retourne une référence à la valeur f64 si c'en est une
    pub fn as_f64(&self) -> Option<f64> {
        if let Value::Vf64(n) = self {
            Some(*n)
        } else {
            None
        }
    }

    /// Retourne une référence à la String si c'en est une
    pub fn as_string(&self) -> Option<&String> {
        if let Value::Vstring(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Retourne une référence à la Vec si c'en est une
    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        if let Value::VvecVal(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Retourne une référence à la HashMap si c'en est une
    pub fn as_hashmap(&self) -> Option<&HashMap<String, Value>> {
        if let Value::VhashMap(m) = self {
            Some(m)
        } else {
            None
        }
    }

    // =========================
    // Méthodes de conversion par valeur (consomment la valeur)
    // Préfixe: into_
    // =========================

    /// Consomme la valeur et retourne la String si c'en est une
    pub fn into_string(self) -> Option<String> {
        if let Value::Vstring(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Consomme la valeur et retourne la Vec si c'en est une
    pub fn into_vec(self) -> Option<Vec<Value>> {
        if let Value::VvecVal(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Consomme la valeur et retourne la HashMap si c'en est une
    pub fn into_hashmap(self) -> Option<HashMap<String, Value>> {
        if let Value::VhashMap(m) = self {
            Some(m)
        } else {
            None
        }
    }

    /// Consomme la valeur et retourne le f64 si c'en est une
    pub fn into_f64(self) -> Option<f64> {
        if let Value::Vf64(n) = self {
            Some(n)
        } else {
            None
        }
    }

    /// Consomme la valeur et retourne le bool si c'en est une
    pub fn into_bool(self) -> Option<bool> {
        if let Value::Vbool(n) = self {
            Some(n)
        } else {
            None
        }
    }

    // =========================
    // Méthodes de conversion avec valeurs par défaut
    // Préfixe: to_
    // =========================

    /// Retourne la valeur f64 ou 0.0 si ce n'est pas un Vf64
    pub fn to_bool(&self) -> bool {
        self.as_bool().unwrap_or(false)
    }

    /// Retourne la valeur f64 ou 0.0 si ce n'est pas un Vf64
    pub fn to_f64(&self) -> f64 {
        self.as_f64().unwrap_or(0.0)
    }

    /// Retourne la String ou une chaîne vide si ce n'est pas un Vstring
    pub fn to_string(&self) -> String {
        self.as_string().cloned().unwrap_or_default()
    }

    /// Retourne le Vec ou un vecteur vide si ce n'est pas un VvecVal
    pub fn to_vec(&self) -> Vec<Value> {
        self.as_vec().cloned().unwrap_or_default()
    }

    /// Retourne la HashMap ou une HashMap vide si ce n'est pas un VhashMap
    pub fn to_hashmap(&self) -> HashMap<String, Value> {
        self.as_hashmap().cloned().unwrap_or_default()
    }

    // =========================
    // Méthodes de conversion mutables
    // Suffix: _mut
    // =========================

    /// Retourne une référence mutable à la String si c'en est une
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        if let Value::Vstring(s) = self {
            Some(s)
        } else {
            None
        }
    }

    /// Retourne une référence mutable à la Vec si c'en est une
    pub fn as_vec_mut(&mut self) -> Option<&mut Vec<Value>> {
        if let Value::VvecVal(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// Retourne une référence mutable à la HashMap si c'en est une
    pub fn as_hashmap_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        if let Value::VhashMap(m) = self {
            Some(m)
        } else {
            None
        }
    }

    /// Retourne une référence mutable au f64 si c'en est un
    pub fn as_f64_mut(&mut self) -> Option<&mut f64> {
        if let Value::Vf64(n) = self {
            Some(n)
        } else {
            None
        }
    }

    /// Retourne une référence mutable au f64 si c'en est un
    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        if let Value::Vbool(n) = self {
            Some(n)
        } else {
            None
        }
    }

    // =========================
    // Méthodes de conversion avancées
    // =========================

    /// Essaye de convertir la valeur en f64
    /// - Si c'est un Vf64, retourne le nombre
    /// - Si c'est un Vstring, essaie de parser la chaîne
    /// - Sinon retourne None
    pub fn try_as_f64(&self) -> Option<f64> {
        match self {
            Value::Vf64(n) => Some(*n),
            Value::Vstring(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Essaye de convertir la valeur en String
    /// - Si c'est un Vstring, retourne la chaîne
    /// - Si c'est un Vf64, convertit en chaîne
    /// - Sinon retourne None
    pub fn try_as_string(&self) -> Option<String> {
        match self {
            Value::Vstring(s) => Some(s.clone()),
            Value::Vf64(n) => Some(n.to_string()),
            _ => None,
        }
    }

    /// Convertit la valeur en String, en essayant de représenter chaque type
    /// de manière appropriée
    pub fn to_string_repr(&self) -> String {
        match self {
            Value::Vf64(n) => n.to_string(),
            Value::Vstring(s) => s.clone(),
            Value::VvecVal(v) => {
                let items: Vec<String> = v.iter().map(|x| x.to_string_repr()).collect();
                format!("[{}]", items.join(", "))
            },
            Value::VhashMap(m) => {
                let items: Vec<String> = m.iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string_repr()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            },
            Value::Vbool(m) => m.to_string(),
            Value::None => "null".to_string(),
        }
    }

    /// Clone profond de la valeur
    pub fn deep_clone(&self) -> Self {
        match self {
            Value::Vf64(n) => Value::Vf64(*n),
            Value::Vstring(s) => Value::Vstring(s.clone()),
            Value::VvecVal(v) => Value::VvecVal(v.iter().map(|x| x.deep_clone()).collect()),
            Value::VhashMap(m) => {
                let mut new_map = HashMap::new();
                for (k, v) in m {
                    new_map.insert(k.clone(), v.deep_clone());
                }
                Value::VhashMap(new_map)
            },
            Value::Vbool(m) => Value::Vbool(*m),
            Value::None => Value::None,
        }
    }
    // =========================
    // Méthodes de comparaison
    // =========================

    /// Compare deux Values pour égalité (comparaison profonde)
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Vf64(a), Value::Vf64(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Vstring(a), Value::Vstring(b)) => a == b,
            (Value::VvecVal(a), Value::VvecVal(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.equals(y))
            },
            (Value::VhashMap(a), Value::VhashMap(b)) => {
                a.len() == b.len() &&
                    a.iter().all(|(k, v)| b.get(k).map_or(false, |bv| v.equals(bv)))
            },
            (Value::None, Value::None) => true,
            _ => false,
        }
    }

}
