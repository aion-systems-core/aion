# Hacker News „Show HN“ (Plan §11.3)

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

**Titel:**

```
Show HN: AION – Deterministic Execution OS & Evidence Fabric for AI Agents
```

**Erster Kommentar (sofort nach Posting):**

> Hi HN,
> 
> I built AION because I was frustrated by the lack of auditability in AI‑powered workflows. When an LLM agent runs a command, how do you prove later *exactly* what the environment looked like? Logs are easy to fake. Containers are great for isolation, but they don't provide a **cryptographic proof of execution**.
> 
> AION is a Rust‑based CLI tool that acts as a **deterministic wrapper** around subprocesses. It freezes time, environment variables (whitelist), CWD, and even the RNG seed. Every run produces a **Capsule** – a zip file containing the output, a manifest of hashes, and a linear **evidence chain** linking this run to previous ones.
> 
> **What makes it different:**
> - **No sandboxing.** AION is an *Execution OS*, not a security sandbox. It doesn't require root, kernel modules, or privilege escalation. It just observes and records deterministically.
> - **Contract‑OS.** The public interface (JSON schemas, error namespaces) is versioned with a spec hash. Breaking changes are explicit.
> - **Replay symmetry.** You can re‑run a capsule on a different machine and AION will tell you *exactly* what drifted (down to environment variable ordering).
> 
> **Current state:** Core, Kernel, Engine, and CLI are implemented and stable. FS/network isolation are **intentionally stubs** – they define the contract surface but don't enforce. If you need strict isolation (e.g., for regulated finance), that's available as an optional enterprise extension – but the open‑source core is fully functional for deterministic logging and replay.
> 
> **Repo:** [GitHub Link]
> **Docs:** [Link to os_contract_spec.md]
> 
> Happy to answer any questions about the architecture, determinism guarantees, or the business model. I'm a student who built this over the last months – feedback is hugely appreciated!
