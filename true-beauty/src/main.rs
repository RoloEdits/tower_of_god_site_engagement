use clap::Parser;
use cli_core::StoryCliArgs;
use project_core::enforce_path_exists;
use true_beauty::config;

mod csv;

#[tokio::main]
async fn main() {
    let args = StoryCliArgs::parse();

    let (series_info, parsed_chapters) = true_beauty::parse_chapters(
        args.start,
        args.end,
        args.pages,
        &config::CONFIG,
        config::TO_SKIP,
    )
    .await;

    csv::write(
        &enforce_path_exists(&args.output),
        &parsed_chapters,
        &series_info,
        config::CONFIG.filename,
    );
}
