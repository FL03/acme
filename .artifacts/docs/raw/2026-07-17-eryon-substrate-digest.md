> Compiled 2026-07-17 by the FABLE harvest workflow (wf_4b89b7ab-b27) from the pzzld Obsidian vault.
> NOTE: the vault reorganized the same day; project trees now live under src/docs/projects/.

# PROJECT ERYON — "The Substrate" — STRUCTURED DIGEST

Source: `/Users/jo3/vaults/pzzld/src/projects/eryon/Project Eryon - The Substrate.md` (398 lines, 100,529 bytes, read in full, lines 1-398, 100% coverage confirmed via `wc -l`).

---

## 1. THESIS

The document is a design/proof paper for **eryon** (aliases: "eryon protocol," "the substrate"), a substrate for distributed computation built on three converging mathematical structures: (1) a topological identification of Universal Turing Machine "headspace" with an oriented 2-simplex, (2) the Neo-Riemannian symmetry group $G \cong D_{12} \times C_2$ (order 48) acting simply transitively on the 48 rooted triads of a generalized Tonnetz, and (3) equivariant cellular sheaves over the substrate's octave-graded configuration space. The unit of computation, a **plant**, is a self-modifying-in-position (but not self-modifying-in-definition) UTM-derivative whose headspace is a point on this 48-configuration torsor; plants execute inside **vnodes** on physical **devices**. The substrate's core claim is that coordination properties conventional distributed systems bolt on by hand — load balancing, consistency detection, addressing, consensus — instead emerge natively from the algebraic/topological structure: load balancing is diffusion under a sheaf Laplacian, consistency is sheaf cohomology, addressing is the group-torsor property, and consensus is restriction-map agreement. The substrate runs in two modes: an unobserved, fully symmetric self-organizing "symmetric phase" and a post-originator "gauge-fixed" deterministic phase triggered by an access-layer handshake (Proton). The paper argues, on autopoietic-systems grounds (self-bounded, self-maintaining, self-producing, and — via tunable policy — goal-directed), that the resulting fabric is **proto-cognitive**. The substrate itself is explicitly scoped as the *foundational base layer only*; higher-order consumer services — ACME (context management), Flow (temporal reconciliation), Chaos (storage), Aether (compute submission), Disarray (inter-cluster consensus/multichain), and Proton (identity/access) — are named as things the substrate's design "naturally pulls into existence" but are documented in companion documents not included in this file. The paper closes with an explicit open hypercomputation conjecture (Zeno-machine analogy under continuous surface motion) that it deliberately does not attempt to resolve.

---

## 2. GLOSSARY

Organized roughly in order of first appearance / conceptual grouping.

### Core identity terms
- **eryon / eryon protocol / the substrate** — the whole system described in this paper. "we construct a distributed computational fabric — referred to throughout as **eryon**, the **eryon protocol**, or simply **the substrate**" (§1, L20). Substrate is explicitly the *foundational base layer* (§1, §4.1, §7.4).
- **originator** — the entity/observer whose engagement triggers gauge-fixing (§6.2). Not otherwise defined beyond its functional role.
- **user-frame** — one trivialization of the graded torsor, unique per user/cluster; the addressable identity concept used at cluster boundaries (§5.5).

### Turing-machine foundation (§2.1–2.2)
- **Wolfram (2,3) Universal Turing Machine** — the smallest known (weakly) universal Turing machine, $|Q|=2$, $|\Sigma|=3$, proved by A. Smith (2007). "the proof establishes _weak_ universality, in which the machine starts on an infinite tape carrying an eventually periodic background pattern rather than an all-blank one... and the (2,3) machine has no halting state" (§2.1, L34).
- **headspace** — "the product $Q \times \Sigma$ representing the joint state of the machine at a point of computation" (§2.2, L38).
- **flag** (of a 2-simplex) — "a pair (orientation, distinguished vertex); a 2-simplex on three vertices has exactly $2 \times 3 = 6$ flags" (§2.2, L38). The headspace-to-2-simplex identification is a bijection between $Q\times\Sigma$ (6 elements) and the 6 flags: control state → orientation, scanned symbol → distinguished vertex.

### Neo-Riemannian / Tonnetz apparatus (§2.3)
- **Tonnetz** — "a simplicial complex whose 2-simplices are triads, whose 1-simplices (edges) are common pitch-class pairs, and whose 0-simplices (vertices) are the twelve pitch classes" (§2.3, L44), per Lewin (1987) / Cohn (1997).
- **P, L, R (the Wechsel)** — Parallel, Leading-tone, Relative involutions on the 24 consonant triads, "each preserving two common tones and moving the remaining voice by step" (§2.3, L44). $\langle P,L,R\rangle \cong D_{12}$, order 24, acts simply transitively on consonant triads.
- **F (fifth-alteration operator)** — the substrate's added fourth involution, "the natural fourth parsimonious move: it displaces exactly one voice by a semitone (the fifth) and preserves two common tones between every triad and its image" (§2.3, L53). Commutes with P,L,R; extends the group to order 48.
- **rooted triad** — a pair (root, quality), formalized as $\mathcal{C} = \mathbb{Z}_{12}\times\{+,-\}\times\{0,1\}$: triple $(r,\varepsilon,a)$ of root, orientation, alteration. "$(r, +, 0)$ is $r$-major, $(r, -, 0)$ is $r$-minor, $(r, +, 1)$ is $r$-augmented, and $(r, -, 1)$ is $r$-diminished" (§2.3, L48).
- **chord-class** — "abbreviates a rooted triad considered independently of layer — one of the 48 torsor addresses of Section 2.3" (§3.2, L143).
- **$G$ / the Neo-Riemannian group** — $G := \langle P,L,R,F\rangle = \langle P,L,R\rangle \times \langle F\rangle \cong D_{12}\times C_2$, order 48, acting **simply transitively** on the 48 rooted triads (§2.3, L57–59). "In the substrate, the 48 rooted triads are identified with the 48 possible headspace configurations of the plant... and the operations P, L, R, F become _headspace adjustment operations_" (§2.3, L59).
- **$G$-torsor** — the property that "every configuration is carried to every other by exactly one group element" (§2.3, L59). Formal statement: $\mathcal{C}$ is a $G$-torsor.
- **$r := LR$** — the order-12 rotation element, "moves through the cycle of fifths" (§2.3, L55).
- **consonant subcomplex** — the classical 24-triad Tonnetz torus: "twelve vertices, the 36 edges lying in consonant triads, and the 24 consonant triangles" with $\chi=0$ (§2.3, L63).
- **full four-quality complex** — the branched 2-complex over all 40 pitch-class sets (augmented/diminished included), 42 edges, $\chi=10$, "not a surface" (§2.3, L63). Substrate's sheaf/gluing/adjacency machinery is defined here; addressing lives on the torsor $\mathcal{C}$.
- **T/I group** — transposition-inversion group, also $\cong D_{12}$; "the PLR group is precisely its centralizer" (§2.3, L75) — Lewin duality, load-bearing for §5.6's wire-encoding discussion.
- **doubly minimal-universal** — coinage for the plant construction: "crosses the smallest known weakly universal computation skeleton (Wolfram (2,3)...) with the smallest transitive symmetry the 48-configuration space admits (the simply transitive $G$-action...)" (§2.4, L83).

