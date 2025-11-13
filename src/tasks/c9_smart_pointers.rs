// This chapter is dedicated to the smart pointers: Box, Rc and RefCell.

use std::cell::RefCell;
use std::collections::{HashSet, hash_set};
use std::fmt::Binary;
use std::rc::Rc;

// Box
// ================================================================================================

// ----- 1 --------------------------------------
// Implement a recursive `BinaryTreeNode` which have:
// - fields:
//   - `value: i32`
//   - `left_child: Option<BinaryTreeNode>`
//   - `right_child: Option<BinaryTreeNode>`
// - methods:
//   - `new(value: i32)`, which creates a note with provided value and without any children
//   - `with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode)` which
//     creates a note using the provided values
//   - `sum(&self)` which computes the sum of all values in the tree
//
// Use `Box` if needed

// IMPLEMENT HERE:

pub struct BinaryTreeNode {
    value: i32,
    left_child: Option<Box<BinaryTreeNode>>,
    right_child: Option<Box<BinaryTreeNode>>,
}

impl BinaryTreeNode {
    pub fn new(value: i32) -> Self {
        Self { value, left_child: None, right_child: None }
    }

    pub fn with_children(value: i32, left_child: BinaryTreeNode, right_child: BinaryTreeNode) -> Self {
        Self {
            value,
            left_child: Some(Box::new(left_child)),
            right_child: Some(Box::new(right_child))
        }
    }

    pub fn sum(&self) -> i32 {
        let sum_child = |child: &Option<Box<BinaryTreeNode>>| match child {
            None => 0,
            Some(node) => node.sum(),
        };

        self.value + sum_child(&self.left_child) + sum_child(&self.right_child)
    }
}

// Rc
// ================================================================================================

// ----- 2 --------------------------------------
// Implement a package dependency tree where multiple packages can depend on the same shared
// library.
//
// Implement the `Package` struct with `name: String` and `dependencies: Vec<Package>` fields.
// Implement methods:
// - `new(name: &str) -> Self` which creates a new package with provided name and without any
//   dependencies.
// - `with_dependencies(name: &str, dependencies: Vec<Package>) -> Self` which creates a new package
//   with provided name and dependencies.
// - `list_dependencies(package: Package) -> Vec<String>` which return a vector of all dependencies
//   of this package (including all recursive dependencies).
//
// Write a test which will reuse the created Packages in several other Packages as dependencies.
// Use `Rc` in the `Package` struct where needed to avoid deep clone.

// IMPLEMENT HERE:

pub struct Package {
    name: String,
    dependencies: Vec<Rc<Package>>,
}

impl Package {
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), dependencies: Vec::new() }
    }

    pub fn with_dependencies(name: &str, dependencies: Vec<Rc<Self>>) -> Self {
        Self { name: String::from(name), dependencies: dependencies }
    }

    /// May return the same dependency multiple times.
    pub fn list_dependencies(&self) -> Vec<String> {
        self.dependencies.iter()
            .flat_map(|dep| dep.list_dependencies())
            .chain(self.dependencies.iter().map(|dep| dep.name.clone()))
            .collect()
    } 
}

