use super::{Tool, ToolContext, ToolOutput};
use anyhow::Result;
use async_trait::async_trait;
use futures::StreamExt;
use reqwest::Url;
use serde::Deserialize;
use serde_json::{Value, json};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;

const MAX_SIZE: usize = 5 * 1024 * 1024; // 5MB
const DEFAULT_TIMEOUT: u64 = 30;
const MAX_TIMEOUT: u64 = 120;

pub struct WebFetchTool {
    client: reqwest::Client,
}

impl WebFetchTool {
    pub fn new() -> Self {
        Self {
            client: crate::provider::shared_http_client(),
        }
    }
}

#[derive(Deserialize)]
struct WebFetchInput {
    url: String,
    #[serde(default)]
    format: Option<String>,
    #[serde(default)]
    timeout: Option<u64>,
}

#[async_trait]
impl Tool for WebFetchTool {
    fn name(&self) -> &str {
        "webfetch"
    }

    fn description(&self) -> &str {
        "Fetch a URL."
    }

    fn parameters_schema(&self) -> Value {
        json!({
            "type": "object",
            "required": ["url"],
            "properties": {
                "intent": super::intent_schema_property(),
                "url": {
                    "type": "string",
                    "description": "URL."
                },
                "format": {
                    "type": "string",
                    "enum": ["text", "markdown", "html"],
                    "description": "Output format."
                },
                "timeout": {
                    "type": "integer",
                    "description": "Timeout in seconds."
                }
            }
        })
    }

    async fn execute(&self, input: Value, _ctx: ToolContext) -> Result<ToolOutput> {
        let params: WebFetchInput = serde_json::from_value(input)?;

        validate_webfetch_url(&params.url)?;

        let timeout = params.timeout.unwrap_or(DEFAULT_TIMEOUT).min(MAX_TIMEOUT);
        let format = params.format.as_deref().unwrap_or("markdown");

        let response = self
            .client
            .get(&params.url)
            .header(
                reqwest::header::USER_AGENT,
                "Mozilla/5.0 (compatible; JCode/1.0)",
            )
            .timeout(Duration::from_secs(timeout))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(anyhow::anyhow!("HTTP error: {}", status));
        }

        // Check content length
        if let Some(len) = response.content_length()
            && len as usize > MAX_SIZE
        {
            return Err(anyhow::anyhow!(
                "Response too large: {} bytes (max {} bytes)",
                len,
                MAX_SIZE
            ));
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        let mut body_bytes = Vec::new();
        let mut truncated = false;
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let remaining = MAX_SIZE.saturating_sub(body_bytes.len());
            if chunk.len() > remaining {
                body_bytes.extend_from_slice(&chunk[..remaining]);
                truncated = true;
                break;
            }
            body_bytes.extend_from_slice(&chunk);
        }

        let mut body = String::from_utf8_lossy(&body_bytes).into_owned();
        if truncated {
            body.push_str(&format!(
                "...\n\n(truncated, showing first {} bytes)",
                MAX_SIZE
            ));
        }

        // Format output
        let output = match format {
            "html" => body,
            "text" => html_to_text(&body),
            "markdown" => {
                if content_type.contains("text/html") {
                    html_to_markdown(&body)
                } else {
                    body
                }
            }
            _ => {
                if content_type.contains("text/html") {
                    html_to_markdown(&body)
                } else {
                    body
                }
            }
        };

        Ok(ToolOutput::new(format!(
            "Fetched {} ({} bytes)\n\n{}",
            params.url,
            output.len(),
            output
        )))
    }
}

fn allow_local_webfetch() -> bool {
    matches!(
        std::env::var("CONCLAVE_ALLOW_LOCAL_WEBFETCH")
            .or_else(|_| std::env::var("JCODE_ALLOW_LOCAL_WEBFETCH"))
            .ok()
            .as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
    )
}

fn validate_webfetch_url(raw_url: &str) -> Result<Url> {
    let url = Url::parse(raw_url)?;
    match url.scheme() {
        "http" | "https" => {}
        _ => return Err(anyhow::anyhow!("URL must start with http:// or https://")),
    }

    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("URL must include a host"))?;
    if !allow_local_webfetch() && is_local_or_private_host(host) {
        anyhow::bail!(
            "webfetch blocked local/private-network URL '{}'. Set CONCLAVE_ALLOW_LOCAL_WEBFETCH=1 only for trusted local fetches.",
            raw_url
        );
    }

    Ok(url)
}

