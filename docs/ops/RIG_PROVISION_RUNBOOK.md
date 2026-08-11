# RIG PROVISION RUNBOOK — the AWS relay — **a DELTA on `qsl-server`'s `packaging/runbook_ubuntu.md`**

**Written AS PERFORMED, 2026-08-11, by NA-0710 (D646, D-1347).** Every command below was executed in
the order shown; the verbatim command log with exit statuses lives in the lane record.

⚠ **THIS FILE IS A DELTA, NOT A REPLACEMENT.** `qsl-server` ships a 136-line production runbook at
`packaging/runbook_ubuntu.md` covering install, update, rollback, token rotation and verification.
**Those parts apply unchanged.** This file records only **what this deployment does differently and
why**, plus the traps that cost time. ⚠ **Authoring a parallel runbook would create two truths that
drift.**

⚠ **SUPERSEDES** `RIG_BRINGUP_RUNBOOK.md` (NA-0704), which documents the **retired laptop rig** —
a different machine, manual, unrecorded, and dead on shutdown. **None of its facts carry**: not the
host, not the OS, not the resources, and ⚠ **not the timings.**

---

# 0. ⚠⚠ FOUR NON-PROBLEMS THAT LOOK LIKE FAULTS — READ THIS FIRST

Each of these was met, diagnosed, and cost real time. **They are normal. Do not chase them.**

### 0.1 Expired certificates on a cold start are NORMAL
Caddy's `tls internal` leaves last **~12 hours** and renew only while caddy runs. A stack that has
been down overnight comes up with an expired leaf and **re-mints it from its own root on start**.
The root is valid to **2036**.

### 0.2 `ufw` reads as running and filters NOTHING
```
systemctl is-active ufw     ->  active      ⚠ MISLEADING
grep ENABLED /etc/ufw/ufw.conf  ->  ENABLED=no   <-- AUTHORITATIVE
```
⚠ A seat reading only `systemctl` concludes a host firewall is blocking 8443 and has **no authorized
remedy**. The conf file is world-readable, so this is answerable **without** the root-only rule set.

### 0.3 caddy logs `failed to install root certificate … sudo` at `level=error` on EVERY boot
This is **the hardening working.** Caddy tries to install its private CA into the *system* trust
store; the service user has no sudo, by design (`NoNewPrivileges=true`, `CapabilityBoundingSet=`).
⚠ **We do not want a private CA in the box's trust store** — clients pin it explicitly with
`qsc relay ca-set`. It recurs at every boot, forever.

### 0.4 ⚠ After a reboot, do NOT trust ssh returning — read the `boot_id`
```
cat /proc/sys/kernel/random/boot_id      # THE ARTIFACT — it must CHANGE
uptime                                   # only a summary
```
⚠ **Measured 2026-08-11:** a liveness poll reported *"ssh returned after 6s"* while the box was still
tearing down; the new boot began 19 s later. **"ssh answered" is not "the box rebooted."** Running
post-boot checks at that moment would have returned all-green **describing the pre-reboot state.**

⚠ **Related, same family:** `systemctl is-enabled ssh` returns **`disabled`** on Ubuntu 26.04 **and
ssh still survives a reboot** — sshd is **socket-activated**; the honest unit is **`ssh.socket`**.
And `systemctl is-active` returning `active` is **not** evidence a service is *serving*: measured
`active` with **no listener and no store** 27 ms after start. **Wait for the LISTENER.**

---

# 1. WHAT THIS DEPLOYMENT DOES DIFFERENTLY (the delta proper)

| # | the shipped runbook says | ⚠ this deployment does | why |
|---|---|---|---|
| 1 | build on the build host **or** target host | ⚠ **cross-build on the build box only** | the target has **no `gcc`, no toolchain, 908 MiB RAM and ZERO swap** — an on-box build is OOM-killed, not slow |
| 2 | `sudo bash scripts/install_ubuntu.sh …` | ⚠ **the shipped scripts are NOT run** | ① executing an opaque installer forfeits the recorded-recipe property this deployment exists for; ② the scripts were **unread** by the provisioning lane, and running unreviewed code with root is not authorized by implication. **Every act was performed explicitly and recorded verbatim.** |
| 3 | public ingress on 443 with automatic TLS | ⚠ **`tls internal` on 8443, SAN = the IP** | **port 80 is closed**, so ACME's HTTP-01 cannot complete. Clients pin the CA root |
| 4 | Caddy from the vendor package | ⚠ **the official static binary, published checksum verified** | a vendor APT repo **permanently widens the machine's trust base**; a one-time verified artifact does not. ⚠ **Caddy publishes SHA-512, not SHA-256** — `sha256sum -c` correctly refuses with *"no properly formatted checksum lines found"* |
| 5 | `MAX_BODY_BYTES` "as needed" (template 1 MiB) | ⚠ **65536 (64 KiB)** | see §3 — **the memory bound**, and it is the only available lever |
| 6 | — | ⚠ **`default_sni` in the Caddyfile** | see §2 |

