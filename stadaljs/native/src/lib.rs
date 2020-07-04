#[macro_use]
extern crate log;

use neon::{declare_types, register_module};
use neon::prelude::*;
use neon::prelude::*;

use failure::Error;

pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
}

declare_types! {
  pub class JsUser for User {
    init(mut cx) {
      let id = cx.argument::<JsNumber>(0)?;
      let first_name: Handle<JsString> = cx.argument::<JsString>(1)?;
      let last_name: Handle<JsString> = cx.argument::<JsString>(2)?;
      let email: Handle<JsString> = cx.argument::<JsString>(3)?;

      Ok(User {
        id: id.value() as i32,
        first_name: first_name.value(),
        last_name: last_name.value(),
        email: email.value(),
      })
    }

    method get(mut cx) {
      let attr: String = cx.argument::<JsString>(0)?.value();

      let this = cx.this();

      match &attr[..] {
        "id" => {
          let id = {
            let guard = cx.lock();
            let user = this.borrow(&guard);
            user.id
          };
          Ok(cx.number(id).upcast())
        },
        "first_name" => {
          let first_name = {
            let guard = cx.lock();
            let user = this.borrow(&guard);
            user.first_name.clone()
          };
          Ok(cx.string(&first_name).upcast())
        },
        "last_name" => {
          let last_name = {
            let guard = cx.lock();
            let user = this.borrow(&guard);
            user.last_name.clone()
          };
          Ok(cx.string(&last_name).upcast())
        },
        "email" => {
          let email = {
            let guard = cx.lock();
            let user = this.borrow(&guard);
            user.email.clone()
          };
          Ok(cx.string(&email).upcast())
        },
        _ => cx.throw_type_error("property does not exist")
      }
    }

    method panic(_) {
      panic!("User.prototype.panic")
    }
  }
}

fn start(mut cx: FunctionContext) -> JsResult<JsString> {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();

        writeln!(stderr, "error: {}", e).unwrap();
        error!("error: {}", e);

        writeln!(stderr, "caused by: {}", e.as_fail()).unwrap();
        error!("error: {}", e);

        writeln!(stderr, "backtrace: {:?}", e.backtrace()).unwrap();
        error!("error: {}", e);

        ::std::process::exit(1);
    }
    Ok(cx.string("hello node"))
}

pub fn run() -> Result<(), Error> {
    Ok(())
}

register_module!(mut m, {
  m.export_function("start", start);
  m.export_class::<JsUser>("User");
  Ok(())
});

