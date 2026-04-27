from dataclasses import dataclass, field

@dataclass
class OllamaConfig:
    base_url: str = "http://localhost:11434"
    model: str = "qwen2.5:7b"

@dataclass
class SandboxConfig:
    sealrun_bin: str = r"C:\Users\noahp\Documents\aion-os-v2\target\release\sealrun.exe"

@dataclass
class AIConfig:
    ollama: OllamaConfig = field(default_factory=OllamaConfig)
    sandbox: SandboxConfig = field(default_factory=SandboxConfig)
