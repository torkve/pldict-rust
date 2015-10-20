extern crate xdxf;
extern crate iron;
extern crate router;
extern crate rustc_serialize;

use rustc_serialize::json;
use xdxf::Xdxf;
use std::sync::Arc;
use std::collections::HashMap;
use std::iter::Extend;

static POL_RUS: &'static str = include_str!("../edicts/polrus/dict.xdxf");
static RUS_POL: &'static str = include_str!("../edicts/ruspol/dict.xdxf");

static CSS: &'static str = include_str!("../xdxf.css");
static CSS_SUGGEST: &'static str = include_str!("../jquery.autocomplete.css");
static JS_SUGGEST: &'static str = include_str!("../jquery.autocomplete.min.js");
static JS: &'static str = include_str!("../xdxf-dict.js");

macro_rules! HTML {() => ("<html>
    <head>
        <title>Słownik polsko-rosyjski i rosyjsko-polski</title>
        <style type='text/css'>{}</style>
        <style type='text/css'>{}</style>
        <meta name='viewport' content='user-scalable=no, width=device-width' />
        <script type='application/javascript' src='http://ajax.googleapis.com/ajax/libs/jquery/1.7.1/jquery.js'></script>
        <script type='application/javascript'>{}</script>
        <script type='application/javascript'>{}</script>
    </head>
    <body>
        <h1>Zrób z tego dobry użytek: </h1>
        <p>
            <input placeholder='Wpisz słowo' id='req' autofocus='autofocus' type='text' name='req' value='' />
            <input id='search' type='button' name='search' value='Szukaj' />
        </p>
        <div id='searchresults'></div> 
    </body>
</html>")}

fn main() {
    let mut dict = Xdxf::load_str(RUS_POL).map_err(|e| panic!("Cannot parse dictionary {:?}", e)).unwrap();
    dict.feed_str(POL_RUS).map_err(|e| panic!("Cannot parse dictionary {:?}", e)).unwrap();
    let dict = Arc::new(dict);

    let mainpage = format!(HTML!(), CSS, CSS_SUGGEST, JS_SUGGEST, JS);
    let mut router = router::Router::new();
    {
        let dict = dict.clone();
        router.get("/", move |req: &mut iron::Request| {
            let query = req.url.clone().into_generic_url().query_pairs().unwrap_or(Vec::new());
            let req: Option<&(String, String)> = query.iter().filter(|kv| kv.0 == "req").next();
            match req {
                None => {
                    Ok(iron::Response::with((
                                iron::status::Ok,
                                iron::mime::Mime(iron::mime::TopLevel::Text, iron::mime::SubLevel::Html, vec!((iron::mime::Attr::Charset, iron::mime::Value::Utf8))),
                                mainpage.trim(),
                                )))
                },
                Some(kv) => {
                    let lookup = dict.lookup(kv.1.trim_left());
                    let mut result = HashMap::new();
                    result.extend(lookup);
                    let js = json::encode(&result).unwrap();
                    Ok(iron::Response::with((
                                iron::status::Ok,
                                iron::mime::Mime(iron::mime::TopLevel::Application, iron::mime::SubLevel::Json, Vec::new()),
                                js.trim()
                                )))
                }
            }
        });
    }
    println!("Dictionary parsed!");
    iron::Iron::new(router).http("localhost:3000").unwrap();
}
