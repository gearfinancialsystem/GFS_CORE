# %%
import json

def parse_json(file_path):
    with open(file_path, 'r') as f:
        data = json.load(f)
    return data

def convertir_floats_en_str(obj):
    if isinstance(obj, dict):
        # Si c'est un dictionnaire, on crée un nouveau dict avec les valeurs converties
        return {cle: convertir_floats_en_str(valeur) for cle, valeur in obj.items()}
    elif isinstance(obj, list):
        # Si c'est une liste, on crée une nouvelle liste avec les éléments convertis
        return [convertir_floats_en_str(element) for element in obj]
    elif isinstance(obj, tuple):
        # Si c'est un tuple, on crée un nouveau tuple avec les éléments convertis
        return tuple(convertir_floats_en_str(element) for element in obj)
    elif isinstance(obj, set):
        # Si c'est un ensemble, on crée un nouvel ensemble avec les éléments convertis
        return {convertir_floats_en_str(element) for element in obj}
    elif isinstance(obj, float) or isinstance(obj, int):
        # Si c'est un float, on le convertit en str
        return str(obj)
    else:
        # Sinon, on retourne la valeur inchangée
        return obj

ct = "ann"
d = parse_json('actus-tests-%s.json' % ct)
d2 = convertir_floats_en_str(d)

json.dump(d2, open('actus-tests-%s-converted.json' % ct, 'w'), indent=4)