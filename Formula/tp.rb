class Tp < Formula
  desc "Teleport"
  homepage "https://github.com/eli-front/tp"
  url "https://github.com/eli-front/tp/releases/tag/v0.1.0"
  sha256 "35fa614d79e77f7b6a9fdf0e753ccf6cbb866c653ea8350eb1d8241140881a5f"

  depends_on "rust" => :build

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/tp"
  end

  test do
    system "#{bin}/tp", "--version"
  end
end

