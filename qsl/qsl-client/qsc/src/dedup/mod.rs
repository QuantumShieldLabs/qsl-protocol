use super::*;

// NA-0644 (D580, ENG-0040): durable relay msg_id dedup for the opt-in acknowledged-pull
// (lease) receive mode. Lease delivery is at-least-once — a lost ack redelivers ids in
// normal operation — so the receive path must recognize "already durably persisted"
// BEFORE unpack and ack-and-skip instead of reprocessing (reprocessing hits the ratchet
// replay-reject, which is a process-exit). The transport caller keeps the invariant:
// an id becomes ack-eligible ONLY after both the item's own durable commit and this
// store's entry for it are on disk (`record` returning Ok IS that second half).
//
// Legacy mode never constructs this store: the legacy pull path stays byte-identical.

const SEEN_IDS_VERSION: u32 = 1;
// Strictly beyond the relay's 30-day retention ceiling: a message older than retention
// no longer exists server-side, so no redelivery of its id can arrive after this window
// and the entry is safe to prune.
const SEEN_IDS_MAX_AGE_SECS: u64 = 31 * 24 * 60 * 60;
const SEEN_IDS_MAX_ENTRIES: usize = 65_536;

#[derive(Serialize, Deserialize)]
struct SeenIdEntry {
    id: String,
    first_seen_unix: u64,
}

#[derive(Deserialize)]
struct SeenIdsFile {
    version: u32,
    entries: Vec<SeenIdEntry>,
}

#[derive(Serialize)]
struct SeenIdsFileView<'a> {
    version: u32,
    entries: &'a std::collections::VecDeque<SeenIdEntry>,
}

pub(crate) struct RelaySeenIds {
    path: PathBuf,
    source: ConfigSource,
    ids: std::collections::HashSet<String>,
    // Arrival order, oldest first; the age/cap prunes pop from the front.
    entries: std::collections::VecDeque<SeenIdEntry>,
}

pub(crate) struct SeenIdsLoad {
    pub(crate) store: RelaySeenIds,
    pub(crate) reset: bool,
}

// The raw route token must never appear in a filename — key the per-mailbox store by a
// hash of the normalized token, like the relay's own at-rest keying.
fn mailbox_store_key(route_token: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(route_token.as_bytes());
    hex_encode(&hash[..8])
}

fn seen_now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl RelaySeenIds {
    // Missing file = empty store. An unreadable/unparseable/wrong-version file resets to
    // empty (`reset` = true so the caller can emit a warning): availability over strictness
    // here is safe because a redelivered dupe that slips past an empty store is still
    // caught by the lease-mode replay-reject backstop instead of reprocessing.
    pub(crate) fn load(cfg_dir: &Path, route_token: &str, source: ConfigSource) -> SeenIdsLoad {
        let name = format!("relay_seen_ids_v1_{}.json", mailbox_store_key(route_token));
        let path = cfg_dir.join(name);
        let mut reset = false;
        let mut entries: std::collections::VecDeque<SeenIdEntry> = Default::default();
        match std::fs::read(&path) {
            Ok(bytes) => match serde_json::from_slice::<SeenIdsFile>(&bytes) {
                Ok(file) if file.version == SEEN_IDS_VERSION => {
                    entries = file.entries.into();
                }
                _ => reset = true,
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => reset = true,
        }
        let now = seen_now_unix();
        while let Some(front) = entries.front() {
            if now.saturating_sub(front.first_seen_unix) > SEEN_IDS_MAX_AGE_SECS {
                entries.pop_front();
            } else {
                break;
            }
        }
        while entries.len() > SEEN_IDS_MAX_ENTRIES {
            entries.pop_front();
        }
        let ids = entries.iter().map(|e| e.id.clone()).collect();
        SeenIdsLoad {
            store: RelaySeenIds {
                path,
                source,
                ids,
                entries,
            },
            reset,
        }
    }

    pub(crate) fn contains(&self, id: &str) -> bool {
        self.ids.contains(id)
    }

    // Durably records the id: Ok means the entry is on disk (write_atomic — temp, fsync,
    // rename). The caller must treat Err as fail-closed and must NOT ack the id.
    pub(crate) fn record(&mut self, id: &str) -> Result<(), ErrorCode> {
        if self.ids.contains(id) {
            return Ok(());
        }
        self.entries.push_back(SeenIdEntry {
            id: id.to_string(),
            first_seen_unix: seen_now_unix(),
        });
        self.ids.insert(id.to_string());
        while self.entries.len() > SEEN_IDS_MAX_ENTRIES {
            if let Some(evicted) = self.entries.pop_front() {
                self.ids.remove(&evicted.id);
            }
        }
        let view = SeenIdsFileView {
            version: SEEN_IDS_VERSION,
            entries: &self.entries,
        };
        let bytes = serde_json::to_vec(&view).map_err(|_| ErrorCode::IoWriteFailed)?;
        write_atomic(&self.path, &bytes, self.source)
    }
}
