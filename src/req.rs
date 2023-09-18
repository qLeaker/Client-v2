use std::io;
use std::io::Read;
use eframe::egui::TextBuffer;
use ureq::Response;
const INTO_STRING_LIMIT: usize = 256 * 1_024 * 1_024;

pub fn into_string(r: Response) -> io::Result<String> {
    #[cfg(feature = "charset")]
        let encoding = Encoding::for_label(self.charset().as_bytes())
        .or_else(|| Encoding::for_label(DEFAULT_CHARACTER_SET.as_bytes()))
        .unwrap();

    let mut buf: Vec<u8> = vec![];
    r.into_reader()
        .take((INTO_STRING_LIMIT + 1) as u64)
        .read_to_end(&mut buf)?;
    if buf.len() > INTO_STRING_LIMIT {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "response too big for into_string",
        ));
    }
    #[cfg(not(feature = "charset"))]
    {
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

pub fn into_image(r: Response) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    r.into_reader()
        .take((INTO_STRING_LIMIT + 1) as u64)
        .read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn request_get(url: &str) -> Option<String> {
    match ureq::get(url.as_str()).call() {
        Ok(response) => Some(into_string(response).unwrap()),

        _ => { return Some(String::new()) }
    }
}

pub fn request_get_image(url: String) -> Option<Vec<u8>> {
    match ureq::get(url.as_str()).call() {
        Ok(response) => Some(into_image(response).unwrap()),

        Err(e) => {  println!("Ошибка: {}", e); return Some(Vec::new()) }
    }
}
