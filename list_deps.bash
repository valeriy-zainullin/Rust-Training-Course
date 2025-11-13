LISTED=
IN_PROGRESS=
KNOWN_CYCLIC_DEPS=$(cat - <<EOF
ncurses
glibc
lib32-glibc
lib32-keyutils
keyutils
krb5
lib32-krb5
libcap
lib32-libcap
EOF
)
list_deps() {
  local pkg="$1"

  if echo "$LISTED" | grep -xF "$pkg" >/dev/null; then
    return
  fi

  # echo "--- IN_PROGRESS check for pkg $pkg ---"
  # echo "$IN_PROGRESS"
  # echo "--- ----------- ---"

  if echo "$IN_PROGRESS" | grep -xF "$pkg" >/dev/null; then
    echo "Cycle found! pkg $pkg" >&2
    exit 1
  fi

  # echo "--- KNOWN_CYCLIC_DEPS check for pkg $pkg ---"
  # echo "$KNOWN_CYCLIC_DEPS"
  # echo "$KNOWN_CYCLIC_DEPS" | grep -xF "$pkg"
  # echo "--- ----------- ---"

  if echo "$KNOWN_CYCLIC_DEPS" | grep -xF "$pkg" >/dev/null; then
    printf "// $pkg produces a cyclic dependency on itself, it's alright, not listing its dependencies\n"
    local pkg_sanitized=$(echo "$pkg" | sed 's/[-.]/_/g')
    printf "let %s = &make_pkg_rc(\"%s\", &[]);\n" "$pkg_sanitized" "$pkg"
    LISTED="$LISTED"$'\n'"$pkg"
    return
  fi

  IN_PROGRESS="$IN_PROGRESS"$'\n'"$pkg"
  # echo "--- IN_PROGRESS after pkg $pkg added ---"
  # echo "$IN_PROGRESS"
  # echo "--- ----------- ---"

  local deps=$(LC_ALL=C pacman -Qi "$pkg" | grep -Po 'Depends On\s*:\s*\K.*' | sed 's/\s\s/ /g' | tr ' ' '\n' | sed 's/[><]\?\=.*$//')
  if [ "$deps" = 'None' ]; then
    local deps=""
  fi

  for dep in $deps; do
    # echo "$pkg -> $dep" >&2
    list_deps "$dep"
  done

  local dep_list=$(echo -n "$deps" | tr '\n' ',')
  local pkg_sanitized=$(echo "$pkg" | sed 's/[-.]/_/g')
  local dep_list_sanitized=$(echo "$dep_list" | sed 's/[-.]/_/g')
  printf "let %s = &make_pkg_rc(\"%s\", &[%s]);\n" "$pkg_sanitized" "$pkg" "$dep_list_sanitized"

  LISTED="$LISTED"$'\n'"$pkg"
  IN_PROGRESS=$(echo "$IN_PROGRESS" | grep -xvF "$pkg")
}
printf '// Some system packages inevitably depend on themselves.\n'
printf '// In this case pacman would chouse an arbitary order (as far as I understand).\n'
printf '// Let'\''s just skip their dependency lists.\n'
printf 'let make_pkg_rc = |pkg, deps: &[&Rc<Package>]| {\n  Rc::new(\n    Package::with_dependencies(\n      pkg,\n      deps.iter().cloned().cloned().collect()\n    )\n  )\n};\n'
PKG=rust
list_deps "$PKG"

pkg_sanitized=$(echo "$PKG" | sed 's/[-.]/_/g')
LISTED=$(echo -n "$LISTED" | grep -vFx '') # Delete leading empty line
LISTED=$(echo -n "$LISTED" | grep -vFx "$PKG") # Delete the package itself
deps=$(echo -n "$LISTED" | sed 's/^\(.*\)$/"\1"/' | tr '\n' ',')
printf 'assert!(HashSet::<_>::from_iter(%s.list_dependencies()) == [%s].iter().cloned().map(String::from).collect());\n' "$pkg_sanitized" "$deps"
