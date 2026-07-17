# acme, second-brain compilation (2026-07-17)

Compiled for the v0.4.1 working branch from the pzzld Obsidian vault, per Joe's directive to harvest the second brain before any design or code work. Two raw digests (the 100KB substrate paper, the full vault sweep) live in `raw/`. This file is the curated load: what acme IS, what it sits on, what is settled, what is open, and where the repo diverges from the spec.

## 1. What acme is (settled by the spec)

`src/docs/Automatic Context Management Engine.md` (created 2026-05-06) is a real spec, not a sketch. Its own summary:

> "ACME is a thin policy layer over a thick mathematical substrate."

The eryon substrate maintains a context sheaf $\mathcal{F}$ over an octave-graded Tonnetz; context fragments ("stalks") are typed by position; the substrate guarantees the sheaf math (sections, gluing, cohomology). ACME supplies every policy that turns that machinery into a usable context layer for agentic workflows. The division of labor, verbatim from the substrate paper §4.1:

> "Consumers of the substrate (notably ACME, which maintains $\mathcal{F}$ on behalf of agentic workflows) supply the policies that determine _what_ gets written, retained, or evicted; the substrate guarantees the underlying sheaf-theoretic well-formedness."

And its necessity claim (§7.4): once the substrate exists, ACME is "nearly necessitated by the design, since the context sheaf $\mathcal{F}$ exists at the substrate level but its retention, decay, and aggregation policies do not — those policies are what ACME supplies."

Four policy families (ACME doc §3):

| Policy | Mechanism | Point |
|---|---|---|
| Relevance | $r(c,q) = \alpha \cdot s_{type} + \beta \cdot e^{-\gamma d(\sigma,\sigma_q)} + \delta \cdot a_{\mathbb{Z}} + \epsilon \cdot \pi_q$ | Multi-component score with independent semantics per term; $\pi_q$ is the only consumer-supplied part (task-specific projection, e.g. embeddings) |
| Decay | $w_t(c) = w_{t-1}(c) \cdot e^{-\lambda d_A(\sigma)}$ | Topological, not temporal: distance to the active frontier, not wall-clock age. Old fragments the frontier moves toward auto-revive. Wall-clock kept only for tie-break and audit |
| Eviction | $\mathrm{argmin}[w \cdot H^0\text{-contribution} - \mu \cdot H^1\text{-contribution}]$ | Evict what supports coverage least and obstruction most. Contributions are estimated; degraded estimates must produce conservative (over-retaining) eviction, never wrong eviction |
| Aggregation | $\mathrm{aggregate}(U)$ at Local / Layer / Global scope | Returns a consistent section ($H^0$) or the obstruction class ($H^1$); scopes degenerate gracefully at small scales |

Consistency contract (§4): two distinct failure states, missing coverage ($H^1 = 0$, section pending) vs obstruction ($H^1 \neq 0$, actual contradiction), with three per-consumer obstruction policies: **strict** (refuse, surface it; the default), **best-effort** (largest clean subsection, annotated), **quorum** (weighted majority under a consumer-supplied weight).

Composition (§5): eviction pages stalks to **Chaos** (topological storage); from ACME's side eviction is a write, from Chaos's side hot residency is a read-through cache. ACME never engages the **Disarray** mainnet directly (cross-cluster stalks arrive through the ordinary attach primitive). Any future service whose state lives as stalks on $\mathcal{F}$ gets ACME's policies for free.

Scale rule (§7): well-defined from a single triad upward; the API surface must stay stable across scales, with new capability arriving via coverage, never via method replacement.

## 2. The substrate underneath (what acme consumes)

Full digest: `raw/2026-07-17-eryon-substrate-digest.md`. The load-bearing minimum:

