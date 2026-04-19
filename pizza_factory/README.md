# 🍕 Distributed Pizza Production System

## 📌 Project Overview

This project implements a **distributed pizza production system** in Rust.
The system simulates a network of agents collaborating to produce pizzas based on predefined recipes.

Each agent is responsible for a subset of capabilities (e.g., dough preparation, topping, baking) and communicates with others using a network protocol.

---

## 🎯 Objectives

* Reverse-engineer an undocumented network protocol
* Implement a **custom agent** compatible with an existing system
* Develop a **client-server architecture using TCP**
* Support **dynamic recipe handling using a DSL**
* Apply Rust concepts: ownership, concurrency, networking

---

## 🧠 System Architecture

```text
Client → pizza_factory → Agent(s) → Processing → Response
```

### Components:

| Component                      | Description                                        |
| ------------------------------ | -------------------------------------------------- |
| **pizza_factory**              | Provided binary simulating the distributed network |
| **Agent (our implementation)** | Handles requests and processes recipes             |
| **Client**                     | Sends commands to the network                      |
| **Protocol**                   | Defines message structures (Request / Response)    |
| **Handler**                    | Contains business logic for processing recipes     |

---

## 📂 Project Structure

```text
Pizza_project/
│
├── pizza_agent/
│   ├── src/
│   │   ├── main.rs        # TCP server (agent)
│   │   ├── client.rs      # Client implementation
│   │   ├── handler.rs     # Business logic
│   │   ├── protocol.rs    # Message definitions
│   ├── Cargo.toml
│
├── pizza_factory/
│   ├── recipes/
│   │   └── examples.recipes
│
└── README.md
```

---

## 🔍 Reverse Engineering

Since the protocol was not documented, we analyzed network traffic using:

* Wireshark
* tcpdump

### Findings:

* Communication via **TCP**
* Messages encoded in **CBOR format**
* Use of **framing**:

```text
[message length (4 bytes)] + [CBOR payload]
```

---

## 🍕 Recipe System (DSL)

Recipes are defined in a Domain Specific Language:

```text
Margherita =
    MakeDough
    -> AddBase(base_type=tomato)
    -> [AddCheese(amount=2), AddBasil(leaves=3)]
    -> Bake(duration=5)
```

### Implementation

We implemented:

* Dynamic loading of recipes from file
* Parsing of recipe names
* Support for multiple pizzas without hardcoding

---

## ⚙️ Agent Behavior

The agent:

* Listens on TCP (`127.0.0.1:9000`)
* Receives CBOR-encoded requests
* Processes known recipes
* Returns structured responses

If a recipe is unknown:

```text
Forwarded to another agent (simulated)
```

---

## 🚀 How to Run the Project

### 1️⃣ Start the pizza factory

```bash
cd archive-2026-03-01T12:59:55/aarch64-apple-darwin
chmod +x pizza_factory

./pizza_factory start \
--recipes-file ~/pizza-project/pizza_factory/recipes/examples.recipes \
--capabilities MakeDough AddBase AddCheese AddBasil Bake AddOliveOil
```

---

### 2️⃣ Start the agent

```bash
cd ~/pizza-project/pizza_agent
cargo run --bin pizza_agent
```

---

### 3️⃣ Send a request

```bash
./pizza_factory client --peer 127.0.0.1:8000 order Margherita
```

---

## ✅ Expected Output

```text
Order completed successfully
```

Example processing:

```text
Dough prepared
Base added
Cheese added
Basil added
Baked
```

---

## 🧪 Testing Strategy

We validated:

* TCP communication (client ↔ agent)
* CBOR serialization/deserialization
* Dynamic recipe handling
* Error cases (missing capability)
* Multi-recipe support

---

## 🛡 Error Handling & Improvements

* Removed unsafe `unwrap()` usage
* Added timeout handling
* Implemented retry logic
* Improved logging for debugging

---

## 🤝 Collaboration Workflow

We used Git with a structured workflow:

* `main` → stable version
* `develop` → ongoing development

### Contributions:

* **Sohaila**:

  * Agent implementation
  * TCP handling
  * DSL parsing

* **Teammate**:

  * Client implementation
  * Protocol design

---

## ⚠️ Limitations

* No real distributed routing between agents
* Forwarding is simulated
* Partial DSL parsing (names only)

---

## 🚀 Future Improvements

* Full DSL execution (step-by-step actions)
* Real peer-to-peer communication
* Gossip protocol implementation
* Load balancing between agents
* Monitoring dashboard

---

## 🏁 Conclusion

This project demonstrates:

* Distributed system design
* Network protocol analysis
* Low-level TCP communication
* Modular Rust architecture

It provides a functional agent capable of integrating into an existing distributed network.

---

## 👩‍💻 Author

* Sohaila
* Adham