pub static FAVICON_ICO: &'static [u8; 4286] = include_bytes!("config/default/favicon.ico");
pub const FAVICON_ICO_PATH: &'static str = "/favicon.ico";
pub static CSS: &'static str = include_str!("config/default/style.css");
pub const CSS_PATH: &'static str = "/style.css";

pub static CNS: [(u32, &'static str); 3] =
    [(200, "OK"), (404, "Not Found"), (500, "Internal Server Error")];//服务器内部错误,服务器(用户)权限不够

// http://tools.jb51.net/table/http_content_type  后缀-格式
// https://www.sitepoint.com/web-foundations/mime-types-complete-list/
// 常用的应该就这么多，再多应该移到配置文件。
pub static ETS: [(&'static str, &'static str); 59] = [("*", "application/octet-stream"),
                                                      ("txt", "text/plain;charset=utf-8"),
                                                      ("lrc", "text/plain;charset=utf-8"),
                                                      ("c", "text/plain;charset=utf-8"),
                                                      ("py", "text/plain;charset=utf-8"),
                                                      ("jl", "text/plain;charset=utf-8"),
                                                      ("toml", "text/plain;charset=utf-8"),
                                                      ("lock", "text/plain;charset=utf-8"),
                                                      ("rs", "text/plain;charset=utf-8"),
                                                      ("text", "text/plain;charset=utf-8"),
                                                      ("css", "text/css;charset=utf-8"),
                                                      ("js", "text/javascript;charset=utf-8"),
                                                      ("json", "application/json;charset=utf-8"),
                                                      ("htm", "text/html;charset=utf-8"),
                                                      ("html", "text/html;charset=utf-8"),
                                                      ("xhtml", "text/html;charset=utf-8"),
                                                      ("xml", "application/xml;charset=utf-8"),
                                                      ("svg", "text/xml;charset=utf-8"),
                                                      ("ps", "postscript"),
                                                      ("pdf", "application/pdf"),
                                                      ("xls", "application/vnd.ms-excel"),
                                                      ("doc", "application/msword"),
                                                      ("ppt", "application/vnd.ms-powerpoint"),
                                                      ("ico", "image/x-icon"),
                                                      ("jpg", "image/jpeg"),
                                                      ("jpeg", "image/jpeg"),
                                                      ("png", "image/png"),
                                                      ("apng", "image/png"),
                                                      ("webp", "image/webp"),
                                                      ("m3u", "audio/mpegurl"),
                                                      ("m3u8", "application/x-mpegURL"),
                                                      ("midi", "audio/mid"),
                                                      ("mid", "audio/mid"),
                                                      ("aif", "audio/aiff"),
                                                      ("aiff", "audio/aiff"),
                                                      ("flac", "audio/flac"),
                                                      ("mp2", "audio/mp2"),
                                                      ("mp3", "audio/mp3"),
                                                      ("ogg", "audio/ogg"),
                                                      ("aac", "audio/aac"),
                                                      ("wav", "audio/wav"),
                                                      ("wma", "audio/x-ms-wma"),
                                                      ("avi", "video/avi"),
                                                      ("3gp", "video/3gpp"),
                                                      ("ts", "video/MP2T"),
                                                      ("mp4", "video/mp4"),
                                                      ("mpg", "video/mpg"),
                                                      ("mpeg", "video/mpg"),
                                                      ("webm", "video/webm"),
                                                      ("mkv", "video/x-matroska"),
                                                      ("wmv", "video/x-ms-wmv"),
                                                      ("mov", "video/quicktime"),
                                                      ("swf", "application/x-shockwave-flash"),
                                                      ("flv", "video/x-flv"),
                                                      ("7z", "application/x-7z-compressed"),
                                                      ("zip", "application/zip"),
                                                      ("gzip", "application/gzip"),
                                                      ("rar", "application/x-rar-compressed"),
                                                      ("iso", "application/iso-image")];
