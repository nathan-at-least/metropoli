# metropoli

Metropoli is a "redecentralized" computation platform based on message passing. It provides a message passing capability to separate WASM runtimes, called `poli` (or singular `polis`). Each `polis` is restricted in interacting with the outside world completely to receiving and sending messages. In addition to pure-WASM `poli`, the host system can provide `ports` which can send and receive messages and also interact directly with the host system.

The term "redecentralized" is a philosophical design criteria to create a technology that resists centralized control or interference in usage, and, with Self-Hosting (see Ambitions below), this applies to the development of metropoli itself.

## Status

This project is an exploratory prototype. There's no guarantee it will be usable yet.

## Ambitions

The audicious goals for metropoli include:

- **Compartmentalization:** Applications can instantiate `poli` from untrusted sources with a guarantee they can only interact via a specific message passing interface.
- **Computational Determinism:** The same `poli` code will always react the same way to the same sequence of incoming messages provided enough computational resources on any host system.
- **Redecentralized Storage:** A distinct redecentralized storage network will enable `poli` to safely store and transmit data universally.
- **Self-Hosting:** The source code can be developed and built entirely within a metropoli application.

### Compartmentalization

Compartmentalization is the "easiest" ambition because it comes from the basic architecture of WASM execution with message passing. This requires that WASM itself provides adequate compartmentalization, which is a key design goal of WASM.

### Computational Determinism

Determinism is an ambitious goal, especially because this is _not_ a primary design goal of WASM. Achieving this ambition may require modifying WASM itself, such as by disabling floating point support (which is not deterministic across hardware platforms), disabling threading features (which would violate the same messages -> same behavior goal), or other changes.

### Redecentralized Storage

This ambition is outside the scope of the base metropoli architecture and will rely on `ports`. However, having a standard interface to this storage system would be a boon to metrpoli applications. This ambition uses some notable terminology:

- "redecentralized" refers to a design intended to resist centralization and gatekeeping, so that anyone may deploy any metropoli app of their choosing without interference by third parties.
- "safety" includes security, privacy, authentication, and availability of data storage.

### Self-Hosting

This ambition is a huge lift, probably the biggest, with several components:

Self-hosted source code development implies revision control and coordination that relies on metropoli which would use Redecentralized Storage for enabling coordination among developers.

Self-hosted builds require the build toolchain to be hosted on metropoli. The initial implementation is in rust, so one path would be to port the entire rust toolchain to metropoli. Another path would be to implement a metropoli-specialize rust->WASM compiler. A third option would be to re-implement metropoli in a new self-hosted language.

This massive effort would bring several benefits:

- By providing redecentralized software development, the fate of metropoli's evolution would be protected by the technology itself.
- If the Computational Determinism ambition is achieved, the distribution of metropoli would be safeguarded against malicious third parties and supply chain bugs due to nondeterminism. Additionally, all developers will have the same capability to produce production-quality runtimes, which enables new developers to take up the mantle of maintenance and also enables developers with divergent mutually incompatible roadmaps to freely pursue their alternative visions.
