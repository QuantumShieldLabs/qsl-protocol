import { invoke } from "@tauri-apps/api/core";
import "./style.css";

const app = document.querySelector("#app");

const state = {
  snapshot: null,
  selectedPeer: "",
  notice: null,
  busy: false,
  receivedBatch: [],
  deliveryBatch: [],
  forms: {
    passphrase: "",
    relayUrl: "",
    inboxToken: "",
    contactLabel: "",
    contactFingerprint: "",
    contactRouteToken: "",
    composePeer: "",
    composeMessage: "",
    receiveMax: "4"
  }
};

function setBusy(busy) {
  state.busy = busy;
  render();
}

function setNotice(kind, title, detail = "") {
  state.notice = { kind, title, detail };
  render();
}

function clearNotice() {
  state.notice = null;
}

function mapError(err) {
  if (!err) {
    return {
      title: "Unknown desktop bridge failure",
      detail: "The Rust bridge returned no structured error."
    };
  }
  if (typeof err === "string") {
    return {
      title: err,
      detail: ""
    };
  }
  return {
    title: err.message || err.code || "Desktop bridge failure",
    detail: err.detail || ""
  };
}

function peerOptions() {
  return state.snapshot?.contacts ?? [];
}

function selectedPeerValue() {
  if (state.selectedPeer) {
    return state.selectedPeer;
  }
  const first = peerOptions()[0];
  return first ? first.label : "";
}

function selectedPeerDetails() {
  const selected = selectedPeerValue();
  if (!selected || state.snapshot?.peer_details?.label !== selected) {
    return null;
  }
  return state.snapshot.peer_details;
}

async function refresh(selectedPeer = selectedPeerValue()) {
  setBusy(true);
  try {
    const snapshot = await invoke("refresh_snapshot", {
      selectedPeer: selectedPeer || null
    });
    state.snapshot = snapshot;
    if (!state.selectedPeer && snapshot.contacts.length > 0) {
      state.selectedPeer = snapshot.contacts[0].label;
    }
    clearNotice();
  } catch (err) {
    const mapped = mapError(err);
    setNotice("danger", mapped.title, mapped.detail);
  } finally {
    setBusy(false);
  }
}

async function callAndRefresh(command, payload = {}, options = {}) {
  setBusy(true);
  try {
    const result = await invoke(command, payload);
    state.snapshot = result.snapshot ?? result;
    if (options.clearPassphrase) {
      state.forms.passphrase = "";
    }
    if (options.selectPeer) {
      state.selectedPeer = options.selectPeer;
    } else if (!state.selectedPeer && state.snapshot.contacts.length > 0) {
      state.selectedPeer = state.snapshot.contacts[0].label;
    }
    if (result.delivery) {
      state.deliveryBatch = result.delivery;
    }
    if (result.received_files) {
      state.receivedBatch = result.received_files;
    }
    if (options.notice) {
      setNotice(options.notice.kind, options.notice.title, options.notice.detail || "");
    } else {
      clearNotice();
      render();
    }
  } catch (err) {
    const mapped = mapError(err);
    setNotice("danger", mapped.title, mapped.detail);
  } finally {
    setBusy(false);
  }
}

function onInput(field, value) {
  state.forms[field] = value;
}

function escapeHtml(value) {
  return String(value)
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll("\"", "&quot;");
}

function noticeHtml(notice) {
  if (!notice) {
    return "";
  }
  return `
    <div class="notice ${escapeHtml(notice.kind)}">
      <strong>${escapeHtml(notice.title)}</strong>
      ${notice.detail ? `<div>${escapeHtml(notice.detail)}</div>` : ""}
    </div>
  `;
}

