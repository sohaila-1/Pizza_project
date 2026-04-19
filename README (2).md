# Chaîne de production de pizzas distribuée

Projet en Rust : développer un agent qui s'intègre dans une chaîne de production de pizzas distribuée.

Plusieurs agents collaborent via TCP pour exécuter les étapes d'une recette (préparation, garniture, cuisson...). Chaque agent reçoit une requête, traite ce qu'il sait faire, et transmet le reste à un autre agent si besoin.

## Structure

```
pizza-project/
├── pizza_agent/
│   ├── main.rs         # serveur TCP
│   ├── client.rs       # client de test
│   ├── handler.rs      # traitement des recettes
│   └── protocol.rs     # messages (Request / Response)
├── pizza_factory/      # binaire fourni
│   └── recipes/
│       └── examples.recipes
└── README.md
```

## Fonctionnement

1. Le client envoie une commande, par exemple `Order Margherita`
2. La `pizza_factory` distribue les tâches aux agents
3. L'agent traite les étapes qu'il gère et renvoie la réponse

## Recettes dynamiques

Plutôt que de coder les recettes en dur, l'agent lit le fichier `pizza_factory/recipes/examples.recipes` et parse les recettes au démarrage.

Exemple de recette :

```
Margherita =
    MakeDough
    -> AddBase(base_type=tomato)
    -> [AddCheese(amount=2), AddBasil(leaves=3)]
    -> Bake(duration=5)
```

Du coup toutes les recettes sont reconnues sans avoir à retoucher le code.

## Lancer le projet

Démarrer la `pizza_factory` :

```bash
cd archive-2026-03-01T12:59:55/aarch64-apple-darwin
chmod +x pizza_factory
./pizza_factory start \
  --recipes-file ~/pizza-project/pizza_factory/recipes/examples.recipes \
  --capabilities MakeDough AddBase AddCheese AddBasil Bake AddOliveOil
```

Lancer l'agent :

```bash
cd ~/pizza-project/pizza_agent
cargo run --bin pizza_agent
```

Tester avec le client :

```bash
./pizza_factory client --peer 127.0.0.1:8000 order Margherita
```

Résultat attendu :

```
Dough prepared
Base added
Cheese added
Basil added
Baked
Order completed successfully
```

## Stack technique

Rust 2021, TCP via `std::net`, sérialisation CBOR avec `ciborium`, multi-threading via `std::thread`, parsing du DSL des recettes.

Pour comprendre le protocole utilisé par la `pizza_factory`, on a utilisé Wireshark pour observer les échanges TCP et identifier le format des messages.

## Tests effectués

- Connexion TCP client / agent
- Envoi et réception de requêtes
- Parsing dynamique des recettes
- Traitement de plusieurs pizzas (Margherita, Funghi, etc.)
- Gestion des erreurs réseau

## Pistes d'amélioration

- Exécution réelle des étapes du DSL
- Forwarding vers d'autres agents
- Optimisation du routage
- Dashboard

## Auteur

Adham Elkhadar
Souhaila Nachafi