fn is_local_or_private_host(host: &str) -> bool {
    let normalized = host.trim_end_matches('.').to_ascii_lowercase();
    let normalized = normalized
        .strip_prefix('[')
        .and_then(|host| host.strip_suffix(']'))
        .unwrap_or(&normalized);
    if normalized == "localhost"
        || normalized.ends_with(".localhost")
        || normalized.ends_with(".local")
    {
        return true;
    }

    match normalized.parse::<IpAddr>() {
        Ok(IpAddr::V4(addr)) => is_local_or_private_ipv4(addr),
        Ok(IpAddr::V6(addr)) => is_local_or_private_ipv6(addr),
        Err(_) => false,
    }
}

fn is_local_or_private_ipv4(addr: Ipv4Addr) -> bool {
    let octets = addr.octets();
    addr.is_loopback()
        || addr.is_private()
        || addr.is_link_local()
        || addr.is_unspecified()
        || addr.is_multicast()
        || addr.is_broadcast()
        || octets == [169, 254, 169, 254]
        || octets[0] == 0
}

fn is_local_or_private_ipv6(addr: Ipv6Addr) -> bool {
    let segments = addr.segments();
    addr.is_loopback()
        || addr.is_unspecified()
        || addr.is_multicast()
        || (segments[0] & 0xfe00) == 0xfc00
        || (segments[0] & 0xffc0) == 0xfe80
}

mod html_regex {
    use regex::Regex;
    use std::sync::OnceLock;

    fn compile_regex(pattern: &str, label: &str) -> Option<Regex> {
        match Regex::new(pattern) {
            Ok(regex) => Some(regex),
            Err(err) => {
                crate::logging::warn(&format!(
                    "webfetch: failed to compile static regex {label}: {}",
                    err
                ));
                None
            }
        }
    }

    macro_rules! static_regex {
        ($name:ident, $pat:expr_2021) => {
            pub fn $name() -> Option<&'static Regex> {
                static RE: OnceLock<Option<Regex>> = OnceLock::new();
                RE.get_or_init(|| compile_regex($pat, stringify!($name)))
                    .as_ref()
            }
        };
    }

    static_regex!(script, r"(?is)<script[^>]*>.*?</script>");
    static_regex!(style, r"(?is)<style[^>]*>.*?</style>");
    static_regex!(tag, r"<[^>]+>");
    static_regex!(whitespace, r"\n\s*\n\s*\n");
    static_regex!(link, r#"(?i)<a[^>]*href=["']([^"']+)["'][^>]*>([^<]*)</a>"#);
    static_regex!(strong, r"(?i)<(?:strong|b)>([^<]*)</(?:strong|b)>");
    static_regex!(em, r"(?i)<(?:em|i)>([^<]*)</(?:em|i)>");
    static_regex!(code, r"(?i)<code>([^<]*)</code>");
    static_regex!(pre_code, r"(?is)<pre[^>]*><code[^>]*>(.+?)</code></pre>");
    static_regex!(li, r"(?i)<li[^>]*>");

    static H_OPEN: OnceLock<Option<[Regex; 6]>> = OnceLock::new();
    static H_CLOSE: OnceLock<Option<[Regex; 6]>> = OnceLock::new();

    pub fn h_open() -> Option<&'static [Regex; 6]> {
        H_OPEN
            .get_or_init(|| {
                let mut compiled = Vec::with_capacity(6);
                for i in 0..6 {
                    let pattern = format!(r"(?i)<h{}[^>]*>", i + 1);
                    compiled.push(compile_regex(&pattern, "heading open")?);
                }
                compiled.try_into().ok()
            })
            .as_ref()
    }

    pub fn h_close() -> Option<&'static [Regex; 6]> {
        H_CLOSE
            .get_or_init(|| {
                let mut compiled = Vec::with_capacity(6);
                for i in 0..6 {
                    let pattern = format!(r"(?i)</h{}>", i + 1);
                    compiled.push(compile_regex(&pattern, "heading close")?);
                }
                compiled.try_into().ok()
            })
            .as_ref()
    }
}