- **Plant**: stateless, immutable topological type-boundary; a UTM-derivative (Wolfram (2,3)) whose 6-element headspace maps onto the 6 flags of a 2-simplex. The 48 configurations are the rooted triads; $G \cong D_{12} \times C_2$ (P, L, R plus the added F involution) acts simply transitively, so the configuration space is a $G$-torsor and **transformations are coordinates** (addressing IS the group action).
- **Vnode**: the mutable, stateful executor bound to a device. All state is vnode state; "plant state does not exist as such."
- **The ephemerality asymmetry (the center of Joe's brief)**: "Plants are stable across cluster changes because they are properties of the topology; vnodes are ephemeral because they come and go with hardware" (§3.1). Earlier and plainer, zettel 202605071408 (2026-05-07): "Each vnode is also responsible for establishing its own temporality, allowing its plant and itself to be ephemeral, relying on the network as a whole to establish actual persistence." The ephemeral entity is native to the substrate; durable continuity across vnode churn is exactly the ACME + Chaos charter.
- **Five sheaves** over the octave-graded Tonnetz $\tilde{X} = X \times \mathbb{Z}$: assignment $\mathcal{A}$ (plant typing), state $\mathcal{S}$ (per-vnode, not cohomological), **context $\mathcal{F}$ (ACME's charge)**, performance $\mathcal{P}$ (Laplacian weights), allocation (vnode to device).
- **Five primitives**: attach, restrict, glue (returns the $H^1$ obstruction on failure), transform (contravariant $G$-pullback), aggregate ($H^0$ over a region). ACME composes these and never modifies them.
- **Two clocks**: every context update carries wall-clock (causal, Lamport-interpretable) plus plant proper-time (local, not globally ordered). Consensus is sheaf merge (restriction-map agreement); it subsumes CRDT merge and can tell missing-data from contradiction, which CRDTs cannot.
- **Wire format**: substrate-wide FFT frequency-domain encoding commitment (protobuf/JSON explicitly rejected). PLR-compatibility of that encoding is flagged open research.
- **Two modes**: symmetric phase (pre-originator; structure defined only up to group action) then gauge-fixed phase (the Proton handshake picks the frame). Naming hygiene on record: the symmetric phase was renamed from "chaos" precisely to avoid colliding with the Chaos storage service.

## 3. The ecosystem (scsys map, migrated 2026-07-13)

1. **Eryon**, the substrate. 1.1 **Flow**, the temporal codec/translator.
2. **Proton**, the flagship portal (GVF interface philosophy; composite platform token: wallet, namespace registry, node registry).
3. **ACME** (this repo).
4. **Disarray**, hybrid multichain whose nodes are whole clusters. 4.1 **Aether**, compute layer. 4.2 **Chaos**, decentralized storage.
5. **Reaction**, social space. 6. **Ell**, generalized intelligence engine.

Sibling contracts that bind acme: **Chaos** holds "memory of past configurations" (storage is Chaos's job, policy is acme's); **Flow** is acme's temporal counterpart and is explicitly "anticipated but not-yet-committed". Until Flow exists there is NO proper-time tiebreak for concurrent context updates; sheaf-merge-or-surface is the only resolution. That is the largest gap acme inherits.

Two roster caveats surfaced during the 2026-07-17 Notion validation: (1) "The Scattered Papers" (the collection's foundation doc, imported to the vault at `src/docs/business/The Scattered Papers.md`) assigns the virtualized mainnet to **Aether** and never mentions Disarray, while the substrate paper and the list above make **Disarray** the multichain and Aether the compute layer. Which service owns mainnet/consensus is an open naming decision for Joe. (2) The Scattered Papers also proposes that Flow might fold into the substrate's M2M gossip with timestamp handling rather than stand alone.

## 4. Joe's brief, translated onto the spec

Joe's words (2026-07-17): acme is "an automated context management engine enabling ephemeral entities with the type of data-fluid persistence necessary to support 'memory' and 'personality'."

Mapping onto the settled design:

- **Ephemeral entities**: the vnode/agent. State-bearing, hardware-bound, expected to die. Already first-class in the substrate.
- **Data-fluid persistence**: stalks flowing between hot residency (ACME policy) and Chaos (content-addressed, position-indexed durable state). Eviction is paging, not deletion; the data moves and identity survives by simplicial coordinates.
- **Memory**: ACME's four policies over $\mathcal{F}$. This half is specified.
- **Personality**: the open half. Nearest primitives in the corpus: the consumer projection $\pi_q$ plus the kernel coefficients (what an entity attends to), Proton's composite identity (who it is, but scoped to user/cluster, not to an agent), and GVF's perception functor $F: \mathcal{L} \to \mathcal{S}$ (how one latent state renders differently per observer). None of these is an agent-personality layer yet. This is where the first design conversation with Joe should start.
- Joe's cartesian doctrine from the axiom flip day (controls and metrics must be fields over the decision space, never collapsed scalars) is literally the ACME doc's §6 argument: conventional relevance collapses to one scalar; ACME keeps relevance a multi-component field with independent semantics per axis. Same instinct, richer space.

## 5. Repo reality vs spec (resolve before writing code)

The crates (v0.4.1, ~1,530 LOC) predate the spec and model a DIFFERENT conception:

- `crates/engine/src/engine.rs`: "focuses on aggregating information from various sources" via `SourceManager` / `Scheduler` / `PipeRouter`, all empty shells; `Scheduler::start` is a `todo!()`.
- `crates/traits`: `Context { type Space }`, `Handler<E>`, `Component {}` markers. `crates/core`: events, timestamps, an `Interface<T>` wrapper. Scaffold-grade; no policy machinery, no sheaf or substrate vocabulary anywhere.
- Timeline: repo scaffold Feb 26; ACME spec 2026-05-06; substrate paper 2026-05-07, reworked after the 2026-07-01 rigor-audit session (zettel 202607011632). The code is months behind the theory and pointed sideways (data-source aggregation vs policy-over-substrate).

**RESOLVED by Joe (2026-07-17 evening): no code work until the idea is extensively planned.** His ruling, near-verbatim: acme is too abstract to wing like eryon was; it needs a special approach, particularly initially, to ensure the level of abstraction is enforced and designed behaviors manifest correctly at implementation time; the second brain is where the ideas are maintained, updated, and evolved across sessions, shaping the implementation path and preventing continual rewrites. Practical consequence: the vault (`/Users/jo3/vaults/pzzld`, now git-versioned) is the design surface; sessions produce and refine vault documents, not crates. The crate divergence noted above stays on record for whenever implementation begins; it blocks nothing now because nothing is being built. For that future date: the ACME doc needs NO eryon runtime to start, since the five primitives define a contract an in-memory store can satisfy for tests and evals.

Also on record, zettel 202605041757 (the v0.3.3 axiom roadmap): "Begin designing a so-called automated context management engine (hint; this might be a separate project by itself that I have already scaffolded at https://github.com/FL03/acme)... We might even end up implementing a basic version here before extracting to perfect elsewhere." The axiom shared-context work was always the intended proving ground.

## 6. Open questions, ranked

1. **Flow does not exist** (substrate §7.4): concurrent context updates have no temporal tiebreak. Bounds what acme can promise about ordering.
2. **Spec/scaffold divergence** (§5 above): deferred, not blocking — implementation itself is deferred by Joe's planning-first ruling.
3. **Personality layer undefined** (§4 above): nothing in the corpus covers per-agent identity/continuity semantics.
4. **Parameter-free kernel conjecture** (ACME §8): can $\alpha, \beta, \gamma, \delta, \epsilon, \mu, \lambda$ be derived from substrate configuration? Would delete most consumer-facing tuning.
5. **$\pi_q$ substrate-native?** (ACME §8): a sheaf of learned features would make relevance fully geometric.
6. **Estimator constructions** (substrate §4.3): $H^0$/$H^1$ contribution estimates are open engineering; must fail conservative (false-positive obstruction OK, false-negative never).
7. **Consumer isolation** (ACME §8): present design assumes trusted consumers; pollution is detectable via $H^1$ but not prevented.
8. **Smears / partial-occupancy stalks** (ACME §8): the recommended policy is resolve-before-considering; relevance-aware mid-transformation queries are research.
9. **PLR-compatible FFT encoding** (substrate §5.6): substrate-side, but acme's transform path inherits whatever lands.

## 7. Provenance cautions

- **RESOLVED (2026-07-17 evening, validated via Notion MCP).** The hold is lifted; Joe directed validation and set the direction: everything centralizes into the vault, Notion is being wound down. Findings: the Notion "Acme" page (v0.0.1, edited 2026-05-31) WAS a newer revision of the vault text (v0.0.0, May 6) — it carried the corrected octave-graded/$D_{12} \times C_2$ terminology and named Chaos/Disarray. Its deltas are now merged into the vault ACME doc (bumped to 0.0.1); the Notion page is superseded. The Notion "Eryon - Technical Specification" (edited 2026-07-01T22:57Z) is the same generation as the vault substrate paper, which was written ~12h later — vault canonical, no merge needed. A 3rd Notion page ("The Scattered Papers", 22KB foundation doc, v0.1.0) had no vault equivalent and was imported to `src/docs/business/The Scattered Papers.md`. An 89KB unnamed Notion page is a superseded May draft of the substrate paper — historical only. The vault is now the single source of truth for all eryon/acme design text.
- **Superseded claims live in the drafts.** The substrate paper corrected its own earlier drafts on record: the $D_{48}$-on-pitch-class-sets claim is retracted (rooted triads and $D_{12} \times C_2$ instead), the universal-cover framing is retracted (plain product $X \times \mathbb{Z}$ instead), and the "chaos phase" was renamed the symmetric phase. `drafts/Eryon - Scratch.md` still describes Chaos as a 0-simplex hypergraph memory substrate (rshyper); the substrate paper superseded that with content-addressed stalk storage. Canonical order: **substrate paper + ACME doc, then sections/, then drafts/, then zettels**, recency breaking ties.
- **The vault reorganized mid-harvest** (2026-07-17 ~16:17 local): project trees moved under `src/docs/projects/`. Paths in this file use the post-move locations.

## 8. Source index

| Source (vault path) | Coverage |
|---|---|
| `src/docs/Automatic Context Management Engine.md` (2026-05-06) | direct read, full |
| `src/docs/projects/eryon/Project Eryon - The Substrate.md` (100KB, 398 lines) | subagent digest, 100% coverage, saved to `raw/` |
| `src/docs/projects/eryon/drafts/` (Overview, Scratch, Thought Experiment, Protocol draft) | direct read, full |
| `src/docs/projects/eryon/sections/` (Plant, Virtual Node, Runtime, Network) + `Eryon - Journal.md` | direct read, full |
| `src/docs/projects/scsys/Scattered-Systems.md` | direct read, full |
| `src/docs/Generative Visual Framework.md` | direct read, full |
| Zettels: `202605041757` (roadmap naming acme + FL03/acme), `202605071408` (plant/vnode ephemerality), `202607011632` (rigor-audit kickoff), `202607011555` (eryon "real dream" motivation), `daily/2026-01-27` (eryon-for-agentic-systems note) | direct read, full |
| 21 additional relevant notes + ~60 examined-and-excluded (auditable list) | vault sweep, saved to `raw/2026-07-17-vault-sweep.md` |

Notable from the sweep: `The Puzzled Substrate.md` and `Aggregating Compute.md` (pre-eryon vnode lineage), `The Triadic UTM.md`, `Busy beaver Topology.md` and `terms/` (Rulespace, Simplex, Configuration Space, NRT: the math bedrock), `202604141026` (the original Chaos zettel: referential content-addressed hypergraph filesystem), `projects/proton/Proton.md`, `202510221555` (2025 seeds: wasm-module-per-block workspaces, plant-hosted spiking neurons, tonnetz-as-Fourier-middleware).

Notion sources (validated 2026-07-17 via Composio Notion MCP, "Puzzled" workspace, Docs database `365a00d4...c1e3`, 10 rows): Acme row `1d1a00d4...51aa` (merged into vault, superseded), Eryon - Technical Specification `158a00d4...06fc` (same generation as vault paper, superseded), The Scattered Papers `75ca00d4...5811` (imported to vault), unnamed 89KB substrate draft `66aa00d4...2390` (historical). Remaining rows are RMS/TBC business docs, out of acme scope. The vault is canonical for all of these going forward; the vault itself is git-versioned as of 2026-07-17 (local repo, baseline commit).
