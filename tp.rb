class TP < Formula
  desc "Teleport"
  homepage "https://github.com/eli-front/tp"
  url "https://github.com/eli-front/tp/releases/tag/v0.1.0"
  sha256 "6a1be5ba87289e1b7367abec75175fd4a7e6b7083e1267fdb36b59eb228eb0ae"

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/tp", "--version"
  end
end