function heroHtml(snapshot) {
  const statusPill = snapshot?.sidecar_ready
    ? '<span class="state-pill ok">Bridge ready</span>'
    : '<span class="state-pill danger">Bridge blocked</span>';
  const unlockPill = snapshot?.session_unlocked
    ? '<span class="state-pill ok">Session unlocked</span>'
    : '<span class="state-pill warn">Session locked</span>';
  const vaultPill = snapshot?.vault.present
    ? `<span class="state-pill ${snapshot.vault.key_source === "passphrase" ? "ok" : "warn"}">Vault ${escapeHtml(snapshot.vault.key_source)}</span>`
    : '<span class="state-pill warn">Vault missing</span>';
  const protocolPill = snapshot?.session_note
    ? `<span class="state-pill warn">${escapeHtml(snapshot.session_note)}</span>`
    : '<span class="state-pill ok">Marker-only shell</span>';
  const doctor = snapshot?.doctor;
  return `
    <section class="hero">
      <div>
        <div class="eyebrow">Linux/macOS prototype • Tauri shell • qsc-owned truth</div>
        <h1>QSC Desktop Prototype</h1>
        <p>
          A bounded desktop shell over the frozen <code>qsc</code> marker surface. This build keeps
          vault, identity, contacts, routing, delivery state, and timeline truth inside the sidecar,
          and surfaces only message-first operator flows.
        </p>
        <div class="tag-row" style="margin-top: 18px;">
          ${statusPill}
          ${unlockPill}
          ${vaultPill}
          ${protocolPill}
        </div>
      </div>
      <div class="hero-status">
        <div class="status-card">
          <div class="status-label">Config Root</div>
          <div class="status-value"><code>${escapeHtml(doctor?.config_dir || "n/a")}</code></div>
        </div>
        <div class="status-card">
          <div class="status-label">Identity</div>
          <div class="status-value">${escapeHtml(snapshot?.identity_fp || "Locked or not initialized")}</div>
        </div>
        <div class="status-card">
          <div class="status-label">Contacts</div>
          <div class="status-value">${snapshot?.contacts?.length ?? 0}</div>
        </div>
        <div class="status-card">
          <div class="status-label">Doctor</div>
          <div class="status-value">${doctor?.ok ? "Pass" : "Needs attention"}</div>
        </div>
      </div>
    </section>
  `;
}

function profilePanelHtml(snapshot) {
  const bootstrapDisabled = state.busy;
  const unlockDisabled = state.busy || !state.forms.passphrase;
  const clearDisabled = state.busy || !snapshot?.session_unlocked;
  return `
    <section class="panel">
      <div class="panel-header">
        <div>
          <h2>Readiness + Unlock</h2>
          <p class="panel-subtitle">Passphrase stays backend-memory only and is injected child-scoped for each sidecar call.</p>
        </div>
        <div class="tag-row">
          <span class="tag"><strong>Vault:</strong> ${escapeHtml(snapshot.vault.present ? snapshot.vault.key_source : "missing")}</span>
          <span class="tag"><strong>Doctor:</strong> ${snapshot.doctor.ok ? "ok" : "check"}</span>
        </div>
      </div>
      <div class="form-grid">
        <div class="form-group">
          <label for="passphrase">Session Passphrase</label>
          <input
            id="passphrase"
            type="password"
            autocomplete="off"
            spellcheck="false"
            value="${escapeHtml(state.forms.passphrase)}"
            placeholder="Stored in backend memory for this app session only"
          />
        </div>
        <div class="button-row">
          <button class="button-primary" ${bootstrapDisabled ? "disabled" : ""} data-action="init-passphrase">Initialize passphrase profile</button>
          <button class="button-secondary" ${unlockDisabled ? "disabled" : ""} data-action="unlock">Unlock existing profile</button>
          <button class="button-danger" ${clearDisabled ? "disabled" : ""} data-action="lock">Clear in-memory unlock</button>
        </div>
      </div>
      <div class="footer-note">
        <strong>Direct gap kept explicit:</strong> keychain-backed active operations remain out of this prototype build because the current frozen <code>qsc</code> shell contract only exposes the passphrase unlock path for sidecar-driven operations.
      </div>
    </section>
  `;
}