### Plant / vnode dynamics (§2.4–2.5, §3.1)
- **plant** — "a UTM-derivative whose transition function takes the form $\delta: Q\times\Sigma \to Q\times\Sigma\times M_{\text{tape}}\times M_{\text{surface}}$" (§2.4, L79–81); *immutable*, stateless, "a topological type-boundary — a specification of which computations are valid at a given surface and which surface-motions are available" (§2.4, L85). "Plant _code_ is a singleton per surface-class: equivalent surfaces in different layers share the same plant via the $\mathbb{Z}$-action. Plant _state_ does not exist as such" (§3.1, L131).
- **surface motion** ($M_{\text{surface}}$) — "a generator of the Neo-Riemannian group $G$ applied to the plant's configuration, carrying it to the uniquely determined image configuration" (§2.4, L81).
- **vnode (virtual node)** — "the mutable, stateful object — the operational realization that actually executes the plant against incoming work... it is the vnode that induces the transformation" (§2.4, L87). Bound to a device.
- **plant-vnode pair / dual dynamical system** — "the plant supplies the mathematical scaffold under which dynamics are well-defined, and the vnode supplies the actual dynamics" (§2.4, L87). Mutation lives here, not in the plant.
- **surface** — "a rooted triad at a layer — a point $\sigma=(c,n)\in\mathcal{C}\times\mathbb{Z}$ of the graded torsor" (§3.1, L129).
- **device** — "a physical host machine" (§3.1, L129).
- **projection maps** — $\pi_{\text{surf}}: \text{Plant}\to\text{Surface}$, $\pi_{\text{vn}}: \text{VNode}\to\text{Plant}$, $\pi_{\text{dev}}: \text{VNode}\to\text{Device}$ (§3.1, L127); $\pi_{\text{vn}}$ is 1:1 per fixed surface, $\pi_{\text{dev}}$ is many-to-one.
- **halting** (of a plant-vnode pair) — "corresponds to a fixed point of the combined transition relation... a property of the pair, not of either component alone" (§2.4, L91), per Kůrka (1997) dynamical-systems framing.
- **repair operator** — response to an *out-of-alphabet event* (head scans symbol $s\notin\sigma(c)$): "the vnode applies surface-motion generators until the scanned symbol enters the alphabet, and only then executes the tape step... a minimal-length word $w$ over $\{P,L,R,F\}$ with $s\in\sigma(w\cdot c)$" (§2.5, L95). Worst-case repair length 3, mean 1.58 (machine-checked over 432 out-of-alphabet pairs); Cayley diameter of $G$ under $\{P,L,R,F\}$ is 6.
- **atlas machine** — "The twelve-symbol global alphabet is covered by the 48 triadic charts... a plant computes in one chart at a time and changes charts precisely when the data demands it. Computation on the substrate is the interleaving of in-chart tape steps with chart transitions" (§2.5, L99).
- **proper time / plant time** — three named semantics: **free retune** (surface motion costs 0 ticks), **unit-cost retune** (each generator application = 1 tick), **two-clock semantics** (tape time and surface time as separate, partially-ordered counters). "The substrate adopts the two-clock semantics with unit surface costs as its operational meaning" (§2.5, L101).

### Octave grading (§2.6)
- **octave-graded Tonnetz $\tilde{X}$** — $\tilde{X} := X\times\mathbb{Z}$, "a $\mathbb{Z}$-indexed stack of complete copies of the Tonnetz complex $X$" (§2.6, L109–111). Explicitly *not* a universal cover — an earlier draft's claim to that effect is corrected: "octave shift acts trivially on pitch classes, so it is not a deck transformation of any cover of $X$ at all" (§2.6, L115).
- **layer** — $X_n := X\times\{n\}$, "the register-$n$ variant of every surface" (§2.6, L113).
- **$G\times\mathbb{Z}$** — combined symmetry group on $\tilde{X}$; $G$ acts horizontally (intra-layer), $\mathbb{Z}$ acts vertically (inter-layer octave shift). $\mathcal{C}\times\mathbb{Z}$ is a $(G\times\mathbb{Z})$-torsor (§2.6, L117).
- **equivariance type** (foreshadowed here, fully defined §5.4) — governs whether workloads distribute horizontally or vertically.

