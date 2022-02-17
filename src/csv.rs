use {hyper::Client, hyper_tls::HttpsConnector, lazy_static::lazy_static, regex::Regex};

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct CovidInstance {
    pub num: u32,
    // pref_num: u32,
    // pref: String,
    pub date: String,
    pub location: String,
    pub age: String,
    pub gender: String,
}

lazy_static! {
    static ref CSV_LINE: Regex = Regex::new(
        // 176230,400009,福岡県,2022/02/11,金,久留米市,20代,男性,,,
        r"([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*),([^,]*)"
    ).expect("");
}

impl TryFrom<&str> for CovidInstance {
    type Error = ();
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        CSV_LINE
            .captures(s)
            .map(|csv| CovidInstance {
                num: csv[1].parse::<u32>().expect(""),
                date: csv[4].to_string(),
                location: csv[6].to_string(),
                age: csv[7].to_string(),
                gender: csv[8].to_string(),
            })
            .ok_or(())
    }
}

pub async fn load_csv() -> hyper::Result<Vec<CovidInstance>> {
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
                .skip(1)
                .filter(|s| 1 < s.len())
                .flat_map(CovidInstance::try_from)
                .collect::<Vec<CovidInstance>>());
        }
    }
    Ok(vec![])
}
