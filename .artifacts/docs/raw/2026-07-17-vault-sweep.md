> Compiled 2026-07-17 by the FABLE harvest workflow (wf_4b89b7ab-b27) from the pzzld Obsidian vault.
> NOTE: the vault reorganized the same day; project trees now live under src/docs/projects/.

# Eryon/ACME Vault Sweep — Findings

## Scope note: path-prefix correction
The vault's actual current structure nests everything under `src/docs/` (e.g. `src/docs/projects/eryon/...`, `src/docs/terms/...`) — there is no separate top-level `src/projects/` or `src/terms/`. The "already covered" list in the task uses the shorter prefixes (`src/projects/eryon/`, etc.); by filename/content these map 1:1 onto `src/docs/projects/eryon/` etc. Treated as identical for exclusion purposes.

---

## RELEVANT — ranked

### 1. `claude/captures/notion-docs-reconciliation.md`
The single most load-bearing pointer for this sweep: it documents that Joe kept a Notion "Docs" database with an **Acme** page and an **Eryon — Technical Specification** page, both last-edited 2026-05-31 — later and (per the reconciliation table) likely fuller than the vault's own copies at the time. It explicitly frames Acme as probably a new Eryon sub-component and instructs that merge direction needs Joe's call before any Obsidian overwrite.

> "| Acme | 2026-05-31 | `#acme` (1 note) + Eryon substrate material (policy machinery / consistency contract) | **Notion** | Likely a new Eryon sub-component; capture into `src/projects/eryon/`, don't overwrite | Med |"

> "| Eryon — Technical Specification | 2026-05-31 | `src/projects/eryon/Project Eryon - The Substrate.md` (large) + 13 `#eryon` notes | **Notion** on date; Obsidian also substantial | Compare side by side — Joe's judgment on merge direction before any edit | Med |"

> "The vault's `src/docs/` notes for these topics are mostly **Feb–Mar stubs**; the Notion versions are **late-May and fuller**. So for RMS, Eryon, Acme, Scattered-Systems, and TBC, **Notion is the more-recent source**"

> "2. **Needs Joe's call:** Eryon, Acme, and Scattered-Systems have real Obsidian content. Pick merge direction per note before editing — I won't overwrite substantive `src/` notes unprompted."

Actionable: there is an out-of-vault Notion Acme page (source URL in doc: `https://app.notion.com/p/365a00d404158082b511d79cea92c1e3`) not yet reconciled into the vault's `Automatic Context Management Engine.md`.

### 2. `src/docs/projects/proton/Proton.md`
Not under `eryon/` so not excluded by the covered list. This is the vault's only standalone doc for **Proton**, the Eryon-substrate sibling service the (covered) substrate paper names as "the user-facing access layer and originator handshake... holding the composite NFT tokens and composite-hash authentication primitive." This doc is an earlier/lighter-weight take (tags: scsys, ui/ux, platform, portal, app) — no NFT/gauge-theoretic language yet, but the same "user's access layer with virtualized subnetworks + VFS" concept.

> "An all-in-one digital workspace engineered for the future with advanced security and a next-generation generative visual engine... Proton imbues each experience with a touch of _Midas_, morphing the interface directly to the context and their user."

> "Proton enables users to quickly scaffold their own custom subnetworks. By scaffolding a custom subnetwork, the users are given access to the aggregated resources and allowed to orchestrate them as they please."

> "Users can manage their files using the built-in virtual filesystem or can partition new containers geared for more specific workloads."

### 3. `claude/Vault Review.md`
Confirms `#acme`, `#plant`, `#substrate`, `#ruliad`, `#virtualization`, `#wasm` are all baseline singleton tags (population = 1), and separately flags an orphaned zettel that is a parallel/precursor conception of the Eryon "Chaos" storage service.

> "Singleton tags (count = 1) at baseline ... `acme, ai, algebra, ... plant, plugin, portal, proposal, reconciliation, retail, robotics, rpc, ruliad, runtime, sandbox, ... substrate, sync, ... virtualization, wasm, web`"

> "[[src/202604141026]] — 'Chaos': a fully referential content-addressed filesystem/hypergraph idea (UUID+hash identity, relationship-scoped access). No home yet — doesn't fit an existing `src/docs/` page."

### 4. `claude/HOME.md`
The vault's operating map. Confirms Eryon is a `#scsys`-classified project (company-owned, one-way graduation from personal), names the canonical Eryon pointer, and its tag taxonomy.