### Substrate entity/execution model (§3)
- **resource census** — a node's first action on init: "an inventory of available CPU, memory, storage, and network capacity on the host," determining $k$, the number of vnode partitions/triads the host will scaffold (§3.4, L155).
- **fragmented Tonnetz / fragment** — "a set of triads persisted locally by a node, glued along their shared 1-simplices into a connected subcomplex of the full Tonnetz" (§3.2, L143). This, not a single triad, is "the substrate's operative primitive at the node level."
- **abundance regime** (multi-vnode) — transformation $g\in G$ implemented as a **route**: "the work is instantiated on the vnode hosting the surface $g\cdot\sigma$ and forwarded there. Transformations are coordinates" (§3.2, L137).
- **scarcity regime** (single vnode) — transformation implemented as **dynamics**: continuous local deformation, producing a **harmonic smear** — "a transient configuration distributed across multiple adjacent triads with continuous weights," terminology from Callender (2004) (§3.2, L139). "Smears remain private to the executing node... resolves to discrete state before crossing the wire" (§3.2, L141).
- **fragment density / phase transition** — the regime choice is "a qualitative property of the cluster's geometric coverage, and the regime transition is a phase transition keyed to that coverage rather than to any particular node count" (§3.2, L143).
- **vertical distribution / lift and project** — moving work between layers; free for *scale-invariant* workloads (RG fixed points under $\mathbb{Z}$-action), costed for *scale-dependent* ones (§3.3, L147).
- **rolling depth (layer redundancy)** — "Layer growth proceeds with a rolling depth: optimization activity within layer $n$ is deferred until layer $n+1$ is also complete and layer $n+2$ has begun scaffolding" (§3.3, L151) — guarantees ≥2 octave-distinct copies of any triad under optimization.
- **self-configuration / scaffolding** — node-level bootstrap: census → $k$ → network registration → gossip-informed selection of $k$ triads extending cluster coverage, preferring "edges that move the fragment outward into uncovered chord-classes" and skipping low-value inversion cycles ($LL$, $PP$, $RR$) (§3.4, L155–159).
- **sub-redundancy regime** vs **redundancy-protected regime** — "In the _sub-redundancy regime_ — clusters with fewer than two complete layers scaffolded — the substrate operates correctly in all its mathematical properties... but no redundancy guarantee against node departure is provided" vs. "In the _redundancy-protected regime_ — at least two complete layers — the rolling-depth threshold ensures every rooted triad is covered at two octave-distinct surfaces" (§3.4, L165).

### Sheaf/interface machinery (§4)
- **cellular sheaf** — assigns stalks $\mathcal{F}(\sigma)$ to surfaces, $\mathcal{F}(e)$ to shared edges, restriction maps $\rho_{\sigma\to e}$ (§4.1, L173). Citing Curry 2014, Robinson 2014, Hansen & Ghrist 2019.
- **section** — an assignment where "adjacent surfaces agree on every overlap"; sections over region $U$ form $H^0(U,\mathcal{F})$ (§4.1, L173).
- **coboundary operator** $(\delta x)_e = \rho_{\tau\to e}(x_\tau) - \rho_{\sigma\to e}(x_\sigma)$; sections are $\ker\delta$; $H^1$ = "the quotient of edge-level data by the disagreements that local re-attachment can produce — the home of the obstruction classes" (§4.1, L175–177).
- **The Five Sheaves** (§4.1, L181–191):
  - **assignment sheaf $\mathcal{A}$**: $\sigma\mapsto \text{Plant}(\sigma)$; $(G\times\mathbb{Z})$-equivariant.
  - **state sheaf $\mathcal{S}$**: $\text{vnode}\mapsto\text{State}$; layer-indexed, per-vnode.
  - **context sheaf $\mathcal{F}$**: "carries the data and coordination information that flows through the substrate... Stalks $\mathcal{F}(\sigma)$ are typed by the plant at $\sigma$." (This is the sheaf ACME will maintain — see §3.)
  - **performance sheaf $\mathcal{P}$**: "load, available resources, plant compatibility"; supplies weighted sheaf-Laplacian weights.
  - **allocation sheaf**: vnode identifiers → devices.
  - Scoping note: $\mathcal{A},\mathcal{F},\mathcal{P}$ are true cellular sheaves over $\tilde{X}$; $\mathcal{S}$ and allocation are indexed over operational objects (vnodes/devices), sheaves over physical adjacency, not carriers of cohomological content over $\tilde{X}$.
- **The Five Primitives** (§4.2, L195): $\mathrm{attach}(\sigma,c)$, $\mathrm{restrict}(\sigma\to e, c_\sigma)$, $\mathrm{glue}(\{c_{\sigma_i}\})$ (succeeds iff $\delta c=0$; else returns obstruction class in $H^1$), $\mathrm{transform}(g,c)$ (pullback $g^*c$, composition law $g^*h^*=(hg)^*$), $\mathrm{aggregate}(U)$ (computes $H^0(U,\mathcal{F})$).
- **obstruction vs. fault** — "$H^1$... is the _capacity_ for obstruction... A _fault_ is current data actually carrying such a pattern" (§4.3, L201). Runtime dichotomy: coverage gaps (no data) → fill via gossip; vanishing $[d]\in\check H^1$ → relax via local correction; nonvanishing class → genuine fault, "the distributed analog of holonomy... require[s] rollback... or escalation" (§4.3, L203).
- **estimator** — because exact cohomology computation is expensive, "routine consistency checks... operate on _estimators_" (§4.3, L205): residual norm $\|\delta x\|$, spectrum of sheaf Laplacian $L=\delta^*\delta$ (Hansen & Ghrist 2019), incremental persistent homology (Carlsson 2009), localized cocycle inspection, probabilistic sampling. Must be conservative in the safety direction (false-positive-obstruction OK; false-negative not).
- **topology-aware decay** — cache eviction driven by simplicial distance $d(\sigma,\tau)$ in $\tilde{X}$, not wall-clock time; eviction policy itself consumer-supplied (§4.4, L207–209).

