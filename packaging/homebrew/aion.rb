class Aion < Formula
  desc "AION-OS deterministic AI execution CLI"
  homepage "https://example.com/aion-os"
  url "https://example.com/aion-os/releases/download/v0.0.0/aion-darwin-amd64.tar.gz"
  sha256 "REPLACE_WITH_RELEASE_SHA256"
  license "MIT"

  def install
    bin.install "aion"
  end
end
