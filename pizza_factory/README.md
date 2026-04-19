# 🍕 Pizza Distributed System – README

## 📌 Description du projet

Ce projet consiste à implémenter un **agent distribué en Rust** capable de s’intégrer dans une chaîne de production de pizzas.

Le système repose sur plusieurs nœuds (agents) qui collaborent pour produire une pizza en exécutant différentes étapes (capabilities).

Le projet inclut :

* un **agent personnalisé**
* un **client**
* une interaction avec le **binaire fourni (pizza_factory)**

---

## 🧠 Principe général

Le système fonctionne selon une architecture distribuée :

```text
Client → Réseau → Agent(s) → Production → Réponse
```

Chaque agent :

* reçoit une requête
* traite ou délègue la tâche
* renvoie une réponse

Le produit (pizza) est représenté par une **chaîne de caractères modifiée à chaque étape**.

---

## 🏗 Structure du projet

```text
pizza_agent/
├── src/
│   ├── main.rs        # Serveur (agent TCP)
│   ├── client.rs      # Client TCP
│   ├── protocol.rs    # Définition des messages (Request / Response)
│   ├── handler.rs     # Logique métier (traitement des commandes)
│
├── Cargo.toml
```

---

## 📂 Description des fichiers

### 🔹 main.rs (Agent)

* Lance un serveur TCP (port 9000)
* Reçoit les requêtes réseau
* Décode les messages CBOR
* Appelle `handler.rs`
* Renvoie une réponse

👉 Rôle :

```text
Gestion réseau (réception / envoi)
```

---

### 🔹 client.rs

* Se connecte à un agent
* Envoie une requête (`Order`)
* Affiche la réponse

👉 Rôle :

```text
Tester le système
```

---

### 🔹 protocol.rs

* Définit les structures :

  * `Request`
  * `Response`
* Utilise `serde` pour la sérialisation

👉 Rôle :

```text
Modélisation des messages réseau
```

---

### 🔹 handler.rs

* Contient la logique métier
* Traite les recettes (ex: Margherita)
* Simule les étapes de production

👉 Exemple :

```text
Dough → Base → Cheese → Bake
```

👉 Rôle :

```text
Traitement des commandes (logique métier)
```

---

## 🔌 Technologies utilisées

* Rust
* TCP (communication réseau)
* CBOR (sérialisation binaire via `ciborium`)
* Serde

---

## 🔍 Reverse Engineering

Le protocole n’étant pas documenté, nous avons utilisé :

* Wireshark
* tcpdump

Nous avons identifié :

* transport en TCP (port 8000)
* messages encodés en CBOR
* framing : taille + message

---

## 🚀 Lancer le projet

### 🟢 1. Lancer le serveur principal

```bash
cd ~/pizza-project/archive-2026-03-01T12:59:55/aarch64-apple-darwin

./pizza_factory start \
--recipes-file ~/pizza-project/pizza_factory/recipes/examples.recipes \
--capabilities MakeDough AddBase AddCheese AddBasil Bake AddOliveOil
```

---

### 🟣 2. Lancer notre agent

```bash
cd ~/pizza-project/pizza_agent
cargo run --bin pizza_agent
```

---

### 🔵 3. Tester avec le client officiel

```bash
./pizza_factory client --peer 127.0.0.1:8000 order Margherita
```

---

## 🧪 Tests réalisés

✔ Communication client ↔ agent
✔ Décodage CBOR
✔ Production de pizza
✔ Gestion des erreurs
✔ Retry / timeout

---

## 🍕 Capabilities

Le système repose sur des étapes de production :

* MakeDough
* AddBase
* AddCheese
* AddBasil
* Bake
* AddOliveOil

Si une capability est absente → erreur :

```text
Action AddBasil not available
```

---

## 🔄 Logique distribuée

* Si une action n’est pas disponible → la production échoue
* Notre agent simule un **forward** vers un autre agent

---

## 🧱 Architecture logicielle

Nous avons séparé :

```text
main.rs     → réseau
handler.rs  → logique métier
```

👉 Cela permet :

* code plus lisible
* meilleure maintenabilité

---

## 🛡 Gestion des erreurs

Améliorations apportées :

* suppression des `unwrap()`
* gestion des erreurs explicite
* ajout de timeouts réseau
* retry côté client

---

## ⚠️ Limitations

* Pas de protocole Gossip implémenté
* Pas de routage réel entre agents
* Forward simulé uniquement

---

## 🚀 Améliorations possibles

* découverte automatique des agents (UDP / Gossip)
* routage intelligent
* dashboard de suivi
* multi-agents réels

---

## ✅ Conclusion

Ce projet démontre :

* la compréhension d’un protocole réseau
* l’implémentation d’un agent distribué
* la communication en TCP avec sérialisation CBOR

Le système est fonctionnel et compatible avec le binaire fourni.

---