### Supporting protocols (§5)
- **M2M gossip / Bloom-and-Merkle** — two-phase: Bloom phase ("which simplices do you have local data for?"), Merkle phase ("do our restrictions agree on the overlap?"). "The wire format carries _section deltas_ restricted to the simplicial overlap, rather than flat key-value diffs" (§5.1, L215).
- **timestamps** — each section update carries wall-clock (Lamport/vector-clock interpretation) and plant proper-time (local, not globally ordered) (§5.1, L217).
- **Flow** (referenced here functionally, named formally §7.4) — the anticipated translator that reconciles plant-local proper-time frames; "deliberately not assumed by the substrate" until it exists; until then concurrent observations resolve via sheaf merge alone (§5.1, L217).
- **topology-aware merge as consensus** — "consensus and context become the same problem under the sheaf framing"; merge = restriction-map agreement on overlaps; equivariance ensures $g^*\circ\rho_{g\sigma\to ge} = \rho_{\sigma\to e}\circ g^*$ (§5.2, L221–225). "This subsumes conventional CRDT merge logic."
- **diffusion-based load balancing** — cost function $\mathrm{cost}(\sigma\to\tau)=d(\sigma,\tau)+\mathrm{liftProjectPenalty}(\sigma,\tau)+\mathrm{congestion}(\tau)$, constrained by equivariance type; dynamics $\dot x = -Lx$ on sheaf Laplacian; equilibria = **harmonic sections** ($\ker L\cong H^0$) (§5.3, L231–233). Self-resolving, backpressure intrinsic, failure modes dynamical (no explicit failover machinery).
- **equivariance type** — metadata tag declaring the subgroup $H\le G\times\mathbb{Z}$ a workload is invariant under; four cases: $G\times\mathbb{Z}$ (fully invariant, spills anywhere), $G\times 1$ (PLR-invariant, $\mathbb{Z}$-variant — horizontal-only spill), $1\times\mathbb{Z}$ (PLR-variant, $\mathbb{Z}$-invariant — vertical-preferred spill), trivial subgroup (must run on exact coordinate, else queue) (§5.4, L239–249).
- **Cluster / Multichain boundary** — intra-cluster = single frame/trivialization; inter-cluster composition handled by **Disarray** (§5.5, L253–255).
- **Disarray** — "the hybrid multichain that hosts the substrate's inter-cluster consensus and Byzantine-fault concerns... The composed fabric is itself an instance of eryon-style virtualization at a higher scale: its nodes are not individual devices but entire clusters and subnets" (§5.5, L255).
- **Proton** — access layer; holds composite NFT tokens bound to ENS-style domain names; "Each subscriber holds a token containing wallet keys, software access credentials, a namespace registry, and a node registry" (§5.5, L257). Node registry also gates device onboarding/partitioning authorization.
- **composite-hash authentication primitive** — three components: (1) hash of cluster's current *shape* (simplicial structure/coverage/layer-grading), (2) hash of cluster *data*, (3) conventional token-key + user-key. "An attacker cannot forge authentication without simultaneously possessing the user's token, the user's provided key, and a faithful replica of the cluster's geometric and data state at the moment of authentication" (§5.5, L261). Self-rotating because cluster shape continuously changes.
- **homomorphic encryption (per-layer, optional)** — "designated layers" could carry PLR-respecting encrypted stalks so vnodes perform surface-motion without decrypting; flagged as a possibility, not a commitment (§5.5, L265).
- **frequency-domain / FFT wire encoding** — the substrate's wire format commitment: "because the substrate is already harmonic in its configuration space, the wire encoding should be harmonic in its representation... frequency-domain via the Fast Fourier Transform" (§5.6, L269). Uses Lewin's DFT-on-$\mathbb{Z}_{12}$ transform (Amiot 2016): transposition acts by modulation; orientation bit $\varepsilon$ lives in phase; alteration bit $a$ lives in magnitude support pattern. PLR-under-frequency-domain is flagged as an *open architectural intent*, not a constructed result (§5.6, L277). Four alignment properties: exact layer (DFT coordinates), PLR-compatibility research question, embarrassingly-parallel decomposition (abundance regime), continuous-frequency superposition representation for harmonic smears (scarcity regime).
- **sonification** — the informal name for the FFT-encoding idea; the paper argues it "understates the structural depth" because the substrate's data is *already* harmonic, unlike arbitrary-data sonification (§5.6, L283).

### Two-mode architecture (§6)
- **symmetric phase** (formerly called "chaos" in an earlier draft, renamed to avoid clashing with the Chaos storage service) — pre-originator mode: "the substrate carries the full $G\times\mathbb{Z}$ symmetry. No frame is privileged... Sections of the sheaves are defined only up to the group action — as orbits, not as specific values" (§6.1, L291). Substrate is "actively maintaining itself... preparing" — not idle.
- **gauge fixing** — "the choice of frame that converts symmetry-equivalent structure into named structure... the originator's engagement selects a trivialization: an octave origin for the $\mathbb{Z}$-grading and a reference configuration for $G$" (§6.2, L297). Mediated by Proton. Identified with a system-wide "halting" in a Kůrka-adjacent but explicitly *not identical* sense (open question noted). Analogized to symmetry-breaking below the Curie temperature.

### Discussion-section concepts (§7)
- **autopoiesis** (Maturana & Varela 1980) — substrate claimed self-bounded (topological constraint per plant), self-maintaining (gossip+cohomology), self-producing (persistence-aware sheaf machinery on node join); the fourth property, goal-directed dynamics, is "reachable by tuning policies on the existing primitives," not present natively (§7.2, L317–319).
- **active inference** (Friston 2010) — lift/project morphisms "can be repurposed as prediction operators in the active-inference sense" (§7.2, L319).
- **proto-cognitive distributed fabric** — the paper's headline claim: "the substrate as a whole _decides_ through diffusion-mediated routing, gossip-mediated consensus, and gradient-descent optimization... not reducible to the plants running on it" (§7.2, L321).
- **self-modification (reframed)** — "What the substrate self-modifies... is not a plant. It is _configuration space itself_" — vnode-plant pairs move position, not definition (§7.3, L327–329).
- **Zeno machines / hypercomputation conjecture** — open conjecture (explicitly not a result): under idealized continuous surface-motion semantics the plant-vnode pair "may... approach behaviors associated with hyper-computational models such as Zeno machines" (§7.3, L333). Two express cautions: finite-precision realizations remain ordinary computable processes; structural analogy ≠ evidence of hypercomputational power.