function setupPanelHtml(snapshot) {
  const peerValue = selectedPeerValue();
  const contactLabel = state.forms.contactLabel || peerValue;
  return `
    <section class="panel">
      <div class="panel-header">
        <div>
          <h2>Relay + Contact Setup</h2>
          <p class="panel-subtitle">Relay token, contact pinning, and device trust stay qsc-owned. The GUI refreshes state after each mutation instead of inferring success from prose.</p>
        </div>
      </div>
      <div class="two-up">
        <div class="form-grid">
          <div class="form-group">
            <label for="relay-url">Relay URL</label>
            <input id="relay-url" type="url" spellcheck="false" value="${escapeHtml(state.forms.relayUrl)}" placeholder="http://127.0.0.1:9455" />
          </div>
          <div class="form-group">
            <label for="inbox-token">Self Inbox Route Token</label>
            <input id="inbox-token" type="text" autocomplete="off" spellcheck="false" value="${escapeHtml(state.forms.inboxToken)}" placeholder="Opaque token stored by qsc" />
          </div>
          <div class="button-row">
            <button class="button-secondary" ${state.busy ? "disabled" : ""} data-action="inbox-set">Set inbox token</button>
            <button class="button-secondary" ${state.busy ? "disabled" : ""} data-action="refresh">Refresh status</button>
          </div>
        </div>
        <div class="form-grid">
          <div class="form-group">
            <label for="contact-label">Contact Label</label>
            <input id="contact-label" type="text" value="${escapeHtml(contactLabel)}" spellcheck="false" placeholder="bob" />
          </div>
          <div class="form-group">
            <label for="contact-fingerprint">Fingerprint</label>
            <input id="contact-fingerprint" type="text" value="${escapeHtml(state.forms.contactFingerprint)}" spellcheck="false" placeholder="identity_fp or verification code" />
          </div>
          <div class="form-group">
            <label for="contact-route-token">Peer Route Token</label>
            <input id="contact-route-token" type="text" value="${escapeHtml(state.forms.contactRouteToken)}" spellcheck="false" placeholder="Opaque peer inbox token" />
          </div>
          <div class="button-row">
            <button class="button-primary" ${state.busy ? "disabled" : ""} data-action="contact-add">Add or refresh contact</button>
          </div>
        </div>
      </div>
      <div class="footer-note">
        <strong>Fail-closed note:</strong> if the sidecar reports <code>protocol_inactive</code>, the prototype surfaces that exact block instead of inventing handshake/session semantics in JavaScript.
      </div>
    </section>
  `;
}

function contactsPanelHtml(snapshot) {
  const contacts = snapshot.contacts || [];
  if (contacts.length === 0) {
    return `
      <section class="panel">
        <div class="panel-header">
          <div>
            <h2>Contacts</h2>
            <p class="panel-subtitle">No unlocked contacts are visible yet.</p>
          </div>
        </div>
        <div class="empty-state">Unlock a passphrase-backed profile, add a contact, then select it to inspect devices and timeline state.</div>
      </section>
    `;
  }
  return `
    <section class="panel">
      <div class="panel-header">
        <div>
          <h2>Contacts</h2>
          <p class="panel-subtitle">Stable-sorted contact list from <code>qsc contacts list</code>.</p>
        </div>
      </div>
      <div class="list-grid">
        ${contacts
          .map((contact) => {
            const active = selectedPeerValue() === contact.label ? "active" : "";
            return `
              <article class="contact-item ${active}">
                <header>
                  <div>
                    <strong>${escapeHtml(contact.label)}</strong>
                    <div class="muted">${escapeHtml(contact.state)} • ${contact.blocked ? "blocked" : "not blocked"}</div>
                  </div>
                  <button class="button-secondary" ${state.busy ? "disabled" : ""} data-peer="${escapeHtml(contact.label)}" data-action="select-peer">Inspect</button>
                </header>
                <div class="contact-meta">
                  <div><span class="metric">Devices:</span> ${contact.device_count}</div>
                  <div><span class="metric">Primary device:</span> <span class="mono">${escapeHtml(contact.primary_device || "none")}</span></div>
                </div>
              </article>
            `;
          })
          .join("")}
      </div>
    </section>
  `;
}

