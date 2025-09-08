from flask import Flask, render_template, request, abort
import json
import os

app = Flask(__name__)

# Chemin vers le fichier JSON
JSON_FILE_PATH_TAXONOMY = os.path.join(app.root_path, '.', 'data', 'actus-dictionary-taxonomy.json')
JSON_FILE_PATH_TERMS = os.path.join(app.root_path, '.', 'data', 'actus-dictionary-terms.json')

# Variable globale pour stocker les données JSON en cache
contracts_data = None
contracts_terms = None

# Charger les données JSON au démarrage de l'application
@app.before_request
def load_json_data_taxonomy():
    global contracts_data
    try:
        with open(JSON_FILE_PATH_TAXONOMY, 'r') as f:
            contracts_data = json.load(f)
    except FileNotFoundError:
        print(f"Erreur : Fichier {JSON_FILE_PATH_TAXONOMY} introuvable.")
        contracts_data = {"taxonomy": {}}
    except json.JSONDecodeError:
        print(f"Erreur : Fichier {JSON_FILE_PATH_TAXONOMY} n'est pas un JSON valide.")
        contracts_data = {"taxonomy": {}}

@app.before_request
def load_json_data_terms():
    global contracts_terms
    try:
        with open(JSON_FILE_PATH_TERMS, 'r') as f:
            contracts_terms = json.load(f)
    except FileNotFoundError:
        print(f"Erreur : Fichier {JSON_FILE_PATH_TERMS} introuvable.")
        contracts_terms = {"terms": {}}
    except json.JSONDecodeError:
        print(f"Erreur : Fichier {JSON_FILE_PATH_TERMS} n'est pas un JSON valide.")
        contracts_terms = {"terms": {}}

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/contract')
def contract_type():
    contract_acronym = request.args.get('type')
    if not contract_acronym:
        abort(400, description="Aucun type de contrat sélectionné.")

    # Rechercher le contrat par acronyme
    contract_info = next(
        (contract for contract in contracts_data['taxonomy'].values()
         if contract.get('acronym') == contract_acronym),
        None
    )
    print(contract_info)

    if not contract_info:
        abort(404, description=f"Aucune information trouvée pour {contract_acronym}")

    ## Si contrat type trouvé alors charge les termes
    contract_terms_sent = contracts_terms["terms"]

    # Group terms by 'group' attribute
    grouped_terms = {}
    for key, value in contract_terms_sent.items():
        group = value.get('group', 'Uncategorized')
        if group not in grouped_terms:
            grouped_terms[group] = []
        grouped_terms[group].append((key, value))

    return render_template('contract_interface.html', contract=contract_info, contract_terms_sent=grouped_terms)

@app.errorhandler(404)
def page_not_found(e):
    return render_template('error.html', error=e), 404

@app.errorhandler(500)
def internal_error(e):
    return render_template('error.html', error=e), 500

if __name__ == '__main__':
    app.run(debug=True)
