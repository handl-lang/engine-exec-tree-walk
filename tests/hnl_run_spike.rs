//! SPIKE (ADR-0175 run layer): `lift >>> search >>> verify` as a `Term::Seq`
//! pipeline of kit-registered stage-ops, evaluated by `TreeWalkEngine`.
//!
//! Scratch evidence for `docs/reports/hnl_run_spike.md` — not shipped
//! behavior. Each "stage" is a `Foreign` op resolved through the
//! `ModuleLookup` registry (the same seam `call_host`/`import` use), with
//! the pipeline value threaded via the native `id` op (returns `incoming`).

use std::rc::Rc;

use handl_language::{Ast, AstNode, EvalSchemeDef, Language, Term, Value};
use handl_platform::{EvalContext, EvalError, Function, Module, ModuleLookup};
use handl_plugin_module::ModuleRegistry;
use hnl_exec::{EvalEngine, EvalRequest};
use hnl_exec_tree_walk::TreeWalkEngine;
use serde_json::json;

/// A pipeline stage: a host function that transforms the threaded value.
/// `lift` seeds the pipeline (ignores incoming), `search`/`verify` consume
/// and produce the threaded record.
struct StageFn {
    name: &'static str,
}

impl Function for StageFn {
    fn call(&self, args: &[Value], _ctx: &mut EvalContext) -> Result<Value, EvalError> {
        let input = args.first().cloned().unwrap_or(Value::Null);
        match self.name {
            // Seed stage: the "lift" of the script's declared input.
            "lift" => Ok(json!({
                "stage": "lift",
                "model": "demo.torch",
                "ir": {"nodes": 3}
            })),
            "search" => {
                let mut obj = input.as_object().cloned().unwrap_or_default();
                obj.insert("stage".into(), json!("search"));
                obj.insert("laws_applied".into(), json!(7));
                Ok(Value::Object(obj))
            }
            "verify" => {
                let obj = input.as_object().cloned().unwrap_or_default();
                if obj.get("laws_applied").and_then(Value::as_i64) == Some(7) {
                    obj.get("model")
                        .map(|m| json!({"stage": "verify", "ok": true, "model": m}))
                        .ok_or_else(|| EvalError::Other("verify: no model".into()))
                } else {
                    Err(EvalError::Other("verify: search did not run".into()))
                }
            }
            other => Err(EvalError::Other(format!("unknown stage {other}"))),
        }
    }
}

/// The `pipeline` module: `pipeline.lift` / `pipeline.search` / `pipeline.verify`.
struct PipelineModule;

impl Module for PipelineModule {
    fn name(&self) -> &str {
        "pipeline"
    }
    fn function(&self, name: &str) -> Option<&dyn Function> {
        // StageFn is zero-sized-per-name; keep one instance per name.
        match name {
            "lift" => Some(&StageFn { name: "lift" }),
            "search" => Some(&StageFn { name: "search" }),
            "verify" => Some(&StageFn { name: "verify" }),
            _ => None,
        }
    }
}

/// One pipeline stage as a `Term`: a foreign op whose single arg is the
/// native `id` op — `id` evaluates under the `Seq`-threaded `incoming`
/// value, so the host function receives the pipeline record as `args[0]`.
fn stage(name: &str) -> Term {
    Term::op_foreign(
        format!("pipeline.{name}"),
        vec![Term::op_native("id", vec![])],
    )
}

#[test]
fn seq_pipeline_of_foreign_stage_ops_round_trips() {
    // `lift >>> search >>> verify` — left-nested Seq, axis 0.
    let pipe = Term::seq(
        Term::seq(stage("lift"), stage("search")),
        stage("verify"),
    );

    let mut lang = Language::new();
    lang.eval
        .merge(EvalSchemeDef::new("e.pipeline", "pipeline", pipe))
        .expect("scheme merges");

    // The "script": one AST node whose kind's eval scheme IS the pipeline.
    let mut ast = Ast::new();
    ast.push(
        AstNode::new("pipeline")
            .with_payload(json!({"script": "lift >>> search >>> verify"})),
    );

    let registry: Rc<dyn ModuleLookup> =
        Rc::new(ModuleRegistry::new().with(PipelineModule));
    let engine = TreeWalkEngine::new().with_modules(registry);

    let result = engine
        .eval(&lang, &EvalRequest::new(ast))
        .expect("pipeline evals");

    assert_eq!(
        result,
        json!({"stage": "verify", "ok": true, "model": "demo.torch"})
    );
}

#[test]
fn seq_pipeline_failure_propagates() {
    // A stage that is not registered must fail loudly, not silently pass.
    let pipe = Term::seq(stage("lift"), stage("nosuch"));
    let mut lang = Language::new();
    lang.eval
        .merge(EvalSchemeDef::new("e.pipeline", "pipeline", pipe))
        .unwrap();
    let mut ast = Ast::new();
    ast.push(AstNode::new("pipeline"));

    let registry: Rc<dyn ModuleLookup> =
        Rc::new(ModuleRegistry::new().with(PipelineModule));
    let engine = TreeWalkEngine::new().with_modules(registry);
    let err = engine.eval(&lang, &EvalRequest::new(ast)).unwrap_err();
    assert!(
        format!("{err:?}").contains("nosuch") || format!("{err:?}").contains("not found"),
        "unexpected error: {err:?}"
    );
}