function peerPanelHtml(snapshot) {
  const peerDetails = selectedPeerDetails();
  const selected = selectedPeerValue();
  const devices = peerDetails?.devices || [];
  const timeline = peerDetails?.timeline || [];
  const composePeer = state.forms.composePeer || selected;
  return `
    <section class="panel">
      <div class="panel-header">
        <div>
          <h2>Peer State</h2>
          <p class="panel-subtitle">Device trust and timeline state are refreshed from the sidecar for one selected peer at a time.</p>
        </div>
        <div class="tag-row">
          <span class="tag"><strong>Selected peer:</strong> ${escapeHtml(selected || "none")}</span>
          <span class="tag"><strong>Timeline items:</strong> ${timeline.length}</span>
        </div>
      </div>
      ${selected ? "" : '<div class="empty-state">Choose a contact to inspect device state and timeline markers.</div>'}
      ${selected ? `
        <div class="two-up">
          <div>
            <h3>Devices</h3>
            ${devices.length === 0 ? '<div class="empty-state">No device list available for this peer yet.</div>' : `
              <div class="list-grid">
                ${devices
                  .map(
                    (device) => `
                      <article class="contact-item">
                        <header>
                          <div>
                            <strong class="mono">${escapeHtml(device.device)}</strong>
                            <div class="muted">${escapeHtml(device.state)}</div>
                          </div>
                          <button class="button-secondary" ${state.busy ? "disabled" : ""} data-device="${escapeHtml(device.device)}" data-peer="${escapeHtml(selected)}" data-action="trust-device">Trust</button>
                        </header>
                      </article>
                    `
                  )
                  .join("")}
              </div>
            `}
          </div>
          <div>
            <h3>Timeline</h3>
            ${timeline.length === 0 ? '<div class="empty-state">No timeline entries have been surfaced for this peer yet.</div>' : `
              <div class="list-grid">
                ${timeline
                  .map(
                    (item) => `
                      <article class="timeline-item">
                        <header>
                          <strong class="mono">${escapeHtml(item.id)}</strong>
                          <span class="state-pill ${item.direction === "out" ? "ok" : "warn"}">${escapeHtml(item.direction)} • ${escapeHtml(item.state)}</span>
                        </header>
                        <div class="timeline-meta">
                          <div><span class="metric">Kind:</span> ${escapeHtml(item.kind)}</div>
                          <div><span class="metric">Timestamp:</span> ${escapeHtml(String(item.ts))}</div>
                        </div>
                      </article>
                    `
                  )
                  .join("")}
              </div>
            `}
          </div>
        </div>
      ` : ""}
      <div class="footer-note">
        Full transcript rendering remains out of scope. The prototype surfaces deterministic timeline state only.
      </div>
    </section>
  `;
}

function messagePanelHtml(snapshot) {
  const selected = selectedPeerValue();
  const receivedItems = state.receivedBatch || [];
  const deliveries = state.deliveryBatch || [];
  return `
    <section class="panel">
      <div class="panel-header">
        <div>
          <h2>Message Session</h2>
          <p class="panel-subtitle">Compose/send and poll/receive stay tied to the allowlisted <code>qsc send</code>, <code>qsc receive</code>, and <code>qsc timeline list</code> subset.</p>
        </div>
      </div>
      <div class="two-up">
        <div class="form-grid">
          <div class="form-group">
            <label for="compose-peer">Peer Label</label>
            <input id="compose-peer" type="text" value="${escapeHtml(composePeer)}" spellcheck="false" placeholder="bob" />
          </div>
          <div class="form-group">
            <label for="compose-message">Message Body</label>
            <textarea id="compose-message" placeholder="Compose one message. Attachments and transcript history remain out of scope.">${escapeHtml(state.forms.composeMessage)}</textarea>
          </div>
          <div class="button-row">
            <button class="button-primary" ${state.busy ? "disabled" : ""} data-action="send-message">Send message</button>
          </div>
          ${deliveries.length ? `
            <div class="notice ok">
              <strong>Latest delivery markers</strong>
              <div>${deliveries.map((delivery) => escapeHtml(delivery)).join("<br />")}</div>
            </div>
          ` : ""}
        </div>
        <div class="form-grid">
          <div class="form-group">
            <label for="receive-max">Receive Max</label>
            <input id="receive-max" type="number" min="1" max="16" value="${escapeHtml(state.forms.receiveMax)}" />
          </div>
          <div class="button-row">
            <button class="button-secondary" ${state.busy ? "disabled" : ""} data-action="receive-message">Poll and receive</button>
            <button class="button-secondary" ${state.busy ? "disabled" : ""} data-action="refresh">Refresh timeline</button>
          </div>
          ${receivedItems.length ? `
            <div class="list-grid">
              ${receivedItems
                .map(
                  (item) => `
                    <article class="receive-item">
                      <header>
                        <strong class="mono">${escapeHtml(item.file_name)}</strong>
                        <span class="state-pill ok">${escapeHtml(item.kind)}</span>
                      </header>
                      <div class="receive-meta">
                        <div><span class="metric">Byte length:</span> ${item.byte_len}</div>
                        <div class="message-preview">${escapeHtml(item.preview)}</div>
                      </div>
                    </article>
                  `
                )
                .join("")}
            </div>
          ` : '<div class="empty-state">No current receive batch. The UI only shows the latest session-local receive output, not a persistent transcript.</div>'}
        </div>
      </div>
      <div class="footer-note">
        The current validated local lane still requires protocol state to exist before delivery becomes truthful. If the sidecar returns <code>protocol_inactive</code>, the prototype reports that exact blocker instead of auto-falling back.
      </div>
    </section>
  `;
}

