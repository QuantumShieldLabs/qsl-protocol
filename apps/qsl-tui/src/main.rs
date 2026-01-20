use anyhow::Result;
use clap::{Parser, ValueEnum};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::{io, time::Duration};

use qsl_tui::demo::{run_demo, Mode, PrivacyMode};

#[derive(Parser, Debug)]
#[command(author, version, about = "QSL Linux TUI demo client")]
struct Args {
    #[arg(long, value_enum, default_value = "local")]
    mode: RunMode,

    #[arg(long)]
    relay_base_url: Option<String>,

    #[arg(long)]
    relay_channel: Option<String>,

    #[arg(long)]
    headless: bool,

    #[arg(long, value_enum, default_value = "basic")]
    privacy_mode: PrivacyRunMode,
}

#[derive(ValueEnum, Clone, Debug)]
enum RunMode {
    Local,
    Relay,
}

#[derive(ValueEnum, Clone, Debug)]
enum PrivacyRunMode {
    Basic,
    Padded,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    let base_url = args
        .relay_base_url
        .or_else(|| std::env::var("QSL_RELAY_BASE_URL").ok())
        .unwrap_or_else(|| "http://127.0.0.1:8080".to_string());
    let channel = args
        .relay_channel
        .or_else(|| std::env::var("QSL_RELAY_CHANNEL").ok())
        .unwrap_or_else(|| "demo".to_string());

    let mode = match args.mode {
        RunMode::Local => Mode::Local,
        RunMode::Relay => Mode::Relay,
    };

    let privacy_mode = match args.privacy_mode {
        PrivacyRunMode::Basic => PrivacyMode::Basic,
        PrivacyRunMode::Padded => PrivacyMode::Padded,
    };

    let result = run_demo(mode, &base_url, &channel, privacy_mode).await;
    if args.headless {
        let meta_note = match args.privacy_mode {
            PrivacyRunMode::Basic => "content_encrypted=true metadata_exposed=channel,timing,packet_size,ip mitigation=none",
            PrivacyRunMode::Padded => {
                "content_encrypted=true metadata_exposed=channel,timing,ip mitigation=padding_buckets_only"
            }
        };
        println!(
            "QSL_TUI_HEADLESS_START mode={:?} base_url={} channel={}",
            args.mode, base_url, channel
        );
        match result {
            Ok(out) => {
                println!(
                    "QSL_TUI_HEADLESS_PAD plain={} padded={} bucket={}",
                    out.padding.plain_len, out.padding.padded_len, out.padding.bucket
                );
                println!(
                    "QSL_TUI_META plaintext_len={} ciphertext_len={} bucket={} mode={:?}",
                    out.padding.plain_len,
                    out.ciphertext_len,
                    out.padding.bucket,
                    args.privacy_mode
                );
                println!("QSL_TUI_META_NOTE {meta_note}");
                println!("QSL_TUI_HEADLESS_OK plaintext={}", out.plaintext);
                return Ok(());
            }
            Err(e) => {
                eprintln!("QSL_TUI_HEADLESS_ERR error={e}");
                return Err(e);
            }
        }
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut running = true;
    while running {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(2),
                ])
                .split(size);

            let title = Paragraph::new(Line::from(vec![Span::styled(
                "QSL Linux TUI Demo",
                Style::default(),
            )]))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("qsl-tui"));
            f.render_widget(title, chunks[0]);

            let mode_line = format!(
                "mode={:?} privacy={:?} base_url={} channel={}",
                args.mode, args.privacy_mode, base_url, channel
            );
            let info = Paragraph::new(mode_line)
                .block(Block::default().borders(Borders::ALL).title("config"));
            f.render_widget(info, chunks[1]);

            let body = match &result {
                Ok(msg) => format!(
                    "demo result: plaintext={} plain_len={} ciphertext_len={} bucket={} mode={:?}",
                    msg.plaintext,
                    msg.padding.plain_len,
                    msg.ciphertext_len,
                    msg.padding.bucket,
                    args.privacy_mode
                ),
                Err(e) => format!("error: {e}"),
            };
            let output = Paragraph::new(body)
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("output"));
            f.render_widget(output, chunks[2]);

            let footer = Paragraph::new("Press q or Enter to exit")
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[3]);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Enter | KeyCode::Esc => running = false,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(LeaveAlternateScreen)?;
    Ok(())
}