**Everything else in `packaging/runbook_ubuntu.md` applies as written**, including token rotation (§7
there) and rollback (§6 there).

---

# 2. ⚠⚠ THE CADDYFILE — TWO CORRECTIONS OVER THE SHIPPED EXAMPLE

`packaging/caddy/Caddyfile.example` **cannot be used as-is** for this deployment, for two independent
reasons. **One of them is a defect in the example itself — see `ENG-0170`.**

```
{
	admin off
	auto_https disable_redirects
	default_sni 18.116.234.219      # (1)
}

https://18.116.234.219:8443 {
	encode gzip
	@relay_api path /v1/*
	log_skip @relay_api
	tls internal

	handle @relay_api {             # (2)
		reverse_proxy 127.0.0.1:8080
	}
	handle {
		respond 404
	}
}
```

**(1) `default_sni` — without it the endpoint serves NO CERTIFICATE.** A site keyed by an **IP** is
matched on SNI, and **RFC 6066 forbids an IP literal in SNI**, so a standards-compliant client sends
none and matches nothing. Measured symptom: **TLS alert 80, `no peer certificate available`.**

**(2) mutually-exclusive `handle` blocks.** ⚠ **In the shipped example a bare `handle { respond 404 }`
written AFTER `reverse_proxy` is ORDERED BEFORE it** — caddy sorts by **directive order, not file
order** — so it matches everything and **nothing is ever proxied.** This preserves the example's
intent (proxy `/v1/*`, 404 everything else) and fixes the ordering.

### 2.1 ⚠ HOW TO CHECK A CADDYFILE — `validate` IS NOT THE CHECK
```
caddy validate --config <file> --adapter caddyfile   # passes the BROKEN config
caddy adapt    --config <file> --adapter caddyfile   # <-- READ THE ROUTE TREE
```
⚠ **`validate` proves the config PARSES and ADAPTS. It says nothing about ROUTING.** The `adapt`
expansion must show `reverse_proxy` under a route **matched** on `/v1/*`, with `static_response`
**after** it. **The consumer that matters is a request.**

---

# 3. ⚠⚠ SIZING — RAM IS THE BINDING CONSTRAINT, NOT DISK

**The relay's peak memory for ONE pull is**
```
PEAK = max_queue_depth × max_body_bytes × 4.5703125
```
The factor is `serde_json` rendering `PullItem.data: Vec<u8>` as a **JSON array of integers**
(mean 2.5703125 digits + 1 separator per byte) **while the raw bytes are still resident**.

⚠ **At the shipped defaults that is `257 × 1 MiB × 4.5703125 = 1174.6 MiB` — on a 908 MiB box with
ZERO swap, from ONE authenticated pull, with no limit violated.**

