use {hyper::Client, hyper_tls::HttpsConnector, regex::Regex};

pub async fn load_csv() -> hyper::Result<Vec<String>> {
    let base = "https://ckan.open-governmentdata.org/dataset/401000_pref_fukuoka_covid19_patients";
    let target = Regex::new("https://ckan[^\"]+csv").expect("wrong regex");
    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let res = client.get(base.parse().expect("wrong url")).await?;
    let buf = hyper::body::to_bytes(res).await?;
    let str = String::from_utf8_lossy(buf.as_ref());
    for l in str.lines() {
        if let Some(url) = target.captures(l) {
            let res = client.get(url[0].parse().expect("wrong url")).await?;
            let buf = hyper::body::to_bytes(res).await?;
            return Ok(String::from_utf8_lossy(buf.as_ref())
                .split('\n')
                .map(|s| s.to_string())
                .collect::<Vec<String>>());
        }
    }
    Ok(vec![])
}
