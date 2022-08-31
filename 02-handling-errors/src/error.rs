use v8;

pub fn print_error(mut scope: v8::TryCatch<v8::HandleScope>) {
  if let Some(exception) = scope.exception() {
    let exception_string = exception.to_string(&mut scope).unwrap().to_rust_string_lossy(&mut scope);

    let message = if let Some(message) = scope.message() {
      message
    } else {
      eprintln!("{}", exception_string);
      return;
    };

    let filename = message
    .get_script_resource_name(&mut scope)
    .map_or_else(
      || "(unknown)".into(),
      |s| {
        s.to_string(&mut scope)
          .unwrap()
          .to_rust_string_lossy(&mut scope)
      },
    );
    let line_number = message.get_line_number(&mut scope).unwrap_or_default();
    let start_column = message.get_start_column();
    let end_column = message.get_end_column();

    eprintln!("{}:{}:{}: {}", filename, line_number, start_column, exception_string);

    let source_line = message
    .get_source_line(&mut scope)
    .map(|s| {
      s.to_string(&mut scope)
        .unwrap()
        .to_rust_string_lossy(&mut scope)
    })
    .unwrap();
    eprintln!("{}: {}", line_number, source_line);
    
    eprint!("{}", " ".repeat(line_number.to_string().len()+2));

    for _ in 0..start_column {
      eprint!(" ");
    }

    for _ in start_column..end_column {
      eprint!("^");
    }

    eprintln!();

    let stack_trace = if let Some(stack_trace) = scope.stack_trace() {
      stack_trace
    } else {
      return;
    };
    let stack_trace = unsafe { v8::Local::<v8::String>::cast(stack_trace) };
    let stack_trace = stack_trace
      .to_string(&mut scope)
      .map(|s| s.to_rust_string_lossy(&mut scope));

    if let Some(stack_trace) = stack_trace {
      eprintln!("{}", stack_trace);
    }
  } else {
    eprintln!("Internal error: no exception");
  }
}

// pub struct OldError {}

// impl OldError {
//   pub fn new(mut scope: v8::TryCatch<v8::HandleScope>) {
//     let exception = scope.exception().unwrap();
//     let exception_string = exception
//       .to_string(&mut scope)
//       .unwrap()
//       .to_rust_string_lossy(&mut scope);
//     let message = if let Some(message) = scope.message() {
//       message
//     } else {
//       eprintln!("{}", exception_string);
//       return;
//     };

//     let filename = message
//     .get_script_resource_name(&mut scope)
//     .map_or_else(
//       || "(unknown)".into(),
//       |s| {
//         s.to_string(&mut scope)
//           .unwrap()
//           .to_rust_string_lossy(&mut scope)
//       },
//     );
//     let line_number = message.get_line_number(&mut scope).unwrap_or_default();

//     eprintln!("{}:{}: {}", filename, line_number, exception_string);

//     let source_line = message
//     .get_source_line(&mut scope)
//     .map(|s| {
//       s.to_string(&mut scope)
//         .unwrap()
//         .to_rust_string_lossy(&mut scope)
//     })
//     .unwrap();
//     eprintln!("{}", source_line);

//     let start_column = message.get_start_column();
//     let end_column = message.get_end_column();

//     for _ in 0..start_column {
//       eprint!(" ");
//     }

//     for _ in start_column..end_column {
//       eprint!("^");
//     }

//     eprintln!();

//     // Print stack trace
//     let stack_trace = if let Some(stack_trace) = scope.stack_trace() {
//       stack_trace
//     } else {
//       return;
//     };
//     let stack_trace = unsafe { v8::Local::<v8::String>::cast(stack_trace) };
//     let stack_trace = stack_trace
//       .to_string(&mut scope)
//       .map(|s| s.to_rust_string_lossy(&mut scope));

//     if let Some(stack_trace) = stack_trace {
//       eprintln!("{}", stack_trace);
//     }
//   }
// }
