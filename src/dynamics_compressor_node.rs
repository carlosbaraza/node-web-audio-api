// ---------------------------------------------------------- //
// ---------------------------------------------------------- //
//    - WARNING - DO NOT EDIT                               - //
//    - This file has been generated                        - //
// ---------------------------------------------------------- //
// ---------------------------------------------------------- //

use crate::*;
use napi::*;
use napi_derive::js_function;
use std::rc::Rc;
use web_audio_api::node::*;

pub(crate) struct NapiDynamicsCompressorNode(Rc<DynamicsCompressorNode>);

impl NapiDynamicsCompressorNode {
    pub fn create_js_class(env: &Env) -> Result<JsFunction> {
        env.define_class(
            "DynamicsCompressorNode",
            constructor,
            &[
                // Attributes
                Property::new("reduction")?
                    .with_getter(get_reduction)
                    .with_property_attributes(PropertyAttributes::Enumerable),
                // Methods

                // AudioNode interface
                Property::new("connect")?
                    .with_method(connect)
                    .with_property_attributes(PropertyAttributes::Enumerable),
                // Property::new("disconnect")?.with_method(disconnect),
            ],
        )
    }

    pub fn unwrap(&self) -> &DynamicsCompressorNode {
        &self.0
    }
}

#[js_function(2)]
fn constructor(ctx: CallContext) -> Result<JsUndefined> {
    let mut js_this = ctx.this_unchecked::<JsObject>();

    // first argument is always AudioContext
    let js_audio_context = ctx.get::<JsObject>(0)?;
    let napi_audio_context = ctx.env.unwrap::<NapiAudioContext>(&js_audio_context)?;
    let audio_context = napi_audio_context.unwrap();

    js_this.define_properties(&[
        Property::new("context")?
            .with_value(&js_audio_context)
            .with_property_attributes(PropertyAttributes::Enumerable),
        // this must be put on the instance and not in the prototype to be reachable
        Property::new("Symbol.toStringTag")?
            .with_value(&ctx.env.create_string("DynamicsCompressorNode")?)
            .with_property_attributes(PropertyAttributes::Static),
    ])?;

    let options = match ctx.try_get::<JsObject>(1)? {
        Either::A(options_js) => {
            let some_attack_js = options_js.get::<&str, JsNumber>("attack")?;
            let attack = if let Some(attack_js) = some_attack_js {
                attack_js.get_double()? as f32
            } else {
                0.003
            };

            let some_knee_js = options_js.get::<&str, JsNumber>("knee")?;
            let knee = if let Some(knee_js) = some_knee_js {
                knee_js.get_double()? as f32
            } else {
                30.
            };

            let some_ratio_js = options_js.get::<&str, JsNumber>("ratio")?;
            let ratio = if let Some(ratio_js) = some_ratio_js {
                ratio_js.get_double()? as f32
            } else {
                12.
            };

            let some_release_js = options_js.get::<&str, JsNumber>("release")?;
            let release = if let Some(release_js) = some_release_js {
                release_js.get_double()? as f32
            } else {
                0.25
            };

            let some_threshold_js = options_js.get::<&str, JsNumber>("threshold")?;
            let threshold = if let Some(threshold_js) = some_threshold_js {
                threshold_js.get_double()? as f32
            } else {
                -24.
            };

            DynamicsCompressorOptions {
                attack,
                knee,
                ratio,
                release,
                threshold,
                channel_config: ChannelConfigOptions::default(),
            }
        }
        Either::B(_) => Default::default(),
    };

    let native_node = Rc::new(DynamicsCompressorNode::new(audio_context, options));

    // AudioParam: DynamicsCompressorNode::threshold
    let native_clone = native_node.clone();
    let param_getter = ParamGetter::DynamicsCompressorNodeThreshold(native_clone);
    let napi_param = NapiAudioParam::new(param_getter);
    let mut js_obj = NapiAudioParam::create_js_object(ctx.env)?;
    ctx.env.wrap(&mut js_obj, napi_param)?;
    js_this.set_named_property("threshold", &js_obj)?;

    // AudioParam: DynamicsCompressorNode::knee
    let native_clone = native_node.clone();
    let param_getter = ParamGetter::DynamicsCompressorNodeKnee(native_clone);
    let napi_param = NapiAudioParam::new(param_getter);
    let mut js_obj = NapiAudioParam::create_js_object(ctx.env)?;
    ctx.env.wrap(&mut js_obj, napi_param)?;
    js_this.set_named_property("knee", &js_obj)?;

    // AudioParam: DynamicsCompressorNode::ratio
    let native_clone = native_node.clone();
    let param_getter = ParamGetter::DynamicsCompressorNodeRatio(native_clone);
    let napi_param = NapiAudioParam::new(param_getter);
    let mut js_obj = NapiAudioParam::create_js_object(ctx.env)?;
    ctx.env.wrap(&mut js_obj, napi_param)?;
    js_this.set_named_property("ratio", &js_obj)?;

    // AudioParam: DynamicsCompressorNode::attack
    let native_clone = native_node.clone();
    let param_getter = ParamGetter::DynamicsCompressorNodeAttack(native_clone);
    let napi_param = NapiAudioParam::new(param_getter);
    let mut js_obj = NapiAudioParam::create_js_object(ctx.env)?;
    ctx.env.wrap(&mut js_obj, napi_param)?;
    js_this.set_named_property("attack", &js_obj)?;

    // AudioParam: DynamicsCompressorNode::release
    let native_clone = native_node.clone();
    let param_getter = ParamGetter::DynamicsCompressorNodeRelease(native_clone);
    let napi_param = NapiAudioParam::new(param_getter);
    let mut js_obj = NapiAudioParam::create_js_object(ctx.env)?;
    ctx.env.wrap(&mut js_obj, napi_param)?;
    js_this.set_named_property("release", &js_obj)?;

    // finalize instance creation
    let napi_node = NapiDynamicsCompressorNode(native_node);
    ctx.env.wrap(&mut js_this, napi_node)?;

    ctx.env.get_undefined()
}

// -------------------------------------------------
// AudioNode Interface
// -------------------------------------------------
connect_method!(NapiDynamicsCompressorNode);
// disconnect_method!(NapiDynamicsCompressorNode);

// -------------------------------------------------
// GETTERS
// -------------------------------------------------

#[js_function(0)]
fn get_reduction(ctx: CallContext) -> Result<JsNumber> {
    let js_this = ctx.this_unchecked::<JsObject>();
    let napi_node = ctx.env.unwrap::<NapiDynamicsCompressorNode>(&js_this)?;
    let node = napi_node.unwrap();

    let value = node.reduction();
    ctx.env.create_double(value as f64)
}

// -------------------------------------------------
// SETTERS
// -------------------------------------------------

// -------------------------------------------------
// METHODS
// -------------------------------------------------