### Companion services (§7.4 — named but not detailed here)
- **ACME** — see Section 3 below.
- **Flow** — ACME's "temporal counterpart... an anticipated but not-yet-committed translator that would reconcile per-plant proper-time differences before the inverse-transform reassembly of distributed workloads can apply" (§7.4, L345).
- **Chaos** — "a deliberately-thin storage service that persists $\mathcal{F}$-stalks as content-addressed durable state with the wire encoding of Section 5.6 carried through to the storage layer" (§7.4, L345). Also called, in §7.2, "the substrate's content-addressed transactional persistence service" providing "memory of past configurations" (§7.2, L319).
- **Aether** — "the compute service through which workloads are submitted to and executed over the substrate" (§7.4, L345).
- **Disarray** — (see above) hybrid multichain, inter-cluster consensus/Byzantine-fault machinery.
- **Proton** — (see above) user-facing access layer, originator handshake, composite NFT/ENS identity, composite-hash authentication.

---

## 3. ACME REFERENCES (verbatim, all occurrences — 5 direct ACME mentions + 2 directly-adjacent Flow mentions that define ACME's temporal counterpart)

**Reference 1 — Abstract (L16):**
> "Higher-order services that the design naturally pulls into existence — the automated context management engine ([[Automatic Context Management Engine|acme]]), the Chaos storage service, the Disarray hybrid-consensus multi-chain — are detailed in companion documents."

(Note: the wikilink target is `[[Automatic Context Management Engine|acme]]` — confirming ACME's full name and that a separate note/document titled "Automatic Context Management Engine" is the intended companion doc, distinct from this substrate paper. NB: the doc's frontmatter alias also lists "eryonSubstrate" etc. but does not itself carry an acme alias — the acme content lives in a separate vault note.)

**Reference 2 — §1 Introduction (L24):**
> "The substrate is presented here as the foundational base layer. Higher-order services that the design naturally pulls into existence — the automated context management engine (ACME), the Chaos storage service, the Disarray hybrid-consensus multichain — are built on top of this base and are documented separately. References to these companion services appear only in Section 7.4's discussion of how the substrate is to be consumed in practice."

**Reference 3 — §4 opening (The Substrate Interface) (L169):**
> "The substrate exposes itself to higher-order services through a structured interface consisting of five sheaves over the octave-graded Tonnetz $\tilde{X}$ and five primitive operations that act on them. This section specifies that interface; it is the contract any service built on top of the substrate is expected to use, and is service-agnostic. Specific services that consume this interface — the automated context management engine (ACME), the Chaos storage service, the Disarray hybrid-consensus multichain — are described in companion documents."

**Reference 4 — §4.1 Five Sheaves, closing scoping note (L191):**
> "The uniformity claim is operational, and it is exact where it matters: all five admit the same primitive operations, share the same wire format, and are reconciled by the same gossip protocol. Consensus, context exchange, performance monitoring, and resource allocation are not five separate subsystems but five instances of the same machinery applied to different sheaves. Consumers of the substrate (notably ACME, which maintains $\mathcal{F}$ on behalf of agentic workflows) supply the policies that determine _what_ gets written, retained, or evicted; the substrate guarantees the underlying sheaf-theoretic well-formedness."

**This is the single most load-bearing sentence for ACME's actual functional role**: ACME is the consumer service that *maintains the context sheaf $\mathcal{F}$*, specifically "on behalf of agentic workflows," and supplies write/retention/eviction *policy* — the substrate itself only guarantees sheaf-theoretic well-formedness (the math), not the policy layer.

