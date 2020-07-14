use neon::prelude::*;

pub fn new_error(mut cx: FunctionContext) -> JsResult<JsError> {
    let msg = cx.argument::<JsString>(0)?.value(&mut cx);

    cx.error(msg)
}

pub fn new_type_error(mut cx: FunctionContext) -> JsResult<JsError> {
    let msg = cx.argument::<JsString>(0)?.value(&mut cx);

    cx.type_error(msg)
}

pub fn new_range_error(mut cx: FunctionContext) -> JsResult<JsError> {
    let msg = cx.argument::<JsString>(0)?.value(&mut cx);

    cx.range_error(msg)
}

pub fn throw_error(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let msg = cx.argument::<JsString>(0)?.value(&mut cx);

    cx.throw_error(msg)
}