function scopePanelHtml(snapshot) {
  return `
    <section class="panel">
      <div class="panel-header">
        <div>
          <h2>Frozen Scope Guard</h2>
          <p class="panel-subtitle">This prototype stays inside the already-frozen shell boundary.</p>
        </div>
      </div>
      <div class="scope-list">
        <span class="scope-item"><strong>Included:</strong> single profile</span>
        <span class="scope-item"><strong>Included:</strong> passphrase unlock</span>
        <span class="scope-item"><strong>Included:</strong> contacts + device trust</span>
        <span class="scope-item"><strong>Included:</strong> timeline state</span>
        <span class="scope-item warn"><strong>Deferred:</strong> keychain-backed active operations</span>
        <span class="scope-item warn"><strong>Deferred:</strong> handshake/session-establish UI</span>
        <span class="scope-item danger"><strong>Out:</strong> attachments UI</span>
        <span class="scope-item danger"><strong>Out:</strong> transcript history UI</span>
        <span class="scope-item danger"><strong>Out:</strong> multiprofile</span>
        <span class="scope-item danger"><strong>Forbidden:</strong> shell passthrough</span>
      </div>
      <dl class="definition-list" style="margin-top: 18px;">
        <div>
          <dt>Bridge mode</dt>
          <dd>${escapeHtml(snapshot.sidecar_source)}</dd>
        </div>
        <div>
          <dt>Memory unlock</dt>
          <dd>${snapshot.session_unlocked ? "active" : "cleared"}</dd>
        </div>
        <div>
          <dt>Doctor symlink safety</dt>
          <dd>${snapshot.doctor.symlink_safe ? "pass" : "fail"}</dd>
        </div>
        <div>
          <dt>Doctor parent safety</dt>
          <dd>${snapshot.doctor.parent_safe ? "pass" : "fail"}</dd>
        </div>
      </dl>
    </section>
  `;
}

