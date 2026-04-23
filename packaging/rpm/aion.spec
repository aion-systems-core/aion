Name:           aion
Version:        0.0.0
Release:        1%{?dist}
Summary:        AION-OS deterministic AI execution CLI
License:        MIT

%description
AION-OS CLI for deterministic AI execution, replay, drift, and governance checks.

%install
mkdir -p %{buildroot}/usr/bin
install -m 0755 aion %{buildroot}/usr/bin/aion

%files
/usr/bin/aion
