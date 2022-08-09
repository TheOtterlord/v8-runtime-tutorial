use clap::{Command, arg};
use v8;
use std::io::{self, Write};

struct Runtime<'s, 'i> {
  context_scope: v8::ContextScope<'i, v8::HandleScope<'s>>,
}

impl<'s, 'i> Runtime<'s, 'i>
where
  's: 'i,
{
  pub fn new(
    isolate_scope: &'i mut v8::HandleScope<'s, ()>,
  ) -> Self {
    let global = v8::ObjectTemplate::new(isolate_scope);

    let context = v8::Context::new_from_template(isolate_scope, global);
    let context_scope = v8::ContextScope::new(isolate_scope, context);

    Runtime { context_scope }
  }

  pub fn repl(&mut self) {
    println!("My Runtime REPL (V8 {})", v8::V8::get_version());

    loop {
      print!("> ");
      io::stdout().flush().unwrap();
  
      let mut buf = String::new();
      match io::stdin().read_line(&mut buf) {
        Ok(n) => {
          if n == 0 {
            println!();
            return;
          }
  
          if let Some(result) = self.run(&buf, "(shell)") {
            println!("{}", result);
          }
        }
        Err(error) => println!("error: {}", error),
      }
    }
  }

  fn run(
    &mut self,
    script: &str,
    filename: &str,
  ) -> Option<String> {
    let scope = &mut v8::HandleScope::new(&mut self.context_scope);
    let mut scope = v8::TryCatch::new(scope);

    let filename = v8::String::new(&mut scope, filename).unwrap();
    let undefined = v8::undefined(&mut scope);
    let script = v8::String::new(&mut scope, script).unwrap();
    let origin = v8::ScriptOrigin::new(
      &mut scope,
      filename.into(),
      0,
      0,
      false,
      0,
      undefined.into(),
      false,
      false,
      false,
    );

    let script = if let Some(script) =
      v8::Script::compile(&mut scope, script, Some(&origin))
    {
      script
    } else {
      assert!(scope.has_caught());
      eprintln!("An error occured when compiling the JavaScript!");
      return None;
    };

    if let Some(result) = script.run(&mut scope) {
      return Some(result.to_string(&mut scope).unwrap().to_rust_string_lossy(&mut scope));
    } else {
      assert!(scope.has_caught());
      eprintln!("An error occured when running the JavaScript!");
      return None;
    }
  }
}

fn main() {
  let cmd = clap::Command::new("my-runtime")
  .bin_name("my-runtime")
  .subcommand_required(false)
  .subcommand(
    Command::new("run")
      .about("Run a file")
      .arg(arg!(<FILE> "The file to run"))
      .arg_required_else_help(true),
  );

  let matches = cmd.get_matches();
  match matches.subcommand() {
    Some(("run", _matches)) => unimplemented!(),
    _ => {
      let platform = v8::new_default_platform(0, false).make_shared();
      v8::V8::initialize_platform(platform);
      v8::V8::initialize();

      let isolate = &mut v8::Isolate::new(v8::CreateParams::default());
      let handle_scope = &mut v8::HandleScope::new(isolate);

      let mut runtime = Runtime::new(handle_scope);
      runtime.repl();
    },
  };
}
