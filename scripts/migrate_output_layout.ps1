param(
  [string]$Source = "aion_output",
  [string]$Target = "aion_output_migrated"
)

New-Item -ItemType Directory -Force -Path $Target | Out-Null
Copy-Item -Recurse -Force "$Source\*" $Target
Write-Host "Migrated output tree from '$Source' to '$Target'."
