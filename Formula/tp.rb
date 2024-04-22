class Tp < Formula
  desc "Teleport"
  homepage "https://github.com/eli-front/tp"

  url "https://github.com/eli-front/tp/archive/refs/tags/v0.1.1.tar.gz", :using => :curl

  sha256 "186cccc264aea6b338303dbb2c2b520c65e7820fef50db915b29bb67d8d69fe3"

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/tp"
  end

  test do
    system "#{bin}/tp", "--version"
  end
end

