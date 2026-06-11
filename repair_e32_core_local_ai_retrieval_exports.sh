#!/usr/bin/env bash
set -euo pipefail

# Focused E32 repair:
# Adds/restores E32 monad-core implementation and root exports required by CLI:
# - build_local_retrieval_plan
# - render_local_retrieval_plan
# - render_local_retrieval_plan_json
# - write_local_retrieval_evidence
# - render_local_retrieval_apply_result
#
# Safety:
# - local-only
# - deterministic
# - no model/API calls
# - no vector DB/network/background indexer
# - backs up touched files
# - does not run package managers

if [[ ! -f "Cargo.toml" || ! -d "crates/monad-core/src" ]]; then
  echo "Run this script from the monad-workspace repository root." >&2
  exit 1
fi

BACKUP_DIR=".monad/script-backups/repair-e32-core-local-ai-retrieval-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$BACKUP_DIR"

backup_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    mkdir -p "$BACKUP_DIR/$(dirname "$path")"
    cp -a "$path" "$BACKUP_DIR/$path"
  fi
}

backup_if_exists "crates/monad-core/src/lib.rs"
backup_if_exists "crates/monad-core/src/local_ai_retrieval.rs"
backup_if_exists "docs/local-ai-retrieval/README.md"
backup_if_exists "docs/roadmap/epic-32-local-ai-retrieval-vector-memory.md"

mkdir -p crates/monad-core/src docs/local-ai-retrieval docs/roadmap

cat > crates/monad-core/src/local_ai_retrieval.rs <<'RS'
//! Local AI retrieval and vector memory foundation.
//!
//! This E32 foundation is local-only and provider-free. It discovers small
//! repository documents, chunks them deterministically, creates deterministic
//! local planning embeddings, assembles retrieval context, and writes generated
//! evidence only when explicitly approved.

