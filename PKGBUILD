# Maintainer: Akeoot <akeoot@pm.me>
pkgname=larpr
pkgver=0.0.0
pkgrel=1
pkgdesc="There's no limit to the larp. Larp more than any larper before."
arch=('x86_64')
url="https://github.com/Akeoott/larpr"
license=('LGPL-3.0')
depends=()
makedepends=('cargo' 'rust' 'git')
source=("larpr::git+https://github.com/Akeoott/larpr.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/larpr"
  git describe --tags --abbrev=0 2>/dev/null || git rev-parse --short HEAD
}

build() {
  cd "$srcdir/larpr"
  cargo build --release
}

package() {
  cd "$srcdir/larpr"
  install -Dm755 target/release/larpr "$pkgdir/usr/bin/larpr"
}