**Two of the three knobs are unavailable:**
- `max_route_count` — buys **zero** memory (a pull is for one route). A **disk** lever only.
- `max_queue_depth` — ⚠ **PINNED at 257** by the attachment design (*"the exact-4-MiB attachment
  needs 256 chunks + 1 manifest"*). **Cutting it breaks attachment delivery.**
- ⭐ **`max_body_bytes` is the only load-bearing lever. Set to 65536.** Peak **73.4 MiB**; worst-case
  store falls from 64.25 GiB to **4.02 GiB** against 26 GiB free. ⚠ **Fixing RAM fixed disk as a
  side effect.**

⚠ **THE ≤8-CONCURRENT-PULL ASSUMPTION IS AN ASSUMPTION, NOT AN ENFORCEMENT** — the relay has **no
concurrency limit at all**. At 73.4 MiB per pull, 8 concurrent pulls ≈ 587 MiB.

⚠ **THE 64 KiB VALUE IS UNFALSIFIED, NOT VALIDATED.** Handshake frames measured **4 279 / 6 436 /
3 364 B**, far under it — but **no message envelope has ever been sent through this relay** (see §6),
so the limit is **untested against the payload class it was chosen to bound**. **128 KiB (peak
146.8 MiB) is the pre-approved fallback if `ERR_TOO_LARGE` ever fires.**

---

# 4. THE INSTALLED SHAPE (product paths, product unit)

| | |
|---|---|
| relay unit | ⚠ **the product's OWN `packaging/systemd/qsl-server.service`, byte-identical** — `ProtectSystem=strict`, `NoNewPrivileges`, `CapabilityBoundingSet=`, `SystemCallFilter=@system-service`, `MemoryDenyWriteExecute`, `StateDirectory=` |
| paths | `/opt/qsl-server/bin/qsl-server` · `/etc/qsl-server/relay.env` (**0600 root:root**) · `/var/lib/qsl-server/relay.db` |
| caddy | `/opt/caddy/bin/caddy` · `/etc/caddy/Caddyfile` · `/var/lib/caddy` (`StateDirectory=caddy`, `XDG_DATA_HOME` pinned there) |
| users | `qslrelay`, `qslcaddy` — `--system`, no home, `nologin`, no sudo |
| CA root for clients | `/var/lib/caddy/caddy/pki/authorities/local/root.crt` |

⚠ **The caddy unit is hand-authored** (the product ships none) and **mirrors every hardening directive
of the relay unit**. Three are deliberately omitted **with reasons recorded**: `EnvironmentFile=`
(caddy takes no env config), `ReadWritePaths=` (caddy writes only to its StateDirectory), and
`CAP_NET_BIND_SERVICE` (8443 is above 1024). ⚠ **Caddy 2.11.4 runs under `MemoryDenyWriteExecute` and
`SystemCallFilter=@system-service` — verified, not assumed.**

---

# 5. VERIFYING IT — THE ROWS THAT ACTUALLY PROVE SOMETHING

⚠ **"The process is running" is not the gate.** Run these from the **build box**:

```
# 1. no-SNI handshake must succeed (proves default_sni)
openssl s_client -connect <ip>:8443 -CAfile <root.crt>      # read "Cipher is", NOT the verify code

# 2. ⭐ THE ROW THAT MATTERS: unauthenticated must be 401 WITH BODY ERR_UNAUTHORIZED
curl --cacert <root.crt> -H 'X-QSL-Route-Token: <22-128 chars>' https://<ip>:8443/v1/pull?max=1

# 3. authenticated must be 200-class
curl --cacert <root.crt> -H 'Authorization: Bearer <token>' … 

# 4. a non-/v1 path must still 404
```

⚠⚠ **ROW 2's BODY IS THE EVIDENCE, NOT ITS STATUS CODE.** `ERR_UNAUTHORIZED` is the **relay's own
string** — a bare 401 could come from the edge; that body could only come from the relay. **It proves
the request traversed the proxy AND that auth is enforced end-to-end**, which no other single row
shows.

⚠ **Rows 2 and 3 are a PAIR.** A negative row without a positive one proves nothing; a positive pair
without row 4 cannot tell "fixed" from "broken differently."

⚠ **`openssl s_client` prints `Verify return code: 0 (ok)` even when the handshake FAILED** — it
printed identically on a certificate-less failure and on a success one hour apart. **Read `Cipher is`
and `no peer certificate available`.**

### 5.1 The token
Generated on the box into `/etc/qsl-server/relay.env` (**0600 root:root**). ⚠ **The value is never
echoed, logged, or written to any record** — path, mode, byte length and a sha256 **prefix** only.
⚠ **Auth must be proven ON before caddy is started**, over loopback: unauthenticated **401**,
authenticated **200-class**. **Starting the TLS front end in front of a token-less relay publishes an
open relay**, and `RELAY_TOKEN=` empty means *auth disabled*.

---

# 6. ⚠⚠ WHAT THIS RELAY HAS AND HAS NOT BEEN SHOWN TO DO

**PROVEN:** built from a named source rev · installed under the product's hardened unit ·
**enabled and returning unaided across a reboot** (19 s, store byte-identical, cert not re-minted) ·
reachable from the build box with the CA pinned · **auth enforced end-to-end through the TLS edge** ·
⚠ **the invite API is LIVE — `POST /v1/invite/create` returns a code, where the retired rig returned
HTTP 404.**

⚠⚠ **NOT PROVEN — AND THE REASON IS A CLIENT DEFECT, NOT A RELAY ONE: no message has ever been sent
through this relay.** The four invite steps all return rc 0 and move real frames, but **the party who
CREATES the invite never obtains a session**, so the two-party message exchange could not run. See
**ENG-0173**–**ENG-0176**. ⚠ **Do not read this runbook as saying the relay is unproven; read it as
saying the client flow above it does not complete.**

**NOT CLAIMED:** any security-group or network property (the operator's) · that the relay is safe to
expose publicly — ⚠ there is an **unauthenticated memory-exhaustion path no configuration can close**
(pre-auth body buffering + no concurrency limit + 908 MiB, zero swap) · that historical timings
transfer to this producer.

---

# 7. THE PRODUCER IDENTITY — cite this whenever a measurement is taken here

⚠ **Every "rig" fact predating 2026-08-10 describes the RETIRED laptop and does not carry.**

```
host / OS / kernel   : ip-172-31-46-89 / Ubuntu 26.04 LTS / 7.0.0-1010-aws
arch / CPU           : x86_64 / 2 vCPU Intel Xeon 8259CL @2.50GHz (KVM)
RAM / swap / disk    : 908 MiB / ⚠ ZERO / 28 GiB root, 26 GiB free  (⚠ /tmp is a 455 MiB RAM tmpfs)
relay source rev     : qsl-server 37ec82072cbbd68e4eaba83e192282fbcb96e5b4
relay binary sha256  : 3439aa04ef0e1b9a69d7d27fae1fb021cd8a48982c68a028dd0562f22bc9ce72
toolchain            : rustc 1.95.0   (qsl-server carries no rust-toolchain.toml)
build                : cross-built on the build box, x86_64-unknown-linux-gnu
                       ⚠ max required symbol GLIBC_2.34 vs the target's 2.43
caddy                : v2.11.4  sha256 b7105518e3ed1c0761f232e44fc09345535533c9cb0abf0e12809416c7ac64d9
connect host (SAN)   : 18.116.234.219  ⚠ IP SAN — requires default_sni
                       ⚠ EIP DEPENDENCY: with no Elastic IP, a stop/start invalidates the SAN
                          AND every client's relay URL
cert chain           : leaf ~12 h · intermediate 7 d · root 2036
store                : /var/lib/qsl-server/relay.db
                       ⚠ sha256 1fe8f611…1489 at 4096 B is the EMPTY-STORE fingerprint
limits               : MAX_BODY_BYTES=65536 (peak 73.4 MiB) · MAX_QUEUE_DEPTH=257 (PINNED)
capability set       : ["push_v1","pull_v1","pull_ack_lease_v1","invite_v1"]
⚠ concurrency        : ≤8 concurrent pulls ASSUMED, NOT ENFORCED
⚠ memory bounding    : caddy self-bounds (GOMEMLIMIT=857534054 from the cgroup).
                       THE RELAY DOES NOT.
⚠ out-of-band access : NONE. amazon-ssm-agent runs but NO IAM ROLE is attached (IMDS 404),
                       so SSM Session Manager is unavailable. A boot failure is recoverable
                       ONLY by the operator via the AWS console.
```

⚠⚠ **TIMINGS DO NOT TRANSFER.** Every wall-clock figure predating this producer was measured on the
retired laptop. Figures taken here: invite create 0.62 s · redeem 0.78 s · accept 1.03 s ·
finish 0.75 s. **Any comparison across producers must say that the producers differ.**

---

# 8. STOPPING THE STACK CLEANLY

```
sudo systemctl stop qsl-caddy      # the public edge first
sudo systemctl stop qsl-server
```
⚠ **Do not `kill -9`** — an abrupt kill risks leaving the SQLite WAL mid-write. Both units are
`enabled`, so a reboot brings them back unaided.

END OF RUNBOOK — if this line is missing, the copy is truncated; request a re-send.
