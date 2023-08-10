//! Rust bindings for viz-js

#![warn(missing_docs)]

use js_sys::Array;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue, UnwrapThrowExt};
use web_sys::SvgsvgElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen (extends = js_sys::Object, js_name = Window)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    type WindowWithViz;

    /// Graphviz instance
    #[wasm_bindgen (extends = js_sys::Object, js_name = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type VizInstance;

    #[wasm_bindgen(method, js_name = instance)]
    async fn instance_raw(viz: &Viz) -> JsValue;
}

/// Graphviz renderer options
#[wasm_bindgen(getter_with_clone)]
pub struct Options {
    /// Output format. See [VizInstance::formats] for available options
    pub format: String,

    /// Graphviz engine. See [VizInstance::engines] for available options
    pub engine: String,

    /// Invert y coordinates in output
    #[wasm_bindgen(js_name = yInvert)]
    pub y_invert: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            format: "dot".to_string(),
            engine: "dot".to_string(),
            y_invert: false,
        }
    }
}

#[wasm_bindgen(module = "/js/viz-standalone.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn graphviz_dummy_hack() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = window, js_name = Viz, getter, method)]
    fn viz(this: &WindowWithViz) -> Viz;

    #[wasm_bindgen (extends = js_sys::Object, js_name = Viz)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    type Viz;

    /// Get current Graphviz version
    #[wasm_bindgen(js_name = graphvizVersion, getter, method)]
    pub fn graphviz_version(viz_instance: &VizInstance) -> String;

    #[wasm_bindgen(js_name = engines, getter, method)]
    fn engines_raw(viz_instance: &VizInstance) -> Array;

    #[wasm_bindgen(js_name = formats, getter, method)]
    fn formats_raw(viz_instance: &VizInstance) -> Array;

    #[wasm_bindgen(js_name = renderString, method, catch)]
    fn render_string_raw(
        viz_instance: &VizInstance,
        src: String,
        options: Options,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = renderSVGElement, method, catch)]
    fn render_svg_element_raw(
        viz_instance: &VizInstance,
        src: String,
        options: Options,
    ) -> Result<JsValue, JsValue>;

    /// Render to JSON. [Options::format] is always `"json"`
    #[wasm_bindgen(js_name = renderJSON, method, catch)]
    pub fn render_json(
        viz_instance: &VizInstance,
        src: String,
        options: Options,
    ) -> Result<JsValue, JsValue>;
}

impl Viz {
    async fn instance(&self) -> VizInstance {
        // HACK: I have no idea why it's needed here. This function does nothing. But when
        // you remove this call, it stops working and `undefined`s appear everywhere. Maybe some
        // wasm-bindgen quirk that I don't understand.
        let _ = graphviz_dummy_hack().await;

        Viz::instance_raw(self)
            .await
            .dyn_into::<VizInstance>()
            .expect_throw("Could not intialize Graphviz")
    }
}

impl VizInstance {
    /// Create new Graphviz instance.
    pub async fn new() -> VizInstance {
        js_sys::global()
            .dyn_into::<WindowWithViz>()
            .expect_throw("Could not intialize Graphviz")
            .viz()
            .instance()
            .await
    }

    /// List available layout engines
    pub fn engines(&self) -> Vec<String> {
        VizInstance::engines_raw(self)
            .into_iter()
            .map(|js| js.as_string().expect_throw("Engine name is not a string"))
            .collect()
    }

    /// List available rendering image formats
    pub fn formats(&self) -> Vec<String> {
        VizInstance::formats_raw(self)
            .into_iter()
            .map(|js| js.as_string().expect_throw("Format is not a string"))
            .collect()
    }

    /// Render to a string
    pub fn render_string(&self, src: String, options: Options) -> Result<String, JsValue> {
        VizInstance::render_string_raw(self, src, options).map(|res| {
            res.as_string()
                .expect_throw("Rendered object is not a string")
        })
    }

    /// Render to SVG element. [Options::format] is always `"svg"`
    pub fn render_svg_element(
        &self,
        src: String,
        options: Options,
    ) -> Result<SvgsvgElement, JsValue> {
        VizInstance::render_svg_element_raw(self, src, options).map(|res| {
            res.try_into()
                .expect_throw("Rendered object is not an svg element")
        })
    }
}