function render() {
  const snapshot = state.snapshot;
  if (!snapshot) {
    app.innerHTML = `
      <main class="shell">
        <section class="hero">
          <div>
            <div class="eyebrow">Desktop prototype boot</div>
            <h1>Loading qsc desktop state</h1>
            <p>The frontend is waiting for the Rust sidecar bridge to collect the current profile posture.</p>
          </div>
          <div class="hero-status">
            <div class="status-card"><div class="status-label">Status</div><div class="status-value">Connecting</div></div>
            <div class="status-card"><div class="status-label">Scope</div><div class="status-value">Message-first only</div></div>
          </div>
        </section>
      </main>
    `;
    return;
  }

  app.innerHTML = `
    <main class="shell">
      ${heroHtml(snapshot)}
      <div style="margin-top: 18px;">${noticeHtml(state.notice)}</div>
      <div class="layout">
        <div class="stack">
          ${profilePanelHtml(snapshot)}
          ${setupPanelHtml(snapshot)}
          ${contactsPanelHtml(snapshot)}
        </div>
        <div class="stack">
          ${peerPanelHtml(snapshot)}
          ${messagePanelHtml(snapshot)}
          ${scopePanelHtml(snapshot)}
        </div>
      </div>
    </main>
  `;

  document.querySelector("#passphrase")?.addEventListener("input", (event) => {
    onInput("passphrase", event.target.value);
  });
  document.querySelector("#relay-url")?.addEventListener("input", (event) => {
    onInput("relayUrl", event.target.value);
  });
  document.querySelector("#inbox-token")?.addEventListener("input", (event) => {
    onInput("inboxToken", event.target.value);
  });
  document.querySelector("#contact-label")?.addEventListener("input", (event) => {
    onInput("contactLabel", event.target.value);
  });
  document.querySelector("#contact-fingerprint")?.addEventListener("input", (event) => {
    onInput("contactFingerprint", event.target.value);
  });
  document.querySelector("#contact-route-token")?.addEventListener("input", (event) => {
    onInput("contactRouteToken", event.target.value);
  });
  document.querySelector("#compose-peer")?.addEventListener("input", (event) => {
    onInput("composePeer", event.target.value);
  });
  document.querySelector("#compose-message")?.addEventListener("input", (event) => {
    onInput("composeMessage", event.target.value);
  });
  document.querySelector("#receive-max")?.addEventListener("input", (event) => {
    onInput("receiveMax", event.target.value);
  });

  document.querySelectorAll("[data-action]").forEach((element) => {
    element.addEventListener("click", async () => {
      const action = element.getAttribute("data-action");
      const peer = element.getAttribute("data-peer");
      const device = element.getAttribute("data-device");

      if (action === "refresh") {
        state.receivedBatch = [];
        state.deliveryBatch = [];
        await refresh(state.selectedPeer || null);
        return;
      }

      if (action === "init-passphrase") {
        if (!state.forms.passphrase) {
          setNotice("warn", "Passphrase required", "Enter a passphrase before initializing a profile.");
          return;
        }
        await callAndRefresh(
          "init_passphrase_profile",
          {
            passphrase: state.forms.passphrase,
            selectedPeer: state.selectedPeer || null
          },
          {
            clearPassphrase: true,
            notice: {
              kind: "ok",
              title: "Passphrase profile initialized",
              detail: "Vault init and identity bootstrap completed through the sidecar."
            }
          }
        );
        return;
      }

      if (action === "unlock") {
        if (!state.forms.passphrase) {
          setNotice("warn", "Passphrase required", "Enter the profile passphrase before unlocking.");
          return;
        }
        await callAndRefresh(
          "unlock_profile",
          {
            passphrase: state.forms.passphrase,
            selectedPeer: state.selectedPeer || null
          },
          {
            clearPassphrase: true,
            notice: {
              kind: "ok",
              title: "Profile unlocked",
              detail: "The passphrase now lives only in backend memory for this desktop session."
            }
          }
        );
        return;
      }

      if (action === "lock") {
        await callAndRefresh(
          "clear_session_unlock",
          {
            selectedPeer: state.selectedPeer || null
          },
          {
            notice: {
              kind: "ok",
              title: "In-memory unlock cleared",
              detail: "No passphrase remains cached in the desktop backend session."
            }
          }
        );
        return;
      }

      if (action === "inbox-set") {
        await callAndRefresh(
          "set_inbox_token",
          {
            token: state.forms.inboxToken,
            selectedPeer: state.selectedPeer || null
          },
          {
            notice: {
              kind: "ok",
              title: "Inbox token updated",
              detail: "The self inbox route token was stored through qsc."
            }
          }
        );
        return;
      }

      if (action === "contact-add") {
        const label = state.forms.contactLabel || selectedPeerValue();
        await callAndRefresh(
          "add_contact",
          {
            label,
            fingerprint: state.forms.contactFingerprint,
            routeToken: state.forms.contactRouteToken || null,
            selectedPeer: label || null
          },
          {
            notice: {
              kind: "ok",
              title: "Contact updated",
              detail: "The sidecar contact store was refreshed and re-read."
            },
            selectPeer: label
          }
        );
        return;
      }

      if (action === "select-peer") {
        state.selectedPeer = peer || "";
        state.forms.composePeer = state.selectedPeer;
        await refresh(state.selectedPeer || null);
        return;
      }

      if (action === "trust-device") {
        await callAndRefresh(
          "trust_device",
          {
            label: peer,
            deviceId: device,
            selectedPeer: peer
          },
          {
            notice: {
              kind: "ok",
              title: "Device trusted",
              detail: "The selected device was pinned through the allowlisted qsc trust command."
            },
            selectPeer: peer
          }
        );
        return;
      }

      if (action === "send-message") {
        const label = state.forms.composePeer || selectedPeerValue();
        await callAndRefresh(
          "send_message",
          {
            relayUrl: state.forms.relayUrl,
            label,
            message: state.forms.composeMessage,
            selectedPeer: label || null
          },
          {
            notice: {
              kind: "ok",
              title: "Send completed",
              detail: "Delivery state was taken from sidecar markers only."
            },
            selectPeer: label
          }
        );
        return;
      }

      if (action === "receive-message") {
        const label = state.forms.composePeer || selectedPeerValue();
        await callAndRefresh(
          "receive_messages",
          {
            relayUrl: state.forms.relayUrl,
            label,
            maxItems: Number(state.forms.receiveMax || "4"),
            selectedPeer: label || null
          },
          {
            notice: {
              kind: "ok",
              title: "Receive cycle finished",
              detail: "Only the latest session-local receive files are shown here."
            },
            selectPeer: label
          }
        );
      }
    });
  });
}

refresh().catch((err) => {
  const mapped = mapError(err);
  setNotice("danger", mapped.title, mapped.detail);
});
