//! Scans `../cases/*.ttl` at compile time and writes a small JSON index
//! (slug, title, path) that the Browse Tests page (`src/tests_page.rs`)
//! fetches at runtime, the same "generate at build.rs time, fetch at
//! runtime" split the sibling `ds-odrl-engine-rs` site and the `dataspace`
//! site's own `build.rs` both use for their own generated artifacts (see
//! those repos' `site/build.rs` and `site/Trunk.toml`) -- so this index
//! stays freshly regenerated on every build instead of drifting from
//! `cases/` the way a hand-maintained list would.
//!
//! Title extraction is a deliberately simple line-scan for a
//! `dct:title "..."` triple, NOT a real Turtle parser: this repo's test
//! cases always put the whole `dct:title "..."` triple on one line (see
//! `cases/inherited-agreement-duty-chain-01.ttl`), and a v1 index has no
//! need for anything more general. If a future case's title spans
//! multiple lines or embeds an escaped `"`, this scan will miss/mangle it
//! -- acceptable for now, called out here rather than silently wrong.
//!
//! Written to a *fixed path in the source tree*
//! (`site/generated/test-index.json`), not `OUT_DIR`: Trunk's own asset
//! pipeline (`index.html`'s `copy-file` directive) needs a stable path to
//! copy from, and `site/Trunk.toml`'s `pre_build` hook (`cargo check`)
//! forces this script to finish writing that file before Trunk's copy
//! step runs -- see that file's own comment for why the ordering has to
//! be enforced that way rather than assumed.

use std::fs;
use std::path::Path;

use serde::Serialize;

#[derive(Serialize)]
struct CaseEntry {
  slug: String,
  title: String,
  path: String,
}

fn extract_title(ttl: &str) -> Option<String> {
  for line in ttl.lines() {
    let line = line.trim();
    if let Some(rest) = line.strip_prefix("dct:title") {
      let rest = rest.trim_start();
      let rest = rest.strip_prefix('"')?;
      // Cut at the first `"` that starts the closing quote (optionally
      // followed by an `@lang` tag and/or a trailing ` ;`) -- good enough
      // for this repo's single-line dct:title triples.
      let end = rest.find('"')?;
      return Some(rest[..end].to_string());
    }
  }
  None
}

fn main() {
  let cases_dir = Path::new("../cases");
  println!("cargo:rerun-if-changed={}", cases_dir.display());

  let mut entries = Vec::new();
  if let Ok(read_dir) = fs::read_dir(cases_dir) {
    for entry in read_dir.flatten() {
      let path = entry.path();
      if path.extension().and_then(|e| e.to_str()) != Some("ttl") {
        continue;
      }
      let slug = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default().to_string();
      println!("cargo:rerun-if-changed={}", path.display());
      let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
      let title = extract_title(&content).unwrap_or_else(|| slug.clone());
      entries.push(CaseEntry { slug: slug.clone(), title, path: format!("cases/{slug}.ttl") });
    }
  }
  entries.sort_by(|a, b| a.slug.cmp(&b.slug));

  let out_dir = Path::new("generated");
  fs::create_dir_all(out_dir).unwrap_or_else(|e| panic!("failed to create {}: {e}", out_dir.display()));
  let json = serde_json::to_string_pretty(&entries).expect("failed to serialize test-index.json");
  let out_path = out_dir.join("test-index.json");
  // Avoid an unconditional write bumping generated/'s mtime on every
  // single `cargo check`, which -- combined with `cargo:rerun-if-changed`
  // watching cases/ -- would otherwise be harmless here (cases/ doesn't
  // change on every build) but matches the write-if-changed discipline
  // the dataspace site's own build.rs documents at length for the exact
  // same reason.
  if fs::read(&out_path).ok().as_deref() != Some(json.as_bytes()) {
    fs::write(&out_path, &json).unwrap_or_else(|e| panic!("failed to write {}: {e}", out_path.display()));
  }
}