> "| `src/projects/{axiom,eryon}/` | Per-project working notes, prompts, instructions. |"

> "**`pzzld`** is Joe's personal handle... **`scsys`** (Scattered-Systems) is his company... Core company projects (Eryon, platform/protocol efforts) are `#scsys`."

> "Active projects: [[Project Axiom]] · [[src/docs/MyFi Platform]] · [[src/projects/eryon/Eryon - Journal|Eryon Journal]] · `src/projects/axiom/` · `src/projects/eryon/`"

### 5. `src/202605071408.md`
A bare zettel (NOT under `eryon/`, so not excluded) — an operator prompt directly directing PlantBase/VNode implementation work. This is the most concrete **design-decision** source in the sweep: defines the plant as a 2-simplex with LPR-transformable vertices, and states the vnode's persistence model explicitly.

> "Continue iterating the `PlantBase` and `VNode` implementations, working on materializing the abstract nature of the plant in the vnode."

> "The runtime will automatically allocate host resources into uniform partitions materialized as virtual nodes (vnodes) where each vnode is configured by its inner plant."

> "The plant is defined as a topological configuration space, specifically a 2-simplex, whose vertices are related by some *intervallic* constraints used to define the exact class of the object while being able to be transformed by any one of three (leading, parallel, relative) transformations in either discrete or continuous spaces thanks to chaining."

> "**Each vnode is also responsible for establishing its own temporality, allowing its plant and itself to be ephemeral, relying on the network as a whole to establish actual persistence.**" — this is the earliest explicit statement of the ephemeral-entity / network-as-persistence-substrate design later formalized in the (covered) substrate paper and ACME doc.

> Scope directive: "I want to continue to restrict your scope to the following directories: `crates/*` and `cmp/*`, `crates/traits`, `crates/vnode` with a majority of your changes falling in either the `crates/plants` or `crates/vnode` directory."

### 6. `src/docs/The Puzzled Substrate.md`
Pre-"Eryon" naming (branded "Puzzled") but the same vnode/plant architecture as the covered substrate paper — earliest concrete definition found in the sweep.

> "Generally speaking, the substrate is enabled by a dynamic runtime software that, upon initialization, automatically partitions host resources into uniform allocations called virtual nodes (vnode). Each instance is treated as an independent workspace with its own persistence, compute, and networking capabilities."

> "##### The Plant - A topological unit of compute — A plant is a 2-simplex $\Delta^2$ categorized and defined by the relationships between its constituent nodes. Additionally, these objects can be transformed using an LPR operator (leading, parallel, relative)..."

### 7. `src/docs/The Triadic UTM.md`
Explicitly named in the task's inspect list. Proof-style note establishing the mathematical claim the (covered) Eryon substrate paper cites verbatim in its abstract: UTM headspace ≅ 2-simplex via embedding into a frequency-domain basis; formalizes the leaky-integrate-and-fire oscillator used elsewhere.

> "Harmonizing computation is possible through the lens of topology... when considering the Wolfram (2, 3) Universal Turing Machine (W-UTM) we are able to assert topological equivalence between the triad and the machine's headspace via the 2-simplex."

### 8. `src/docs/Busy beaver Topology.md`
Explicitly named in inspect list. Introduces headspace/rulespace and the "collision" concept for busy-beaver rulesets — the informational-poverty argument (headspace alone carries no context) that motivates layering a plant/context-sheaf on top.

> "Like [[cspace]], the [[Rulespace#Headspace|headspace]] fails to contain any valuable context and is incapable of informing the machine's response given a certain key."

### 9. `src/docs/Aggregating Compute.md`
Explicitly named in inspect list. An early, un-fleshed scsys idea note describing the same virtualization/vnode partitioning scheme as the Puzzled Substrate, independent of the Tonnetz/NRT math layer — likely the seed that later grew mathematical structure.

> "All registered devices or nodes are required to install the node software, enabling the virtualization of host resources. During this process, resources are divided equally into uniform partitions called _virtual nodes_."

### 10. `src/docs/WebAssembly Sandbox (Wand).md`
Explicitly named in inspect list. Precursor conception of "spaces" as isolated virtual environments hosting a single neural agent — conceptual ancestor of the vnode-hosts-agentic-workload idea, tied to the (covered) Generative Visual Framework.