use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::{gated_generated_write, GatedWriteRequest, GatedWriteResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalAiEmbeddingProvider {
    DeterministicLocal,
    ExternalBlocked,
}

impl LocalAiEmbeddingProvider {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeterministicLocal => "deterministic-local",
            Self::ExternalBlocked => "external-blocked",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalAiVectorStore {
    LocalJson,
    ExternalBlocked,
}

impl LocalAiVectorStore {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalJson => "local-json",
            Self::ExternalBlocked => "external-blocked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalRetrievalContract {
    chunk_word_budget: usize,
    chunk_overlap: usize,
    embedding_provider: LocalAiEmbeddingProvider,
    vector_store: LocalAiVectorStore,
    index_path: PathBuf,
    rules: Vec<String>,
}

impl LocalRetrievalContract {
    #[must_use]
    pub fn default_contract() -> Self {
        Self {
            chunk_word_budget: 120,
            chunk_overlap: 20,
            embedding_provider: LocalAiEmbeddingProvider::DeterministicLocal,
            vector_store: LocalAiVectorStore::LocalJson,
            index_path: PathBuf::from(".monad/indexes/local-ai-retrieval-index.json"),
            rules: vec![
                "Retrieval is local-only in E32.".to_string(),
                "Embeddings are deterministic local planning vectors.".to_string(),
                "No model provider or vector database is contacted.".to_string(),
                "No background indexer is started.".to_string(),
                "Only generated retrieval evidence is written with --yes.".to_string(),
            ],
        }
    }

    #[must_use]
    pub const fn chunk_word_budget(&self) -> usize {
        self.chunk_word_budget
    }

    #[must_use]
    pub const fn chunk_overlap(&self) -> usize {
        self.chunk_overlap
    }

    #[must_use]
    pub const fn embedding_provider(&self) -> LocalAiEmbeddingProvider {
        self.embedding_provider
    }

    #[must_use]
    pub const fn vector_store(&self) -> LocalAiVectorStore {
        self.vector_store
    }

    #[must_use]
    pub fn index_path(&self) -> &Path {
        &self.index_path
    }

    #[must_use]
    pub fn rules(&self) -> &[String] {
        &self.rules
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalAiRetrievalDocument {
    path: PathBuf,
    title: String,
    byte_len: u64,
}

impl LocalAiRetrievalDocument {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, title: impl Into<String>, byte_len: u64) -> Self {
        Self {
            path: path.into(),
            title: title.into(),
            byte_len,
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn byte_len(&self) -> u64 {
        self.byte_len
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalAiDocumentChunk {
    id: String,
    source_path: PathBuf,
    ordinal: usize,
    text: String,
    word_count: usize,
}

impl LocalAiDocumentChunk {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        source_path: impl Into<PathBuf>,
        ordinal: usize,
        text: impl Into<String>,
        word_count: usize,
    ) -> Self {
        Self {
            id: id.into(),
            source_path: source_path.into(),
            ordinal,
            text: text.into(),
            word_count,
        }
    }

    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub fn source_path(&self) -> &Path {
        &self.source_path
    }

    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    pub const fn word_count(&self) -> usize {
        self.word_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalAiEmbedding {
    provider: LocalAiEmbeddingProvider,
    dimensions: Vec<i32>,
}

impl LocalAiEmbedding {
    #[must_use]
    pub fn new(provider: LocalAiEmbeddingProvider, dimensions: Vec<i32>) -> Self {
        Self {
            provider,
            dimensions,
        }
    }

    #[must_use]
    pub const fn provider(&self) -> LocalAiEmbeddingProvider {
        self.provider
    }

    #[must_use]
    pub fn dimensions(&self) -> &[i32] {
        &self.dimensions
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalAiIndexedChunk {
    chunk: LocalAiDocumentChunk,
    embedding: LocalAiEmbedding,
}

impl LocalAiIndexedChunk {
    #[must_use]
    pub fn new(chunk: LocalAiDocumentChunk, embedding: LocalAiEmbedding) -> Self {
        Self { chunk, embedding }
    }

    #[must_use]
    pub const fn chunk(&self) -> &LocalAiDocumentChunk {
        &self.chunk
    }

    #[must_use]
    pub const fn embedding(&self) -> &LocalAiEmbedding {
        &self.embedding
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalAiRetrievalQuery {
    query: String,
    max_chunks: usize,
}

impl LocalAiRetrievalQuery {
    #[must_use]
    pub fn new(query: impl Into<String>, max_chunks: usize) -> Self {
        Self {
            query: query.into(),
            max_chunks,
        }
    }

    #[must_use]
    pub fn query(&self) -> &str {
        &self.query
    }

    #[must_use]
    pub const fn max_chunks(&self) -> usize {
        self.max_chunks
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalAiRetrievalContext {
    query: LocalAiRetrievalQuery,
    selected_chunks: Vec<LocalAiDocumentChunk>,
    context_text: String,
}

impl LocalAiRetrievalContext {
    #[must_use]
    pub fn new(
        query: LocalAiRetrievalQuery,
        mut selected_chunks: Vec<LocalAiDocumentChunk>,
        context_text: impl Into<String>,
    ) -> Self {
        selected_chunks.sort_by(|left, right| {
            left.source_path()
                .cmp(right.source_path())
                .then(left.ordinal().cmp(&right.ordinal()))
        });

        Self {
            query,
            selected_chunks,
            context_text: context_text.into(),
        }
    }

    #[must_use]
    pub const fn query(&self) -> &LocalAiRetrievalQuery {
        &self.query
    }

    #[must_use]
    pub fn selected_chunks(&self) -> &[LocalAiDocumentChunk] {
        &self.selected_chunks
    }

    #[must_use]
    pub fn context_text(&self) -> &str {
        &self.context_text
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalAiRetrievalPlan {
    command: String,
    contract: LocalRetrievalContract,
    documents: Vec<LocalAiRetrievalDocument>,
    indexed_chunks: Vec<LocalAiIndexedChunk>,
    context: LocalAiRetrievalContext,
    evidence_paths: Vec<PathBuf>,
    safety_notes: Vec<String>,
}

impl LocalAiRetrievalPlan {
    #[must_use]
    pub fn new(
        contract: LocalRetrievalContract,
        mut documents: Vec<LocalAiRetrievalDocument>,
        mut indexed_chunks: Vec<LocalAiIndexedChunk>,
        context: LocalAiRetrievalContext,
    ) -> Self {
        documents.sort_by(|left, right| left.path().cmp(right.path()));
        indexed_chunks.sort_by(|left, right| left.chunk().id().cmp(right.chunk().id()));

        Self {
            command: "retrieval-plan".to_string(),
            contract,
            documents,
            indexed_chunks,
            context,
            evidence_paths: vec![
                PathBuf::from(".monad/reports/local-ai-retrieval-plan.md"),
                PathBuf::from(".monad/reports/local-ai-retrieval-plan.json"),
                PathBuf::from(".monad/indexes/local-ai-retrieval-index.json"),
            ],
            safety_notes: vec![
                "No AI model provider is called by Monad.".to_string(),
                "No external vector database is contacted by Monad.".to_string(),
                "No network access is performed by Monad.".to_string(),
                "No background indexer is started by Monad.".to_string(),
            ],
        }
    }

    #[must_use]
    pub const fn contract(&self) -> &LocalRetrievalContract {
        &self.contract
    }

    #[must_use]
    pub fn documents(&self) -> &[LocalAiRetrievalDocument] {
        &self.documents
    }

    #[must_use]
    pub fn indexed_chunks(&self) -> &[LocalAiIndexedChunk] {
        &self.indexed_chunks
    }

    #[must_use]
    pub const fn context(&self) -> &LocalAiRetrievalContext {
        &self.context
    }

    #[must_use]
    pub fn evidence_paths(&self) -> &[PathBuf] {
        &self.evidence_paths
    }

    #[must_use]
    pub fn safety_notes(&self) -> &[String] {
        &self.safety_notes
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalAiRetrievalApplyResult {
    write_results: Vec<GatedWriteResult>,
}

impl LocalAiRetrievalApplyResult {
    #[must_use]
    pub fn new(write_results: Vec<GatedWriteResult>) -> Self {
        Self { write_results }
    }

    #[must_use]
    pub fn write_results(&self) -> &[GatedWriteResult] {
        &self.write_results
    }
}

#[must_use]
pub fn build_local_retrieval_plan(root: impl AsRef<Path>) -> LocalAiRetrievalPlan {
    let root = root.as_ref();
    let contract = LocalRetrievalContract::default_contract();
    let documents = discover_retrieval_documents(root);
    let chunks = chunk_retrieval_documents(
        root,
        &documents,
        contract.chunk_word_budget(),
        contract.chunk_overlap(),
    );
    let indexed_chunks = index_retrieval_chunks(&chunks, contract.embedding_provider());
    let query = LocalAiRetrievalQuery::new("Monad local governance evidence", 3);
    let context = assemble_local_retrieval_context(query, &indexed_chunks);

    LocalAiRetrievalPlan::new(contract, documents, indexed_chunks, context)
}

#[must_use]
pub fn discover_retrieval_documents(root: &Path) -> Vec<LocalAiRetrievalDocument> {
    let mut paths = vec![
        PathBuf::from("README.md"),
        PathBuf::from("monad.toml"),
        PathBuf::from("Cargo.toml"),
    ];

    collect_markdown_paths(root, Path::new("docs"), &mut paths);
    paths.sort();
    paths.dedup();

    paths
        .into_iter()
        .filter(|path| root.join(path).is_file())
        .take(32)
        .map(|path| {
            let absolute = root.join(&path);
            let byte_len = fs::metadata(&absolute).map_or(0, |metadata| metadata.len());
            let title = derive_title(&absolute, &path);
            LocalAiRetrievalDocument::new(path, title, byte_len)
        })
        .collect()
}

#[must_use]
pub fn chunk_retrieval_documents(
    root: &Path,
    documents: &[LocalAiRetrievalDocument],
    chunk_word_budget: usize,
    chunk_overlap: usize,
) -> Vec<LocalAiDocumentChunk> {
    let mut chunks = Vec::new();

    for document in documents {
        let text = fs::read_to_string(root.join(document.path())).unwrap_or_default();
        let words = text.split_whitespace().collect::<Vec<_>>();
        if words.is_empty() {
            continue;
        }

        let budget = chunk_word_budget.max(1);
        let overlap = chunk_overlap.min(budget.saturating_sub(1));
        let step = budget.saturating_sub(overlap).max(1);
        let mut start = 0usize;
        let mut ordinal = 0usize;

        while start < words.len() {
            let end = (start + budget).min(words.len());
            let chunk_text = words[start..end].join(" ");
            chunks.push(LocalAiDocumentChunk::new(
                make_chunk_id(document.path(), ordinal),
                document.path().to_path_buf(),
                ordinal,
                chunk_text,
                end - start,
            ));

            if end == words.len() {
                break;
            }

            start += step;
            ordinal += 1;
        }
    }

    chunks.sort_by(|left, right| left.id().cmp(right.id()));
    chunks
}

#[must_use]
pub fn index_retrieval_chunks(
    chunks: &[LocalAiDocumentChunk],
    provider: LocalAiEmbeddingProvider,
) -> Vec<LocalAiIndexedChunk> {
    chunks
        .iter()
        .cloned()
        .map(|chunk| {
            let embedding = embed_text_locally(chunk.text(), provider);
            LocalAiIndexedChunk::new(chunk, embedding)
        })
        .collect()
}

#[must_use]
pub fn embed_text_locally(text: &str, provider: LocalAiEmbeddingProvider) -> LocalAiEmbedding {
    if provider == LocalAiEmbeddingProvider::ExternalBlocked {
        return LocalAiEmbedding::new(provider, vec![0; 8]);
    }

    let mut dimensions = vec![0i32; 8];
    for token in text.split_whitespace() {
        let hash = stable_hash(token.as_bytes());
        let index = (hash as usize) % dimensions.len();
        let sign = if hash & 1 == 0 { 1 } else { -1 };
        dimensions[index] += sign;
    }

    LocalAiEmbedding::new(provider, dimensions)
}

#[must_use]
pub fn assemble_local_retrieval_context(
    query: LocalAiRetrievalQuery,
    indexed_chunks: &[LocalAiIndexedChunk],
) -> LocalAiRetrievalContext {
    let query_terms = normalize_terms(query.query());
    let mut scored = indexed_chunks
        .iter()
        .map(|indexed| {
            let score = overlap_score(&query_terms, indexed.chunk().text());
            (score, indexed.chunk().clone())
        })
        .collect::<Vec<_>>();

    scored.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then(left.1.source_path().cmp(right.1.source_path()))
            .then(left.1.ordinal().cmp(&right.1.ordinal()))
    });

    let selected_chunks = scored
        .into_iter()
        .filter(|(score, _)| *score > 0)
        .take(query.max_chunks())
        .map(|(_, chunk)| chunk)
        .collect::<Vec<_>>();

    let context_text = selected_chunks
        .iter()
        .map(|chunk| {
            format!(
                "[{}#{}]\n{}",
                chunk.source_path().display(),
                chunk.ordinal(),
                chunk.text()
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");

    LocalAiRetrievalContext::new(query, selected_chunks, context_text)
}

pub fn write_local_retrieval_evidence(
    root: impl AsRef<Path>,
) -> Result<LocalAiRetrievalApplyResult, String> {
    let root = root.as_ref();
    let plan = build_local_retrieval_plan(root);
    let requests = [
        GatedWriteRequest::new(
            ".monad/reports/local-ai-retrieval-plan.md",
            render_local_retrieval_plan(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/reports/local-ai-retrieval-plan.json",
            render_local_retrieval_plan_json(&plan),
            true,
        ),
        GatedWriteRequest::new(
            ".monad/indexes/local-ai-retrieval-index.json",
            render_local_retrieval_index_json(&plan),
            true,
        ),
    ];

    let write_results = requests
        .iter()
        .map(|request| gated_generated_write(root, request))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(LocalAiRetrievalApplyResult::new(write_results))
}

#[must_use]
pub fn render_local_retrieval_plan(plan: &LocalAiRetrievalPlan) -> String {
    let mut lines = vec![
        "Monad local AI retrieval and vector memory plan".to_string(),
        String::new(),
        "Contract:".to_string(),
        format!(
            "  embedding_provider: {}",
            plan.contract().embedding_provider().as_str()
        ),
        format!("  vector_store: {}", plan.contract().vector_store().as_str()),
        format!("  index_path: {}", plan.contract().index_path().display()),
        String::new(),
        format!("Documents: {}", plan.documents().len()),
    ];

    for document in plan.documents() {
        lines.push(format!(
            "  - {} bytes={} title={}",
            document.path().display(),
            document.byte_len(),
            document.title()
        ));
    }

    lines.push(String::new());
    lines.push(format!("Indexed chunks: {}", plan.indexed_chunks().len()));
    for indexed in plan.indexed_chunks().iter().take(8) {
        lines.push(format!(
            "  - {} provider={} dims={:?}",
            indexed.chunk().id(),
            indexed.embedding().provider().as_str(),
            indexed.embedding().dimensions()
        ));
    }

    lines.push(String::new());
    lines.push("Context assembly:".to_string());
    lines.push(format!("  query: {}", plan.context().query().query()));
    lines.push(format!(
        "  selected_chunks: {}",
        plan.context().selected_chunks().len()
    ));

    lines.push(String::new());
    lines.push("Evidence outputs:".to_string());
    for path in plan.evidence_paths() {
        lines.push(format!("  - {}", path.display()));
    }

    lines.push(String::new());
    lines.push("Safety notes:".to_string());
    for note in plan.safety_notes() {
        lines.push(format!("  - {note}"));
    }

    lines.join("\n")
}

#[must_use]
pub fn render_local_retrieval_plan_json(plan: &LocalAiRetrievalPlan) -> String {
    serde_json::to_string_pretty(plan).unwrap_or_else(|_| {
        "{\n  \"command\": \"retrieval-plan\",\n  \"error\": \"local retrieval plan serialization failed\"\n}".to_string()
    })
}

#[must_use]
pub fn render_local_retrieval_index_json(plan: &LocalAiRetrievalPlan) -> String {
    serde_json::to_string_pretty(plan.indexed_chunks()).unwrap_or_else(|_| {
        "{\n  \"error\": \"local retrieval index serialization failed\"\n}".to_string()
    })
}

#[must_use]
pub fn render_local_retrieval_apply_result(result: &LocalAiRetrievalApplyResult) -> String {
    let mut lines = vec![
        "Monad local AI retrieval evidence write result".to_string(),
        String::new(),
        "Results:".to_string(),
    ];

    for write_result in result.write_results() {
        match write_result {
            GatedWriteResult::Written(path) | GatedWriteResult::SkippedIdentical(path) => {
                lines.push(format!("  - [{}] {}", write_result.as_str(), path.display()));
            }
            GatedWriteResult::ApprovalRequired(message) | GatedWriteResult::Blocked(message) => {
                lines.push(format!("  - [{}] {message}", write_result.as_str()));
            }
        }
    }

    lines.push(String::new());
    lines.push("No AI model provider was called.".to_string());
    lines.push("No vector database was contacted.".to_string());
    lines.push("No background indexer was started.".to_string());

    lines.join("\n")
}

fn collect_markdown_paths(root: &Path, relative_dir: &Path, paths: &mut Vec<PathBuf>) {
    let absolute = root.join(relative_dir);
    let Ok(entries) = fs::read_dir(&absolute) else {
        return;
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if let Ok(relative) = path.strip_prefix(root) {
                collect_markdown_paths(root, relative, paths);
            }
        } else if path.extension().and_then(|value| value.to_str()) == Some("md")
            && let Ok(relative) = path.strip_prefix(root)
        {
            paths.push(relative.to_path_buf());
        }
    }
}

fn derive_title(absolute_path: &Path, relative_path: &Path) -> String {
    if let Ok(text) = fs::read_to_string(absolute_path)
        && let Some(title) = text.lines().map(str::trim).find(|line| line.starts_with("# "))
    {
        return title.trim_start_matches("# ").trim().to_string();
    }

    relative_path
        .file_stem()
        .map_or_else(|| "document".to_string(), |stem| stem.to_string_lossy().to_string())
}

fn make_chunk_id(path: &Path, ordinal: usize) -> String {
    let slug = path
        .to_string_lossy()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    format!("chunk:{slug}:{ordinal}")
}

fn stable_hash(bytes: &[u8]) -> u64 {
    let mut value = 14_695_981_039_346_656_037u64;
    for byte in bytes {
        value ^= u64::from(*byte);
        value = value.wrapping_mul(1_099_511_628_211);
    }
    value
}

fn normalize_terms(text: &str) -> Vec<String> {
    let mut terms = text
        .split_whitespace()
        .map(|term| {
            term.chars()
                .filter(|ch| ch.is_ascii_alphanumeric())
                .collect::<String>()
                .to_ascii_lowercase()
        })
        .filter(|term| !term.is_empty())
        .collect::<Vec<_>>();

    terms.sort();
    terms.dedup();
    terms
}

fn overlap_score(query_terms: &[String], text: &str) -> usize {
    let text_terms = normalize_terms(text);
    query_terms
        .iter()
        .filter(|term| text_terms.binary_search(term).is_ok())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_root(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        std::env::temp_dir().join(format!(
            "monad-local-ai-retrieval-{test_name}-{}-{unique}",
            std::process::id()
        ))
    }

    fn workspace(test_name: &str) -> PathBuf {
        let root = temp_root(test_name);
        assert!(fs::create_dir_all(root.join("docs")).is_ok());
        assert!(fs::write(root.join("Cargo.toml"), "[workspace]\n").is_ok());
        assert!(fs::write(root.join("monad.toml"), "schema_version = 1\n").is_ok());
        assert!(
            fs::write(
                root.join("docs/retrieval.md"),
                "# Retrieval\n\nMonad local governance evidence retrieval memory context.\n",
            )
            .is_ok()
        );
        root
    }

    #[test]
    fn local_embedding_is_deterministic() {
        let left = embed_text_locally("local governance memory", LocalAiEmbeddingProvider::DeterministicLocal);
        let right = embed_text_locally("local governance memory", LocalAiEmbeddingProvider::DeterministicLocal);

        assert_eq!(left, right);
        assert_eq!(left.dimensions().len(), 8);
    }

    #[test]
    fn documents_are_discovered_locally() {
        let root = workspace("documents");
        let documents = discover_retrieval_documents(&root);

        assert!(documents.iter().any(|document| document.path() == Path::new("Cargo.toml")));
        assert!(documents.iter().any(|document| document.path() == Path::new("docs/retrieval.md")));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn retrieval_context_selects_relevant_chunks() {
        let root = workspace("context");
        let plan = build_local_retrieval_plan(&root);

        assert!(!plan.indexed_chunks().is_empty());
        assert!(plan.context().context_text().contains("Monad"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn text_render_mentions_provider_boundary() {
        let root = workspace("text");
        let plan = build_local_retrieval_plan(&root);
        let text = render_local_retrieval_plan(&plan);

        assert!(text.contains("Monad local AI retrieval and vector memory plan"));
        assert!(text.contains("No AI model provider is called by Monad"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn json_render_contains_retrieval_plan_command() {
        let root = workspace("json");
        let plan = build_local_retrieval_plan(&root);
        let json = render_local_retrieval_plan_json(&plan);

        assert!(json.contains("\"command\": \"retrieval-plan\""));
        assert!(json.contains("deterministic-local"));

        let _ = fs::remove_dir_all(root);
    }
}
RS

cat > docs/local-ai-retrieval/README.md <<'MD'
# Local AI Retrieval and Vector Memory

E32 adds Monad's local AI retrieval and vector memory foundation.

## Command surface

```bash
monad retrieval-plan --dry-run
monad retrieval-plan --dry-run --format=json
monad retrieval-plan --yes
monad local-retrieval --dry-run
monad vector-memory --dry-run
```

## Safety boundaries

This foundation does **not** call AI model providers, contact vector databases,
access networks, start background indexers, invoke package managers, or mutate
user-owned source files.
MD

cat > docs/roadmap/epic-32-local-ai-retrieval-vector-memory.md <<'MD'
# E32 — Local AI Retrieval and Vector Memory Foundation

## Product Area

Local AI Retrieval and Vector Memory Foundation

## Objective

Add Monad's local retrieval/vector-memory contract, deterministic document
chunking, provider abstraction, local index proof of concept, retrieval context
assembly model, and smoke tests.

## Work Packets

- WP-E32-001 — Define local retrieval and vector memory contract
- WP-E32-002 — Add document chunking model
- WP-E32-003 — Add embedding provider abstraction
- WP-E32-004 — Add local index storage proof of concept
- WP-E32-005 — Add retrieval query and context assembly model
- WP-E32-006 — Add retrieval fixtures and smoke tests
MD

python3 - <<'PY'
from pathlib import Path

path = Path("crates/monad-core/src/lib.rs")
text = path.read_text()

if "pub mod local_ai_retrieval;" not in text:
    anchors = [
        "pub mod mcp_integration;\n",
        "pub mod plugin_system;\n",
        "pub mod template_registry;\n",
        "pub mod report_store;\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, anchor + "pub mod local_ai_retrieval;\n", 1)
            break
    else:
        raise SystemExit("Could not find lib.rs module insertion point for local_ai_retrieval.")

pub_use = """pub use local_ai_retrieval::{
    LocalAiDocumentChunk, LocalAiEmbedding, LocalAiEmbeddingProvider, LocalAiIndexedChunk,
    LocalAiRetrievalApplyResult, LocalAiRetrievalContext, LocalAiRetrievalDocument,
    LocalAiRetrievalPlan, LocalAiRetrievalQuery, LocalAiVectorStore, LocalRetrievalContract,
    assemble_local_retrieval_context, build_local_retrieval_plan, chunk_retrieval_documents,
    discover_retrieval_documents, embed_text_locally, index_retrieval_chunks,
    render_local_retrieval_apply_result, render_local_retrieval_index_json,
    render_local_retrieval_plan, render_local_retrieval_plan_json, write_local_retrieval_evidence,
};
"""
if "pub use local_ai_retrieval::" not in text:
    anchors = [
        "pub use mcp_integration::{\n",
        "pub use plugin_system::{\n",
        "pub use template_registry::{\n",
        "pub use report_store::{\n",
    ]
    for anchor in anchors:
        if anchor in text:
            text = text.replace(anchor, pub_use + anchor, 1)
            break
    else:
        raise SystemExit("Could not find lib.rs pub use insertion point for local_ai_retrieval.")

path.write_text(text)
PY

cargo fmt

echo "Applied focused E32 core/export repair."
echo "Backup written under: $BACKUP_DIR"
echo
echo "Run focused verification:"
echo "  cargo check -p monad-core"
echo "  cargo test -p monad-core --lib local_ai_retrieval"
echo "  cargo test -p monad-cli retrieval_plan"
echo "  cargo run -p monad-cli -- retrieval-plan --dry-run"
echo
echo "Then run full verification:"
echo "  cargo fmt --check"
echo "  cargo test"
echo "  cargo clippy --all-targets --all-features -- -D warnings"
echo "  tools/scripts/verify-local-ai-retrieval.sh"
echo "  tools/scripts/verify-e32.sh"
