use std::process::Command;

#[derive(Clone)]
pub struct ConnectionInfo {
    pub protocol: String,      // TCP, UDP
    pub local_addr: String,    // local address:port
    pub remote_addr: String,   // remote address:port (or * for listening)
    pub state: String,         // LISTEN, ESTABLISHED, etc.
    pub pid: Option<u32>,
    pub process_name: Option<String>,
}

#[derive(Default)]
pub struct ConnectionData {
    pub connections: Vec<ConnectionInfo>,
    pub listening_ports: Vec<ConnectionInfo>,
}

impl ConnectionData {
    pub fn update(&mut self) {
        self.connections.clear();
        self.listening_ports.clear();

        #[cfg(target_os = "macos")]
        self.update_macos();

        #[cfg(target_os = "linux")]
        self.update_linux();
    }

    #[cfg(target_os = "macos")]
    fn update_macos(&mut self) {
        // Use netstat on macOS
        if let Ok(output) = Command::new("netstat")
            .args(["-anp", "tcp"])
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                self.parse_netstat_output(&stdout, "TCP");
            }
        }

        if let Ok(output) = Command::new("netstat")
            .args(["-anp", "udp"])
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                self.parse_netstat_output(&stdout, "UDP");
            }
        }
    }

    #[cfg(target_os = "macos")]
    fn parse_netstat_output(&mut self, output: &str, protocol: &str) {
        // macOS netstat format:
        // Active Internet connections (including servers)
        // Proto Recv-Q Send-Q  Local Address          Foreign Address        (state)
        // tcp4       0      0  127.0.0.1.631          *.*                    LISTEN

        for line in output.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let local_addr = parts[3].to_string();
                let remote_addr = parts[4].to_string();
                let state = if parts.len() >= 6 {
                    parts[5].to_string()
                } else {
                    "".to_string()
                };

                let conn = ConnectionInfo {
                    protocol: protocol.to_string(),
                    local_addr: format_macos_addr(&local_addr),
                    remote_addr: format_macos_addr(&remote_addr),
                    state: state.clone(),
                    pid: None,
                    process_name: None,
                };

                if state == "LISTEN" || remote_addr == "*.*" {
                    self.listening_ports.push(conn);
                } else if state == "ESTABLISHED" || state == "CLOSE_WAIT" || state == "TIME_WAIT" {
                    self.connections.push(conn);
                }
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn update_linux(&mut self) {
        // Use ss command on Linux (faster than netstat)
        if let Ok(output) = Command::new("ss")
            .args(["-tunapH"])
            .output()
        {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                self.parse_ss_output(&stdout);
            }
        }
    }

    #[cfg(target_os = "linux")]
    fn parse_ss_output(&mut self, output: &str) {
        // ss -tunapH format:
        // tcp   LISTEN     0      128     0.0.0.0:22     0.0.0.0:*     users:(("sshd",pid=1234,fd=3))

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let protocol = parts[0].to_uppercase();
                let state = parts[1].to_string();
                let local_addr = parts[4].to_string();
                let remote_addr = if parts.len() > 5 { parts[5].to_string() } else { "*:*".to_string() };

                // Try to extract PID and process name
                let (pid, process_name) = if parts.len() > 6 {
                    parse_ss_process_info(parts[6])
                } else {
                    (None, None)
                };

                let conn = ConnectionInfo {
                    protocol,
                    local_addr,
                    remote_addr: remote_addr.clone(),
                    state: state.clone(),
                    pid,
                    process_name,
                };

                if state == "LISTEN" {
                    self.listening_ports.push(conn);
                } else if state == "ESTAB" || state == "CLOSE-WAIT" || state == "TIME-WAIT" {
                    self.connections.push(conn);
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn format_macos_addr(addr: &str) -> String {
    // Convert macOS format "127.0.0.1.8080" to "127.0.0.1:8080"
    if addr == "*.*" {
        return "*:*".to_string();
    }

    // Find the last dot - that separates the port
    if let Some(last_dot) = addr.rfind('.') {
        let (ip_part, port) = addr.split_at(last_dot);
        format!("{}:{}", ip_part, &port[1..])
    } else {
        addr.to_string()
    }
}

#[cfg(target_os = "linux")]
fn parse_ss_process_info(info: &str) -> (Option<u32>, Option<String>) {
    // Parse users:(("sshd",pid=1234,fd=3))
    let mut pid = None;
    let mut name = None;

    if let Some(start) = info.find("((\"") {
        if let Some(end) = info[start + 3..].find("\"") {
            name = Some(info[start + 3..start + 3 + end].to_string());
        }
    }

    if let Some(pid_start) = info.find("pid=") {
        let after_pid = &info[pid_start + 4..];
        if let Some(pid_end) = after_pid.find(|c: char| !c.is_numeric()) {
            pid = after_pid[..pid_end].parse().ok();
        }
    }

    (pid, name)
}
