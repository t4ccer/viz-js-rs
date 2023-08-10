use viz_js::VizInstance;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen_futures::spawn_local;

fn main() {
    spawn_local(run())
}

async fn run() {
    let graphviz = VizInstance::new().await;
    let svg = graphviz
        .render_svg_element("digraph {a -> b;}".to_string(), viz_js::Options::default())
        .expect("Could not render graphviz");

    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    document
        .body()
        .unwrap_throw()
        .set_inner_html(&svg.outer_html());
}
