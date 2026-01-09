use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

pub struct ActorClient {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl ActorClient {
    pub fn spawn(path: &str) -> Result<Self, String> {
        let mut child = Command::new(path)
            .arg("--name")
            .arg("qshield-cli")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("spawn actor: {e}"))?;
        let stdin = child.stdin.take().ok_or_else(|| "actor stdin unavailable".to_string())?;
        let stdout = child.stdout.take().ok_or_else(|| "actor stdout unavailable".to_string())?;
        Ok(Self {
            child,
            stdin,
            stdout: BufReader::new(stdout),
            next_id: 1,
        })
    }

    pub fn call(&mut self, op: &str, params: Value) -> Result<Value, String> {
        let id = format!("{}", self.next_id);
        self.next_id += 1;
        let req = serde_json::json!({
            "id": id,
            "op": op,
            "params": params,
        });
        let line = serde_json::to_string(&req).map_err(|e| format!("encode request: {e}"))?;
        self.stdin
            .write_all(line.as_bytes())
            .and_then(|_| self.stdin.write_all(b"\n"))
            .map_err(|e| format!("write to actor: {e}"))?;
        self.stdin.flush().map_err(|e| format!("flush actor: {e}"))?;

        let mut resp_line = String::new();
        self.stdout
            .read_line(&mut resp_line)
            .map_err(|e| format!("read actor response: {e}"))?;
        if resp_line.trim().is_empty() {
            return Err("empty actor response".to_string());
        }
        let resp: Value = serde_json::from_str(&resp_line)
            .map_err(|e| format!("parse actor response: {e}"))?;
        let ok = resp.get("ok").and_then(|v| v.as_bool()).unwrap_or(false);
        if ok {
            resp.get("result")
                .cloned()
                .ok_or_else(|| "actor response missing result".to_string())
        } else {
            let err_obj = resp.get("error").cloned().unwrap_or_else(|| serde_json::json!({}));
            let msg = err_obj
                .get("message")
                .and_then(|v| v.as_str())
                .unwrap_or("actor error")
                .to_string();
            Err(msg)
        }
    }
}

impl Drop for ActorClient {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