fn html_to_text(html: &str) -> String {
    let mut text = html.to_string();

    let (Some(script), Some(style), Some(tag), Some(whitespace)) = (
        html_regex::script(),
        html_regex::style(),
        html_regex::tag(),
        html_regex::whitespace(),
    ) else {
        return html.trim().to_string();
    };

    text = script.replace_all(&text, "").to_string();
    text = style.replace_all(&text, "").to_string();

    text = text.replace("<br>", "\n");
    text = text.replace("<br/>", "\n");
    text = text.replace("<br />", "\n");
    text = text.replace("</p>", "\n\n");
    text = text.replace("</div>", "\n");
    text = text.replace("</li>", "\n");
    text = text.replace("</tr>", "\n");

    text = tag.replace_all(&text, "").to_string();

    text = text.replace("&nbsp;", " ");
    text = text.replace("&lt;", "<");
    text = text.replace("&gt;", ">");
    text = text.replace("&amp;", "&");
    text = text.replace("&quot;", "\"");
    text = text.replace("&#39;", "'");

    text = whitespace.replace_all(&text, "\n\n").to_string();

    text.trim().to_string()
}

fn html_to_markdown(html: &str) -> String {
    let mut md = html.to_string();

    let (
        Some(script),
        Some(style),
        Some(link),
        Some(strong),
        Some(em),
        Some(code),
        Some(pre_code),
        Some(li),
        Some(tag),
        Some(whitespace),
    ) = (
        html_regex::script(),
        html_regex::style(),
        html_regex::link(),
        html_regex::strong(),
        html_regex::em(),
        html_regex::code(),
        html_regex::pre_code(),
        html_regex::li(),
        html_regex::tag(),
        html_regex::whitespace(),
    )
    else {
        return html.trim().to_string();
    };

    md = script.replace_all(&md, "").to_string();
    md = style.replace_all(&md, "").to_string();

    if let (Some(h_open), Some(h_close)) = (html_regex::h_open(), html_regex::h_close()) {
        for i in 0..6 {
            let prefix = "#".repeat(i + 1);
            md = h_open[i]
                .replace_all(&md, &format!("\n{} ", prefix))
                .to_string();
            md = h_close[i].replace_all(&md, "\n").to_string();
        }
    }

    md = link.replace_all(&md, "[$2]($1)").to_string();
    md = strong.replace_all(&md, "**$1**").to_string();
    md = em.replace_all(&md, "*$1*").to_string();
    md = code.replace_all(&md, "`$1`").to_string();
    md = pre_code.replace_all(&md, "\n```\n$1\n```\n").to_string();
    md = li.replace_all(&md, "\n- ").to_string();

    md = md.replace("<br>", "\n");
    md = md.replace("<br/>", "\n");
    md = md.replace("<br />", "\n");
    md = md.replace("</p>", "\n\n");

    md = tag.replace_all(&md, "").to_string();

    md = md.replace("&nbsp;", " ");
    md = md.replace("&lt;", "<");
    md = md.replace("&gt;", ">");
    md = md.replace("&amp;", "&");
    md = md.replace("&quot;", "\"");
    md = md.replace("&#39;", "'");

    md = whitespace.replace_all(&md, "\n\n").to_string();

    md.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webfetch_allows_public_https_url() {
        assert!(validate_webfetch_url("https://example.com/page").is_ok());
    }

    #[test]
    fn webfetch_rejects_localhost() {
        let err = validate_webfetch_url("http://localhost:3000").unwrap_err();
        assert!(err.to_string().contains("blocked local/private-network"));
    }

    #[test]
    fn webfetch_rejects_private_ipv4() {
        let err = validate_webfetch_url("http://192.168.1.10/status").unwrap_err();
        assert!(err.to_string().contains("blocked local/private-network"));
    }

    #[test]
    fn webfetch_rejects_link_local_metadata_ip() {
        let err = validate_webfetch_url("http://169.254.169.254/latest/meta-data").unwrap_err();
        assert!(err.to_string().contains("blocked local/private-network"));
    }

    #[test]
    fn webfetch_rejects_unique_local_ipv6() {
        let err = validate_webfetch_url("http://[fd00::1]/status").unwrap_err();
        assert!(err.to_string().contains("blocked local/private-network"));
    }

    #[test]
    fn webfetch_rejects_local_domains() {
        let err = validate_webfetch_url("https://service.local/").unwrap_err();
        assert!(err.to_string().contains("blocked local/private-network"));
    }
}