**Reference 5 — §5.1 M2M Gossip (Flow, ACME's temporal counterpart, functional mention) (L217):**
> "Where they are concurrent in wall-clock time, proper-time comparison would require translating between plant-local time frames — a translation that is the contract of the anticipated **Flow** service (Section 7.4) and is deliberately not assumed by the substrate. Until such a translator is present, concurrent observations are resolved by the sheaf merge of Section 5.2 alone: agreement under restriction merges, disagreement surfaces for reconciliation rather than being silently ordered."

**Reference 6 — §7.4 Engineering Tradeoffs and Consumer Services, the full ACME+Flow definition (L345):**
> "The substrate is the foundational base layer. Higher-order services that the substrate's structure naturally pulls into existence are built on top of the five-sheaf, five-primitive interface and are documented separately. The most immediate of these is the **automated context management engine** (ACME): once the substrate exists, an automated context layer is nearly necessitated by the design, since the context sheaf $\mathcal{F}$ exists at the substrate level but its retention, decay, and aggregation policies do not — those policies are what ACME supplies. Alongside it sits **Flow**, ACME's temporal counterpart: an anticipated but not-yet-committed translator that would reconcile per-plant proper-time differences before the inverse-transform reassembly of distributed workloads can apply (Section 5.1), addressing the temporal asymmetries inherent to the substrate's plant-local proper-time semantics."

**Reference 7 — §7.2 Distributed Cognition and Autopoiesis (memory framing, adjacent to but not naming ACME directly — included because it's the "memory" half of the context-management story) (L319):**
> "The remaining property — goal-directed dynamics with predictive models — is reachable by tuning policies on the existing primitives. The performance sheaf supports preference among consistent configurations; the lift and project morphisms can be repurposed as prediction operators in the active-inference sense (Friston 2010). Memory of past configurations is provided by **Chaos** (Section 7.4), the substrate's content-addressed transactional persistence service."

### ACME's role, synthesized from the above
ACME is **not defined in this document** — this document only establishes the substrate that ACME sits on top of, and repeatedly defers ACME's actual mechanics to "companion documents." What this document *does* establish about ACME's contract with the substrate:
1. ACME is the **most immediate** higher-order consumer service the substrate's design "necessitates" (§7.4) — stronger language than used for any other named service.
2. ACME's job is specifically to supply **policy** for the **context sheaf $\mathcal{F}$**: retention, decay, aggregation ("those policies are what ACME supplies," §7.4) and write/retain/evict decisions ("§4.1: notably ACME, which maintains $\mathcal{F}$ on behalf of agentic workflows").
3. ACME operates "**on behalf of agentic workflows**" — the only place in the document that names the consumer-facing use case (agent workflows) explicitly.
4. ACME has a **named temporal counterpart, Flow**, which is explicitly "anticipated but not-yet-committed" — i.e., unbuilt/unspecified even at the level this paper operates. Flow's job: reconcile plant-local proper-time frames so that concurrent (non-causally-ordered) section updates from different plants can be compared/merged, which matters for inverse-FFT reassembly of distributed workloads (tying it to §5.1 gossip and §5.6 wire encoding).
5. Until Flow exists, ACME (or any consumer) is stuck resolving concurrent updates via sheaf-merge-only (agreement-or-surface-for-reconciliation), with no proper-time tiebreak available.
6. The substrate's stated division of labor: **substrate guarantees sheaf-theoretic well-formedness (the math); ACME/consumer services supply the policy.** This is the single clearest architectural contract statement for what ACME will need to implement.
7. Storage/memory is explicitly *not* ACME's job — that's **Chaos** ("memory of past configurations is provided by Chaos," §7.2), a "deliberately-thin," content-addressed, FFT-wire-encoding-carrying persistence service (§7.4). ACME and Chaos are named as siblings, not the same service — ACME governs the context sheaf's *policy*; Chaos is the *storage substrate* underneath the $\mathcal{F}$-stalks.

---

## 4. PERSISTENCE + MEMORY MODEL

- **No persistence model is specified in this document.** The substrate paper is explicit that persistence is out of scope for the base layer and belongs to **Chaos**, a companion service: "a deliberately-thin storage service that persists $\mathcal{F}$-stalks as content-addressed durable state with the wire encoding of Section 5.6 carried through to the storage layer, refusing the conventional separation between _what is computed_ and _how it is stored_" (§7.4, L345).
- **What state does exist at the substrate level and where it lives:**
  - **Plant state does not exist** — "Plant _state_ does not exist as such — what one might naively call 'plant state' is in fact vnode state, maintained per executable instance independent of corresponding instances elsewhere" (§3.1, L131). This is a hard architectural claim: state is bound to the *ephemeral* vnode, not the *permanent* plant (which is a stateless type-boundary/spec).
  - **State sheaf $\mathcal{S}$**: "records the runtime state of each vnode. Layer-indexed; each vnode has its own state independent of corresponding vnodes in other layers" (§4.1, L183). This is the closest thing to a per-instance memory primitive at the substrate level, and it is explicitly scoped as *not* a proper cellular sheaf over $\tilde{X}$ in the cohomological sense (§4.1, L191) — it's indexed over vnodes/devices (physical adjacency), not simplices.
  - **Context sheaf $\mathcal{F}$**: "carries the data and coordination information that flows through the substrate" (§4.1, L185) — this is the sheaf that carries what would colloquially be called "context" for agentic workflows, and it is the sheaf ACME is chartered to manage policy for. Its stalks are typed by the plant at each surface. The substrate guarantees this sheaf's mathematical well-formedness (sections, cohomology, gluing) but supplies *no* retention/decay/eviction policy itself — that's ACME's contract (§4.1 L191, §7.4 L345).
  - **Topology-aware decay** (§4.4) is the substrate's one built-in "memory fading" mechanism, but it governs *cache eviction rate as a function of simplicial distance*, not persistence — "Cache eviction... becomes here a question of which simplices' stalks to keep resident, optimized against the projected growth of the disagreement residual if those stalks go cold. The specific eviction policy is consumer-supplied" (§4.4, L209).
- **Ephemerality is architecturally central and asymmetric**: "Plants are stable across cluster changes because they are properties of the topology; vnodes are ephemeral because they come and go with hardware" (§3.1, L131). This directly maps onto the user's stated interest: ACME will need to manage "ephemeral entities" — the vnode is the substrate's own ephemeral entity, its state (the $\mathcal{S}$ sheaf) explicitly has no cross-instance continuity guarantee, and any durable "personality" or continuity of an agent/entity across vnode churn is out-of-band, presumably ACME/Chaos's job.
- **Timestamps and time-frames (relevant to context flowing between entities):** each section (context) update carries a wall-clock timestamp (Lamport/vector-clock-interpretable, giving a causal partial order across nodes) and a *plant proper-time* timestamp (local to the plant, not globally ordered) (§5.1, L217). Merge/consensus prefers the causally-later update when a causal relationship exists; when concurrent, plant-proper-time comparison would require Flow's not-yet-built translator, and until Flow exists, concurrent context updates are resolved purely by sheaf-merge agreement-or-surface-for-reconciliation (§5.1, L217; §7.4, L345). **This is the key open gap for ACME**: there is currently no way to compare "when" two concurrent pieces of context/state happened across different plant-local clocks.
- **Homomorphic encryption option** (§5.5, L265) is presented as a way certain *layers* could carry encrypted stalks that vnodes can still transform without decrypting — relevant if ACME/Chaos need privacy-preserving context storage on some layers, but flagged explicitly as "a possibility the substrate's geometric structure makes available, not as a commitment."
- **Identity/continuity across boundaries** is handled by Proton's composite NFT tokens + composite-hash authentication (shape hash + data hash + key pair) (§5.5, L257–263) — this is the closest thing to a "personality"/identity persistence primitive in the doc, but it's scoped to *user/cluster* identity, not to individual agentic-entity context, and it belongs to Proton, not ACME.
- **Vertical (octave) replication** (§3.3, L147) provides one substrate-native form of redundancy/continuity: scale-invariant workloads can be freely replicated to adjacent layers "as the result at any layer is equivalent," and the rolling-depth-2 scaffolding rule (§3.3, L151) guarantees at least two octave-distinct copies of any triad under active optimization — a structural (not policy) redundancy floor that exists independent of ACME.

---

## 5. ARCHITECTURE

**Layering (bottom to top):**
1. **Mathematical foundations** (§2): Wolfram (2,3) UTM + Neo-Riemannian group theory + octave grading — pure math, no distributed-systems content yet.
2. **The Substrate proper** (§3): four entity classes (Plant, VNode, Surface, Device) related by three projection maps; two execution regimes (abundance/scarcity) selected by resource availability; vertical (octave) distribution; self-configuring bootstrap via resource census + gossip-informed scaffolding.
3. **The Substrate Interface** (§4): five sheaves ($\mathcal{A}, \mathcal{S}, \mathcal{F}, \mathcal{P}$, allocation) + five primitives (attach, restrict, glue, transform, aggregate) — this is explicitly named as "the contract any service built on top of the substrate is expected to use, and is service-agnostic" (§4, L169). This is the API boundary ACME will integrate against.
4. **Supporting protocols** (§5): gossip (Bloom+Merkle), merge-as-consensus, diffusion load balancing, equivariance typing, cluster/multichain boundary (Disarray/Proton), wire encoding (FFT).
5. **Two-mode architecture** (§6): symmetric phase (pre-originator, autonomous) vs. gauge-fixed phase (post-originator, deterministic) — a runtime-mode axis orthogonal to the layering above.
6. **Higher-order consumer services** (§7.4, out of scope of this doc): ACME, Flow, Chaos, Aether, Disarray, Proton — each "built on top of the five-sheaf, five-primitive interface" (§7.4, L345).

**Topology / addressing:** The addressing scheme *is* the group-torsor structure itself — there is no separate addressing layer. A surface's address is its torsor coordinate $(c, n) \in \mathcal{C}\times\mathbb{Z}$; routing in the abundance regime literally is "transformations are coordinates" (§3.2, L137).

**Protocols named:**
- Gossip: two-phase Bloom-and-Merkle (§5.1).
- Merge/consensus: restriction-map agreement, no separate consensus protocol needed intra-cluster (§5.2).
- Load balancing: diffusion under sheaf Laplacian, self-resolving, no explicit failover (§5.3).
- Wire format: FFT-based frequency-domain encoding for all workloads/transformations/stalk values (§5.6) — this is a substrate-wide commitment, not per-service.
- Inter-cluster: Disarray hybrid multichain, treating whole clusters as nodes at a higher self-similar scale (§5.5).
- Identity/auth: Proton composite-NFT + ENS + three-factor composite-hash (shape+data+keys) (§5.5).

**WASM / component-model plans:** **Not mentioned anywhere in this document.** No occurrence of "wasm," "WebAssembly," "component model," or "WIT" in the text. (This connects to Axiom-project memory about a "unified runtime WASM component vision," but that is not part of this Eryon substrate doc — it appears to be a separate/unrelated axiom-project concern, not something this file addresses.)

**How pieces compose:** The paper is careful to keep composition *loosely coupled but operationally required* — e.g., "Proton and the substrate... are designed independently and remain separable in principle... yet operationally the combination is what constitutes a complete eryon node" (§6.2, L299). The same pattern is implied for ACME/Flow/Chaos/Aether/Disarray: each is a distinct, separately-documented service consuming the same five-sheaf/five-primitive interface, self-similar in how they compose (§7.4, L345, closing sentence: "context that is position-typed, storage that is topologically-addressed, consensus that is sheaf-cohomological, identity that is gauge-theoretic, authentication that is geometrically-fingerprinted, reassembly that is harmonically lossless — by virtue of being built over the substrate rather than over flat data").

**Scale-invariance / recursion:** Self-similarity is a stated architectural principle — the same scaffolding algorithm builds every layer (§3.4, L161); Disarray's inter-cluster fabric is "itself an instance of eryon-style virtualization at a higher scale" (§5.5, L255); this is explicitly tied back to the $\mathbb{Z}$-equivariance / renormalization-group analogy of §2.6.

---

## 6. DECISIONS + CONSTRAINTS

**Explicit design decisions:**
- Plant is **immutable/stateless by design**; all mutation is pushed into the vnode. Justified purely on inheritance-of-mathematical-properties grounds (§2.4, L85–87; reiterated §7.3, L325). This is the single most emphasized invariant in the document — the substrate "refuses" the "naive reading" of self-modifying plants.
- Configuration space extension must be done on **rooted triads**, not pitch-class sets — an explicit correction of an earlier draft's error (the augmented-triad 3-to-1 collapse breaking simple transitivity / the false "$D_{48}$" claim) (§2.3, L46).
- Octave grading is a **product** $\tilde X = X\times\mathbb{Z}$, deliberately *not* a connected universal cover — explicit correction of an earlier draft that called it that; rejected because a connected cover would entangle layers and make a surface's layer path-dependent (§2.6, L115).
- **Two-clock semantics with unit surface costs** is the adopted operational meaning of "repair cost" among three considered alternatives (free retune, unit-cost retune, two-clock); free retune is explicitly named only as an idealization tied to the open hypercomputation conjecture (§2.5, L101–103).
- **Smears stay node-local** — never cross the wire in continuous form; must resolve to discrete state before transport, "keeping the inter-node consensus problem in the linear-algebraic regime rather than the transport-theoretic regime" (§3.2, L141).
- **Estimator must be conservative in the safety direction**: false positive (reporting possible obstruction when none exists) is acceptable; false negative (reporting consistency when a fault exists) is not (§4.3, L205).
- **No minimum deployment threshold** — "a single node hosting a single triad is a valid instance of the substrate, with all of its mathematical properties intact" (§3.4, L163; reiterated §7.4, L341) — explicit design constraint that the substrate must degrade gracefully to n=1.
- **Homomorphic encryption is per-layer opt-in, not substrate-wide**, explicitly rejected as a uniform commitment "which would force every operation through the homomorphic machinery whether or not the privacy guarantee was needed" (§5.5, L265).
- **FFT/frequency-domain wire encoding is a substrate-wide commitment**, explicitly rejected alternatives being protobuf/MessagePack/JSON (arbitrary, orthogonal to substrate math) and compressed/lossy codecs (survive transport but not arbitrary recomposition) (§5.6, L269–271).
- **"Symmetric phase" naming** deliberately replaces an earlier draft's use of "chaos" specifically to avoid collision with the planned Chaos storage service name (§6.1, L291) — a naming-hygiene decision explicitly recorded.
- Substrate scope is hard-bounded: "References to these companion services appear only in Section 7.4's discussion of how the substrate is to be consumed in practice" (§1, L24) — i.e., this document deliberately does not spec ACME/Flow/Chaos/Aether/Disarray/Proton; that's an explicit scoping constraint on the document itself.

**Rejected alternatives (explicitly named):**
- $D_{48}$ dihedral-of-order-48 acting on pitch-class sets — rejected/retracted (§2.3, L46).
- Universal-cover framing of octave grading — rejected/retracted (§2.6, L115).
- Uniform substrate-wide homomorphic encryption — rejected in favor of per-layer opt-in (§5.5, L265).
- Flat/byte-blob or lossy-codec wire encodings — rejected in favor of FFT (§5.6, L271).
- Central scheduling / explicit failover machinery / tunable replication-factor-and-quorum parameters (contrasted against conventional systems throughout, most explicitly at §3.3 L151 and §5.3 L235) — rejected in favor of emergent/geometric mechanisms.

**Invariants (permanent, load-bearing):**
- $G\cong D_{12}\times C_2$, order 48, acts simply transitively on the 48 rooted triads (§2.3).
- Repair-length bounds (worst-case 3, mean 1.58) and Cayley diameter 6 — "permanent constants; no workload, cluster size, or future design decision can inflate them" (§2.5, L97).
- Presheaf pullback contravariance $g^*h^*=(hg)^*$ holds for the entire $G$-action (§2.3 L73, reused throughout §4–5).
- Naturality square $g^*\circ\rho_{g\sigma\to ge}=\rho_{\sigma\to e}\circ g^*$ "commutes by construction and requires no separate enforcement" (§5.2, L223–225).

---

## 7. OPEN QUESTIONS

Explicitly flagged by the document as unresolved, conjectural, or deferred:

1. **Hypercomputation conjecture (Zeno machines)** — "We offer the following as an open conjecture rather than a result... such a pair may, in the limit of continuous surface-motion semantics, approach behaviors associated with hyper-computational models such as Zeno machines... a research question we do not attempt to settle" (§7.3, L333). Explicitly the paper's most prominent unresolved item.
2. **Expressiveness of plant-vnode pair vs. plain (2,3) machine** — "whether it is _strictly_ more expressive, in any precise sense, is open. We do not claim a result here" (§7.3, L331).
3. **Kůrka-fixed-point correspondence for gauge fixing** — "whether the two can be related by a precise correspondence, in which the substrate's fixed point is genuinely a Kůrka fixed point of some derived system, is an open question deserving formal treatment" (§6.2, L301).
4. **PLR-compatible frequency-domain encoding** — "we scope it precisely rather than assert it... We flag this as architectural intent rather than a constructed result: the encoding under which the full $G$-action factors cleanly through frequency-band operations remains to be characterized, and implementation work will need to construct it explicitly" (§5.6, L277).
5. **Estimator construction for cohomological consistency checks** — "Specific estimator constructions and their error properties are an active engineering concern rather than a settled choice" (§4.3, L205).
6. **Flow (temporal reconciliation) is unbuilt** — "an anticipated but not-yet-committed translator" (§7.4, L345); until it exists, concurrent (non-causally-related) context updates cannot be resolved by proper-time comparison at all (§5.1, L217). This is the most directly ACME-relevant open gap.
7. **Efficient sheaf cohomology at scale** — "computing obstructions even for small complexes can be expensive, and incremental cohomology computation on a growing filtration is an active research area in computational topology" (§7.4, L337).
8. **$O(1)$ lift/project estimation** — "must be cheap to _estimate_... even when the actual lift operation is expensive. This requires careful API design at the plant level" (§7.4, L337) — flagged as unresolved engineering work, not yet designed.
9. **$n$-note generalization (beyond triads/3-simplices)** — "Implementation of this generalization is deferred but architecturally non-blocking" (§7.4, L343) — a stated future-extension axis, not yet built.
10. **Homomorphic-encryption scheme details** — mentioned only as a possibility; "mentioned here as a possibility the substrate's geometric structure makes available, not as a commitment the present design relies on" (§5.5, L265) — no concrete scheme chosen.
11. **ACME/Flow/Chaos/Aether/Disarray/Proton internal designs** — none are specified in this document at all; all are pointed at "companion documents" not included here. For the purposes of scaffolding ACME, this is the largest practical open item: this substrate document defines ACME's *contract surface* (the five-sheaf/five-primitive interface, ownership of $\mathcal{F}$ policy) but supplies zero implementation detail for ACME itself.
12. **No explicit versioning/compatibility strategy** is discussed for how ACME (or any consumer service) evolves against a substrate whose five-sheaf/five-primitive interface might itself change — not raised as a question in the text, but notably absent given how much weight is placed on "the contract any service built on top of the substrate is expected to use" (§4, L169).

---

**File-level note for the requesting agent:** This document (v1, createdAt 2026-05-07) is the *substrate* spec only. It names ACME as the "most immediate" consumer service and gives ACME a precise but narrow contract (own the context sheaf $\mathcal{F}$'s retention/decay/aggregation/eviction policy, on behalf of agentic workflows, guaranteed only sheaf-well-formedness by the substrate) — but the actual ACME design lives in a separate vault note titled **"Automatic Context Management Engine"** (wikilinked at L16 as `[[Automatic Context Management Engine|acme]]`), which was not part of this read and should be fetched next if the full ACME spec is needed.