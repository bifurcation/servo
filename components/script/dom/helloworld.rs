/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::str::DOMString;
use dom::bindings::codegen::Bindings::HelloWorldBinding;
use dom::bindings::codegen::Bindings::HelloWorldBinding::HelloWorldMethods;
use dom::bindings::error::Fallible;
use dom::bindings::js::Root;
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::globalscope::GlobalScope;

#[dom_struct]
pub struct HelloWorld {
    reflector_: Reflector,
}

impl HelloWorld {
    fn new_inherited() -> HelloWorld {
        HelloWorld {
            reflector_: Reflector::new(),
        }
    }

    pub fn new(global: &GlobalScope) -> Root<HelloWorld> {
        reflect_dom_object(box HelloWorld::new_inherited(), global, HelloWorldBinding::Wrap)
    }
}

impl HelloWorldMethods for HelloWorld {
    fn Message(&self) -> DOMString {
        DOMString::from("bonjour, monde!".to_owned())
    }

    fn SayToTheWorld(&self, message: DOMString) -> Fallible<DOMString> {
        let prefix = "to the world: ".to_owned();
        let together = prefix + message.as_ref();
        println!("{}", together);
        Ok(DOMString::from(together))
    }
}
