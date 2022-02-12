use {
    hyper::{body::Bytes, Client},
    hyper_tls::HttpsConnector,
    regex::Regex,
};

pub async fn load_csv() -> hyper::Result<Vec<String>> {
    let base = "https://ckan.open-governmentdata.org/dataset/401000_pref_fukuoka_covid19_patients";
    let target = Regex::new("https://ckan[^\"]+csv").expect("wrong regex");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(base.parse().expect("wrong url")).await?;
    let buf: Bytes = hyper::body::to_bytes(res).await?;
    let str = String::from_utf8_lossy(buf.as_ref());
    for l in str.lines() {
        if let Some(rurl) = target.captures(l) {
            let url = &rurl[0];
            dbg!(&url);
            let res = client.get(url.parse().expect("wrong url")).await?;
            let buf = hyper::body::to_bytes(res).await?;
            let str = String::from_utf8_lossy(buf.as_ref());
            dbg!(
                &str.split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[0..2]
            );
            return Ok(str
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        }
    }
    Ok(vec![])
}
