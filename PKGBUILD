# Maintainer: Akeoot <akeoot@pm.me>
pkgname=larpr
pkgver=0.0.0
pkgrel=1
pkgdesc="There's no limit to the larp. Larp more than any larper before."
arch=('x86_64')
url="https://github.com/Akeoott/larpr"
license=('LGPL-3.0-or-later')
depends=()
makedepends=('cargo' 'git')

source=("larpr::git+https://github.com/Akeoott/larpr.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/$pkgname"
  git describe --long --tags | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
  cd "$srcdir/$pkgname"
  cargo build --release
}

package() {
  cd "$srcdir/$pkgname"
  install -Dm755 target/release/larpr "$pkgdir/usr/bin/larpr"
}
