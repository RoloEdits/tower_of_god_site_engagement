use line_core::SeriesInfo;
use lore_olympus::config::{ChapterInfo, CommentSum};
use static_assertions::assert_eq_size_val;
use std::collections::VecDeque;
use std::path::Path;

pub fn write(
    path: &Path,
    chapter_info: &VecDeque<ChapterInfo>,
    series_info: &SeriesInfo,
    filename: &str,
) {
    let csv_name = format!("{filename}.csv");
    let mut writer = csv::Writer::from_path(path.join(csv_name)).unwrap();

    let header = [
        "title",
        "author",
        "genre",
        "status",
        "release_day",
        "views",
        "subscribers",
        "rating",
        "chapter",
        "chapter_length",
        "comments",
        "total_comments",
        "likes",
        "total_likes",
        "published",
        "user",
        "comment_body",
        "post_date",
        "upvotes",
        "downvotes",
        "reply_count",
        "scrape_date",
        "season",
    ];

    writer
        .write_record(header)
        .expect("Couldn't write to file.");

    let title = series_info.title.as_str();
    let author = series_info.author.as_str();
    let genre = series_info.genre.as_str();
    let status = series_info.status.as_str();
    let release_day = series_info.release_day.as_str();
    let views = series_info.views.to_string();
    let subscribers = series_info.subscribers.to_string();
    let rating = series_info.rating.to_string();
    let total_comments = chapter_info.sum_total_comments().to_string();
    let total_likes = series_info.sum_total_likes().to_string();

    for chapter in chapter_info {
        let season = chapter.season.to_string();
        let meaningful_chapter_number = chapter.meaningful_chapter_number.to_string();

        // let chapter_number = chapter.chapter_number.to_string();
        let comments = chapter.comments.to_string();
        let likes = chapter.likes.to_string();
        let date = chapter.date.as_str();
        let chapter_length = chapter.chapter_length.to_string();
        let current_utc_date = project_core::get_current_utc_date();

        for comment in &chapter.user_comments {
            let user = comment.user.as_ref().unwrap().as_str();
            let comment_body = comment.body.as_ref().unwrap().as_str();
            let post_date = comment.post_date.as_ref().unwrap().as_str();
            let upvotes = comment.upvotes.unwrap().to_string();
            let downvotes = comment.downvotes.unwrap().to_string();
            let reply_count = comment.reply_count.unwrap().to_string();

            let record_data = [
                title,
                author,
                genre,
                status,
                release_day,
                &views,
                &subscribers,
                &rating,
                &meaningful_chapter_number,
                &chapter_length,
                &comments,
                &total_comments,
                &likes,
                &total_likes,
                date,
                user,
                comment_body,
                post_date,
                &upvotes,
                &downvotes,
                &reply_count,
                &current_utc_date,
                &season,
            ];

            // Sanity check to make sure both header and record_data match in length
            assert_eq_size_val!(header, record_data);

            writer
                .write_record(record_data)
                .expect("Couldn't write to file.");
        }
    }

    writer.flush().expect("Couldn't flush buffer.");
}
