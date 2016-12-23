/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

[NoInterfaceObject]
interface GlobalHelloWorld {
  readonly attribute HelloWorld hello;
};

Window implements GlobalHelloWorld;

interface HelloWorld {
  readonly attribute DOMString message;

  [Throws]
  DOMString sayToTheWorld(DOMString message);
};