> "spaces are independent virtual environments complete with their own set of computational resources and capable of hosting a single neural agent. These spaces can be kept isolated or orchestrated together, enabled by their unique composition."

### 11–14. Term-note glossary underpinning the Eryon math (`src/docs/terms/`)
Not separately named in the task but directly cross-referenced by "The Triadic UTM" / "Busy beaver Topology," both of which the task did name. Four are substantive:

- **`Rulespace.md`** — formalizes headspace $H=\{(q,\alpha):q\in Q \land \alpha\in A\}$ and rulespace as morphisms over it; references Wolfram's ruliad.
- **`Simplex.md`** — formal n-simplex / simplicial-complex definitions underlying the plant-as-2-simplex claim.
- **`Configuration Space.md`** — c-space / world-space formalism (robotics-derived) that "Puzzled Substrate" reuses for the plant.
- **`The Neo-Riemannian Theory (NRT).md`** — full triad/LPR-transformation table and tonnetz definition, the direct math source for the substrate's Tonnetz-graded sheaves.

> Rulespace: "If a [[Configuration Space|configuration space]] defines the set of all possible configurations of a dynamical system then allow a _**headspace**_ to define the set of all possible state-symbol pairs that a given Turing machine (TM) may assume."

> NRT: LPR table — "L: [Major] Decrement the root by a semitone and move to the fifth / [Minor] Increment the fifth, move to the root" (etc.) — "these transformations can be considered **contravariant functors** acting over categories [of] triads."

### 15. `src/202604141026.md` ("Chaos")
Bare zettel, not under `eryon/`. Independent, earlier conception of what the covered substrate paper later names the **Chaos** storage service — content-addressed, hash+UUID+timestamp identified, deliberately "chaotic" rather than schema'd.

> "Chaos is a dynamic data-layer optimized to enable the production of outcomes. The system comes equipped with a built-in virtual filesystem (VFS), various analytical utilities, intelligent & efficient search... a thin identification layer names every piece of information using an `appellation(name, uuid, timestamp)` and enabling this with a simple hash... giving every entry an inherent uniqueness."

Also proposes the referential-hypergraph filesystem idea that seeds it:
> "What is we create a filesystem that was entirely referential? ... each piece of content is associated with a unique id and hash... Then we create a hyper-graph that defines relationships between objects..."

### 16. `src/202510221555.md`
Migrated Apple Notes zettel (Oct 2025) with early conceptual seeds later formalized by Eryon/ACME: CodeBox/VDS (wasm-module-per-block generative workspace), a spiked-neuron/plant musing, and a tonnetz-as-Fourier-middleware musing that anticipates the substrate's frequency-domain triad embedding.

> "**Virtual Design Studio (VDS)** — a generative workspace for composing next-gen experiences via drag-n-drop, natural language, or code, where each code block executes as a single wasm module."

> "What if a plant was the infrastructure needed to host isolated spiked neurons — could that give them tonality?"

> "Could a tonnetz work as a type of middleware for a Fourier transform, where each part lies within the bounds of some triad?"

### 17. `src/docs/Puzzled Gateway.md`
Explicitly named-adjacent (not in task's list but caught by (a)'s grep). Describes the node-software vocabulary the substrate later reuses.

> "The Puzzled Gateway is a lightweight node software enabling users to seamlessly register, scaffold, and extend their clusters."

### 18. `src/docs/projects/axiom/Project Axiom.md`
A MOC/index page — low content value, but its capture-log entries point at two (covered) zettels that carry an "eryon tease," useful as a cross-reference trail:
> "[[src/202607011555]] (2026-07-01) — self-host redis/postgres musing, dev.6 handoff decision, project 'eryon' tease, bot-runtime-as-cost-coverage motivation monologue"

### 19. `src/daily/2026-02-21.md`
One line, low content, but establishes Eryon/scsys as a standing background concern in early 2026 independent of any Axiom sprint:
> "While #eryon and #scsys are always in the back of our minds, we must focus first on freeing up some time."

### 20. `src/docs/terms/Autopoietic Biological Systems.md`
Flagged as a **gap**, not content: the covered substrate paper's abstract claims the fabric is "proto-cognitive in the autopoietic sense," but this term note is an empty template stub (no definition, no background) — the vault has no actual grounding for that claim yet.

