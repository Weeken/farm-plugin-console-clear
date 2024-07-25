#![deny(clippy::all)]

use farmfe_core::{
  config::{config_regex::ConfigRegex, Config, Mode},
  module::ModuleType,
  plugin::Plugin,
  serde_json::{self, Value},
  swc_ecma_ast::Program,
};

use farmfe_macro_plugin::farm_plugin;

use farmfe_toolkit::{common::PathFilter, swc_ecma_visit::FoldWith};

mod remove_console;

use serde;

#[derive(serde::Deserialize, Default, Debug)]
pub struct ClearOptions {
  exclude: Vec<ConfigRegex>,
  include: Vec<ConfigRegex>,
}

#[farm_plugin]
pub struct FarmPluginConsoleClear {
  clear_options: ClearOptions,
}

impl FarmPluginConsoleClear {
  fn new(_config: &Config, _options: String) -> Self {
    let options: Value = serde_json::from_str(&_options).unwrap_or_default();
    let mut clear_options = ClearOptions {
      exclude: vec![ConfigRegex::new("node_modules/")],
      include: vec![ConfigRegex::new("src/")],
    };
    if let Some(exclude) = options.get("exclude") {
      let e_result = exclude.as_array().unwrap();
      for e in e_result.iter() {
        clear_options
          .exclude
          .push(ConfigRegex::new(e.as_str().unwrap()))
      }
    }
    if let Some(include) = options.get("include") {
      let i_result = include.as_array().unwrap();
      for e in i_result.iter() {
        clear_options
          .include
          .push(ConfigRegex::new(e.as_str().unwrap()))
      }
    }
    Self {
      clear_options: clear_options,
    }
  }
}

impl Plugin for FarmPluginConsoleClear {
  fn name(&self) -> &str {
    "FarmPluginConsoleClear"
  }

  fn process_module(
    &self,
    _param: &mut farmfe_core::plugin::PluginProcessModuleHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<()>> {
    if matches!(_context.config.mode, Mode::Production) {
      let filter = PathFilter::new(&self.clear_options.include, &self.clear_options.exclude);
      if matches!(
        _param.module_type,
        ModuleType::Jsx | ModuleType::Tsx | ModuleType::Ts | ModuleType::Js
      ) {
        let is_filter = filter.execute(&_param.module_id.relative_path());
        if is_filter {
          let ast = _param.meta.as_script_mut().take_ast();
          let program = Program::Module(ast);
          // println!("{:?}", program);
          let program_ = program.fold_with(&mut remove_console::remove_console());
          // println!("{:?}", _param.module_id.relative_path());
          // println!("==================================");
          // println!("{:?}", program_);
          _param
            .meta
            .as_script_mut()
            .set_ast(program_.expect_module());
          return Ok(Some(()));
        }
      }
    }
    Ok(None)
  }
}