#[test]
fn test_list_dependencies() {
    // IMPLEMENT HERE:

    // #### Find an existing package dependency tree.
    // Warning: this only works if there are no cycles in dependency
    //   graph for the package (it means dependency graph is a tree).
    // Another precaution is that package name should not contain double
    //   quotes (which it usually does not, otherwise paths and package
    //   archive names inside fs are becoming unusual).
    // ```bash
    // LISTED=
    // IN_PROGRESS=
    // KNOWN_CYCLIC_DEPS=$(cat - <<EOF
    // ncurses
    // glibc
    // lib32-glibc
    // lib32-keyutils
    // keyutils
    // krb5
    // lib32-krb5
    // libcap
    // lib32-libcap
    // EOF
    // )
    // list_deps() {
    //   local pkg="$1"
    // 
    //   if echo "$LISTED" | grep -xF "$pkg" >/dev/null; then
    //     return
    //   fi
    // 
    //   # echo "--- IN_PROGRESS check for pkg $pkg ---"
    //   # echo "$IN_PROGRESS"
    //   # echo "--- ----------- ---"
    // 
    //   if echo "$IN_PROGRESS" | grep -xF "$pkg" >/dev/null; then
    //     echo "Cycle found! pkg $pkg" >&2
    //     exit 1
    //   fi
    // 
    //   # echo "--- KNOWN_CYCLIC_DEPS check for pkg $pkg ---"
    //   # echo "$KNOWN_CYCLIC_DEPS"
    //   # echo "$KNOWN_CYCLIC_DEPS" | grep -xF "$pkg"
    //   # echo "--- ----------- ---"
    // 
    //   if echo "$KNOWN_CYCLIC_DEPS" | grep -xF "$pkg" >/dev/null; then
    //     printf "// $pkg produces a cyclic dependency on itself, it's alright, not listing its dependencies\n"
    //     local pkg_sanitized=$(echo "$pkg" | sed 's/[-.]/_/g')
    //     printf "let %s = &make_pkg_rc(\"%s\", &[]);\n" "$pkg_sanitized" "$pkg"
    //     LISTED="$LISTED"$'\n'"$pkg"
    //     return
    //   fi
    // 
    //   IN_PROGRESS="$IN_PROGRESS"$'\n'"$pkg"
    //   # echo "--- IN_PROGRESS after pkg $pkg added ---"
    //   # echo "$IN_PROGRESS"
    //   # echo "--- ----------- ---"
    // 
    //   local deps=$(LC_ALL=C pacman -Qi "$pkg" | grep -Po 'Depends On\s*:\s*\K.*' | sed 's/\s\s/ /g' | tr ' ' '\n' | sed 's/[><]\?\=.*$//')
    //   if [ "$deps" = 'None' ]; then
    //     local deps=""
    //   fi
    // 
    //   for dep in $deps; do
    //     # echo "$pkg -> $dep" >&2
    //     list_deps "$dep"
    //   done
    // 
    //   local dep_list=$(echo -n "$deps" | tr '\n' ',')
    //   local pkg_sanitized=$(echo "$pkg" | sed 's/[-.]/_/g')
    //   local dep_list_sanitized=$(echo "$dep_list" | sed 's/[-.]/_/g')
    //   printf "let %s = &make_pkg_rc(\"%s\", &[%s]);\n" "$pkg_sanitized" "$pkg" "$dep_list_sanitized"
    // 
    //   LISTED="$LISTED"$'\n'"$pkg"
    //   IN_PROGRESS=$(echo "$IN_PROGRESS" | grep -xvF "$pkg")
    // }
    // printf '// Some system packages inevitably depend on themselves.\n'
    // printf '// In this case pacman would chouse an arbitary order (as far as I understand).\n'
    // printf '// Let'\''s just skip their dependency lists.\n'
    // printf 'let make_pkg_rc = |pkg, deps: &[&Rc<Package>]| {\n  Rc::new(\n    Package::with_dependencies(\n      pkg,\n      deps.iter().cloned().cloned().collect()\n    )\n  )\n};\n'
    // PKG=rust
    // list_deps "$PKG"
    // 
    // pkg_sanitized=$(echo "$PKG" | sed 's/[-.]/_/g')
    // LISTED=$(echo -n "$LISTED" | grep -vFx '') # Delete leading empty line
    // LISTED=$(echo -n "$LISTED" | grep -vFx "$PKG") # Delete the package itself
    // deps=$(echo -n "$LISTED" | sed 's/^\(.*\)$/"\1"/' | tr '\n' ',')
    // printf 'assert!(HashSet::<_>::from_iter(%s.list_dependencies()) == [%s].iter().cloned().map(String::from).collect());\n' "$pkg_sanitized" "$deps"
    // ```
    // Some system packages inevitably depend on themselves.
    // In this case pacman would chouse an arbitary order (as far as I understand).
    // Let's just skip their dependency lists.
    let make_pkg_rc = |pkg, deps: &[&Rc<Package>]| {
    Rc::new(
        Package::with_dependencies(
        pkg,
        deps.iter().cloned().cloned().collect()
        )
    )
    };
    // glibc produces a cyclic dependency on itself, it's alright, not listing its dependencies
    let glibc = &make_pkg_rc("glibc", &[]);
    // lib32-glibc produces a cyclic dependency on itself, it's alright, not listing its dependencies
    let lib32_glibc = &make_pkg_rc("lib32-glibc", &[]);
    let lib32_gcc_libs = &make_pkg_rc("lib32-gcc-libs", &[lib32_glibc]);
    // ncurses produces a cyclic dependency on itself, it's alright, not listing its dependencies
    let ncurses = &make_pkg_rc("ncurses", &[]);
    let libncursesw_so = &make_pkg_rc("libncursesw.so", &[lib32_gcc_libs,lib32_glibc,ncurses]);
    let readline = &make_pkg_rc("readline", &[glibc,libncursesw_so,ncurses]);
    let libreadline_so = &make_pkg_rc("libreadline.so", &[glibc,libncursesw_so,ncurses]);
    let bash = &make_pkg_rc("bash", &[readline,libreadline_so,glibc,ncurses]);
    let acl = &make_pkg_rc("acl", &[glibc]);
    let attr = &make_pkg_rc("attr", &[glibc]);
    let gcc_libs = &make_pkg_rc("gcc-libs", &[glibc]);
    let gmp = &make_pkg_rc("gmp", &[gcc_libs,glibc]);
    // libcap produces a cyclic dependency on itself, it's alright, not listing its dependencies
    let libcap = &make_pkg_rc("libcap", &[]);
    let openssl = &make_pkg_rc("openssl", &[glibc]);
    let coreutils = &make_pkg_rc("coreutils", &[acl,attr,glibc,gmp,libcap,openssl]);
    let findutils = &make_pkg_rc("findutils", &[glibc]);
    let libtasn1 = &make_pkg_rc("libtasn1", &[glibc]);
    let libffi = &make_pkg_rc("libffi", &[glibc]);
    let libp11_kit = &make_pkg_rc("libp11-kit", &[glibc,libffi,libtasn1]);
    let lib32_libffi = &make_pkg_rc("lib32-libffi", &[lib32_glibc,libffi]);
    let lib32_libtasn1 = &make_pkg_rc("lib32-libtasn1", &[lib32_glibc,libtasn1]);
    let libp11_kit_so = &make_pkg_rc("libp11-kit.so", &[lib32_glibc,lib32_libffi,lib32_libtasn1,libp11_kit]);
    let p11_kit = &make_pkg_rc("p11-kit", &[coreutils,glibc,libtasn1,libp11_kit,libp11_kit_so]);
    let ca_certificates_utils = &make_pkg_rc("ca-certificates-utils", &[bash,coreutils,findutils,p11_kit]);
    let ca_certificates_mozilla = &make_pkg_rc("ca-certificates-mozilla", &[ca_certificates_utils]);
    let ca_certificates = &make_pkg_rc("ca-certificates", &[ca_certificates_mozilla]);
    let brotli = &make_pkg_rc("brotli", &[glibc]);
    let libbrotlidec_so = &make_pkg_rc("libbrotlidec.so", &[glibc]);
    // krb5 produces a cyclic dependency on itself, it's alright, not listing its dependencies
    let krb5 = &make_pkg_rc("krb5", &[]);
    let sh = &make_pkg_rc("sh", &[readline,libreadline_so,glibc,ncurses]);
    let zlib = &make_pkg_rc("zlib", &[glibc]);
    let sqlite = &make_pkg_rc("sqlite", &[readline,zlib,glibc]);
    let util_linux_libs = &make_pkg_rc("util-linux-libs", &[glibc,sqlite]);
    let e2fsprogs = &make_pkg_rc("e2fsprogs", &[sh,util_linux_libs]);
    // keyutils produces a cyclic dependency on itself, it's alright, not listing its dependencies
    let keyutils = &make_pkg_rc("keyutils", &[]);
    let gdbm = &make_pkg_rc("gdbm", &[glibc,sh,readline,libreadline_so]);
    let libgdbm_so = &make_pkg_rc("libgdbm.so", &[glibc,sh,readline,libreadline_so]);
    let libcrypto_so = &make_pkg_rc("libcrypto.so", &[lib32_glibc,openssl]);
    let libsasl = &make_pkg_rc("libsasl", &[glibc,gdbm,libgdbm_so,openssl,libcrypto_so]);
    let libldap = &make_pkg_rc("libldap", &[libsasl]);
    let libevent = &make_pkg_rc("libevent", &[openssl]);
    let libverto_module_base = &make_pkg_rc("libverto-module-base", &[glibc,libevent]);
    let lmdb = &make_pkg_rc("lmdb", &[glibc]);
    let libcom_err_so = &make_pkg_rc("libcom_err.so", &[sh,util_linux_libs]);
    let libkeyutils_so = &make_pkg_rc("libkeyutils.so", &[glibc,sh]);
    let libss_so = &make_pkg_rc("libss.so", &[sh,util_linux_libs]);
    let lib32_e2fsprogs = &make_pkg_rc("lib32-e2fsprogs", &[lib32_glibc,e2fsprogs]);
    // lib32-keyutils produces a cyclic dependency on itself, it's alright, not listing its dependencies
    let lib32_keyutils = &make_pkg_rc("lib32-keyutils", &[]);
    let lib32_openssl = &make_pkg_rc("lib32-openssl", &[lib32_glibc,openssl]);
    let libxcrypt = &make_pkg_rc("libxcrypt", &[glibc]);
    let lib32_libxcrypt = &make_pkg_rc("lib32-libxcrypt", &[lib32_glibc,libxcrypt]);
    let lib32_libldap = &make_pkg_rc("lib32-libldap", &[lib32_openssl,lib32_libxcrypt,libldap]);
    let libverto_so = &make_pkg_rc("libverto.so", &[lib32_e2fsprogs,lib32_gcc_libs,lib32_glibc,lib32_keyutils,lib32_libldap,lib32_openssl,krb5]);
    let libgssapi_krb5_so = &make_pkg_rc("libgssapi_krb5.so", &[bash,e2fsprogs,glibc,keyutils,libldap,libverto_module_base,openssl,lmdb,libcom_err_so,libkeyutils_so,libss_so,libverto_so]);
    let libunistring = &make_pkg_rc("libunistring", &[glibc]);
    let libunistring_so = &make_pkg_rc("libunistring.so", &[glibc]);
    let libidn2 = &make_pkg_rc("libidn2", &[libunistring,libunistring_so]);
    let lib32_libunistring = &make_pkg_rc("lib32-libunistring", &[libunistring,lib32_glibc]);
    let libidn2_so = &make_pkg_rc("libidn2.so", &[libidn2,lib32_glibc,lib32_libunistring]);
    let libnghttp2 = &make_pkg_rc("libnghttp2", &[glibc]);
    let libnghttp2_so = &make_pkg_rc("libnghttp2.so", &[lib32_glibc,libnghttp2]);
    let libnghttp3 = &make_pkg_rc("libnghttp3", &[glibc]);
    let libnghttp3_so = &make_pkg_rc("libnghttp3.so", &[lib32_glibc,libnghttp3]);
    let libpsl = &make_pkg_rc("libpsl", &[libidn2,libunistring,libidn2_so,libunistring_so]);
    let lib32_libidn2 = &make_pkg_rc("lib32-libidn2", &[libidn2,lib32_glibc,lib32_libunistring]);
    let libpsl_so = &make_pkg_rc("libpsl.so", &[libpsl,lib32_glibc,lib32_libidn2,lib32_libunistring]);
    let libssh2 = &make_pkg_rc("libssh2", &[openssl,zlib]);
    let lib32_zlib = &make_pkg_rc("lib32-zlib", &[lib32_glibc,zlib]);
    let libssh2_so = &make_pkg_rc("libssh2.so", &[lib32_openssl,lib32_zlib,libssh2]);
    let libz_so = &make_pkg_rc("libz.so", &[lib32_glibc,zlib]);
    let xz = &make_pkg_rc("xz", &[sh]);
    let lz4 = &make_pkg_rc("lz4", &[glibc]);
    let zstd = &make_pkg_rc("zstd", &[glibc,gcc_libs,zlib,xz,lz4]);
    let libzstd_so = &make_pkg_rc("libzstd.so", &[zstd,lib32_glibc]);
    let libssl_so = &make_pkg_rc("libssl.so", &[lib32_glibc,openssl]);
    let curl = &make_pkg_rc("curl", &[ca_certificates,brotli,libbrotlidec_so,krb5,libgssapi_krb5_so,libidn2,libidn2_so,libnghttp2,libnghttp2_so,libnghttp3,libnghttp3_so,libpsl,libpsl_so,libssh2,libssh2_so,zlib,libz_so,zstd,libzstd_so,openssl,libcrypto_so,libssl_so]);
    let jansson = &make_pkg_rc("jansson", &[glibc]);
    let bzip2 = &make_pkg_rc("bzip2", &[glibc,sh]);
    let libbz2_so = &make_pkg_rc("libbz2.so", &[glibc,sh]);
    let libcurl_so = &make_pkg_rc("libcurl.so", &[ca_certificates,brotli,libbrotlidec_so,krb5,libgssapi_krb5_so,libidn2,libidn2_so,libnghttp2,libnghttp2_so,libnghttp3,libnghttp3_so,libpsl,libpsl_so,libssh2,libssh2_so,zlib,libz_so,zstd,libzstd_so,openssl,libcrypto_so,libssl_so]);
    let json_c = &make_pkg_rc("json-c", &[glibc]);
    let liblzma_so = &make_pkg_rc("liblzma.so", &[lib32_glibc,xz]);
    let libelf = &make_pkg_rc("libelf", &[bzip2,libbz2_so,curl,libcurl_so,glibc,json_c,xz,liblzma_so,zlib,zstd,libzstd_so]);
    let binutils = &make_pkg_rc("binutils", &[glibc,jansson,libelf,zlib,zstd]);
    let mpfr = &make_pkg_rc("mpfr", &[glibc,gmp]);
    let libmpc = &make_pkg_rc("libmpc", &[glibc,gmp,mpfr]);
    let libisl_so = &make_pkg_rc("libisl.so", &[gmp]);
    let gcc = &make_pkg_rc("gcc", &[gcc_libs,binutils,libmpc,zstd,libisl_so]);
    let libedit = &make_pkg_rc("libedit", &[glibc,libncursesw_so,ncurses]);
    let icu = &make_pkg_rc("icu", &[gcc_libs,glibc,sh]);
    let libxml2 = &make_pkg_rc("libxml2", &[bash,glibc,icu,readline,xz,zlib]);
    let llvm_libs = &make_pkg_rc("llvm-libs", &[gcc_libs,zlib,zstd,libffi,libedit,libxml2]);
    let rust = &make_pkg_rc("rust", &[bash,curl,gcc,gcc_libs,glibc,libssh2,llvm_libs,openssl,zlib]);

    let rust_calculated_deps = HashSet::<_>::from_iter(rust.list_dependencies());
    let rust_expected_deps = [
            "glibc","lib32-glibc","lib32-gcc-libs","ncurses","libncursesw.so",
            "readline","libreadline.so","bash","acl","attr","gcc-libs","gmp",
            "libcap","openssl","coreutils","findutils","libtasn1","libffi",
            "libp11-kit","lib32-libffi","lib32-libtasn1","libp11-kit.so",
            "p11-kit","ca-certificates-utils","ca-certificates-mozilla",
            "ca-certificates","brotli","libbrotlidec.so","krb5","sh",
            "zlib","sqlite","util-linux-libs","e2fsprogs","keyutils",
            "gdbm","libgdbm.so","libcrypto.so","libsasl","libldap","libevent",
            "libverto-module-base","lmdb","libcom_err.so","libkeyutils.so",
            "libss.so","lib32-e2fsprogs","lib32-keyutils","lib32-openssl",
            "libxcrypt","lib32-libxcrypt","lib32-libldap","libverto.so",
            "libgssapi_krb5.so","libunistring","libunistring.so","libidn2",
            "lib32-libunistring","libidn2.so","libnghttp2","libnghttp2.so",
            "libnghttp3","libnghttp3.so","libpsl","lib32-libidn2","libpsl.so",
            "libssh2","lib32-zlib","libssh2.so","libz.so","xz","lz4","zstd",
            "libzstd.so","libssl.so","curl","jansson","bzip2","libbz2.so",
            "libcurl.so","json-c","liblzma.so","libelf","binutils","mpfr",
            "libmpc","libisl.so","gcc","libedit","icu","libxml2","llvm-libs"
        ]
            .iter()
            .cloned()
            .map(String::from)
            .collect::<HashSet<_>>();
    // println!("{:?}", rust_calculated_deps.difference(&rust_expected_deps).collect::<Vec<_>>());
    assert!(rust_calculated_deps == rust_expected_deps);
}

// RefCell
// ================================================================================================

// ----- 3 --------------------------------------
// Create a simple `SharedCounter` where multiple owners can increment its value without mutable
// reference.
//
// Implement `new() -> Self` constructor, `increment(&self)` and `get(&self) -> i32` methods.
// Use `RefCell` where needed.

// IMPLEMENT HERE:
pub struct SharedCounter {
    value: RefCell<i32>,
}

impl SharedCounter {
    pub fn new() -> Self {
        Self { value: RefCell::new(0), }
    }

    pub fn increment(&self) {
        *self.value.borrow_mut() += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}