### 21. `README.md`
One line, vault-level framing only (not eryon/acme-specific), but relevant as the umbrella self-description under which ACME's "second brain" framing sits:
> "This vault is designed as a second brain used to support various personal and professional endeavors."

---

## EXCLUDED — already covered per task instructions (not re-digested)
- `src/docs/Automatic Context Management Engine.md`
- `src/docs/Generative Visual Framework.md`
- `src/docs/projects/scsys/Scattered-Systems.md`
- `src/202605041757.md`
- `src/202607011632.md`
- `src/202607011555.md`
- `src/daily/2026-01-27.md`
- Everything under `src/docs/projects/eryon/` (Project Eryon - The Substrate.md, Eryon - Journal.md, drafts/{Overview, Scratch, Thought Experiment, The Eryon Protocol (draft)}.md, sections/{Network, Plant, Runtime, Virtual Node}.md, Eryon - Canvas.canvas)

---

## IRRELEVANT — examined and ruled out (one line each, for auditability)

**Axiom sprint-prompt zettels** (grep-matched only on incidental "wasm" or the shepherd-jargon "plant"/"disarray"-adjacent terms; no eryon/acme content): `src/202603260834.md`, `src/202603301756.md`, `src/202604011241.md`, `src/202604011450.md`, `src/202604012249.md`, `src/202604021508.md`, `src/202604021652.md`, `src/202604031922.md`, `src/202604061324.md`, `src/202604130726.md`, `src/202604140629.md`, `src/202604171841.md`, `src/202605111425.md`, `src/202605170924.md`, `src/202606261702.md`, `src/202606301539.md`, `src/202607011917.md`, `src/202607012217.md`, `src/202607031534.md`, `src/202607032026.md`, `src/202607130929.md`.

**Claude-authored axiom sprint captures** (same reason — "plant" = shepherd:plant term, `#scsys` tag only): `claude/captures/axiom-v0.3.3-dev.0-sprint.md`, `claude/captures/axiom-v037-dev1-status.md`, `claude/captures/axiom-v037-dev2-status.md`, `claude/captures/axiom-v037-dev6-status.md`, `claude/captures/axiom-v037-dev8-close-dev9-live-flip.md`, `claude/captures/axiom-v037-dev8-sprint-live.md`, `claude/captures/axiom-v038-dev1-close-dev2-open.md`, `claude/captures/shepherd-plugin-v628-refactor.md`, `claude/Shepherd.md`.

**Axiom project docs**: `src/docs/projects/axiom/Host Environment Isolation.md` (ephemeral = a `$TMPDIR` path, not vnode-related), `prompts/AXIOM_REFOCUSING.md`, `prompts/axiom@v0.3.4-dev.6.md`, `prompts/axiom@v0.3.5-dev.0.md`, `prompts/axiom@v0.3.5-dev.9.md`, `prompts/axiom@v0.3.6-dev.0.md`, `prompts/axiom@v0.3.6-dev.9.md`, `Type Driven Synthesis.md`.

**RMS/scsys-adjacent**: `src/docs/projects/rms/docs/TBC - Recommendations.md`, `TBC x SCSYS Brief.md`, `RMS - Proposal.md`, `RMS - Recipe Studio.md`, `TBC - Menu Ideas.md`, `src/docs/projects/scsys/Scattered-Systems - Ideas.md`.

**WASM/Docker generic**: `src/docs/terms/WebAssembly System Interface (WASI).md`, `src/docs/terms/WebAssembly.md`, `src/docs/Setting up Docker for WASM Containers.md`.

**Other src/docs**: `MyFi Platform.md`, `Smart Shop.md`, `SmartList.md`, `Composite Governance.md` (DAO/governance idea, no eryon tie despite thematic adjacency to "agent contracts"), `D-Sync.md` (generic sync-protocol idea), `DataFlow.md` (unrelated data-editor app, "flock" footnote only), `AutoBuild.md` (Rust tooling idea), `The Plan (2026).md` (personal life plan).

**Generic glossary term notes** (grep-matched on math/CS vocabulary shared with Eryon but no eryon/acme-specific content): `src/docs/terms/Driver.md`, `Turing Machine.md`, `Space.md`, `Multiway Computation.md` (empty template stub), `Music Theory.md`, `Spiking Neural Networks (SNN).md`, `Artificial Intelligence.md`, `Blockchain.md`.

**Daily**: `src/daily/2026-07-13.md` (only re-points to the already-covered HOME capture-log entry).